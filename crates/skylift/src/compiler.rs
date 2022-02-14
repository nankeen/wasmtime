use crate::RemoteId;
use crate::{
    convert::internal2rpc::from_build_module_req, skylift_grpc::compiler_client::CompilerClient,
};
use anyhow::Result;
use cranelift_codegen::ir::MemFlags;
use cranelift_codegen::print_errors::pretty_error;
use cranelift_codegen::{binemit, settings, Context};
use cranelift_codegen::{
    ir::{self, ExternalName, InstBuilder},
    isa::TargetIsa,
};
use cranelift_wasm::{
    DefinedFuncIndex, FuncTranslator, FunctionBuilder, SignatureIndex, WasmFuncType,
};
use object::write::Object;
use std::{
    any::Any,
    cmp,
    collections::BTreeMap,
    fmt, mem,
    sync::{Arc, Mutex},
};
use tokio::runtime::Runtime;
use tonic::{codegen::InterceptedService, transport::Channel, Request};
use tracing::instrument;
use wasmtime_cranelift::{
    blank_sig, indirect_signature, value_type, wasmtime_call_conv, CompiledFunction, ObjectBuilder,
    TrampolineRelocSink,
};
use wasmtime_environ::{
    CompileError, EntityRef, FlagValue, FunctionBodyData, FunctionInfo, Module, ModuleTranslation,
    PrimaryMap, Trampoline, Tunables, TypeTables,
};

/// [`Compiler`] implements `wasmtime_environ::Compiler`.
///
/// It is a thin wrapper on top of tonic gRPC client specifically for the
/// `Compiler` service.
pub struct Compiler {
    /// `client` - Handler for client session, according to tonic specs this should
    /// be cheap to clone as the underlying implementation uses mpsc channel.
    client: CompilerClient<InterceptedService<Channel, RemoteId>>,
    runtime: Arc<Runtime>,
    isa: Box<dyn TargetIsa>,
    translators: Mutex<Vec<FuncTranslator>>,
}

impl Compiler {
    #[instrument(skip(isa))]
    pub(crate) fn new(
        mut client: CompilerClient<InterceptedService<Channel, RemoteId>>,
        runtime: Arc<Runtime>,
        isa: Box<dyn TargetIsa>,
    ) -> Self {
        let remote_flags: BTreeMap<String, FlagValue> = bincode::deserialize(
            &runtime
                .block_on(client.get_flags(Request::new(())))
                .unwrap()
                .into_inner()
                .flags
                .expect("could not get flags")
                .value,
        )
        .expect("could not deserialize flag");

        let remote_isa_flags: BTreeMap<String, FlagValue> = bincode::deserialize(
            &runtime
                .block_on(client.get_isa_flags(Request::new(())))
                .unwrap()
                .into_inner()
                .flags
                .expect("could not get isa flags")
                .value,
        )
        .expect("could not deserialize isa flag");

        let local_flags: BTreeMap<_, _> = isa
            .flags()
            .iter()
            .map(|val| (val.name.to_string(), to_flag_value(&val)))
            .collect();

        let local_isa_flags: BTreeMap<_, _> = isa
            .isa_flags()
            .iter()
            .map(|val| (val.name.to_string(), to_flag_value(&val)))
            .collect();

        // Assert local and remote flags are identical
        assert!(remote_flags == local_flags);
        assert!(remote_isa_flags == local_isa_flags);

        Self {
            client,
            runtime,
            isa,
            translators: Default::default(),
        }
    }

    #[instrument]
    fn take_translator(&self) -> FuncTranslator {
        let candidate = self.translators.lock().unwrap().pop();
        candidate.unwrap_or_else(FuncTranslator::new)
    }

    #[instrument(skip(translator))]
    fn save_translator(&self, translator: FuncTranslator) {
        self.translators.lock().unwrap().push(translator);
    }

