use cranelift_codegen::{isa::TargetIsa, ir};
use cranelift_codegen::print_errors::pretty_error;
use cranelift_codegen::{Context, binemit};
use wasmtime_environ::CompileError;

use crate::{CompiledFunction, Relocation, RelocationTarget};

pub fn finish_trampoline(
    mut context: Context,
    isa: &dyn TargetIsa,
) -> Result<CompiledFunction, CompileError> {
    let mut code_buf = Vec::new();
    let mut reloc_sink = TrampolineRelocSink::default();
    let mut trap_sink = binemit::NullTrapSink {};
    let mut stack_map_sink = binemit::NullStackMapSink {};
    context
        .compile_and_emit(
            isa,
            &mut code_buf,
            &mut reloc_sink,
            &mut trap_sink,
            &mut stack_map_sink,
        )
        .map_err(|error| CompileError::Codegen(pretty_error(&context.func, Some(isa), error)))?;

    let unwind_info = context
        .create_unwind_info(isa)
        .map_err(|error| CompileError::Codegen(pretty_error(&context.func, Some(isa), error)))?;

    Ok(CompiledFunction {
        body: code_buf,
        jt_offsets: context.func.jt_offsets,
        unwind_info,
        relocations: reloc_sink.relocs,
        stack_slots: Default::default(),
        value_labels_ranges: Default::default(),
        info: Default::default(),
        address_map: Default::default(),
        traps: Vec::new(),
    })
}

/// We don't expect trampoline compilation to produce many relocations, so
/// this `RelocSink` just asserts that it doesn't recieve most of them, but
/// handles libcall ones.
#[derive(Default)]
struct TrampolineRelocSink {
    relocs: Vec<Relocation>,
}

impl binemit::RelocSink for TrampolineRelocSink {
    fn reloc_external(
        &mut self,
        offset: binemit::CodeOffset,
        _srcloc: ir::SourceLoc,
        reloc: binemit::Reloc,
        name: &ir::ExternalName,
        addend: binemit::Addend,
    ) {
        let reloc_target = if let ir::ExternalName::LibCall(libcall) = *name {
            RelocationTarget::LibCall(libcall)
        } else {
            panic!("unrecognized external name")
        };
        self.relocs.push(Relocation {
            reloc,
            reloc_target,
            offset,
            addend,
        });
    }
    fn reloc_constant(
        &mut self,
        _code_offset: binemit::CodeOffset,
        _reloc: binemit::Reloc,
        _constant_offset: ir::ConstantOffset,
    ) {
        panic!("trampoline compilation should not produce constant relocs");
    }
    fn reloc_jt(
        &mut self,
        _offset: binemit::CodeOffset,
        _reloc: binemit::Reloc,
        _jt: ir::JumpTable,
    ) {
        panic!("trampoline compilation should not produce jump table relocs");
    }
}
