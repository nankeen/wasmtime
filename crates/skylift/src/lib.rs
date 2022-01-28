// pub mod skylift_capnp {
//     include!(concat!(env!("OUT_DIR"), "/schema/skylift_capnp.rs"));
// }

pub mod skylift_grpc {
    tonic::include_proto!("skylift");
}

mod builder;
pub mod compiler;
pub mod convert;

use std::path::Path;

pub use builder::builder;
use skylift_grpc::NewBuilderResponse;
use tonic::{metadata::MetadataValue, service::Interceptor, Request, Status};
use tracing_flame::FlameLayer;
use tracing_subscriber::{filter, fmt, prelude::*};
use uuid::Uuid;

pub const REMOTE_ID_HEADER: &str = "remote_id";

#[derive(std::hash::Hash, Debug, PartialEq, Eq, Clone)]
pub struct RemoteId(String);

impl RemoteId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_hyphenated_ref().to_string())
    }
}

impl std::ops::Deref for RemoteId {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for RemoteId {
    fn from(item: String) -> Self {
        Self(item)
    }
}

impl From<&str> for RemoteId {
    fn from(item: &str) -> Self {
        Self(item.to_string())
    }
}

impl From<NewBuilderResponse> for RemoteId {
    fn from(item: NewBuilderResponse) -> Self {
        item.remote_id.into()
    }
}

impl Interceptor for RemoteId {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        req.metadata_mut().insert(
            REMOTE_ID_HEADER,
            MetadataValue::from_str(&self.0)
                .map_err(|_| Status::internal("invalid metadata value"))?,
        );
        Ok(req)
    }
}

pub fn setup_global_subscriber(log_path: impl AsRef<Path>) -> impl Drop {
    let fmt_layer = fmt::Layer::default();

    let (flame_layer, _guard) = FlameLayer::with_file(log_path).unwrap();

    tracing_subscriber::registry()
        .with(filter::EnvFilter::from_default_env())
        .with(fmt_layer)
        .with(flame_layer.with_threads_collapsed(true))
        .init();
    _guard
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