    #[instrument]
    fn host_to_wasm_trampoline(&self, ty: &WasmFuncType) -> Result<CompiledFunction, CompileError> {
        let isa = &*self.isa;
        let value_size = mem::size_of::<u128>();
        let pointer_type = isa.pointer_type();

        // The wasm signature we're calling in this trampoline has the actual
        // ABI of the function signature described by `ty`
        let wasm_signature = indirect_signature(isa, ty);

        // The host signature has the `VMTrampoline` signature where the ABI is
        // fixed.
        let mut host_signature = blank_sig(isa, wasmtime_call_conv(isa));
        host_signature.params.push(ir::AbiParam::new(pointer_type));
        host_signature.params.push(ir::AbiParam::new(pointer_type));

        let mut func_translator = self.take_translator();
        let mut context = Context::new();
        context.func = ir::Function::with_name_signature(ExternalName::user(0, 0), host_signature);

        // This trampoline will load all the parameters from the `values_vec`
        // that is passed in and then call the real function (also passed
        // indirectly) with the specified ABI.
        //
        // All the results are then stored into the same `values_vec`.
        let mut builder = FunctionBuilder::new(&mut context.func, func_translator.context());
        let block0 = builder.create_block();

        builder.append_block_params_for_function_params(block0);
        builder.switch_to_block(block0);
        builder.seal_block(block0);

        let (vmctx_ptr_val, caller_vmctx_ptr_val, callee_value, values_vec_ptr_val) = {
            let params = builder.func.dfg.block_params(block0);
            (params[0], params[1], params[2], params[3])
        };

        // Load the argument values out of `values_vec`.
        let mflags = ir::MemFlags::trusted();
        let callee_args = wasm_signature
            .params
            .iter()
            .enumerate()
            .map(|(i, r)| {
                match i {
                    0 => vmctx_ptr_val,
                    1 => caller_vmctx_ptr_val,
                    _ =>
                    // i - 2 because vmctx and caller vmctx aren't passed through `values_vec`.
                    {
                        builder.ins().load(
                            r.value_type,
                            mflags,
                            values_vec_ptr_val,
                            ((i - 2) * value_size) as i32,
                        )
                    }
                }
            })
            .collect::<Vec<_>>();

        // Call the indirect function pointer we were given
        let new_sig = builder.import_signature(wasm_signature);
        let call = builder
            .ins()
            .call_indirect(new_sig, callee_value, &callee_args);
        let results = builder.func.dfg.inst_results(call).to_vec();

        // Store the return values into `values_vec`.
        let mflags = ir::MemFlags::trusted();
        for (i, r) in results.iter().enumerate() {
            builder
                .ins()
                .store(mflags, *r, values_vec_ptr_val, (i * value_size) as i32);
        }
        builder.ins().return_(&[]);
        builder.finalize();

        let func = self.finish_trampoline(context, isa)?;
        self.save_translator(func_translator);
        Ok(func)
    }

    #[instrument]
    fn wasm_to_host_trampoline(
        &self,
        ty: &WasmFuncType,
        host_fn: usize,
    ) -> Result<CompiledFunction, CompileError> {
        let isa = &*self.isa;
        let pointer_type = isa.pointer_type();
        let wasm_signature = indirect_signature(isa, ty);
        // The host signature has an added parameter for the `values_vec` input
        // and output.
        let mut host_signature = blank_sig(isa, wasmtime_call_conv(isa));
        host_signature.params.push(ir::AbiParam::new(pointer_type));

        // Compute the size of the values vector. The vmctx and caller vmctx are passed separately.
        let value_size = mem::size_of::<u128>();
        let values_vec_len = (value_size * cmp::max(ty.params().len(), ty.returns().len())) as u32;

        let mut context = Context::new();
        context.func =
            ir::Function::with_name_signature(ir::ExternalName::user(0, 0), wasm_signature);

        let ss = context.func.create_stack_slot(ir::StackSlotData::new(
            ir::StackSlotKind::ExplicitSlot,
            values_vec_len,
        ));

        let mut func_translator = self.take_translator();
        let mut builder = FunctionBuilder::new(&mut context.func, func_translator.context());
        let block0 = builder.create_block();

        builder.append_block_params_for_function_params(block0);
        builder.switch_to_block(block0);
        builder.seal_block(block0);

        let values_vec_ptr_val = builder.ins().stack_addr(pointer_type, ss, 0);
        let mflags = MemFlags::trusted();
        for i in 0..ty.params().len() {
            let val = builder.func.dfg.block_params(block0)[i + 2];
            builder
                .ins()
                .store(mflags, val, values_vec_ptr_val, (i * value_size) as i32);
        }

        let block_params = builder.func.dfg.block_params(block0);
        let vmctx_ptr_val = block_params[0];
        let caller_vmctx_ptr_val = block_params[1];

        let callee_args = vec![vmctx_ptr_val, caller_vmctx_ptr_val, values_vec_ptr_val];

        let new_sig = builder.import_signature(host_signature);

        let callee_value = builder.ins().iconst(pointer_type, host_fn as i64);
        builder
            .ins()
            .call_indirect(new_sig, callee_value, &callee_args);

        let mflags = MemFlags::trusted();
        let mut results = Vec::new();
        for (i, r) in ty.returns().iter().enumerate() {
            let load = builder.ins().load(
                value_type(isa, *r),
                mflags,
                values_vec_ptr_val,
                (i * value_size) as i32,
            );
            results.push(load);
        }
        builder.ins().return_(&results);
        builder.finalize();

        let func = self.finish_trampoline(context, isa)?;
        self.save_translator(func_translator);
        Ok(func)
    }

