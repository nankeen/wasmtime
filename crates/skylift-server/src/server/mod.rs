pub(crate) mod service;
mod session;

use std::convert::TryFrom;

use anyhow::Result;
use session::CompilerSession;
use skylift::{
    convert::rpc2internal,
    skylift_grpc::{compiler_server::CompilerServer, BuildModuleRequest},
};
use tonic::transport::Server;
use wasmparser::WasmFeatures;
use wasmtime_environ::Tunables;

#[tokio::main]
pub async fn run_server(addr: &str) -> Result<()> {
    let compiler_service = service::CompilerService::new()?;
    let compiler_server = CompilerServer::new(compiler_service);

    Server::builder()
        .add_service(compiler_server)
        .serve(addr.parse()?)
        .await?;
    Ok(())
}

pub enum ServerError {
    BadCompileEnv,
}

pub struct CompileEnv {
    tunables: Tunables,
    features: WasmFeatures,
    paged_memory_initialization: bool,
}

impl TryFrom<&BuildModuleRequest> for CompileEnv {
    type Error = ServerError;

    fn try_from(req: &BuildModuleRequest) -> Result<Self, Self::Error> {
        let tunables = req
            .tunables
            .as_ref()
            .ok_or_else(|| ServerError::BadCompileEnv)
            .map(rpc2internal::from_tunables)?
            .ok_or_else(|| ServerError::BadCompileEnv)?;
        let features = req
            .features
            .as_ref()
            .ok_or_else(|| ServerError::BadCompileEnv)
            .map(rpc2internal::from_wasm_features)?;
        let paged_memory_initialization = req.paged_memory_initialization;

        Ok(Self {
            tunables,
            features,
            paged_memory_initialization,
        })
    }
}

#[cfg(all(feature = "cache"))]
struct HashedCompileEnv<'a>(&'a Box<dyn wasmtime_environ::Compiler>, &'a CompileEnv);

#[cfg(all(feature = "cache"))]
impl std::hash::Hash for HashedCompileEnv<'_> {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        use std::collections::BTreeMap;

        // Hash the compiler's state based on its target and configuration.
        let compiler = self.0;
        compiler.triple().hash(hasher);
        compiler
            .flags()
            .into_iter()
            .collect::<BTreeMap<_, _>>()
            .hash(hasher);
        compiler
            .isa_flags()
            .into_iter()
            .collect::<BTreeMap<_, _>>()
            .hash(hasher);

        // Hash configuration state read for compilation
        let env = self.1;
        env.tunables.hash(hasher);
        env.features.hash(hasher);

        // Catch accidental bugs of reusing across crate versions.
        env!("CARGO_PKG_VERSION").hash(hasher);
    }
}
