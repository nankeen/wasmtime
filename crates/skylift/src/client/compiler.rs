use crate::RemoteId;
use crate::{
    convert::{internal2rpc, rpc2internal},
    skylift_grpc::compiler_client::CompilerClient,
};
use anyhow::{Error, Result};
use cranelift_wasm::{DefinedFuncIndex, WasmFuncType};
use object::write::Object;
use std::{any::Any, collections::BTreeMap, sync::Arc};
use tokio::{runtime::Runtime, sync::RwLock};
use tonic::{codegen::InterceptedService, transport::Channel, Request};
use wasmtime_environ::{
    CompileError, FlagValue, FunctionBodyData, FunctionInfo, ModuleTranslation, PrimaryMap,
    Trampoline, Tunables, TypeTables,
};

/// [`Compiler`] implements `wasmtime_environ::Compiler`.
///
/// It is a thin wrapper on top of tonic gRPC client specifically for the
/// `Compiler` service.
#[derive(Clone)]
pub struct Compiler {
    /// `client` - Handler for client session, according to tonic specs this should
    /// be cheap to clone as the underlying implementation uses mpsc channel.
    client: CompilerClient<InterceptedService<Channel, RemoteId>>,
    runtime: Arc<Runtime>,
    translation_set: Arc<RwLock<bool>>,
    triple: target_lexicon::Triple,
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
            translation_set: Arc::new(RwLock::new(false)),
            triple,
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
        unimplemented!("compile_function should not be used on skylift mode")
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
        &self.triple
    }

    fn flags(&self) -> BTreeMap<String, FlagValue> {
        let mut client = self.client.clone();
        let flags = self
            .runtime
            .block_on(client.get_flags(Request::new(())))
            .unwrap()
            .into_inner();
        bincode::deserialize(&flags.flags.expect("could not get flags").value).unwrap()
    }

    fn isa_flags(&self) -> BTreeMap<String, FlagValue> {
        let mut client = self.client.clone();
        let flags = self
            .runtime
            .block_on(client.get_isa_flags(Request::new(())))
            .unwrap()
            .into_inner();
        bincode::deserialize(&flags.flags.expect("could not get isa flags").value).unwrap()
    }
}
