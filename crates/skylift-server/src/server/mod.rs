pub(crate) mod service;
mod session;

use anyhow::Result;
use session::CompilerSession;
use skylift::skylift_grpc::compiler_server::CompilerServer;
use tonic::transport::Server;

#[tokio::main]
pub async fn run_server(addr: &str) -> Result<()> {
    let compiler_service = service::CompilerService::default();
    let compiler_server = CompilerServer::new(compiler_service);

    Server::builder()
        .add_service(compiler_server)
        .serve(addr.parse()?)
        .await?;
    Ok(())
}
