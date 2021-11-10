use crate::{
    convert::{internal2rpc, rpc2internal},
    skylift_grpc::{
        compiler_server::Compiler, BuildResponse, Empty, EnableRequest, NewBuilderResponse,
        SetRequest, SettingsResponse, Triple,
    },
    RemoteId, REMOTE_ID_HEADER,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

enum CompilerSession {
    Build(SessionBuilder),
    Compile(SessionCompiler),
}

type SessionBuilder = Arc<Mutex<Box<dyn wasmtime_environ::CompilerBuilder>>>;
type SessionCompiler = Arc<Mutex<Box<dyn wasmtime_environ::Compiler>>>;

#[derive(Default)]
pub(crate) struct CompilerService {
    // sessions: Mutex<HashMap<RemoteId, CompilerSession>>,
    builders: Mutex<HashMap<RemoteId, SessionBuilder>>,
    compilers: Mutex<HashMap<RemoteId, SessionCompiler>>,
}

#[tonic::async_trait]
impl Compiler for CompilerService {
    async fn new_builder(
        &self,
        _req: Request<Empty>,
    ) -> Result<Response<NewBuilderResponse>, Status> {
        let id = RemoteId::new();
        let mut builders = self.builders.lock().await;
        let response = Response::new(NewBuilderResponse {
            remote_id: id.to_string(),
        });

        builders.insert(id, Arc::new(Mutex::new(wasmtime_cranelift::builder())));
        Ok(response)
    }

    async fn set_target(&self, req: Request<Triple>) -> Result<Response<Empty>, Status> {
        // Retrieve builder id from metadata (headers)
        let remote_id = req
            .metadata()
            .get(REMOTE_ID_HEADER)
            .map(|id_meta| id_meta.to_str().ok())
            .flatten()
            .ok_or_else(|| Status::failed_precondition("invalid builder id"))?
            .into();

        let triple = rpc2internal::from_triple(req.get_ref())
            .ok_or_else(|| Status::invalid_argument("bad triple provided"))?;

        // Acquire builder and execute
        let builder = self
            .builders
            .lock()
            .await
            .get(&remote_id)
            .ok_or_else(|| Status::failed_precondition("invalid builder id"))?
            .clone();

        builder
            .lock()
            .await
            .target(triple)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn get_triple(&self, req: Request<Empty>) -> Result<Response<Triple>, Status> {
        // Retrieve builder id from metadata (headers)
        let remote_id = req
            .metadata()
            .get(REMOTE_ID_HEADER)
            .map(|id_meta| id_meta.to_str().ok())
            .flatten()
            .ok_or_else(|| Status::failed_precondition("invalid builder id"))?
            .into();

        let builder = self
            .builders
            .lock()
            .await
            .get(&remote_id)
            .ok_or_else(|| Status::failed_precondition("invalid builder id"))?
            .clone();

        let builder_lock = builder.lock().await;

        Ok(Response::new(internal2rpc::from_triple(builder_lock.triple())))
    }

    async fn set_settings(&self, req: Request<SetRequest>) -> Result<Response<Empty>, Status> {
        // Retrieve builder id from metadata (headers)
        let remote_id = req
            .metadata()
            .get(REMOTE_ID_HEADER)
            .map(|id_meta| id_meta.to_str().ok())
            .flatten()
            .ok_or_else(|| Status::failed_precondition("invalid builder id"))?
            .into();

        let settings = req.get_ref();

        let builder = self
            .builders
            .lock()
            .await
            .get(&remote_id)
            .ok_or_else(|| Status::failed_precondition("invalid builder id"))?
            .clone();

        builder
            .lock()
            .await
            .set(&settings.name, &settings.value)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
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
            .ok_or_else(|| Status::failed_precondition("invalid builder id"))?
            .into();

        // Get the builder to build a compiler according to the settings so far
        let builder = self
            .builders
            .lock()
            .await
            .get(&remote_id)
            .ok_or_else(|| Status::failed_precondition("invalid builder id"))?
            .clone();

        let compiler = builder.lock().await.build();

        // Save the compiler created into the session
        let mut compilers = self.compilers.lock().await;
        let response = Response::new(BuildResponse {
            remote_id: remote_id.to_string(),
        });

        compilers.insert(remote_id, Arc::new(Mutex::new(compiler)));
        Ok(response)
    }
}
