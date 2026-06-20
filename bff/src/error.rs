use std::ffi::OsString;

use derive_more::{Constructor, Display, Error, From};

use crate::bigfile::platforms::{Platform, Style};
use crate::bigfile::versions::Version;
use crate::class::ClassType;
use crate::names::Name;

#[derive(Debug, Constructor, Display, Error)]
#[display(
    "unimplemented class {} (version: {}, platform: {}) for resource {}",
    class_name,
    version,
    platform,
    resource_name
)]
pub struct UnimplementedClassError {
    pub resource_name: Name,
    pub class_name: Name,
    pub version: Version,
    pub platform: Platform,
}

#[derive(Debug, Constructor, Display, Error)]
#[display(
    "Unsupported BigFile version, platform combination: {}, {}",
    version,
    platform
)]
pub struct UnimplementedVersionPlatformError {
    pub version: Version,
    pub platform: Platform,
}

#[derive(Debug, Constructor, Display, Error)]
#[display("Unsupported BigFile version: {}", version)]
pub struct UnimplementedVersionError {
    pub version: Version,
}

#[derive(Debug, Constructor, Display, Error)]
#[display("Invalid BigFile extension {:#?}", extension)]
pub struct InvalidExtensionError {
    pub extension: OsString,
}

#[derive(Debug, Constructor, Display, Error)]
#[display("Invalid Platform/Style combination: {} {}", platform, style)]
pub struct InvalidPlatformStyleError {
    pub platform: Platform,
    pub style: Style,
}

#[derive(Debug, Constructor, Display, Error)]
#[display("Missing cooked source resource {}", name)]
pub struct MissingSourceResourceError {
    pub name: Name,
}

#[derive(Debug, Constructor, Display, Error)]
#[display(
    "Wrong source resource class for {}: expected {:?}, got {:?}",
    name,
    expected,
    actual
)]
pub struct WrongSourceClassError {
    pub name: Name,
    pub expected: ClassType,
    pub actual: ClassType,
}

#[derive(Debug, Constructor, Display, Error)]
#[display("Unsupported source resource variant for {}", name)]
pub struct UnsupportedSourceVariantError {
    pub name: Name,
}

#[derive(Debug, Constructor, Display, Error)]
#[display("Unknown source flag bits {:#x} in raw flags {:#x}", unknown, raw)]
pub struct UnknownSourceFlagsError {
    pub raw: u32,
    pub unknown: u32,
}

#[derive(Debug, Constructor, Display, Error)]
#[display("Unsupported source flag {}", flag)]
pub struct UnsupportedSourceFlagError {
    pub flag: String,
}

#[derive(Debug, Display, Error, From)]
pub enum Error {
    BinRW(binrw::Error),
    Fmt(std::fmt::Error),
    InvalidExtension(InvalidExtensionError),
    InvalidPlatformStyle(InvalidPlatformStyleError),
    Io(std::io::Error),
    ParseInt(std::num::ParseIntError),
    UnimplementedClass(UnimplementedClassError),
    UnimplementedVersion(UnimplementedVersionError),
    UnimplementedVersionPlatform(UnimplementedVersionPlatformError),
    Utf8(std::string::FromUtf8Error),
    UnimplementedImportExport,
    ImportBadArtifact,
    UnconsumedInput,
    MissingSourceResource(MissingSourceResourceError),
    WrongSourceClass(WrongSourceClassError),
    UnsupportedSourceVariant(UnsupportedSourceVariantError),
    UnknownSourceFlags(UnknownSourceFlagsError),
    UnsupportedSourceFlag(UnsupportedSourceFlagError),
}
