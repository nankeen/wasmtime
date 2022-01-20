use super::to_any_bincode;
use std::collections::BTreeMap;

use crate::skylift_grpc::{
    triple::{Architecture, BinaryFormat, Environment, OperatingSystem, Vendor}, Triple, FlagMap
};

pub(crate) fn from_flag_map(flag_map: &BTreeMap<String, wasmtime_environ::FlagValue>) -> FlagMap {
    FlagMap {
        flags: to_any_bincode(flag_map),
    }
}

pub(crate) fn from_triple(triple: &target_lexicon::Triple) -> Triple {
    let architecture = from_architecture(&triple.architecture) as i32;
    let binary_format = from_binary_format(&triple.binary_format) as i32;
    let environment = from_environment(&triple.environment) as i32;
    let operating_system = from_operating_system(&triple.operating_system) as i32;
    let vendor = from_vendor(&triple.vendor) as i32;
    Triple {
        architecture,
        binary_format,
        environment,
        operating_system,
        vendor,
    }
}

pub(crate) fn from_architecture(architecture: &target_lexicon::Architecture) -> Architecture {
    match architecture {
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Arm) => {
            Architecture::Arm
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armeb) => {
            Architecture::Armeb
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv4) => {
            Architecture::Armv4
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv4t) => {
            Architecture::Armv4t
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv5t) => {
            Architecture::Armv5t
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv5te) => {
            Architecture::Armv5te
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv5tej) => {
            Architecture::Armv5tej
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6) => {
            Architecture::Armv6
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6j) => {
            Architecture::Armv6j
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6k) => {
            Architecture::Armv6k
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6z) => {
            Architecture::Armv6z
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6kz) => {
            Architecture::Armv6kz
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6t2) => {
            Architecture::Armv6t2
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6m) => {
            Architecture::Armv6m
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv7) => {
            Architecture::Armv7
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv7a) => {
            Architecture::Armv7a
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv7ve) => {
            Architecture::Armv7ve
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv7m) => {
            Architecture::Armv7m
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv7r) => {
            Architecture::Armv7r
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv7s) => {
            Architecture::Armv7s
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8) => {
            Architecture::Armv8
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8a) => {
            Architecture::Armv8a
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8_1a) => {
            Architecture::Armv81a
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8_2a) => {
            Architecture::Armv82a
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8_3a) => {
            Architecture::Armv83a
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8_4a) => {
            Architecture::Armv84a
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8_5a) => {
            Architecture::Armv85a
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8mBase) => {
            Architecture::Armv8mBase
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8mMain) => {
            Architecture::Armv8mMain
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8r) => {
            Architecture::Armv8r
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armebv7r) => {
            Architecture::Armebv7r
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbeb) => {
            Architecture::Thumbeb
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv4t) => {
            Architecture::Thumbv4t
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv6m) => {
            Architecture::Thumbv6m
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv7a) => {
            Architecture::Thumbv7a
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv7em) => {
            Architecture::Thumbv7em
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv7m) => {
            Architecture::Thumbv7m
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv7neon) => {
            Architecture::Thumbv7neon
        }
        target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv8mBase) => {
            Architecture::Thumbv8mBase
        }
        target_lexicon::Architecture::AmdGcn => Architecture::AmdGcn,
        target_lexicon::Architecture::Aarch64(target_lexicon::Aarch64Architecture::Aarch64) => {
            Architecture::Aarch64
        }
        target_lexicon::Architecture::Aarch64(target_lexicon::Aarch64Architecture::Aarch64be) => {
            Architecture::Aarch64be
        }
        target_lexicon::Architecture::Asmjs => Architecture::Asmjs,
        target_lexicon::Architecture::Avr => Architecture::Avr,
        target_lexicon::Architecture::Hexagon => Architecture::Hexagon,
        target_lexicon::Architecture::X86_32(target_lexicon::X86_32Architecture::I386) => {
            Architecture::I386
        }
        target_lexicon::Architecture::X86_32(target_lexicon::X86_32Architecture::I586) => {
            Architecture::I586
        }
        target_lexicon::Architecture::X86_32(target_lexicon::X86_32Architecture::I686) => {
            Architecture::I686
        }
        target_lexicon::Architecture::Mips32(target_lexicon::Mips32Architecture::Mips) => {
            Architecture::Mips
        }
        target_lexicon::Architecture::Mips32(target_lexicon::Mips32Architecture::Mipsel) => {
            Architecture::Mipsel
        }
        target_lexicon::Architecture::Mips32(target_lexicon::Mips32Architecture::Mipsisa32r6) => {
            Architecture::Mipsisa32r6
        }
        target_lexicon::Architecture::Mips32(target_lexicon::Mips32Architecture::Mipsisa32r6el) => {
            Architecture::Mipsisa32r6el
        }
        target_lexicon::Architecture::Mips64(target_lexicon::Mips64Architecture::Mips64) => {
            Architecture::Mips64
        }
        target_lexicon::Architecture::Mips64(target_lexicon::Mips64Architecture::Mips64el) => {
            Architecture::Mips64el
        }
        target_lexicon::Architecture::Mips64(target_lexicon::Mips64Architecture::Mipsisa64r6) => {
            Architecture::Mipsisa64r6
        }
        target_lexicon::Architecture::Mips64(target_lexicon::Mips64Architecture::Mipsisa64r6el) => {
            Architecture::Mipsisa64r6el
        }
        target_lexicon::Architecture::Msp430 => Architecture::Msp430,
        target_lexicon::Architecture::Nvptx64 => Architecture::Nvptx64,
        target_lexicon::Architecture::Powerpc => Architecture::Powerpc,
        target_lexicon::Architecture::Powerpc64 => Architecture::Powerpc64,
        target_lexicon::Architecture::Powerpc64le => Architecture::Powerpc64le,
        target_lexicon::Architecture::Riscv32(target_lexicon::Riscv32Architecture::Riscv32) => {
            Architecture::Riscv32
        }
        target_lexicon::Architecture::Riscv32(target_lexicon::Riscv32Architecture::Riscv32gc) => {
            Architecture::Riscv32gc
        }
        target_lexicon::Architecture::Riscv32(target_lexicon::Riscv32Architecture::Riscv32i) => {
            Architecture::Riscv32i
        }
        target_lexicon::Architecture::Riscv32(target_lexicon::Riscv32Architecture::Riscv32imac) => {
            Architecture::Riscv32imac
        }
        target_lexicon::Architecture::Riscv32(target_lexicon::Riscv32Architecture::Riscv32imc) => {
            Architecture::Riscv32imc
        }
        target_lexicon::Architecture::Riscv64(target_lexicon::Riscv64Architecture::Riscv64) => {
            Architecture::Riscv64
        }
        target_lexicon::Architecture::Riscv64(target_lexicon::Riscv64Architecture::Riscv64gc) => {
            Architecture::Riscv64gc
        }
        target_lexicon::Architecture::Riscv64(target_lexicon::Riscv64Architecture::Riscv64imac) => {
            Architecture::Riscv64imac
        }
        target_lexicon::Architecture::S390x => Architecture::S390x,
        target_lexicon::Architecture::Sparc => Architecture::Sparc,
        target_lexicon::Architecture::Sparc64 => Architecture::Sparc64,
        target_lexicon::Architecture::Sparcv9 => Architecture::Sparcv9,
        target_lexicon::Architecture::Wasm32 => Architecture::Wasm32,
        target_lexicon::Architecture::Wasm64 => Architecture::Wasm64,
        target_lexicon::Architecture::X86_64 => Architecture::X8664,
        _ => Architecture::Unknown,
    }
}

