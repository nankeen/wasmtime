//! Implementation of a "compiler builder" for cranelift
//!
//! This module contains the implementation of how Cranelift is configured, as
//! well as providing a function to return the default configuration to build.

use crate::skylift_grpc::{compiler_builder_client::CompilerBuilderClient, Empty, SetRequest};
use anyhow::Result;
use std::fmt;
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
pub struct Builder {
    runtime: Runtime,
    /// `client` - Handler for client session, according to tonic specs this should
    /// be cheap to clone as the underlying implementation uses mpsc channel.
    client: CompilerBuilderClient<Channel>,
    cache: Box<BuilderCache>,
}

pub fn builder() -> Box<dyn CompilerBuilder> {
    // Establish connection to server
    Box::new(Builder::new("http://[::1]:1337").unwrap())
}

impl Builder {
    pub fn new(addr: &'static str) -> Result<Self> {
        let runtime = Runtime::new()?;
        let client = runtime.block_on(async { CompilerBuilderClient::connect(addr).await })?;

        Ok(Self {
            runtime,
            client,
            cache: Box::new(BuilderCache::default()),
        })
    }
}

impl CompilerBuilder for Builder {
    fn triple(&self) -> &target_lexicon::Triple {
        // FIXME: Immutable self borrow is very annoying
        self.cache.triple.as_ref().unwrap()
    }

    fn clone(&self) -> Box<dyn CompilerBuilder> {
        unimplemented!("not implemented");
    }

    fn target(&mut self, _target: target_lexicon::Triple) -> Result<()> {
        unimplemented!("not implemented");
    }

    fn set(&mut self, name: &str, value: &str) -> Result<()> {
        let mut client = self.client.clone();
        let request = SetRequest {
            name: name.to_string(),
            value: value.to_string(),
        };
        self.runtime
            .block_on(async move { client.set_settings(tonic::Request::new(request)).await })
            .unwrap();
        unimplemented!("not implemented");
    }

    fn enable(&mut self, _name: &str) -> Result<()> {
        unimplemented!("not implemented");
    }

    fn build(&self) -> Box<dyn wasmtime_environ::Compiler> {
        let mut client = self.client.clone();
        self.runtime
            .block_on(async move { client.get_triple(tonic::Request::new(Empty {})).await })
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
