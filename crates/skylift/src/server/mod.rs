pub(crate) mod builder;
// pub(crate) mod compiler;

use crate::skylift_grpc::compiler_builder_server::CompilerBuilderServer;
use anyhow::Result;
use tonic::transport::Server;

#[tokio::main]
pub async fn run_server(addr: &str) -> Result<()> {
    let compiler_builder = builder::CompilerBuilderService::default();
    Server::builder()
        .add_service(CompilerBuilderServer::new(compiler_builder))
        .serve(addr.parse()?)
        .await?;
    Ok(())
}
