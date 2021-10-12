fn main() {
    capnpc::CompilerCommand::new()
        .file("schema/skylift.capnp")
        .run()
        .unwrap();
}
