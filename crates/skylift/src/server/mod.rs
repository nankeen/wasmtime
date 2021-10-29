pub(crate) mod builder;
// pub(crate) mod compiler;

use anyhow::Result;
use tonic::transport::Server;

#[tokio::main]
pub async fn run_server(addr: &str) -> Result<()> {
    Server::builder();
    Ok(())
}
