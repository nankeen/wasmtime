pub mod skylift_capnp {
    include!(concat!(env!("OUT_DIR"), "/schema/skylift_capnp.rs"));
}

mod builder;
pub(crate) mod convert;
mod server;

pub use server::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
