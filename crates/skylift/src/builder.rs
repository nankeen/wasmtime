//! Implementation of a "compiler builder" for cranelift
//!
//! This module contains the implementation of how Cranelift is configured, as
//! well as providing a function to return the default configuration to build.

use crate::{
    compiler::Compiler,
    convert::internal2rpc,
    skylift_grpc::{compiler_client::CompilerClient, SetRequest},
    RemoteId,
};
use anyhow::Result;
use cranelift_codegen::{
    isa,
    settings::{self, Configurable, SetError},
};
use std::fmt;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tonic::{codegen::InterceptedService, transport::Channel, Request};
use tracing::instrument;
use wasmtime_environ::{CompilerBuilder, Setting};

/// [`Builder`] implements `wasmtime_environ::CompilerBuilder`.
///
/// It is a thin wrapper on top of tonic gRPC client specifically for the
/// `CompilerBuilder` service.
#[derive(Clone)]
pub struct Builder {
    flags: settings::Builder,
    isa_flags: isa::Builder,
    /// `client` - Handler for client session, according to tonic specs this should
    /// be cheap to clone as the underlying implementation uses mpsc channel.
    client: CompilerClient<InterceptedService<Channel, RemoteId>>,
    runtime: Arc<Runtime>,
}

pub fn builder() -> Box<dyn CompilerBuilder> {
    // Establish connection to server
    Box::new(Builder::new(&std::env::var("COMPILE_SERVER").unwrap()).unwrap())
}

impl Builder {
    pub fn new(addr: &str) -> Result<Self> {
        let runtime = Runtime::new()?;
        let addr = addr.to_string();
        let isa_flags =
            cranelift_native::builder().expect("host machine is not a supported target");

        // Create a new client
        let client = runtime.block_on(async move {
            // Connect to the endpoint
            let channel = Channel::from_shared(addr)?.connect().await?;

            // A temporary client to retrieve the builder id
            let mut tmp_client = CompilerClient::new(channel.clone());
            let builder_id: RemoteId = tmp_client
                .new_builder(Request::new(()))
                .await?
                .into_inner()
                .into();

            // A new client that would include the builder id
            let client = CompilerClient::with_interceptor(channel, builder_id);

            Ok::<_, anyhow::Error>(client)
        })?;

        // Mirror remote flags operation
        let mut flags = settings::builder();

        // There are two possible traps for division, and this way
        // we get the proper one if code traps.
        flags
            .enable("avoid_div_traps")
            .expect("should be valid flag");

        // We don't use probestack as a stack limit mechanism
        flags
            .set("enable_probestack", "false")
            .expect("should be valid flag");

        let triple = isa_flags.triple().clone();

        let mut builder = Self {
            flags,
            isa_flags,
            client,
            runtime: Arc::new(runtime),
        };
        builder.target(triple)?;

        Ok(builder)
    }
}

impl CompilerBuilder for Builder {
    fn triple(&self) -> &target_lexicon::Triple {
        // FIXME: Immutable self borrow is very annoying
        self.isa_flags.triple()
    }

    fn clone(&self) -> Box<dyn CompilerBuilder> {
        Box::new(Clone::clone(self))
    }

    fn target(&mut self, target: target_lexicon::Triple) -> Result<()> {
        let mut client = self.client.clone();
        let request = internal2rpc::from_triple(&target);
        self.runtime
            .block_on(client.set_target(Request::new(request)))?;
        self.isa_flags = isa::lookup(target)?;
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

        // Forward this to Cranelift
        if let Err(err) = self.flags.set(name, value) {
            match err {
                SetError::BadName(_) => {
                    // Try the target-specific flags.
                    self.isa_flags.set(name, value)?;
                }
                _ => return Err(err.into()),
            }
        }

        Ok(())
    }

    fn enable(&mut self, _name: &str) -> Result<()> {
        unimplemented!("not implemented");
    }

    #[instrument]
    fn build(&self) -> Box<dyn wasmtime_environ::Compiler> {
        let mut client = self.client.clone();
        self.runtime
            .block_on(client.build(Request::new(())))
            .unwrap();

        let isa = self
            .isa_flags
            .clone()
            .finish(settings::Flags::new(self.flags.clone()));
        Box::new(Compiler::new(
            self.client.clone(),
            self.runtime.clone(),
            isa,
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
