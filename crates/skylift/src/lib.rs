// pub mod skylift_capnp {
//     include!(concat!(env!("OUT_DIR"), "/schema/skylift_capnp.rs"));
// }

pub mod skylift_grpc {
    tonic::include_proto!("skylift");
}

pub mod client;
mod convert;
mod server;

pub use server::run_server;
use skylift_grpc::NewBuilderResponse;
use tonic::{metadata::MetadataValue, service::Interceptor, Request, Status};
use uuid::Uuid;

pub const BUILDER_ID_HEADER: &str = "builder_id";

#[derive(std::hash::Hash, Debug, PartialEq, Eq, Clone)]
pub(crate) struct BuilderId(String);

impl BuilderId {
    pub(crate) fn new() -> Self {
        Self(Uuid::new_v4().to_hyphenated_ref().to_string())
    }
}

impl std::ops::Deref for BuilderId {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for BuilderId {
    fn from(item: String) -> Self {
        Self(item)
    }
}

impl From<&str> for BuilderId {
    fn from(item: &str) -> Self {
        Self(item.to_string())
    }
}

impl From<NewBuilderResponse> for BuilderId {
    fn from(item: NewBuilderResponse) -> Self {
        item.builder_id.into()
    }
}

impl Interceptor for BuilderId {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        req.metadata_mut().insert(
            BUILDER_ID_HEADER,
            MetadataValue::from_str(&self.0)
                .map_err(|_| Status::internal("invalid metadata value"))?,
        );
        Ok(req)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