    #[instrument(skip(isa, context))]
    fn finish_trampoline(
        &self,
        mut context: Context,
        isa: &dyn TargetIsa,
    ) -> Result<CompiledFunction, CompileError> {
        let mut code_buf = Vec::new();
        let mut reloc_sink = TrampolineRelocSink::default();
        let mut trap_sink = binemit::NullTrapSink {};
        let mut stack_map_sink = binemit::NullStackMapSink {};
        context
            .compile_and_emit(
                isa,
                &mut code_buf,
                &mut reloc_sink,
                &mut trap_sink,
                &mut stack_map_sink,
            )
            .map_err(|error| {
                CompileError::Codegen(pretty_error(&context.func, Some(isa), error))
            })?;

        let unwind_info = context.create_unwind_info(isa).map_err(|error| {
            CompileError::Codegen(pretty_error(&context.func, Some(isa), error))
        })?;

        Ok(CompiledFunction {
            body: code_buf,
            jt_offsets: context.func.jt_offsets,
            unwind_info,
            relocations: reloc_sink.relocs,
            stack_slots: Default::default(),
            value_labels_ranges: Default::default(),
            info: Default::default(),
            address_map: Default::default(),
            traps: Vec::new(),
        })
    }
}

impl wasmtime_environ::Compiler for Compiler {
    fn compile_function(
        &self,
        _translation: &ModuleTranslation<'_>,
        _index: DefinedFuncIndex,
        _data: FunctionBodyData<'_>,
        _tunables: &Tunables,
        _types: &TypeTables,
    ) -> Result<Box<dyn Any + Send>, CompileError> {
        unimplemented!("compile_function should not be used with remote compiler")
    }

    fn emit_obj(
        &self,
        _module: &ModuleTranslation,
        _types: &TypeTables,
        _funcs: PrimaryMap<DefinedFuncIndex, Box<dyn Any + Send>>,
        _emit_dwarf: bool,
        _obj: &mut Object,
    ) -> Result<(PrimaryMap<DefinedFuncIndex, FunctionInfo>, Vec<Trampoline>)> {
        unimplemented!("emit_obj should not be used with remote compiler")
    }

    #[instrument]
    fn emit_trampoline_obj(
        &self,
        ty: &WasmFuncType,
        host_fn: usize,
        obj: &mut Object,
    ) -> Result<(Trampoline, Trampoline)> {
        let host_to_wasm = self.host_to_wasm_trampoline(ty)?;
        let wasm_to_host = self.wasm_to_host_trampoline(ty, host_fn)?;
        let module = Module::new();
        let mut builder = ObjectBuilder::new(obj, &module, &*self.isa);
        let a = builder.trampoline(SignatureIndex::new(0), &host_to_wasm);
        let b = builder.trampoline(SignatureIndex::new(1), &wasm_to_host);
        builder.unwind_info();
        builder.finish()?;
        Ok((a, b))
    }

    #[instrument]
    fn triple(&self) -> &target_lexicon::Triple {
        self.isa.triple()
    }

    #[instrument]
    fn flags(&self) -> BTreeMap<String, FlagValue> {
        self.isa
            .flags()
            .iter()
            .map(|val| (val.name.to_string(), to_flag_value(&val)))
            .collect()
    }

    #[instrument]
    fn isa_flags(&self) -> BTreeMap<String, FlagValue> {
        self.isa
            .isa_flags()
            .iter()
            .map(|val| (val.name.to_string(), to_flag_value(val)))
            .collect()
    }

    #[instrument(skip(tunables, wasm, features, paged_memory_initialization))]
    fn build_module(
        &self,
        wasm: &[u8],
        tunables: &Tunables,
        features: &wasmparser::WasmFeatures,
        paged_memory_initialization: bool,
    ) -> Result<Vec<u8>> {
        let mut client = self.client.clone();
        let request = from_build_module_req(wasm, tunables, features, paged_memory_initialization);
        let resp = self
            .runtime
            .block_on(client.build_module(Request::new(request)))?
            .into_inner()
            .serialized_module
            .ok_or_else(|| anyhow::Error::msg("could not find serialized module"))?;
        Ok(resp.value)
    }
}

impl fmt::Debug for Compiler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("Skylift Compiler")
            .field("target", self.isa.triple())
            .finish()
    }
}

fn to_flag_value(v: &settings::Value) -> FlagValue {
    match v.kind() {
        settings::SettingKind::Enum => FlagValue::Enum(v.as_enum().unwrap().into()),
        settings::SettingKind::Num => FlagValue::Num(v.as_num().unwrap()),
        settings::SettingKind::Bool => FlagValue::Bool(v.as_bool().unwrap()),
        settings::SettingKind::Preset => unreachable!(),
    }
}
