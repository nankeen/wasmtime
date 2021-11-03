use crate::skylift_grpc::compiler_server::Compiler;
use crate::skylift_grpc::{
    BuildReponse, Empty, EnableRequest, NewBuilderResponse, SetRequest, SettingsResponse, Triple,
};
use std::collections::HashMap;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

#[derive(std::hash::Hash, Debug)]
struct BuilderId(String);

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
        Err(Status::unimplemented("not implemented"))
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
