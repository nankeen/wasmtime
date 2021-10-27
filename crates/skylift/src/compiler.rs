use crate::skylift_capnp;
use wasmtime_environ::Compiler;

pub(crate) struct CompilerImpl(Box<dyn Compiler>);

impl CompilerImpl {
    pub(crate) fn new(compiler: Box<dyn Compiler>) -> Self {
        Self(compiler)
    }
}

impl skylift_capnp::compiler::Server for CompilerImpl {}
