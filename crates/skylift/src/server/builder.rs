use crate::{
    skylift_grpc::{
        compiler_server::Compiler, BuildReponse, Empty, EnableRequest, NewBuilderResponse,
        SetRequest, SettingsResponse, Triple,
    },
    BuilderId,
};
use std::collections::HashMap;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub(crate) struct CompilerService {
    builders: Mutex<HashMap<BuilderId, Mutex<Box<dyn wasmtime_environ::CompilerBuilder>>>>,
}

#[tonic::async_trait]
impl Compiler for CompilerService {
    async fn new_builder(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<NewBuilderResponse>, Status> {
        let id = BuilderId::new();
        let mut builders = self.builders.lock().await;
        builders.insert(id, Mutex::new(wasmtime_cranelift::builder()));
        Ok(Response::new(NewBuilderResponse {
            builder_id: id.to_simple().to_string(),
        }))
    }

    async fn set_target(&self, _request: Request<Triple>) -> Result<Response<Empty>, Status> {
        Err(Status::unimplemented("not implemented"))
    }

    async fn get_triple(&self, _request: Request<Empty>) -> Result<Response<Triple>, Status> {
        Err(Status::unimplemented("not implemented"))
    }

    async fn set_settings(&self, _request: Request<SetRequest>) -> Result<Response<Empty>, Status> {
        Err(Status::unimplemented("not implemented"))
    }

    async fn enable_settings(
        &self,
        _request: Request<EnableRequest>,
    ) -> Result<Response<Empty>, Status> {
        Err(Status::unimplemented("not implemented"))
    }

    async fn get_settings(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<SettingsResponse>, Status> {
        Err(Status::unimplemented("not implemented"))
    }

    async fn build(&self, _request: Request<Empty>) -> Result<Response<BuildReponse>, Status> {
        Err(Status::unimplemented("not implemented"))
    }
}
