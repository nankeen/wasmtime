//! Implementation of a "compiler builder" for cranelift
//!
//! This module contains the implementation of how Cranelift is configured, as
//! well as providing a function to return the default configuration to build.

use crate::{
    convert::internal2rpc,
    skylift_grpc::{compiler_client::CompilerClient, Empty, SetRequest},
    BuilderId,
};
use anyhow::Result;
use std::fmt;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tonic::{transport, Request};
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
    builder_id: BuilderId,
    cache: Arc<BuilderCache>,
    /// `client` - Handler for client session, according to tonic specs this should
    /// be cheap to clone as the underlying implementation uses mpsc channel.
    client: CompilerClient<transport::Channel>,
    runtime: Arc<Runtime>,
}

pub fn builder() -> Box<dyn CompilerBuilder> {
    // Establish connection to server
    Box::new(Builder::new("http://[::1]:1337").unwrap())
}

impl Builder {
    pub fn new(addr: &'static str) -> Result<Self> {
        let runtime = Runtime::new()?;
        let (client, builder_id) = runtime.block_on(async move {
            let mut client = CompilerClient::connect(addr).await?;
            let builder_id = client
                .new_builder(Request::new(Empty {}))
                .await?
                .into_inner()
                .into();
            Ok::<_, anyhow::Error>((client, builder_id))
        })?;

        Ok(Self {
            builder_id,
            cache: Arc::new(BuilderCache::default()),
            client,
            runtime: Arc::new(runtime),
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
            .block_on(client.set_target(Request::new(request)))?;
        Ok(())
    }

    fn set(&mut self, name: &str, value: &str) -> Result<()> {
        let mut client = self.client.clone();
        let request = SetRequest {
            name: name.to_string(),
            value: value.to_string(),
        };
        self.runtime
            .block_on(client.set_settings(Request::new(request)))?;
        Ok(())
    }

    fn enable(&mut self, _name: &str) -> Result<()> {
        unimplemented!("not implemented");
    }

    fn build(&self) -> Box<dyn wasmtime_environ::Compiler> {
        let mut client = self.client.clone();
        self.runtime
            .block_on(client.get_triple(Request::new(Empty {})))
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
