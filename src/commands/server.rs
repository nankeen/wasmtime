use crate::CommonOptions;
use anyhow::Result;
use skylift_server::run_server;
use structopt::{clap::AppSettings, StructOpt};
use tracing::{info, instrument};

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

    #[structopt(flatten)]
    common: CommonOptions,
}

impl ServerCommand {
    /// Executes the command
    #[instrument(skip(self))]
    pub fn execute(&self) -> Result<()> {
        self.common.init_logging();
        info!("Starting server on {}", self.host);

        let config = self.common.config(None)?;
        run_server(config.cache_config, &self.host)
    }
}