pub(crate) fn from_binary_format(bin_fmt: &target_lexicon::BinaryFormat) -> BinaryFormat {
    match bin_fmt {
        target_lexicon::BinaryFormat::Elf => BinaryFormat::Elf,
        target_lexicon::BinaryFormat::Coff => BinaryFormat::Coff,
        target_lexicon::BinaryFormat::Macho => BinaryFormat::Macho,
        target_lexicon::BinaryFormat::Wasm => BinaryFormat::Wasm,
        _ => BinaryFormat::Unknown,
    }
}

pub(crate) fn from_environment(env: &target_lexicon::Environment) -> Environment {
    match env {
        target_lexicon::Environment::AmdGiz => Environment::AmdGiz,
        target_lexicon::Environment::Android => Environment::Android,
        target_lexicon::Environment::Androideabi => Environment::Androideabi,
        target_lexicon::Environment::Eabi => Environment::Eabi,
        target_lexicon::Environment::Eabihf => Environment::Eabihf,
        target_lexicon::Environment::Gnu => Environment::Gnu,
        target_lexicon::Environment::Gnuabi64 => Environment::Gnuabi64,
        target_lexicon::Environment::Gnueabi => Environment::Gnueabi,
        target_lexicon::Environment::Gnueabihf => Environment::Gnueabihf,
        target_lexicon::Environment::Gnuspe => Environment::Gnuspe,
        target_lexicon::Environment::Gnux32 => Environment::Gnux32,
        target_lexicon::Environment::GnuIlp32 => Environment::GnuIlp32,
        target_lexicon::Environment::Macabi => Environment::Macabi,
        target_lexicon::Environment::Musl => Environment::Musl,
        target_lexicon::Environment::Musleabi => Environment::Musleabi,
        target_lexicon::Environment::Musleabihf => Environment::Musleabihf,
        target_lexicon::Environment::Muslabi64 => Environment::Muslabi64,
        target_lexicon::Environment::Msvc => Environment::Msvc,
        target_lexicon::Environment::Kernel => Environment::Kernel,
        target_lexicon::Environment::Uclibc => Environment::Uclibc,
        target_lexicon::Environment::Uclibceabi => Environment::Uclibceabi,
        target_lexicon::Environment::Sgx => Environment::Sgx,
        target_lexicon::Environment::Softfloat => Environment::Softfloat,
        target_lexicon::Environment::Spe => Environment::Spe,
        _ => Environment::Unknown,
    }
}

