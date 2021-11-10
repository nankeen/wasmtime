pub(crate) mod builder;
// pub(crate) mod compiler;

use crate::skylift_grpc::compiler_server::CompilerServer;
use anyhow::Result;
use tonic::transport::Server;

#[tokio::main]
pub async fn run_server(addr: &str) -> Result<()> {
    let compiler_service = builder::CompilerService::default();
    let compiler_server = CompilerServer::new(compiler_service);

    Server::builder()
        .add_service(compiler_server)
        .serve(addr.parse()?)
        .await?;
    Ok(())
}
