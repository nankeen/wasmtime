@0xc9102ed568f73baa;

# TODO: Comments

# Different kinds of `Setting` values that can be configured in a
# `CompilerBuilder`
enum SettingKind {
    # The setting is an enumeration, meaning it's one of a set of values.
    enum @0;
    # The setting is a number.
    num @1;
    # The setting is a boolean.
    bool @2;
    # The setting is a preset.
    preset @3;
}

# Description of compiler settings.
struct Setting {
    name @0 :Text;
    description @1 :Text;
    kind @2 :SettingKind;
    values @3 :List(Text);
}

interface Compiler {
    # TODO: Compiler interface
}

# The "architecture" field.
enum Architecture {
    unknown @0;
    arm @1;
    armeb @2;
    armv4 @3;
    armv4t @4;
    armv5t @5;
    armv5te @6;
    armv5tej @7;
    armv6 @8;
    armv6j @9;
    armv6k @10;
    armv6z @11;
    armv6kz @12;
    armv6t2 @13;
    armv6m @14;
    armv7 @15;
    armv7a @16;
    armv7ve @17;
    armv7m @18;
    armv7r @19;
    armv7s @20;
    armv8 @21;
    armv8a @22;
    armv81a @23;
    armv82a @24;
    armv83a @25;
    armv84a @26;
    armv85a @27;
    armv8mBase @28;
    armv8mMain @29;
    armv8r @30;
    armebv7r @31;
    thumbeb @32;
    thumbv4t @33;
    thumbv6m @34;
    thumbv7a @35;
    thumbv7em @36;
    thumbv7m @37;
    thumbv7neon @38;
    thumbv8mBase @39;
    thumbv8mMain @40;
    amdGcn @41;
    aarch64 @42;
    aarch64be @43;
    asmjs @44;
    avr @45;
    hexagon @46;
    i386 @47;
    i586 @48;
    i686 @49;
    mips @50;
    mipsel @51;
    mipsisa32r6 @52;
    mipsisa32r6el @53;
    mips64 @54;
    mips64el @55;
    mipsisa64r6 @56;
    mipsisa64r6el @57;
    msp430 @58;
    nvptx64 @59;
    powerpc @60;
    powerpc64 @61;
    powerpc64le @62;
    riscv32 @63;
    riscv32gc @64;
    riscv32i @65;
    riscv32imac @66;
    riscv32imc @67;
    riscv64 @68;
    riscv64gc @69;
    riscv64imac @70;
    s390x @71;
    sparc @72;
    sparc64 @73;
    sparcv9 @74;
    wasm32 @75;
    wasm64 @76;
    x8664 @77;
}

enum Vendor {
    unknown @0;
    amd @1;
    apple @2;
    experimental @3;
    fortanix @4;
    nvidia @5;
    pc @6;
    rumprun @7;
    sun @8;
    uwp @9;
    wrs @10;
}

enum OperatingSystem {
    unknown @0;
    amdHsa @1;
    bitrig @2;
    cloudabi @3;
    cuda @4;
    darwin @5;
    dragonfly @6;
    emscripten @7;
    freebsd @8;
    fuchsia @9;
    haiku @10;
    hermit @11;
    illumos @12;
    ios @13;
    l4re @14;
    linux @15;
    macOSX @16;
    nebulet @17;
    netbsd @18;
    none @19;
    openbsd @20;
    psp @21;
    redox @22;
    solaris @23;
    tvos @24;
    uefi @25;
    vxWorks @26;
    wasi @27;
    windows @28;
}

enum Environment {
    unknown @0;
    amdGiz @1;
    android @2;
    androideabi @3;
    eabi @4;
    eabihf @5;
    gnu @6;
    gnuabi64 @7;
    gnueabi @8;
    gnueabihf @9;
    gnuspe @10;
    gnux32 @11;
    gnuIlp32 @12;
    macabi @13;
    musl @14;
    musleabi @15;
    musleabihf @16;
    muslabi64 @17;
    msvc @18;
    kernel @19;
    uclibc @20;
    uclibceabi @21;
    sgx @22;
    softfloat @23;
    spe @24;
}

enum BinaryFormat {
    unknown @0;
    elf @1;
    coff @2;
    macho @3;
    wasm @4;
}

# A target "triple". Historically such things had three fields, though they've
# added additional fields over time.
struct Triple {
    # The "architecture" (and sometimes the subarchitecture).
    architecture @0 :Architecture;

    # The "vendor" (whatever that means).
    vendor @1 :Vendor;

    # The "operating system" (sometimes also the environment).
    operatingSystem @2 :OperatingSystem;

    # The "environment" on top of the operating system (often omitted for
    # operating systems with a single predominant environment).
    environment @3 :Environment;

    # The "binary format" (rarely used).
    binaryFormat @4 :BinaryFormat;
}

interface CompilerBuilder {
    # Sets the target of compilation to the target specified.
    target @0 (target :Triple);

    # Returns the currently configured target triple that compilation will
    # produce artifacts for.
    triple @1 () -> (triple :Triple);

    # Compiler-specific method to configure various settings in the compiler
    # itself.
    # This is expected to be defined per-compiler. Compilers should return
    # errors for unknown names/values.
    set @2 (name :Text, val :Text);

    # Compiler-specific method for configuring settings.
    # Same as `set` except it is exclusively for enabling boolean flags.
    enable @3 (name :Text);

    # Returns a list of all possible settings that can be configured with
    # `set` and `enable`.
    settings @4 () -> (settings :List(Setting));

    # Builds a new [`Compiler`] object from this configuration.
    build @5 () -> (compiler: Compiler);
}
