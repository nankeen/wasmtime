pub mod rpc2internal {
    /**
     * This module converts Cap'n Proto RPC types into their respective internal representations
     * For conversion from internal representation, see `internal2rpc`.
     */
    use crate::skylift_capnp;

    pub(crate) fn from_triple(
        triple: &skylift_capnp::triple::Reader<'_>,
    ) -> Result<target_lexicon::Triple, ::capnp::NotInSchema> {
        let architecture = triple.get_architecture().map(from_architecture)?;
        let binary_format = triple.get_binary_format().map(from_binary_format)?;
        let environment = triple.get_environment().map(from_environment)?;
        let operating_system = triple.get_operating_system().map(from_operating_system)?;
        let vendor = triple.get_vendor().map(from_vendor)?;

        Ok(target_lexicon::Triple {
            architecture,
            binary_format,
            environment,
            operating_system,
            vendor,
        })
    }

    pub(crate) fn from_architecture(
        architecture: skylift_capnp::Architecture,
    ) -> target_lexicon::Architecture {
        match architecture {
            skylift_capnp::Architecture::Unknown => target_lexicon::Architecture::Unknown,
            skylift_capnp::Architecture::Arm => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Arm)
            }
            skylift_capnp::Architecture::Armeb => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armeb)
            }
            skylift_capnp::Architecture::Armv4 => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv4)
            }
            skylift_capnp::Architecture::Armv4t => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv4t)
            }
            skylift_capnp::Architecture::Armv5t => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv5t)
            }
            skylift_capnp::Architecture::Armv5te => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv5te)
            }
            skylift_capnp::Architecture::Armv5tej => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv5tej)
            }
            skylift_capnp::Architecture::Armv6 => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6)
            }
            skylift_capnp::Architecture::Armv6j => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6j)
            }
            skylift_capnp::Architecture::Armv6k => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6k)
            }
            skylift_capnp::Architecture::Armv6z => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6z)
            }
            skylift_capnp::Architecture::Armv6kz => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6kz)
            }
            skylift_capnp::Architecture::Armv6t2 => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6t2)
            }
            skylift_capnp::Architecture::Armv6m => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv6m)
            }
            skylift_capnp::Architecture::Armv7 => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv7)
            }
            skylift_capnp::Architecture::Armv7a => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv7a)
            }
            skylift_capnp::Architecture::Armv7ve => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv7ve)
            }
            skylift_capnp::Architecture::Armv7m => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv7m)
            }
            skylift_capnp::Architecture::Armv7r => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv7r)
            }
            skylift_capnp::Architecture::Armv7s => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv7s)
            }
            skylift_capnp::Architecture::Armv8 => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8)
            }
            skylift_capnp::Architecture::Armv8a => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8a)
            }
            skylift_capnp::Architecture::Armv81a => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8_1a)
            }
            skylift_capnp::Architecture::Armv82a => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8_2a)
            }
            skylift_capnp::Architecture::Armv83a => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8_3a)
            }
            skylift_capnp::Architecture::Armv84a => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8_4a)
            }
            skylift_capnp::Architecture::Armv85a => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8_5a)
            }
            skylift_capnp::Architecture::Armv8mBase => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8mBase)
            }
            skylift_capnp::Architecture::Armv8mMain => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8mMain)
            }
            skylift_capnp::Architecture::Armv8r => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armv8r)
            }
            skylift_capnp::Architecture::Armebv7r => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Armebv7r)
            }
            skylift_capnp::Architecture::Thumbeb => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbeb)
            }
            skylift_capnp::Architecture::Thumbv4t => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv4t)
            }
            skylift_capnp::Architecture::Thumbv6m => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv6m)
            }
            skylift_capnp::Architecture::Thumbv7a => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv7a)
            }
            skylift_capnp::Architecture::Thumbv7em => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv7em)
            }
            skylift_capnp::Architecture::Thumbv7m => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv7m)
            }
            skylift_capnp::Architecture::Thumbv7neon => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv7neon)
            }
            skylift_capnp::Architecture::Thumbv8mBase => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Thumbv8mBase)
            }
            skylift_capnp::Architecture::Thumbv8mMain => {
                target_lexicon::Architecture::Arm(target_lexicon::ArmArchitecture::Arm)
            }
            skylift_capnp::Architecture::AmdGcn => target_lexicon::Architecture::AmdGcn,
            skylift_capnp::Architecture::Aarch64 => {
                target_lexicon::Architecture::Aarch64(target_lexicon::Aarch64Architecture::Aarch64)
            }
            skylift_capnp::Architecture::Aarch64be => target_lexicon::Architecture::Aarch64(
                target_lexicon::Aarch64Architecture::Aarch64be,
            ),
            skylift_capnp::Architecture::Asmjs => target_lexicon::Architecture::Asmjs,
            skylift_capnp::Architecture::Avr => target_lexicon::Architecture::Avr,
            skylift_capnp::Architecture::Hexagon => target_lexicon::Architecture::Hexagon,
            skylift_capnp::Architecture::I386 => {
                target_lexicon::Architecture::X86_32(target_lexicon::X86_32Architecture::I386)
            }
            skylift_capnp::Architecture::I586 => {
                target_lexicon::Architecture::X86_32(target_lexicon::X86_32Architecture::I586)
            }
            skylift_capnp::Architecture::I686 => {
                target_lexicon::Architecture::X86_32(target_lexicon::X86_32Architecture::I686)
            }
            skylift_capnp::Architecture::Mips => {
                target_lexicon::Architecture::Mips32(target_lexicon::Mips32Architecture::Mips)
            }
            skylift_capnp::Architecture::Mipsel => {
                target_lexicon::Architecture::Mips32(target_lexicon::Mips32Architecture::Mipsel)
            }
            skylift_capnp::Architecture::Mipsisa32r6 => target_lexicon::Architecture::Mips32(
                target_lexicon::Mips32Architecture::Mipsisa32r6,
            ),
            skylift_capnp::Architecture::Mipsisa32r6el => target_lexicon::Architecture::Mips32(
                target_lexicon::Mips32Architecture::Mipsisa32r6el,
            ),
            skylift_capnp::Architecture::Mips64 => {
                target_lexicon::Architecture::Mips64(target_lexicon::Mips64Architecture::Mips64)
            }
            skylift_capnp::Architecture::Mips64el => {
                target_lexicon::Architecture::Mips64(target_lexicon::Mips64Architecture::Mips64el)
            }
            skylift_capnp::Architecture::Mipsisa64r6 => target_lexicon::Architecture::Mips64(
                target_lexicon::Mips64Architecture::Mipsisa64r6,
            ),
            skylift_capnp::Architecture::Mipsisa64r6el => target_lexicon::Architecture::Mips64(
                target_lexicon::Mips64Architecture::Mipsisa64r6el,
            ),
            skylift_capnp::Architecture::Msp430 => target_lexicon::Architecture::Msp430,
            skylift_capnp::Architecture::Nvptx64 => target_lexicon::Architecture::Nvptx64,
            skylift_capnp::Architecture::Powerpc => target_lexicon::Architecture::Powerpc,
            skylift_capnp::Architecture::Powerpc64 => target_lexicon::Architecture::Powerpc64,
            skylift_capnp::Architecture::Powerpc64le => target_lexicon::Architecture::Powerpc64le,
            skylift_capnp::Architecture::Riscv32 => {
                target_lexicon::Architecture::Riscv32(target_lexicon::Riscv32Architecture::Riscv32)
            }
            skylift_capnp::Architecture::Riscv32gc => target_lexicon::Architecture::Riscv32(
                target_lexicon::Riscv32Architecture::Riscv32gc,
            ),
            skylift_capnp::Architecture::Riscv32i => {
                target_lexicon::Architecture::Riscv32(target_lexicon::Riscv32Architecture::Riscv32i)
            }
            skylift_capnp::Architecture::Riscv32imac => target_lexicon::Architecture::Riscv32(
                target_lexicon::Riscv32Architecture::Riscv32imac,
            ),
            skylift_capnp::Architecture::Riscv32imc => target_lexicon::Architecture::Riscv32(
                target_lexicon::Riscv32Architecture::Riscv32imc,
            ),
            skylift_capnp::Architecture::Riscv64 => {
                target_lexicon::Architecture::Riscv64(target_lexicon::Riscv64Architecture::Riscv64)
            }
            skylift_capnp::Architecture::Riscv64gc => target_lexicon::Architecture::Riscv64(
                target_lexicon::Riscv64Architecture::Riscv64gc,
            ),
            skylift_capnp::Architecture::Riscv64imac => target_lexicon::Architecture::Riscv64(
                target_lexicon::Riscv64Architecture::Riscv64imac,
            ),
            skylift_capnp::Architecture::S390x => target_lexicon::Architecture::S390x,
            skylift_capnp::Architecture::Sparc => target_lexicon::Architecture::Sparc,
            skylift_capnp::Architecture::Sparc64 => target_lexicon::Architecture::Sparc64,
            skylift_capnp::Architecture::Sparcv9 => target_lexicon::Architecture::Sparcv9,
            skylift_capnp::Architecture::Wasm32 => target_lexicon::Architecture::Wasm32,
            skylift_capnp::Architecture::Wasm64 => target_lexicon::Architecture::Wasm64,
            skylift_capnp::Architecture::X8664 => target_lexicon::Architecture::X86_64,
        }
    }

    pub(crate) fn from_binary_format(
        bin_fmt: skylift_capnp::BinaryFormat,
    ) -> target_lexicon::BinaryFormat {
        match bin_fmt {
            skylift_capnp::BinaryFormat::Unknown => target_lexicon::BinaryFormat::Unknown,
            skylift_capnp::BinaryFormat::Elf => target_lexicon::BinaryFormat::Elf,
            skylift_capnp::BinaryFormat::Coff => target_lexicon::BinaryFormat::Coff,
            skylift_capnp::BinaryFormat::Macho => target_lexicon::BinaryFormat::Macho,
            skylift_capnp::BinaryFormat::Wasm => target_lexicon::BinaryFormat::Wasm,
        }
    }

    pub(crate) fn from_environment(env: skylift_capnp::Environment) -> target_lexicon::Environment {
        match env {
            skylift_capnp::Environment::Unknown => target_lexicon::Environment::Unknown,
            skylift_capnp::Environment::AmdGiz => target_lexicon::Environment::AmdGiz,
            skylift_capnp::Environment::Android => target_lexicon::Environment::Android,
            skylift_capnp::Environment::Androideabi => target_lexicon::Environment::Androideabi,
            skylift_capnp::Environment::Eabi => target_lexicon::Environment::Eabi,
            skylift_capnp::Environment::Eabihf => target_lexicon::Environment::Eabihf,
            skylift_capnp::Environment::Gnu => target_lexicon::Environment::Gnu,
            skylift_capnp::Environment::Gnuabi64 => target_lexicon::Environment::Gnuabi64,
            skylift_capnp::Environment::Gnueabi => target_lexicon::Environment::Gnueabi,
            skylift_capnp::Environment::Gnueabihf => target_lexicon::Environment::Gnueabihf,
            skylift_capnp::Environment::Gnuspe => target_lexicon::Environment::Gnuspe,
            skylift_capnp::Environment::Gnux32 => target_lexicon::Environment::Gnux32,
            skylift_capnp::Environment::GnuIlp32 => target_lexicon::Environment::GnuIlp32,
            skylift_capnp::Environment::Macabi => target_lexicon::Environment::Macabi,
            skylift_capnp::Environment::Musl => target_lexicon::Environment::Musl,
            skylift_capnp::Environment::Musleabi => target_lexicon::Environment::Musleabi,
            skylift_capnp::Environment::Musleabihf => target_lexicon::Environment::Musleabihf,
            skylift_capnp::Environment::Muslabi64 => target_lexicon::Environment::Muslabi64,
            skylift_capnp::Environment::Msvc => target_lexicon::Environment::Msvc,
            skylift_capnp::Environment::Kernel => target_lexicon::Environment::Kernel,
            skylift_capnp::Environment::Uclibc => target_lexicon::Environment::Uclibc,
            skylift_capnp::Environment::Uclibceabi => target_lexicon::Environment::Uclibceabi,
            skylift_capnp::Environment::Sgx => target_lexicon::Environment::Sgx,
            skylift_capnp::Environment::Softfloat => target_lexicon::Environment::Softfloat,
            skylift_capnp::Environment::Spe => target_lexicon::Environment::Spe,
        }
    }

    pub(crate) fn from_operating_system(
        os: skylift_capnp::OperatingSystem,
    ) -> target_lexicon::OperatingSystem {
        match os {
            skylift_capnp::OperatingSystem::Unknown => target_lexicon::OperatingSystem::Unknown,
            skylift_capnp::OperatingSystem::AmdHsa => target_lexicon::OperatingSystem::AmdHsa,
            skylift_capnp::OperatingSystem::Bitrig => target_lexicon::OperatingSystem::Bitrig,
            skylift_capnp::OperatingSystem::Cloudabi => target_lexicon::OperatingSystem::Cloudabi,
            skylift_capnp::OperatingSystem::Cuda => target_lexicon::OperatingSystem::Cuda,
            skylift_capnp::OperatingSystem::Darwin => target_lexicon::OperatingSystem::Darwin,
            skylift_capnp::OperatingSystem::Dragonfly => target_lexicon::OperatingSystem::Dragonfly,
            skylift_capnp::OperatingSystem::Emscripten => {
                target_lexicon::OperatingSystem::Emscripten
            }
            skylift_capnp::OperatingSystem::Freebsd => target_lexicon::OperatingSystem::Freebsd,
            skylift_capnp::OperatingSystem::Fuchsia => target_lexicon::OperatingSystem::Fuchsia,
            skylift_capnp::OperatingSystem::Haiku => target_lexicon::OperatingSystem::Haiku,
            skylift_capnp::OperatingSystem::Hermit => target_lexicon::OperatingSystem::Hermit,
            skylift_capnp::OperatingSystem::Illumos => target_lexicon::OperatingSystem::Illumos,
            skylift_capnp::OperatingSystem::Ios => target_lexicon::OperatingSystem::Ios,
            skylift_capnp::OperatingSystem::L4re => target_lexicon::OperatingSystem::L4re,
            skylift_capnp::OperatingSystem::Linux => target_lexicon::OperatingSystem::Linux,
            // Note: This is just a place holder
            skylift_capnp::OperatingSystem::MacOSX => target_lexicon::OperatingSystem::MacOSX {
                major: 10,
                minor: 10,
                patch: 10,
            },
            skylift_capnp::OperatingSystem::Nebulet => target_lexicon::OperatingSystem::Nebulet,
            skylift_capnp::OperatingSystem::Netbsd => target_lexicon::OperatingSystem::Netbsd,
            skylift_capnp::OperatingSystem::Nonee => target_lexicon::OperatingSystem::None_,
            skylift_capnp::OperatingSystem::Openbsd => target_lexicon::OperatingSystem::Openbsd,
            skylift_capnp::OperatingSystem::Psp => target_lexicon::OperatingSystem::Psp,
            skylift_capnp::OperatingSystem::Redox => target_lexicon::OperatingSystem::Redox,
            skylift_capnp::OperatingSystem::Solaris => target_lexicon::OperatingSystem::Solaris,
            skylift_capnp::OperatingSystem::Tvos => target_lexicon::OperatingSystem::Tvos,
            skylift_capnp::OperatingSystem::Uefi => target_lexicon::OperatingSystem::Uefi,
            skylift_capnp::OperatingSystem::VxWorks => target_lexicon::OperatingSystem::VxWorks,
            skylift_capnp::OperatingSystem::Wasi => target_lexicon::OperatingSystem::Wasi,
            skylift_capnp::OperatingSystem::Windows => target_lexicon::OperatingSystem::Windows,
        }
    }

    pub(crate) fn from_vendor(vendor: skylift_capnp::Vendor) -> target_lexicon::Vendor {
        match vendor {
            skylift_capnp::Vendor::Unknown => target_lexicon::Vendor::Unknown,
            skylift_capnp::Vendor::Amd => target_lexicon::Vendor::Amd,
            skylift_capnp::Vendor::Apple => target_lexicon::Vendor::Apple,
            skylift_capnp::Vendor::Experimental => target_lexicon::Vendor::Experimental,
            skylift_capnp::Vendor::Fortanix => target_lexicon::Vendor::Fortanix,
            skylift_capnp::Vendor::Nvidia => target_lexicon::Vendor::Nvidia,
            skylift_capnp::Vendor::Pc => target_lexicon::Vendor::Pc,
            skylift_capnp::Vendor::Rumprun => target_lexicon::Vendor::Rumprun,
            skylift_capnp::Vendor::Sun => target_lexicon::Vendor::Sun,
            skylift_capnp::Vendor::Uwp => target_lexicon::Vendor::Uwp,
            skylift_capnp::Vendor::Wrs => target_lexicon::Vendor::Wrs,
        }
    }
}

