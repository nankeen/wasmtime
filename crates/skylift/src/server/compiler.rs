use crate::skylift_capnp::compiler;
use capnp::capability::Promise;
use wasmtime_environ::Compiler;

pub(crate) struct CompilerImpl(Box<dyn Compiler>);

impl CompilerImpl {
    pub(crate) fn new(compiler: Box<dyn Compiler>) -> Self {
        Self(compiler)
    }
}

impl compiler::Server for CompilerImpl {
    fn ping(
        &mut self,
        _: compiler::PingParams,
        mut result: compiler::PingResults,
    ) -> Promise<(), ::capnp::Error> {
        result.get().set_pong("hello");
        Promise::ok(())
    }
}
