use crate::{
    convert::{internal2rpc, rpc2internal},
    server::compiler::CompilerImpl,
    skylift_capnp::compiler_builder,
};
use capnp::capability::Promise;
use capnp_rpc::pry;
use wasmtime_cranelift::builder;
use wasmtime_environ::CompilerBuilder;

pub(crate) struct CompilerBuilderImpl(Box<dyn CompilerBuilder>);

impl CompilerBuilderImpl {
    pub(crate) fn new() -> Self {
        Self(builder())
    }
}

impl compiler_builder::Server for CompilerBuilderImpl {
    fn target(
        &mut self,
        params: compiler_builder::TargetParams,
        _result: compiler_builder::TargetResults,
    ) -> Promise<(), ::capnp::Error> {
        let target = pry!(pry!(params.get()).get_target());
        let target_triple = pry!(rpc2internal::from_triple(&target));

        Promise::ok(pry!(self.0.target(target_triple).map_err(|_| {
            capnp::Error::failed("failed to set triple".to_string())
        })))
    }

    fn triple(
        &mut self,
        _triple: compiler_builder::TripleParams,
        mut result: compiler_builder::TripleResults,
    ) -> Promise<(), ::capnp::Error> {
        let mut builder = result.get().init_triple();
        internal2rpc::to_triple_builder(&mut builder, self.0.triple());

        Promise::ok(())
    }

    fn build(
        &mut self,
        _: compiler_builder::BuildParams,
        mut result: compiler_builder::BuildResults,
    ) -> Promise<(), ::capnp::Error> {
        let compiler = self.0.build();
        result
            .get()
            .set_compiler(capnp_rpc::new_client(CompilerImpl::new(compiler)));

        Promise::ok(())
    }
}
