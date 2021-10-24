use crate::skylift_capnp;

pub(crate) fn from_triple(
    triple: skylift_capnp::triple::Reader<'_>,
) -> Result<target_lexicon::Triple, ::capnp::NotInSchema> {
    let architecture = triple.get_architecture().map(from_architecture)?;
    let vendor = triple.get_vendor().map(from_vendor)?;
    let operating_system = triple.get_operating_system().map(from_operating_system)?;
    let environment = triple.get_environment().map(from_environment)?;
    let binary_format = triple.get_binary_format().map(from_binary_format)?;

    Ok(target_lexicon::Triple {
        architecture,
        vendor,
        operating_system,
        environment,
        binary_format,
    })
}

// TODO: Implement full conversion for the following
pub(crate) fn from_architecture(
    architecture: skylift_capnp::Architecture,
) -> target_lexicon::Architecture {
    match architecture {
        _ => target_lexicon::Architecture::Unknown,
    }
}

pub(crate) fn from_vendor(vendor: skylift_capnp::Vendor) -> target_lexicon::Vendor {
    match vendor {
        _ => target_lexicon::Vendor::Unknown,
    }
}

pub(crate) fn from_operating_system(
    os: skylift_capnp::OperatingSystem,
) -> target_lexicon::OperatingSystem {
    match os {
        _ => target_lexicon::OperatingSystem::Unknown,
    }
}

pub(crate) fn from_environment(env: skylift_capnp::Environment) -> target_lexicon::Environment {
    match env {
        _ => target_lexicon::Environment::Unknown,
    }
}

pub(crate) fn from_binary_format(
    bin_fmt: skylift_capnp::BinaryFormat,
) -> target_lexicon::BinaryFormat {
    match bin_fmt {
        _ => target_lexicon::BinaryFormat::Unknown,
    }
}
