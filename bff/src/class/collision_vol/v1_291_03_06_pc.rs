use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, Mat4f, ObjectLinkHeaderV1_06_63_02PC};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(..BffStruct)]
struct CollisionVolInfo {
    local_transform: Mat4f,
    inv_local_transform: Mat4f,
}

#[derive(..BffStruct)]
#[br(import(_link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct CollisionVolBodyV1_291_03_06PC {
    infos: DynArray<CollisionVolInfo>,
    in_message_id: u32,
    out_message_id: u32,
    node_name_params: [Name; 12],
    float_params: [f32; 12],
    anim_frame_names: DynArray<Name>,
    material_anim_names: DynArray<Name>,
    agent_name: Name,
    anim_start_time: f32,
}

pub type CollisionVolV1_291_03_06PC =
    TrivialClass<ObjectLinkHeaderV1_06_63_02PC, CollisionVolBodyV1_291_03_06PC>;

impl Export for CollisionVolV1_291_03_06PC {}
impl Import for CollisionVolV1_291_03_06PC {}
