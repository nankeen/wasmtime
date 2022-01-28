use crate::RemoteId;
use crate::{
    convert::internal2rpc::from_build_module_req, skylift_grpc::compiler_client::CompilerClient,
};
use anyhow::Result;
use cranelift_wasm::{DefinedFuncIndex, WasmFuncType};
use object::write::Object;
use std::{any::Any, collections::BTreeMap, fmt, sync::Arc};
use tokio::runtime::Runtime;
use tonic::{codegen::InterceptedService, transport::Channel, Request};
use tracing::{instrument, trace};
use wasmtime_environ::{
    CompileError, FlagValue, FunctionBodyData, FunctionInfo, ModuleTranslation, PrimaryMap,
    Trampoline, Tunables, TypeTables,
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
    triple: target_lexicon::Triple,
    flags: BTreeMap<String, FlagValue>,
    isa_flags: BTreeMap<String, FlagValue>,
}

impl Compiler {
    pub(crate) fn new(
        mut client: CompilerClient<InterceptedService<Channel, RemoteId>>,
        runtime: Arc<Runtime>,
        triple: target_lexicon::Triple,
    ) -> Self {
        let flags = bincode::deserialize(
            &runtime
                .block_on(client.get_flags(Request::new(())))
                .unwrap()
                .into_inner()
                .flags
                .expect("could not get flags")
                .value,
        )
        .expect("could not deserialize flag");

        let isa_flags = bincode::deserialize(
            &runtime
                .block_on(client.get_isa_flags(Request::new(())))
                .unwrap()
                .into_inner()
                .flags
                .expect("could not get isa flags")
                .value,
        )
        .expect("could not deserialize isa flag");

        Self {
            client,
            runtime,
            triple,
            flags,
            isa_flags,
        }
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

    fn emit_trampoline_obj(
        &self,
        _ty: &WasmFuncType,
        _host_fn: usize,
        _obj: &mut Object,
    ) -> Result<(Trampoline, Trampoline)> {
        unimplemented!("emit_trampoline_obj should not be used with remote compiler")
    }

    #[instrument]
    fn triple(&self) -> &target_lexicon::Triple {
        &self.triple
    }

    #[instrument]
    fn flags(&self) -> BTreeMap<String, FlagValue> {
        self.flags.clone()
    }

    #[instrument]
    fn isa_flags(&self) -> BTreeMap<String, FlagValue> {
        self.isa_flags.clone()
    }

    #[instrument(
        level = "info",
        skip(tunables, wasm, features, paged_memory_initialization)
    )]
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
            .field("target", &self.triple)
            .finish()
    }
}
