use crate::BffResult;
use crate::class::node::Node;
use crate::error::UnsupportedSourceVariantError;
use crate::names::Name;
use crate::source::classes::node::NodeSourcePart;
use crate::source::part::{Named, ToSourcePart};

impl ToSourcePart<NodeSourcePart> for Node {
    fn to_source_part(&self, name: Name) -> BffResult<NodeSourcePart> {
        match self {
            Node::NodeV1_06_63_02PC(class) => Named {
                name,
                value: class.as_ref(),
            }
            .try_into(),
            Node::NodeV1_291_03_06PC(_) => Err(UnsupportedSourceVariantError::new(name).into()),
            Node::NodeV1_381_67_09PC(_) => Err(UnsupportedSourceVariantError::new(name).into()),
        }
    }
}
