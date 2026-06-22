use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::BffResult;
use crate::class::{Class, ClassType};
use crate::error::{MissingSourceResourceError, WrongSourceClassError};
use crate::names::Name;
use crate::source::part::{SourcePart, ToSourcePart};
use crate::source::project::CookedProject;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UserDefineSourcePart {
    pub name: Name,
    pub data: String,
}

impl SourcePart for UserDefineSourcePart {
    fn from_project(name: &Name, project: &CookedProject) -> BffResult<Self> {
        let bff_class = project
            .class(name)
            .ok_or_else(|| MissingSourceResourceError::new(*name))?;

        let Class::UserDefine(user_define) = &bff_class.class else {
            return Err(WrongSourceClassError::new(
                *name,
                ClassType::UserDefine,
                bff_class.class.class_type(),
            )
            .into());
        };

        user_define.to_source_part(*name)
    }
}
