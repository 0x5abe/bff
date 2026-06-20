use crate::BffResult;
use crate::class::user_define::UserDefine;
use crate::names::Name;
use crate::source::classes::node::UserDefineSourcePart;
use crate::source::part::{Named, ToSourcePart};

impl ToSourcePart<UserDefineSourcePart> for UserDefine {
    fn to_source_part(&self, name: Name) -> BffResult<UserDefineSourcePart> {
        match self {
            UserDefine::UserDefineV1_291_03_06PC(class) => Named {
                name,
                value: class.as_ref(),
            }
            .try_into(),
            UserDefine::UserDefineV1_381_67_09PC(class) => Named {
                name,
                value: class.as_ref(),
            }
            .try_into(),
        }
    }
}
