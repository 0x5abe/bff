use crate::BffResult;
use crate::class::anim_frame::AnimFrame;
use crate::names::Name;
use crate::source::classes::node::AnimFrameSourcePartBuild;
use crate::source::part::{Named, ToSourcePart};

impl ToSourcePart<AnimFrameSourcePartBuild> for AnimFrame {
    fn to_source_part(&self, name: Name) -> BffResult<AnimFrameSourcePartBuild> {
        match self {
            Self::AnimFrameV1_06_63_02PC(class) => {
                Named {
                    name,
                    value: class.as_ref(),
                }
                .try_into()
            }
        }
    }
}
