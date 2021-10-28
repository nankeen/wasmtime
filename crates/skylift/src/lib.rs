pub mod skylift_capnp {
    include!(concat!(env!("OUT_DIR"), "/schema/skylift_capnp.rs"));
}

pub mod client;
mod convert;
mod server;

pub use server::run_server;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
