use tonic::Status;

pub(super) enum CompilerSession {
    Build(Box<dyn wasmtime_environ::CompilerBuilder>),
    Compile(Box<dyn wasmtime_environ::Compiler>),
}

impl CompilerSession {
    pub(super) fn map_builder_mut<F, A>(&mut self, f: F) -> Result<A, Status>
    where
        F: FnOnce(&mut Box<dyn wasmtime_environ::CompilerBuilder>) -> A,
    {
        match self {
            CompilerSession::Build(builder) => Ok(f(builder)),
            _ => Err(Status::failed_precondition("session is not in build state")),
        }
    }

    pub(super) fn map_builder<F, A>(&self, f: F) -> Result<A, Status>
    where
        F: FnOnce(&Box<dyn wasmtime_environ::CompilerBuilder>) -> A,
    {
        match self {
            CompilerSession::Build(builder) => Ok(f(builder)),
            _ => Err(Status::failed_precondition("session is not in build state")),
        }
    }

    pub(super) fn map_compiler_mut<F, A>(&mut self, f: F) -> Result<A, Status>
    where
        F: FnOnce(&mut Box<dyn wasmtime_environ::Compiler>) -> A,
    {
        match self {
            CompilerSession::Compile(compiler) => Ok(f(compiler)),
            _ => Err(Status::failed_precondition(
                "session is not in compile state",
            )),
        }
    }

    pub(super) fn map_compiler<F, A>(&self, f: F) -> Result<A, Status>
    where
        F: FnOnce(&Box<dyn wasmtime_environ::Compiler>) -> A,
    {
        match self {
            CompilerSession::Compile(compiler) => Ok(f(compiler)),
            _ => Err(Status::failed_precondition(
                "session is not in compile state",
            )),
        }
    }
}
