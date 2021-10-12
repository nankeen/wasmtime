use cranelift_codegen::isa;
use cranelift_codegen::settings::{self, Configurable};
use wasmtime_environ::CompilerBuilder;

#[derive(Clone, Default)]
pub struct LinkOptions {
    /// A debug-only setting used to synthetically insert 0-byte padding between
    /// compiled functions to simulate huge compiled artifacts and exercise
    /// logic related to jump veneers.
    pub padding_between_functions: usize,

    /// A debug-only setting used to force inter-function calls in a wasm module
    /// to always go through "jump veneers" which are typically only generated
    /// when functions are very far from each other.
    pub force_jump_veneers: bool,
}

#[derive(Clone)]
struct Builder {
    flags: settings::Builder,
    isa_flags: isa::Builder,
    linkopts: LinkOptions,
}

pub fn builder() {
    let mut flags = settings::builder();
    flags.enable("avoid_div_traps");
}