pub mod internal2rpc {
    use crate::skylift_capnp;

    pub(crate) fn to_triple_builder<'a>(
        dst: &mut skylift_capnp::triple::Builder<'a>,
        src: &target_lexicon::Triple,
    ) {
        dst.set_architecture(from_architecture(&src.architecture));
        dst.set_binary_format(from_binary_format(&src.binary_format));
        dst.set_environment(from_environment(&src.environment));
        dst.set_operating_system(from_operating_system(&src.operating_system));
        dst.set_vendor(from_vendor(&src.vendor));
    }

    // TODO: Implement struct mapping
    pub(crate) fn from_architecture(
        architecture: &target_lexicon::Architecture,
    ) -> skylift_capnp::Architecture {
        match architecture {
            _ => skylift_capnp::Architecture::Unknown,
        }
    }

    pub(crate) fn from_binary_format(
        bin_fmt: &target_lexicon::BinaryFormat,
    ) -> skylift_capnp::BinaryFormat {
        match bin_fmt {
            _ => skylift_capnp::BinaryFormat::Unknown,
        }
    }

    pub(crate) fn from_environment(
        env: &target_lexicon::Environment,
    ) -> skylift_capnp::Environment {
        match env {
            _ => skylift_capnp::Environment::Unknown,
        }
    }

    pub(crate) fn from_operating_system(
        os: &target_lexicon::OperatingSystem,
    ) -> skylift_capnp::OperatingSystem {
        match os {
            _ => skylift_capnp::OperatingSystem::Unknown,
        }
    }

    pub(crate) fn from_vendor(vendor: &target_lexicon::Vendor) -> skylift_capnp::Vendor {
        match vendor {
            _ => skylift_capnp::Vendor::Unknown,
        }
    }
}
