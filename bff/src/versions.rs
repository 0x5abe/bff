use derive_more::Display;
use scanf::sscanf;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Display, Clone)]
pub enum Version {
    #[display(
        fmt = "v{}.{:02}.{:02}.{:02} - Asobo Studio - Internal Cross Technology",
        "_0",
        "_1",
        "_2",
        "_3"
    )]
    Asobo(u32, u32, u32, u32),
    #[display(
        fmt = "v{}.{:02} - Asobo Studio - Internal Cross Technology",
        "_0",
        "_1"
    )]
    AsoboLegacy(u32, u32),
    #[display(
        fmt = "TotemTech Data v{}.{} (c) 1999-2002 Kalisto Entertainment - All right reserved",
        "_0",
        "_1"
    )]
    Kalisto(u32, u32),
    // The space is intentional :(
    // This format is used in Shaun White Snowboarding: World Stage by Ubisoft as well
    #[display(fmt = "Bigfile Data v{}.{} ", "_0", "_1")]
    BlackSheep(u32, u32),
    // Used in The Mighty Quest for Epic Loot by Ubisoft
    #[display(
        fmt = "Opal {}.{} BigFile | Data Version v{}.{} | CVT {} | CVANIM {} | CVMESH {} | CVSHADER {} |",
        "opal_version.0",
        "opal_version.1",
        "data_version.0",
        "data_version.1",
        "cvt",
        "cvanim",
        "cvmesh",
        "cvshader"
    )]
    Ubisoft {
        opal_version: (u32, u32),
        data_version: (u32, u32),
        cvt: u32,
        cvanim: u32,
        cvmesh: u32,
        cvshader: u32,
    },
    Other(String),
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        #![allow(clippy::just_underscores_and_digits)]
        let (mut _0, mut _1, mut _2, mut _3, mut _4, mut _5, mut _6, mut _7): (
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
        ) = Default::default();

        if sscanf!(
            value,
            "v{}.{}.{}.{} - Asobo Studio - Internal Cross Technology",
            _0, _1, _2, _3,
        )
        .is_ok()
        {
            Self::Asobo(_0, _1, _2, _3)
        } else if sscanf!(
            value,
            "v{}.{} - Asobo Studio - Internal Cross Technology",
            _0, _1
        )
        .is_ok()
        {
            Self::AsoboLegacy(_0, _1)
        } else if sscanf!(
            value,
            "TotemTech Data v{}.{} (c) 1999-2002 Kalisto Entertainment - All right reserved",
            _0, _1
        )
        .is_ok()
        {
            Self::Kalisto(_0, _1)
        } else if sscanf!(value, "Bigfile Data v{}.{} ", _0, _1).is_ok() {
            Self::BlackSheep(_0, _1)
        } else if sscanf!(value, "Opal {}.{} BigFile | Data Version v{}.{} | CVT {} | CVANIM {} | CVMESH {} | CVSHADER {} |",
            _0, _1, _2, _3, _4, _5, _6, _7).is_ok() {
            Self::Ubisoft {
                opal_version: (_0, _1),
                data_version: (_2, _3),
                cvt: _4,
                cvanim: _5,
                cvmesh: _6,
                cvshader: _7,
            }
        } else {
            Self::Other(value.to_string())
        }
    }
}

impl Serialize for Version {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        Ok(string.as_str().into())
    }
}

pub type VersionTriple = (u32, u32, u32);
