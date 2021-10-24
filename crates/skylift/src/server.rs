use crate::convert::from_triple;
use crate::skylift_capnp::compiler_builder;
use anyhow::Result;
use capnp::capability::Promise;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::AsyncReadExt;
use tokio::net::TcpListener;
use wasmtime_cranelift::builder;
use wasmtime_environ::CompilerBuilder;

struct CompilerBuilderImpl(Box<dyn CompilerBuilder>);

impl CompilerBuilderImpl {
    pub fn new() -> Self {
        Self(builder())
    }
}

impl compiler_builder::Server for CompilerBuilderImpl {
    fn target(
        &mut self,
        params: compiler_builder::TargetParams,
        _result: compiler_builder::TargetResults,
    ) -> Promise<(), ::capnp::Error> {
        let target = params
            .get()
            .and_then(compiler_builder::target_params::Reader::get_target)
            .and_then(|tgt| {
                from_triple(tgt).map_err(|_| capnp::Error::failed("triple argument conversion failed".to_string()))
            });
        self.0.target(target.unwrap());

        Promise::err(::capnp::Error::unimplemented(
            "method not implemented".to_string(),
        ))
    }

    fn triple(
        &mut self,
        _triple: compiler_builder::TripleParams,
        _result: compiler_builder::TripleResults,
    ) -> Promise<(), ::capnp::Error> {
        Promise::err(::capnp::Error::unimplemented(
            "method not implemented".to_string(),
        ))
    }

    fn set(
        &mut self,
        _param: compiler_builder::SetParams,
        _result: compiler_builder::SetResults,
    ) -> Promise<(), ::capnp::Error> {
        Promise::err(::capnp::Error::unimplemented(
            "method not implemented".to_string(),
        ))
    }

    fn enable(
        &mut self,
        _param: compiler_builder::EnableParams,
        _result: compiler_builder::EnableResults,
    ) -> Promise<(), ::capnp::Error> {
        Promise::err(::capnp::Error::unimplemented(
            "method not implemented".to_string(),
        ))
    }

    fn settings(
        &mut self,
        _settings: compiler_builder::SettingsParams,
        _result: compiler_builder::SettingsResults,
    ) -> Promise<(), ::capnp::Error> {
        Promise::err(::capnp::Error::unimplemented(
            "method not implemented".to_string(),
        ))
    }

    fn build(
        &mut self,
        _: compiler_builder::BuildParams,
        _: compiler_builder::BuildResults,
    ) -> Promise<(), ::capnp::Error> {
        Promise::err(::capnp::Error::unimplemented(
            "method not implemented".to_string(),
        ))
    }
}

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
