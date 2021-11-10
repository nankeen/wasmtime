use crate::skylift_grpc::compiler_client::CompilerClient;
use crate::RemoteId;
use anyhow::Result;
use cranelift_wasm::{DefinedFuncIndex, WasmFuncType};
use object::write::Object;
use std::{any::Any, collections::BTreeMap, sync::Arc};
use tokio::runtime::Runtime;
use tonic::{codegen::InterceptedService, transport::Channel};
use wasmtime_environ::{
    CompileError, FlagValue, FunctionBodyData, FunctionInfo, ModuleTranslation, PrimaryMap,
    Trampoline, Tunables, TypeTables,
};

#[derive(Default, Clone)]
struct CompilerCache {
    pub triple: Option<target_lexicon::Triple>,
}

/// [`Compiler`] implements `wasmtime_environ::Compiler`.
///
/// It is a thin wrapper on top of tonic gRPC client specifically for the
/// `Compiler` service.
#[derive(Clone)]
pub struct Compiler {
    cache: CompilerCache,
    /// `client` - Handler for client session, according to tonic specs this should
    /// be cheap to clone as the underlying implementation uses mpsc channel.
    client: CompilerClient<InterceptedService<Channel, RemoteId>>,
    runtime: Arc<Runtime>,
}

impl Compiler {
    pub(crate) fn new(
        client: CompilerClient<InterceptedService<Channel, RemoteId>>,
        runtime: Arc<Runtime>,
        triple: target_lexicon::Triple,
    ) -> Self {
        Self {
            client,
            runtime,
            cache: CompilerCache {
                triple: Some(triple)
            }
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
        unimplemented!("compile_function is not implemented")
    }

    fn emit_obj(
        &self,
        _module: &ModuleTranslation,
        _types: &TypeTables,
        _funcs: PrimaryMap<DefinedFuncIndex, Box<dyn Any + Send>>,
        _emit_dwarf: bool,
        _obj: &mut Object,
    ) -> Result<(PrimaryMap<DefinedFuncIndex, FunctionInfo>, Vec<Trampoline>)> {
        unimplemented!("emit_obj is not implemented")
    }

    fn emit_trampoline_obj(
        &self,
        _ty: &WasmFuncType,
        _host_fn: usize,
        _obj: &mut Object,
    ) -> Result<(Trampoline, Trampoline)> {
        unimplemented!("emit_trampoline_obj is not implemented")
    }

    fn triple(&self) -> &target_lexicon::Triple {
        self.cache.triple.as_ref().unwrap()
    }

    fn flags(&self) -> BTreeMap<String, FlagValue> {
        unimplemented!("flags is not implemented")
    }

    fn isa_flags(&self) -> BTreeMap<String, FlagValue> {
        unimplemented!("isa_flags is not implemented")
    }
}
