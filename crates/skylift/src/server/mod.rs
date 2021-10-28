pub(crate) mod builder;
pub(crate) mod compiler;

use crate::{server::builder::CompilerBuilderImpl, skylift_capnp::compiler_builder};
use anyhow::Result;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::AsyncReadExt;
use tokio::net::TcpListener;

#[tokio::main]
pub async fn run_server(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let skylift_client: compiler_builder::Client =
        capnp_rpc::new_client(CompilerBuilderImpl::new());

    loop {
        let (stream, _) = listener.accept().await?;

        stream.set_nodelay(true)?;
        let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
        let network = twoparty::VatNetwork::new(
            reader,
            writer,
            rpc_twoparty_capnp::Side::Server,
            Default::default(),
        );

        let rpc_system = RpcSystem::new(Box::new(network), Some(skylift_client.clone().client));

        tokio::task::spawn_local(Box::pin(rpc_system));
    }
}
