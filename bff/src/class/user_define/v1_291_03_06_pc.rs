use std::collections::HashMap;
use std::ffi::OsString;

use crate::BffResult;
use crate::class::trivial_class::TrivialClass;
use crate::error::Error;
use crate::helpers::{PascalString, ResourceObjectLinkHeaderV1_06_63_02PC};
use crate::source::classes::node::UserDefineSourcePart;
use crate::source::part::Named;
use crate::traits::{Artifact, Export, Import};

#[derive(..BffStruct)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct UserDefineBodyV1_291_03_06PC {
    #[serde(skip)]
    pub data: PascalString,
}

pub type UserDefineV1_291_03_06PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, UserDefineBodyV1_291_03_06PC>;

impl Export for UserDefineV1_291_03_06PC {
    fn export(&self) -> BffResult<HashMap<OsString, Artifact>> {
        Ok(HashMap::from([(
            OsString::from("data"),
            Artifact::Text(self.body.data.to_string()),
        )]))
    }
}

impl Import for UserDefineV1_291_03_06PC {
    fn import(&mut self, artifacts: &HashMap<OsString, Artifact>) -> BffResult<()> {
        let data_name = OsString::from("data");
        let Artifact::Text(data) = artifacts.get(&data_name).ok_or(Error::ImportBadArtifact)?
        else {
            return Err(Error::ImportBadArtifact);
        };
        self.body.data = PascalString::from(data.clone());
        Ok(())
    }
}

impl TryFrom<Named<'_, UserDefineV1_291_03_06PC>> for UserDefineSourcePart {
    type Error = Error;

    fn try_from(named: Named<'_, UserDefineV1_291_03_06PC>) -> Result<Self, Self::Error> {
        Ok(Self {
            name: named.name,
            data: named.value.body.data.to_string(),
        })
    }
}
