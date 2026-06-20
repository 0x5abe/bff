use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    Mat3x4f,
    Mat4f,
    Quat,
    RGBA,
    Rect,
    ResourceObjectLinkHeaderV1_06_63_02PC,
    Sphere,
    Vec3f,
};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(..BffStruct)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct NodeBodyV1_291_03_06PC {
    parent_name: Name,
    head_child_name: Name,
    prev_node_name: Name,
    next_node_name: Name,
    object_name: Name,
    user_define_name: Name,
    light_data_name: Name,
    radiosity_bitmap_name: Name,
    unk_name: Name,
    inverse_world_transform: Mat4f,
    rot_in_world_matrix: Mat3x4f,
    inverse_rot_in_world_matrix: Mat3x4f,
    rot_in_world: Quat,
    translation: Vec3f,
    flags: u32,
    rotation: Quat,
    scale: f32,
    scale_in_world: f32,
    inv_scale_in_world: f32,
    occluder_zone_id: u32,
    color: RGBA,
    b_sphere_world: Sphere,
    display_seads_rect: Rect<u16>,
    collide_seads_rect: Rect<u16>,
    world_transform: Mat4f,
    collide_seads_id: u32,
    display_seads_id: u32,
    world_id: u16,
    start: f32,
    end: f32,
}

pub type NodeV1_291_03_06PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, NodeBodyV1_291_03_06PC>;

impl Export for NodeV1_291_03_06PC {}
impl Import for NodeV1_291_03_06PC {}
