use crate::{
    convert::{internal2rpc, rpc2internal},
    skylift_grpc::{
        compiler_server::Compiler, BuildResponse, Empty, EnableRequest, ModuleTranslation,
        NewBuilderResponse, SetRequest, SettingsResponse, Triple,
    },
    RemoteId, REMOTE_ID_HEADER,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

enum CompilerSession {
    Build(Box<dyn wasmtime_environ::CompilerBuilder>),
    Compile {
        compiler: Box<dyn wasmtime_environ::Compiler>,
        module_translation: wasmtime_environ::ModuleTranslation<'static>,
    },
}

#[derive(Default)]
pub(crate) struct CompilerService {
    sessions: Mutex<HashMap<RemoteId, Arc<Mutex<CompilerSession>>>>,
    // builders: Mutex<HashMap<RemoteId, SessionBuilder>>,
    // compilers: Mutex<HashMap<RemoteId, SessionCompiler>>,
}

#[tonic::async_trait]
impl Compiler for CompilerService {
    async fn new_builder(
        &self,
        _req: Request<Empty>,
    ) -> Result<Response<NewBuilderResponse>, Status> {
        let id = RemoteId::new();
        let mut sessions = self.sessions.lock().await;
        let response = Response::new(NewBuilderResponse {
            remote_id: id.to_string(),
        });

        sessions.insert(
            id,
            Arc::new(Mutex::new(CompilerSession::Build(wasmtime_cranelift::builder()))),
        );
        Ok(response)
    }

    async fn set_target(&self, req: Request<Triple>) -> Result<Response<Empty>, Status> {
        // Retrieve builder id from metadata (headers)
        let remote_id = req
            .metadata()
            .get(REMOTE_ID_HEADER)
            .map(|id_meta| id_meta.to_str().ok())
            .flatten()
            .ok_or_else(|| Status::failed_precondition("invalid remote id"))?
            .into();

        let triple = rpc2internal::from_triple(req.get_ref())
            .ok_or_else(|| Status::invalid_argument("bad triple provided"))?;

        // Acquire session and set target
        let session_lock = self
            .sessions
            .lock()
            .await
            .get(&remote_id)
            .ok_or_else(|| Status::failed_precondition("invalid remote id"))?
            .clone();
        let mut session = session_lock
            .lock()
            .await;

        match &mut *session {
            CompilerSession::Build(builder) => {
                builder
                    .target(triple)
                    .map_err(|e| Status::internal(e.to_string()))?;

                Ok(Response::new(Empty {}))
            }
            _ => Err(Status::failed_precondition("session is in invalid state"))
        }
    }

    async fn get_triple(&self, req: Request<Empty>) -> Result<Response<Triple>, Status> {
        // Retrieve builder id from metadata (headers)
        let remote_id = req
            .metadata()
            .get(REMOTE_ID_HEADER)
            .map(|id_meta| id_meta.to_str().ok())
            .flatten()
            .ok_or_else(|| Status::failed_precondition("invalid remote id"))?
            .into();

        let session_lock = self
            .sessions
            .lock()
            .await
            .get(&remote_id)
            .ok_or_else(|| Status::failed_precondition("invalid remote id"))?
            .clone();
        let session = session_lock
            .lock()
            .await;

        match &*session {
            CompilerSession::Build(builder) => {
                Ok(Response::new(internal2rpc::from_triple(
                    builder.triple(),
                )))
            }
            _ => Err(Status::failed_precondition("session is in invalid state"))
        }
    }

    async fn set_settings(&self, req: Request<SetRequest>) -> Result<Response<Empty>, Status> {
        // Retrieve builder id from metadata (headers)
        let remote_id = req
            .metadata()
            .get(REMOTE_ID_HEADER)
            .map(|id_meta| id_meta.to_str().ok())
            .flatten()
            .ok_or_else(|| Status::failed_precondition("invalid remote id"))?
            .into();

        let settings = req.get_ref();

        let session_lock = self
            .sessions
            .lock()
            .await
            .get(&remote_id)
            .ok_or_else(|| Status::failed_precondition("invalid remote id"))?
            .clone();
        let mut session = session_lock
            .lock()
            .await;

        match &mut *session {
            CompilerSession::Build(builder) => {
                builder
                    .set(&settings.name, &settings.value)
                    .map_err(|e| Status::internal(e.to_string()))?;

                Ok(Response::new(Empty {}))
            }
            _ => Err(Status::failed_precondition("session is in invalid state"))
        }
    }

    async fn enable_settings(
        &self,
        _req: Request<EnableRequest>,
    ) -> Result<Response<Empty>, Status> {
        Err(Status::unimplemented("not implemented"))
    }

    async fn get_settings(
        &self,
        _req: Request<Empty>,
    ) -> Result<Response<SettingsResponse>, Status> {
        Err(Status::unimplemented("not implemented"))
    }

    async fn build(&self, req: Request<Empty>) -> Result<Response<BuildResponse>, Status> {
        // Retrieve builder id from metadata (headers)
        let remote_id = req
            .metadata()
            .get(REMOTE_ID_HEADER)
            .map(|id_meta| id_meta.to_str().ok())
            .flatten()
            .ok_or_else(|| Status::failed_precondition("invalid remote id"))?
            .into();

        // Get the builder to build a compiler according to the settings so far
        let session_lock = self
            .sessions
            .lock()
            .await
            .get(&remote_id)
            .ok_or_else(|| Status::failed_precondition("invalid remote id"))?
            .clone();
        let mut session = session_lock
            .lock()
            .await;

        let compiler = match &*session {
            CompilerSession::Build(builder) => {
                Ok(builder.build())
            }
            _ => Err(Status::failed_precondition("session is in invalid state"))
        }?;

        let response = Response::new(BuildResponse {
            remote_id: remote_id.to_string(),
        });

        *session = CompilerSession::Compile {
            compiler,
            module_translation: wasmtime_environ::ModuleTranslation::default(),
        };

        Ok(response)
    }

    async fn set_translations(
        &self,
        req: Request<ModuleTranslation>,
    ) -> Result<Response<Empty>, Status> {
        // Retrieve builder id from metadata (headers)
        let remote_id = req
            .metadata()
            .get(REMOTE_ID_HEADER)
            .map(|id_meta| id_meta.to_str().ok())
            .flatten()
            .ok_or_else(|| Status::failed_precondition("invalid remote id"))?
            .into();

        // Get the builder to build a compiler according to the settings so far
        let session_lock = self
            .sessions
            .lock()
            .await
            .get(&remote_id)
            .ok_or_else(|| Status::failed_precondition("invalid remote id"))?
            .clone();
        let mut session = session_lock
            .lock()
            .await;

        match &mut *session {
            CompilerSession::Compile {
                module_translation, ..
            } => {
                let new_translation = req.into_inner();
                *module_translation = rpc2internal::from_module_translation(&new_translation);
                Ok(Response::new(Empty {}))
            }
            _ => Err(Status::failed_precondition("session is in invalid state"))
        }
    }
}
