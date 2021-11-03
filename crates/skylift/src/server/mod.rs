pub(crate) mod builder;
// pub(crate) mod compiler;

use crate::skylift_grpc::compiler_server::CompilerServer;
use anyhow::Result;
use tonic::transport::Server;

#[tokio::main]
pub async fn run_server(addr: &str) -> Result<()> {
    let compiler_builder = builder::CompilerService::default();
    Server::builder()
        .add_service(CompilerServer::new(compiler_builder))
        .serve(addr.parse()?)
        .await?;
    Ok(())
}
