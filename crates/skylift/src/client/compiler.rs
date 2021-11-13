use crate::RemoteId;
use crate::{convert::internal2rpc, skylift_grpc::compiler_client::CompilerClient};
use anyhow::Result;
use cranelift_wasm::{DefinedFuncIndex, WasmFuncType};
use object::write::Object;
use std::{any::Any, collections::BTreeMap, sync::Arc};
use tokio::{runtime::Runtime, sync::RwLock};
use tonic::{codegen::InterceptedService, transport::Channel, Request};
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
    translation_set: Arc<RwLock<bool>>,
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
            cache: CompilerCache {
                triple: Some(triple),
            },
        }
    }

    async fn set_translation(&self, translation: &ModuleTranslation<'_>) -> Result<()> {
        if !*self.translation_set.read().await {
            let mut client = self.client.clone();
            client
                .set_translation(Request::new(internal2rpc::from_module_translation(
                    translation,
                )))
                .await?;

            *self.translation_set.write().await = true;
        }
        Ok(())
    }
}

impl wasmtime_environ::Compiler for Compiler {
    fn compile_function(
        &self,
        translation: &ModuleTranslation<'_>,
        _index: DefinedFuncIndex,
        _data: FunctionBodyData<'_>,
        _tunables: &Tunables,
        _types: &TypeTables,
    ) -> Result<Box<dyn Any + Send>, CompileError> {
        self.runtime
            .block_on(async move {
                self.set_translation(translation).await?;
                Ok::<_, anyhow::Error>(())
            })
            .map_err(|err| CompileError::Codegen(err.to_string()))?;
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
