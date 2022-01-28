use super::CompilerSession;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use skylift::{
    convert::{internal2rpc, rpc2internal},
    skylift_grpc::{
        compiler_server::Compiler, BuildModuleRequest, BuildModuleResponse, BuildResponse,
        EnableRequest, FlagMap, NewBuilderResponse, SetRequest, SettingsResponse, Triple,
    },
    RemoteId, REMOTE_ID_HEADER,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};
use tracing::{instrument, trace};
use wasmtime_environ::ModuleEnvironment;
use wasmtime_jit::TypeTables;

#[derive(Default)]
pub(crate) struct CompilerService {
    sessions: RwLock<HashMap<RemoteId, Arc<RwLock<CompilerSession>>>>,
}

fn get_remote_id<T>(req: &Request<T>) -> Result<RemoteId, Status> {
    // Retrieve builder id from metadata (headers)
    Ok(req
        .metadata()
        .get(REMOTE_ID_HEADER)
        .map(|id_meta| id_meta.to_str().ok())
        .flatten()
        .ok_or_else(|| Status::failed_precondition("invalid remote id"))?
        .into())
}

impl CompilerService {
    async fn get_session(
        &self,
        remote_id: &RemoteId,
    ) -> Result<Arc<RwLock<CompilerSession>>, Status> {
        // Acquire session and set target
        Ok(self
            .sessions
            .read()
            .await
            .get(remote_id)
            .ok_or_else(|| Status::failed_precondition("invalid remote id"))?
            .clone())
    }
}

#[tonic::async_trait]
impl Compiler for CompilerService {
    async fn new_builder(&self, _req: Request<()>) -> Result<Response<NewBuilderResponse>, Status> {
        let id = RemoteId::new();
        let mut sessions = self.sessions.write().await;
        let response = Response::new(NewBuilderResponse {
            remote_id: id.to_string(),
        });

        sessions.insert(
            id,
            Arc::new(RwLock::new(CompilerSession::Build(
                wasmtime_cranelift::builder(),
            ))),
        );
        Ok(response)
    }

    async fn set_target(&self, req: Request<Triple>) -> Result<Response<()>, Status> {
        let triple = rpc2internal::from_triple(req.get_ref())
            .ok_or_else(|| Status::invalid_argument("bad triple provided"))?;

        // Retrieve builder id from metadata (headers)
        let session_lock = self.get_session(&get_remote_id(&req)?).await?;
        let mut session = session_lock.write().await;

        session.map_builder_mut(|builder| {
            builder
                .target(triple)
                .map_err(|e| Status::internal(e.to_string()))
                .map(Response::new)
        })?
    }

    async fn get_triple(&self, req: Request<()>) -> Result<Response<Triple>, Status> {
        // Retrieve builder id from metadata (headers)
        let session_lock = self.get_session(&get_remote_id(&req)?).await?;
        let session = session_lock.read().await;

        session.map_builder(|builder| Response::new(internal2rpc::from_triple(builder.triple())))
    }

    async fn set_settings(&self, req: Request<SetRequest>) -> Result<Response<()>, Status> {
        let settings = req.get_ref();

        // Retrieve builder id from metadata (headers)
        let session_lock = self.get_session(&get_remote_id(&req)?).await?;
        let mut session = session_lock.write().await;

        session.map_builder_mut(|builder| {
            builder
                .set(&settings.name, &settings.value)
                .map_err(|e| Status::internal(e.to_string()))
                .map(Response::new)
        })?
    }

    async fn enable_settings(&self, _req: Request<EnableRequest>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented"))
    }

    async fn get_settings(&self, _req: Request<()>) -> Result<Response<SettingsResponse>, Status> {
        Err(Status::unimplemented("not implemented"))
    }

    #[instrument(skip_all)]
    async fn build(&self, req: Request<()>) -> Result<Response<BuildResponse>, Status> {
        // Get the builder to build a compiler according to the settings so far
        let remote_id = get_remote_id(&req)?;
        trace!("building remote compiler with remote id: {:?}", remote_id);
        let session_lock = self.get_session(&remote_id).await?;
        let mut session = session_lock.write().await;

        let compiler = session.map_builder_mut(|builder| builder.build())?;

        let response = Response::new(BuildResponse {
            remote_id: remote_id.to_string(),
        });

        *session = CompilerSession::Compile(compiler);
        trace!("build end");

        Ok(response)
    }

