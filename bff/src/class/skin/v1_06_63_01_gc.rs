use crate::class::trivial_class::TrivialClass;
use crate::helpers::{BffMap, DynArray, ObjectLinkHeaderV1_06_63_02PC};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(..BffStruct)]
struct Unknown1 {
    unknown1: [u8; 8],
}

#[derive(..BffStruct)]
struct BlendRelated {
    index: u32,
    blend: f32,
}

#[derive(..BffStruct)]
struct ResourceBlend {
    unknown: u16,
    blend_related1s: DynArray<BlendRelated>,
    blend_related2s: DynArray<BlendRelated>,
}

#[derive(..BffStruct)]
struct Bone {
    bone_name: Name,
    resource_blends: DynArray<ResourceBlend>,
}

#[derive(..BffStruct)]
#[br(import(_link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct SkinBodyV1_06_63_01GC {
    pub mesh_names: DynArray<Name>,
    unknown0s: DynArray<Unknown1>,
    bones: DynArray<Bone>,
    is_class_id: u8,
    #[br(if(is_class_id != 0))]
    anim_class_ids: Option<BffMap<i32, i32>>,
    #[br(if(is_class_id != 0))]
    sound_class_ids: Option<BffMap<i32, i32>>,
}

pub type SkinV1_06_63_01GC = TrivialClass<ObjectLinkHeaderV1_06_63_02PC, SkinBodyV1_06_63_01GC>;

impl Export for SkinV1_06_63_01GC {}
impl Import for SkinV1_06_63_01GC {}
