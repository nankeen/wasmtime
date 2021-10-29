//! Implementation of a "compiler builder" for cranelift
//!
//! This module contains the implementation of how Cranelift is configured, as
//! well as providing a function to return the default configuration to build.

use anyhow::Result;
use std::fmt;
use wasmtime_environ::{CompilerBuilder, Setting};

struct Builder;

pub fn builder() -> Box<dyn CompilerBuilder> {
    // Establish connection to server
    Box::new(Builder::new("localhost:1337").unwrap())
}

impl Builder {
    pub fn new(addr: &str) -> Result<Self> {
        Ok(Self)
    }
}

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
