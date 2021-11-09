use crate::{BUILDER_ID_HEADER, BuilderId, convert::rpc2internal, skylift_grpc::{
        compiler_server::Compiler, BuildReponse, Empty, EnableRequest, NewBuilderResponse,
        SetRequest, SettingsResponse, Triple,
    }};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use wasmtime_environ::CompilerBuilder;

type BuilderMapValue = Arc<Mutex<Box<dyn CompilerBuilder>>>;

#[derive(Debug, Default)]
pub(crate) struct CompilerService {
    builders: Mutex<HashMap<BuilderId, BuilderMapValue>>,
}

#[tonic::async_trait]
impl Compiler for CompilerService {
    async fn new_builder(
        &self,
        _req: Request<Empty>,
    ) -> Result<Response<NewBuilderResponse>, Status> {
        let id = BuilderId::new();
        let mut builders = self.builders.lock().await;
        let response = Response::new(NewBuilderResponse {
            builder_id: id.to_string(),
        });

        builders.insert(id, Arc::new(Mutex::new(wasmtime_cranelift::builder())));
        Ok(response)
    }

    async fn set_target(&self, req: Request<Triple>) -> Result<Response<Empty>, Status> {
        // Retrieve builder id from metadata (headers)
        let builder_id = req
            .metadata()
            .get(BUILDER_ID_HEADER)
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
            .get(&builder_id)
            .ok_or_else(|| Status::failed_precondition("invalid builder id"))?
            .clone();

        builder
            .lock()
            .await
            .target(triple)
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Empty {}))
    }

    async fn get_triple(&self, _req: Request<Empty>) -> Result<Response<Triple>, Status> {
        Err(Status::unimplemented("not implemented"))
    }

    async fn set_settings(&self, req: Request<SetRequest>) -> Result<Response<Empty>, Status> {
        // Retrieve builder id from metadata (headers)
        let builder_id = req
            .metadata()
            .get(BUILDER_ID_HEADER)
            .map(|id_meta| id_meta.to_str().ok())
            .flatten()
            .ok_or_else(|| Status::failed_precondition("invalid builder id"))?
            .into();

        let settings = req.get_ref();

        let builder = self
            .builders
            .lock()
            .await
            .get(&builder_id)
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

    async fn build(&self, _req: Request<Empty>) -> Result<Response<BuildReponse>, Status> {
        Err(Status::unimplemented("not implemented"))
    }
}
