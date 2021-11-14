use super::CompilerSession;
use crate::{
    convert::{internal2rpc, rpc2internal},
    skylift_grpc::{
        compiler_server::Compiler, BuildResponse, CompileFunctionRequest, CompiledFunction,
        EnableRequest, FlagMap, ModuleTranslation, NewBuilderResponse, SetRequest,
        SettingsResponse, Triple,
    },
    RemoteId, REMOTE_ID_HEADER,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};

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

    async fn build(&self, req: Request<()>) -> Result<Response<BuildResponse>, Status> {
        // Get the builder to build a compiler according to the settings so far
        let remote_id = get_remote_id(&req)?;
        let session_lock = self.get_session(&remote_id).await?;
        let mut session = session_lock.write().await;

        let compiler = session.map_builder_mut(|builder| builder.build())?;

        let response = Response::new(BuildResponse {
            remote_id: remote_id.to_string(),
        });

        *session = CompilerSession::Compile {
            compiler,
            module_translation: None,
        };

        Ok(response)
    }

    async fn set_translation(
        &self,
        req: Request<ModuleTranslation>,
    ) -> Result<Response<()>, Status> {
        // Retrieve builder id from metadata (headers)
        let session_lock = self.get_session(&get_remote_id(&req)?).await?;
        let mut session = session_lock.write().await;

        session.map_compiler_mut(|_, module_translation| {
            let new_translation = req.into_inner();
            *module_translation = Some(Box::new(
                rpc2internal::from_module_translation(new_translation)
                    .ok_or_else(|| Status::invalid_argument("bad module provided"))?,
            ));
            Ok::<_, Status>(Response::new(()))
        })?
    }

    async fn get_flags(&self, req: Request<()>) -> Result<Response<FlagMap>, Status> {
        // Retrieve builder id from metadata (headers)
        let session_lock = self.get_session(&get_remote_id(&req)?).await?;
        let session = session_lock.read().await;

        session.map_compiler(|compiler, _| {
            Response::new(internal2rpc::from_flag_map(&compiler.flags()))
        })
    }

    async fn get_isa_flags(&self, req: Request<()>) -> Result<Response<FlagMap>, Status> {
        // Retrieve builder id from metadata (headers)
        let session_lock = self.get_session(&get_remote_id(&req)?).await?;
        let session = session_lock.read().await;

        session.map_compiler(|compiler, _| {
            Response::new(internal2rpc::from_flag_map(&compiler.isa_flags()))
        })
    }

    async fn compile_function(
        &self,
        req: Request<CompileFunctionRequest>,
    ) -> Result<Response<CompiledFunction>, Status> {
        // Retrieve builder id from metadata (headers)
        let session_lock = self.get_session(&get_remote_id(&req)?).await?;
        let session = session_lock.read().await;

        session.map_compiler(|compiler, translation| {
            let (index, data, tunables, types) =
                rpc2internal::from_compile_function_request(req.into_inner()).ok_or_else(|| {
                    Status::invalid_argument("bad compiler request provided, could not deserialize")
                })?;

            compiler
                .compile_function(
                    &*translation
                        .as_ref()
                        .ok_or_else(|| Status::failed_precondition("translation not defined"))?,
                    index,
                    data,
                    &tunables,
                    &types,
                )
                .map_err(|err| Status::internal(err.to_string()))?
                .downcast_ref::<wasmtime_cranelift::CompiledFunction>()
                .ok_or_else(|| Status::internal("could not downcast any to CompiledFunction"))
                .map(internal2rpc::from_compiled_function)
                .map(Response::new)
        })?
    }
}