    #[instrument(skip_all)]
    async fn build_module(
        &self,
        req: Request<BuildModuleRequest>,
    ) -> Result<Response<BuildModuleResponse>, Status> {
        // Require tunables, features, paged_memory_initialization
        trace!("building module");
        let wasm = &req.get_ref().wasm;
        let tunables = req
            .get_ref()
            .tunables
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing tunables argument"))
            .map(rpc2internal::from_tunables)?
            .ok_or_else(|| Status::invalid_argument("could not deserialize tunable argument"))?;
        let features = req
            .get_ref()
            .features
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("missing features argument"))
            .map(rpc2internal::from_wasm_features)?;
        let paged_memory_initialization = req.get_ref().paged_memory_initialization;

        // First a `ModuleEnvironment` is created which records type information
        // about the wasm module. This is where the WebAssembly is parsed and
        // validated. Afterwards `types` will have all the type information for
        // this module.
        let (_, translations, types) = ModuleEnvironment::new(&tunables, &features)
            .translate(&wasm)
            .map_err(|_| Status::invalid_argument("failed to parse WebAssembly module"))?;

        // Obtain session
        self.get_session(&get_remote_id(&req)?)
            .await?
            .read()
            .await
            .map_compiler(|compiler| -> anyhow::Result<_> {
                // Perform a two-level map/reduce here to get the final list of
                // compilation artifacts. The first level of map/reduce maps over all
                // modules found and reduces to collection into a vector. The second
                // level of map/reduce here maps over all functions within each wasm
                // module found and collects into an ELF image via `emit_obj`.
                let artifacts = translations
                    .into_par_iter()
                    .map(|mut translation| -> anyhow::Result<_> {
                        let functions = std::mem::take(&mut translation.function_body_inputs);
                        let functions = functions.into_iter().collect::<Vec<_>>();

                        let funcs = functions
                            .into_par_iter()
                            .map(|(index, func)| {
                                compiler.compile_function(
                                    &translation,
                                    index,
                                    func,
                                    &tunables,
                                    &types,
                                )
                            })
                            .collect::<Result<Vec<_>, _>>()?
                            .into_iter()
                            .collect();

                        let mut obj = compiler.object()?;
                        let (funcs, trampolines) = compiler.emit_obj(
                            &translation,
                            &types,
                            funcs,
                            tunables.generate_native_debuginfo,
                            &mut obj,
                        )?;

                        // If configured, attempt to use paged memory initialization
                        // instead of the default mode of memory initialization
                        if paged_memory_initialization {
                            translation.try_paged_init();
                        }

                        let (mmap, _) = wasmtime_jit::finish_compile(
                            translation,
                            obj,
                            funcs,
                            trampolines,
                            &tunables,
                        )?;
                        Ok(mmap)
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                let types = TypeTables {
                    wasm_signatures: types.wasm_signatures,
                    module_signatures: types.module_signatures,
                    instance_signatures: types.instance_signatures,
                };
                // artifacts: impl IntoIterator<Item = &'a MmapVec>,
                // target: &str,
                // shared_flags: BTreeMap<String, FlagValue>,
                // isa_flags: BTreeMap<String, FlagValue>,
                // tunables: Tunables,
                // features: &wasmparser::WasmFeatures,
                // types: &'a TypeTables,
                Ok(Response::new(BuildModuleResponse {
                    serialized_module: Some(prost_types::Any {
                        value: wasmtime::SerializedModule::from_raw(
                            &artifacts,
                            &compiler.triple().to_string(),
                            compiler.flags(),
                            compiler.isa_flags(),
                            tunables,
                            &features,
                            &types,
                        )
                        .to_bytes(&wasmtime::ModuleVersionStrategy::WasmtimeVersion)?,
                        ..Default::default()
                    }),
                }))
            })?
            .map_err(|msg| Status::internal(format!("compilation failed {}", msg)))
    }

    async fn get_flags(&self, req: Request<()>) -> Result<Response<FlagMap>, Status> {
        // Retrieve builder id from metadata (headers)
        let session_lock = self.get_session(&get_remote_id(&req)?).await?;
        let session = session_lock.read().await;

        session
            .map_compiler(|compiler| Response::new(internal2rpc::from_flag_map(&compiler.flags())))
    }

    async fn get_isa_flags(&self, req: Request<()>) -> Result<Response<FlagMap>, Status> {
        // Retrieve builder id from metadata (headers)
        let session_lock = self.get_session(&get_remote_id(&req)?).await?;
        let session = session_lock.read().await;

        session.map_compiler(|compiler| {
            Response::new(internal2rpc::from_flag_map(&compiler.isa_flags()))
        })
    }
}
