pub(crate) mod builder;
// pub(crate) mod compiler;

use crate::skylift_grpc::compiler_server::CompilerServer;
use anyhow::Result;
use tonic::transport::Server;

#[tokio::main]
pub async fn run_server(addr: &str) -> Result<()> {
    let compiler_server = builder::CompilerService::default();
    let compiler_service = CompilerServer::new(compiler_server);

    Server::builder()
        .add_service(compiler_service)
        .serve(addr.parse()?)
        .await?;
    Ok(())
}
