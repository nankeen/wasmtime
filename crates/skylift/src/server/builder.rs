use crate::skylift_grpc::compiler_builder_server::CompilerBuilder;
use crate::skylift_grpc::{Empty, EnableRequest, SetRequest, SettingsResponse, Triple};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub(crate) struct CompilerBuilderService;

#[tonic::async_trait]
impl CompilerBuilder for CompilerBuilderService {
    async fn set_target(&self, request: Request<Triple>) -> Result<Response<Empty>, Status> {
        unimplemented!();
    }

    async fn get_triple(&self, request: Request<Empty>) -> Result<Response<Triple>, Status> {
        unimplemented!();
    }

    async fn set_settings(&self, request: Request<SetRequest>) -> Result<Response<Empty>, Status> {
        unimplemented!();
    }

    async fn enable_settings(
        &self,
        request: Request<EnableRequest>,
    ) -> Result<Response<Empty>, Status> {
        unimplemented!();
    }

    async fn get_settings(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<SettingsResponse>, Status> {
        unimplemented!();
    }

    async fn build(&self, request: Request<Empty>) -> Result<Response<Empty>, Status> {
        unimplemented!();
    }
}
