//! Implementation of a "compiler builder" for cranelift
//!
//! This module contains the implementation of how Cranelift is configured, as
//! well as providing a function to return the default configuration to build.

use crate::{
    convert::internal2rpc,
    skylift_grpc::{compiler_client::CompilerClient, Empty, SetRequest},
};
use anyhow::Result;
use std::fmt;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tonic::transport::Channel;
use wasmtime_environ::{CompilerBuilder, Setting};

#[derive(Default)]
struct BuilderCache {
    pub triple: Option<target_lexicon::Triple>,
}

/// [`Builder`] implements `wasmtime_environ::CompilerBuilder`.
///
/// It is a thin wrapper on top of tonic gRPC client specifically for the
/// `CompilerBuilder` service.
#[derive(Clone)]
pub struct Builder {
    runtime: Arc<Runtime>,
    /// `client` - Handler for client session, according to tonic specs this should
    /// be cheap to clone as the underlying implementation uses mpsc channel.
    client: CompilerClient<Channel>,
    cache: Arc<BuilderCache>,
}

pub fn builder() -> Box<dyn CompilerBuilder> {
    // Establish connection to server
    Box::new(Builder::new("http://[::1]:1337").unwrap())
}

impl Builder {
    pub fn new(addr: &'static str) -> Result<Self> {
        let runtime = Runtime::new()?;
        let client = runtime.block_on(CompilerClient::connect(addr))?;

        Ok(Self {
            runtime: Arc::new(runtime),
            cache: Arc::new(BuilderCache::default()),
            client,
        })
    }
}

impl CompilerBuilder for Builder {
    fn triple(&self) -> &target_lexicon::Triple {
        // FIXME: Immutable self borrow is very annoying
        self.cache.triple.as_ref().unwrap()
    }

    fn clone(&self) -> Box<dyn CompilerBuilder> {
        Box::new(Clone::clone(self))
    }

    fn target(&mut self, target: target_lexicon::Triple) -> Result<()> {
        let mut client = self.client.clone();
        let request = internal2rpc::from_triple(&target);
        self.runtime
            .block_on(client.set_target(tonic::Request::new(request)))?;
        Ok(())
    }

    fn set(&mut self, name: &str, value: &str) -> Result<()> {
        let mut client = self.client.clone();
        let request = SetRequest {
            name: name.to_string(),
            value: value.to_string(),
        };
        self.runtime
            .block_on(client.set_settings(tonic::Request::new(request)))?;
        Ok(())
    }

    fn enable(&mut self, _name: &str) -> Result<()> {
        unimplemented!("not implemented");
    }

    fn build(&self) -> Box<dyn wasmtime_environ::Compiler> {
        let mut client = self.client.clone();
        self.runtime
            .block_on(client.get_triple(tonic::Request::new(Empty {})))
            .unwrap();
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
