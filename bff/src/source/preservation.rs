use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::ClassType;
use crate::names::Name;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct PreservedFragment {
    pub resource_name: Name,
    pub kind: PreservedFragmentKind,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PreservedFragmentKind {
    UnsupportedSourcePart {
        class_type: ClassType,
        label: String,
    },
}
