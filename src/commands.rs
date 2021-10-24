//! The module for the Wasmtime CLI commands.

mod compile;
mod config;
mod run;
mod settings;
mod wast;
mod server;

pub use self::{compile::*, config::*, run::*, settings::*, wast::*, server::*};
