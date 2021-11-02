pub mod rpc2internal {
    /**
     * This module converts Cap'n Proto RPC types into their respective internal representations
     * For conversion from internal representation, see `internal2rpc`.
     */
    use crate::skylift_grpc::{
        triple::{Architecture, BinaryFormat, Environment, OperatingSystem, Vendor},
        Triple,
    };

    pub(crate) fn from_triple(triple: &Triple) -> Option<target_lexicon::Triple> {
        let architecture = from_architecture(Architecture::from_i32(triple.architecture)?);
        let binary_format = from_binary_format(BinaryFormat::from_i32(triple.binary_format)?);
        let environment = from_environment(Environment::from_i32(triple.environment)?);
        let operating_system =
            from_operating_system(OperatingSystem::from_i32(triple.operating_system)?);
        let vendor = from_vendor(Vendor::from_i32(triple.environment)?);

        Some(target_lexicon::Triple {
            architecture,
            binary_format,
            environment,
            operating_system,
            vendor,
        })
    }

    pub(crate) fn from_architecture(architecture: Architecture) -> target_lexicon::Architecture {
        match architecture {
            Architecture::Unknown => target_lexicon::Architecture::Unknown,
            Architecture::Arm => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Arm)
            }
            Architecture::Armeb => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armeb)
            }
            Architecture::Armv4 => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv4)
            }
            Architecture::Armv4t => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv4t)
            }
            Architecture::Armv5t => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv5t)
            }
            Architecture::Armv5te => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv5te)
            }
            Architecture::Armv5tej => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv5tej)
            }
            Architecture::Armv6 => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6)
            }
            Architecture::Armv6j => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6j)
            }
            Architecture::Armv6k => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6k)
            }
            Architecture::Armv6z => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6z)
            }
            Architecture::Armv6kz => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6kz)
            }
            Architecture::Armv6t2 => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6t2)
            }
            Architecture::Armv6m => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6m)
            }
            Architecture::Armv7 => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv7)
            }
            Architecture::Armv7a => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv7a)
            }
            Architecture::Armv7ve => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv7ve)
            }
            Architecture::Armv7m => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv7m)
            }
            Architecture::Armv7r => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv7r)
            }
            Architecture::Armv7s => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv7s)
            }
            Architecture::Armv8 => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8)
            }
            Architecture::Armv8a => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8a)
            }
            Architecture::Armv81a => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8_1a)
            }
            Architecture::Armv82a => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8_2a)
            }
            Architecture::Armv83a => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8_3a)
            }
            Architecture::Armv84a => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8_4a)
            }
            Architecture::Armv85a => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8_5a)
            }
            Architecture::Armv8mBase => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8mBase)
            }
            Architecture::Armv8mMain => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8mMain)
            }
            Architecture::Armv8r => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8r)
            }
            Architecture::Armebv7r => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armebv7r)
            }
            Architecture::Thumbeb => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbeb)
            }
            Architecture::Thumbv4t => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv4t)
            }
            Architecture::Thumbv6m => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv6m)
            }
            Architecture::Thumbv7a => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv7a)
            }
            Architecture::Thumbv7em => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv7em)
            }
            Architecture::Thumbv7m => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv7m)
            }
            Architecture::Thumbv7neon => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv7neon)
            }
            Architecture::Thumbv8mBase => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv8mBase)
            }
            Architecture::Thumbv8mMain => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Arm)
            }
            Architecture::AmdGcn => target_lexicon::Architecture::AmdGcn,
            Architecture::Aarch64 => {
                target_lexicon::Architecture::Aarch64(target_lexicon::Aarch64Architecture::Aarch64)
            }
            Architecture::Aarch64be => target_lexicon::Architecture::Aarch64(
                target_lexicon::Aarch64Architecture::Aarch64be,
            ),
            Architecture::Asmjs => target_lexicon::Architecture::Asmjs,
            Architecture::Avr => target_lexicon::Architecture::Avr,
            Architecture::Hexagon => target_lexicon::Architecture::Hexagon,
            Architecture::I386 => {
                target_lexicon::Architecture::X86_32(target_lexicon::X86_32Architecture::I386)
            }
            Architecture::I586 => {
                target_lexicon::Architecture::X86_32(target_lexicon::X86_32Architecture::I586)
            }
            Architecture::I686 => {
                target_lexicon::Architecture::X86_32(target_lexicon::X86_32Architecture::I686)
            }
            Architecture::Mips => {
                target_lexicon::Architecture::Mips32(target_lexicon::Mips32Architecture::Mips)
            }
            Architecture::Mipsel => {
                target_lexicon::Architecture::Mips32(target_lexicon::Mips32Architecture::Mipsel)
            }
            Architecture::Mipsisa32r6 => target_lexicon::Architecture::Mips32(
                target_lexicon::Mips32Architecture::Mipsisa32r6,
            ),
            Architecture::Mipsisa32r6el => target_lexicon::Architecture::Mips32(
                target_lexicon::Mips32Architecture::Mipsisa32r6el,
            ),
            Architecture::Mips64 => {
                target_lexicon::Architecture::Mips64(target_lexicon::Mips64Architecture::Mips64)
            }
            Architecture::Mips64el => {
                target_lexicon::Architecture::Mips64(target_lexicon::Mips64Architecture::Mips64el)
            }
            Architecture::Mipsisa64r6 => target_lexicon::Architecture::Mips64(
                target_lexicon::Mips64Architecture::Mipsisa64r6,
            ),
            Architecture::Mipsisa64r6el => target_lexicon::Architecture::Mips64(
                target_lexicon::Mips64Architecture::Mipsisa64r6el,
            ),
            Architecture::Msp430 => target_lexicon::Architecture::Msp430,
            Architecture::Nvptx64 => target_lexicon::Architecture::Nvptx64,
            Architecture::Powerpc => target_lexicon::Architecture::Powerpc,
            Architecture::Powerpc64 => target_lexicon::Architecture::Powerpc64,
            Architecture::Powerpc64le => target_lexicon::Architecture::Powerpc64le,
            Architecture::Riscv32 => {
                target_lexicon::Architecture::Riscv32(target_lexicon::Riscv32Architecture::Riscv32)
            }
            Architecture::Riscv32gc => target_lexicon::Architecture::Riscv32(
                target_lexicon::Riscv32Architecture::Riscv32gc,
            ),
            Architecture::Riscv32i => {
                target_lexicon::Architecture::Riscv32(target_lexicon::Riscv32Architecture::Riscv32i)
            }
            Architecture::Riscv32imac => target_lexicon::Architecture::Riscv32(
                target_lexicon::Riscv32Architecture::Riscv32imac,
            ),
            Architecture::Riscv32imc => target_lexicon::Architecture::Riscv32(
                target_lexicon::Riscv32Architecture::Riscv32imc,
            ),
            Architecture::Riscv64 => {
                target_lexicon::Architecture::Riscv64(target_lexicon::Riscv64Architecture::Riscv64)
            }
            Architecture::Riscv64gc => target_lexicon::Architecture::Riscv64(
                target_lexicon::Riscv64Architecture::Riscv64gc,
            ),
            Architecture::Riscv64imac => target_lexicon::Architecture::Riscv64(
                target_lexicon::Riscv64Architecture::Riscv64imac,
            ),
            Architecture::S390x => target_lexicon::Architecture::S390x,
            Architecture::Sparc => target_lexicon::Architecture::Sparc,
            Architecture::Sparc64 => target_lexicon::Architecture::Sparc64,
            Architecture::Sparcv9 => target_lexicon::Architecture::Sparcv9,
            Architecture::Wasm32 => target_lexicon::Architecture::Wasm32,
            Architecture::Wasm64 => target_lexicon::Architecture::Wasm64,
            Architecture::X8664 => target_lexicon::Architecture::X86_64,
        }
    }

    pub(crate) fn from_binary_format(bin_fmt: BinaryFormat) -> target_lexicon::BinaryFormat {
        match bin_fmt {
            BinaryFormat::Unknown => target_lexicon::BinaryFormat::Unknown,
            BinaryFormat::Elf => target_lexicon::BinaryFormat::Elf,
            BinaryFormat::Coff => target_lexicon::BinaryFormat::Coff,
            BinaryFormat::Macho => target_lexicon::BinaryFormat::Macho,
            BinaryFormat::Wasm => target_lexicon::BinaryFormat::Wasm,
        }
    }

    pub(crate) fn from_environment(env: Environment) -> target_lexicon::Environment {
        match env {
            Environment::Unknown => target_lexicon::Environment::Unknown,
            Environment::AmdGiz => target_lexicon::Environment::AmdGiz,
            Environment::Android => target_lexicon::Environment::Android,
            Environment::Androideabi => target_lexicon::Environment::Androideabi,
            Environment::Eabi => target_lexicon::Environment::Eabi,
            Environment::Eabihf => target_lexicon::Environment::Eabihf,
            Environment::Gnu => target_lexicon::Environment::Gnu,
            Environment::Gnuabi64 => target_lexicon::Environment::Gnuabi64,
            Environment::Gnueabi => target_lexicon::Environment::Gnueabi,
            Environment::Gnueabihf => target_lexicon::Environment::Gnueabihf,
            Environment::Gnuspe => target_lexicon::Environment::Gnuspe,
            Environment::Gnux32 => target_lexicon::Environment::Gnux32,
            Environment::GnuIlp32 => target_lexicon::Environment::GnuIlp32,
            Environment::Macabi => target_lexicon::Environment::Macabi,
            Environment::Musl => target_lexicon::Environment::Musl,
            Environment::Musleabi => target_lexicon::Environment::Musleabi,
            Environment::Musleabihf => target_lexicon::Environment::Musleabihf,
            Environment::Muslabi64 => target_lexicon::Environment::Muslabi64,
            Environment::Msvc => target_lexicon::Environment::Msvc,
            Environment::Kernel => target_lexicon::Environment::Kernel,
            Environment::Uclibc => target_lexicon::Environment::Uclibc,
            Environment::Uclibceabi => target_lexicon::Environment::Uclibceabi,
            Environment::Sgx => target_lexicon::Environment::Sgx,
            Environment::Softfloat => target_lexicon::Environment::Softfloat,
            Environment::Spe => target_lexicon::Environment::Spe,
        }
    }

    pub(crate) fn from_operating_system(os: OperatingSystem) -> target_lexicon::OperatingSystem {
        match os {
            OperatingSystem::Unknown => target_lexicon::OperatingSystem::Unknown,
            OperatingSystem::AmdHsa => target_lexicon::OperatingSystem::AmdHsa,
            OperatingSystem::Bitrig => target_lexicon::OperatingSystem::Bitrig,
            OperatingSystem::Cloudabi => target_lexicon::OperatingSystem::Cloudabi,
            OperatingSystem::Cuda => target_lexicon::OperatingSystem::Cuda,
            OperatingSystem::Darwin => target_lexicon::OperatingSystem::Darwin,
            OperatingSystem::Dragonfly => target_lexicon::OperatingSystem::Dragonfly,
            OperatingSystem::Emscripten => target_lexicon::OperatingSystem::Emscripten,
            OperatingSystem::Freebsd => target_lexicon::OperatingSystem::Freebsd,
            OperatingSystem::Fuchsia => target_lexicon::OperatingSystem::Fuchsia,
            OperatingSystem::Haiku => target_lexicon::OperatingSystem::Haiku,
            OperatingSystem::Hermit => target_lexicon::OperatingSystem::Hermit,
            OperatingSystem::Illumos => target_lexicon::OperatingSystem::Illumos,
            OperatingSystem::Ios => target_lexicon::OperatingSystem::Ios,
            OperatingSystem::L4re => target_lexicon::OperatingSystem::L4re,
            OperatingSystem::Linux => target_lexicon::OperatingSystem::Linux,
            // Note: This is just a place holder
            OperatingSystem::Macosx => target_lexicon::OperatingSystem::MacOSX {
                major: 10,
                minor: 10,
                patch: 10,
            },
            OperatingSystem::Nebulet => target_lexicon::OperatingSystem::Nebulet,
            OperatingSystem::Netbsd => target_lexicon::OperatingSystem::Netbsd,
            OperatingSystem::None => target_lexicon::OperatingSystem::None_,
            OperatingSystem::Openbsd => target_lexicon::OperatingSystem::Openbsd,
            OperatingSystem::Psp => target_lexicon::OperatingSystem::Psp,
            OperatingSystem::Redox => target_lexicon::OperatingSystem::Redox,
            OperatingSystem::Solaris => target_lexicon::OperatingSystem::Solaris,
            OperatingSystem::Tvos => target_lexicon::OperatingSystem::Tvos,
            OperatingSystem::Uefi => target_lexicon::OperatingSystem::Uefi,
            OperatingSystem::VxWorks => target_lexicon::OperatingSystem::VxWorks,
            OperatingSystem::Wasi => target_lexicon::OperatingSystem::Wasi,
            OperatingSystem::Windows => target_lexicon::OperatingSystem::Windows,
        }
    }

    pub(crate) fn from_vendor(vendor: Vendor) -> target_lexicon::Vendor {
        match vendor {
            Vendor::Unknown => target_lexicon::Vendor::Unknown,
            Vendor::Amd => target_lexicon::Vendor::Amd,
            Vendor::Apple => target_lexicon::Vendor::Apple,
            Vendor::Experimental => target_lexicon::Vendor::Experimental,
            Vendor::Fortanix => target_lexicon::Vendor::Fortanix,
            Vendor::Nvidia => target_lexicon::Vendor::Nvidia,
            Vendor::Pc => target_lexicon::Vendor::Pc,
            Vendor::Rumprun => target_lexicon::Vendor::Rumprun,
            Vendor::Sun => target_lexicon::Vendor::Sun,
            Vendor::Uwp => target_lexicon::Vendor::Uwp,
            Vendor::Wrs => target_lexicon::Vendor::Wrs,
        }
    }
}

pub mod internal2rpc {
    use crate::skylift_grpc::{
        triple::{Architecture, BinaryFormat, Environment, OperatingSystem, Vendor},
        Triple,
    };

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

    // TODO: Implement struct mapping
    pub(crate) fn from_architecture(architecture: &target_lexicon::Architecture) -> Architecture {
        match architecture {
            _ => Architecture::Unknown,
        }
    }

    pub(crate) fn from_binary_format(bin_fmt: &target_lexicon::BinaryFormat) -> BinaryFormat {
        match bin_fmt {
            _ => BinaryFormat::Unknown,
        }
    }

    pub(crate) fn from_environment(env: &target_lexicon::Environment) -> Environment {
        match env {
            _ => Environment::Unknown,
        }
    }

    pub(crate) fn from_operating_system(os: &target_lexicon::OperatingSystem) -> OperatingSystem {
        match os {
            _ => OperatingSystem::Unknown,
        }
    }

    pub(crate) fn from_vendor(vendor: &target_lexicon::Vendor) -> Vendor {
        match vendor {
            _ => Vendor::Unknown,
        }
    }
}