pub(crate) fn from_operating_system(os: &target_lexicon::OperatingSystem) -> OperatingSystem {
    match os {
        target_lexicon::OperatingSystem::AmdHsa => OperatingSystem::AmdHsa,
        target_lexicon::OperatingSystem::Bitrig => OperatingSystem::Bitrig,
        target_lexicon::OperatingSystem::Cloudabi => OperatingSystem::Cloudabi,
        target_lexicon::OperatingSystem::Cuda => OperatingSystem::Cuda,
        target_lexicon::OperatingSystem::Darwin => OperatingSystem::Darwin,
        target_lexicon::OperatingSystem::Dragonfly => OperatingSystem::Dragonfly,
        target_lexicon::OperatingSystem::Emscripten => OperatingSystem::Emscripten,
        target_lexicon::OperatingSystem::Freebsd => OperatingSystem::Freebsd,
        target_lexicon::OperatingSystem::Fuchsia => OperatingSystem::Fuchsia,
        target_lexicon::OperatingSystem::Haiku => OperatingSystem::Haiku,
        target_lexicon::OperatingSystem::Hermit => OperatingSystem::Hermit,
        target_lexicon::OperatingSystem::Illumos => OperatingSystem::Illumos,
        target_lexicon::OperatingSystem::Ios => OperatingSystem::Ios,
        target_lexicon::OperatingSystem::L4re => OperatingSystem::L4re,
        target_lexicon::OperatingSystem::Linux => OperatingSystem::Linux,
        target_lexicon::OperatingSystem::MacOSX { .. } => OperatingSystem::Macosx,
        target_lexicon::OperatingSystem::Nebulet => OperatingSystem::Nebulet,
        target_lexicon::OperatingSystem::Netbsd => OperatingSystem::Netbsd,
        target_lexicon::OperatingSystem::None_ => OperatingSystem::None,
        target_lexicon::OperatingSystem::Openbsd => OperatingSystem::Openbsd,
        target_lexicon::OperatingSystem::Psp => OperatingSystem::Psp,
        target_lexicon::OperatingSystem::Redox => OperatingSystem::Redox,
        target_lexicon::OperatingSystem::Solaris => OperatingSystem::Solaris,
        target_lexicon::OperatingSystem::Tvos => OperatingSystem::Tvos,
        target_lexicon::OperatingSystem::Uefi => OperatingSystem::Uefi,
        target_lexicon::OperatingSystem::VxWorks => OperatingSystem::VxWorks,
        target_lexicon::OperatingSystem::Wasi => OperatingSystem::Wasi,
        target_lexicon::OperatingSystem::Windows => OperatingSystem::Windows,
        _ => OperatingSystem::Unknown,
    }
}

pub(crate) fn from_vendor(vendor: &target_lexicon::Vendor) -> Vendor {
    match vendor {
        target_lexicon::Vendor::Amd => Vendor::Amd,
        target_lexicon::Vendor::Apple => Vendor::Apple,
        target_lexicon::Vendor::Experimental => Vendor::Experimental,
        target_lexicon::Vendor::Fortanix => Vendor::Fortanix,
        target_lexicon::Vendor::Nvidia => Vendor::Nvidia,
        target_lexicon::Vendor::Pc => Vendor::Pc,
        target_lexicon::Vendor::Rumprun => Vendor::Rumprun,
        target_lexicon::Vendor::Sun => Vendor::Sun,
        target_lexicon::Vendor::Uwp => Vendor::Uwp,
        target_lexicon::Vendor::Wrs => Vendor::Wrs,
        _ => Vendor::Unknown,
    }
}
