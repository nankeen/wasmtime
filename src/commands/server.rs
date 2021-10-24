use structopt::{clap::AppSettings, StructOpt};
use skylift::run_server;
use anyhow::Result;

lazy_static::lazy_static! {
    static ref AFTER_HELP: String = {
        crate::FLAG_EXPLANATIONS.to_string()
    };
}

/// Runs the WebAssembly JIT server
#[derive(StructOpt)]
#[structopt(name = "run", setting = AppSettings::TrailingVarArg, after_help = AFTER_HELP.as_str())]
pub struct ServerCommand {
    #[structopt(long = "host")]
    host: String,
}

impl ServerCommand {
    /// Executes the command
    pub fn execute(&self) -> Result<()> {
        println!("Starting server on {}", self.host);

        run_server(&self.host)
    }
}
