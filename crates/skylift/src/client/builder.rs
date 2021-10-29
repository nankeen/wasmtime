//! Implementation of a "compiler builder" for cranelift
//!
//! This module contains the implementation of how Cranelift is configured, as
//! well as providing a function to return the default configuration to build.

use crate::skylift_capnp::compiler_builder;
use anyhow::Result;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use std::fmt;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use wasmtime_environ::{CompilerBuilder, Setting};

use futures::{AsyncReadExt, FutureExt};

struct Builder {
    runtime: Runtime,
    client: Arc<Mutex<compiler_builder::Client>>,
}

pub fn builder() -> Box<dyn CompilerBuilder> {
    // Establish connection to server
    Box::new(Builder::new("localhost:1337").unwrap())
}

impl Builder {
    pub fn new(addr: &str) -> Result<Self> {
        let runtime = Runtime::new()?;

        let addr = addr.to_string();

        Ok(Self {
            client: Arc::new(Mutex::new(runtime.block_on(Builder::start_client(&addr))?)),
            runtime,
        })
    }

    async fn start_client(addr: &str) -> Result<compiler_builder::Client> {
        use std::net::ToSocketAddrs;

        let addr = addr
            .to_socket_addrs()?
            .next()
            .expect("could not parse address");
        let stream = tokio::net::TcpStream::connect(&addr).await?;
        stream.set_nodelay(true)?;
        let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();

        let network = Box::new(twoparty::VatNetwork::new(
            reader,
            writer,
            rpc_twoparty_capnp::Side::Client,
            Default::default(),
        ));

        let mut rpc_system = RpcSystem::new(network, None);
        let client: compiler_builder::Client =
            rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

        tokio::task::spawn_local(Box::pin(rpc_system.map(|_| ())));
        Ok(client)
    }
}

unsafe impl Send for Builder {}
unsafe impl Sync for Builder {}

impl CompilerBuilder for Builder {
    fn triple(&self) -> &target_lexicon::Triple {
        unimplemented!("not implemented");
    }

    fn clone(&self) -> Box<dyn CompilerBuilder> {
        unimplemented!("not implemented");
    }

    fn target(&mut self, _target: target_lexicon::Triple) -> Result<()> {
        unimplemented!("not implemented");
    }

    fn set(&mut self, _name: &str, _value: &str) -> Result<()> {
        unimplemented!("not implemented");
    }

    fn enable(&mut self, _name: &str) -> Result<()> {
        unimplemented!("not implemented");
    }

    fn build(&self) -> Box<dyn wasmtime_environ::Compiler> {
        unimplemented!("not implemented");
    }

    fn settings(&self) -> Vec<Setting> {
        unimplemented!("not implemented");
    }
}

impl fmt::Debug for Builder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Builder").finish()
    }
}
