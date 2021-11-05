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
use uuid::Uuid;

#[derive(std::hash::Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct BuilderId(Uuid);

impl BuilderId {
    pub(crate) fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::ops::Deref for BuilderId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<NewBuilderResponse> for BuilderId {
    fn from(item: NewBuilderResponse) -> Self {
        Self(Uuid::parse_str(&item.builder_id).unwrap())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
