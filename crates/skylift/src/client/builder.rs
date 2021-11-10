//! Implementation of a "compiler builder" for cranelift
//!
//! This module contains the implementation of how Cranelift is configured, as
//! well as providing a function to return the default configuration to build.

use crate::{
    client::Compiler,
    convert::{internal2rpc, rpc2internal},
    skylift_grpc::{compiler_client::CompilerClient, Empty, SetRequest},
    RemoteId,
};
use anyhow::{Result, anyhow};
use std::fmt;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tonic::{codegen::InterceptedService, transport::Channel, Request};
use wasmtime_environ::{CompilerBuilder, Setting};

#[derive(Default, Clone)]
struct BuilderCache {
    pub triple: Option<target_lexicon::Triple>,
}

/// [`Builder`] implements `wasmtime_environ::CompilerBuilder`.
///
/// It is a thin wrapper on top of tonic gRPC client specifically for the
/// `CompilerBuilder` service.
#[derive(Clone)]
pub struct Builder {
    cache: BuilderCache,
    /// `client` - Handler for client session, according to tonic specs this should
    /// be cheap to clone as the underlying implementation uses mpsc channel.
    client: CompilerClient<InterceptedService<Channel, RemoteId>>,
    runtime: Arc<Runtime>,
}

pub fn builder() -> Box<dyn CompilerBuilder> {
    // Establish connection to server
    Box::new(Builder::new("http://[::1]:1337").unwrap())
}

impl Builder {
    pub fn new(addr: &'static str) -> Result<Self> {
        let runtime = Runtime::new()?;
        let addr = addr.to_string();

        // Create a new client
        let (triple, client) = runtime.block_on(async move {
            // Connect to the endpoint
            let channel = Channel::from_shared(addr)?.connect().await?;

            // A temporary client to retrieve the builder id
            let mut tmp_client = CompilerClient::new(channel.clone());
            let builder_id: RemoteId = tmp_client
                .new_builder(Request::new(Empty {}))
                .await?
                .into_inner()
                .into();

            // A new client that would include the builder id
            let mut client = CompilerClient::with_interceptor(channel, builder_id);

            let triple = rpc2internal::from_triple(
                client.get_triple(Request::new(Empty {})).await?.get_ref(),
            )
            .ok_or(anyhow!("can't retrieve triple"))?;

            Ok::<_, anyhow::Error>((triple, client))
        })?;

        Ok(Self {
            cache: BuilderCache { triple: Some(triple) },
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
        self.cache.triple = Some(target);
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
            .block_on(client.build(Request::new(Empty {})))
            .unwrap();

        Box::new(Compiler::new(
            self.client.clone(),
            self.runtime.clone(),
            self.triple().clone(),
        ))
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
