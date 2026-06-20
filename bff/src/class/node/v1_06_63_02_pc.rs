use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames, JsonSchema)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct NodeBodyV1_06_63_02PC {
    parent_name: Name,                    // baked
    head_child_name: Name,                // y
    prev_node_name: Name,                 // baked
    next_node_name: Name,                 // y
    object_name: Name,                    // y
    user_define_name: Name,               // pulled
    radiosity_bitmap_name: Name,          // optional
    unk_name: Name,                       // n
    inverse_world_transform: Mat4f,       // baked
    rot_in_world_matrix: Mat3x4f,         // baked
    inverse_rot_in_world_matrix: Mat3x4f, // baked
    rot_in_world: Quat,                   // baked
    translation: Vec3f,                   // y
    flags: u32,                           // y
    rotation: Quat,                       // y
    scale: f32,                           // y as Vec3f cause other games have non uniform scale
    scale_in_world: f32,                  // baked
    inv_scale_in_world: f32,              // baked
    occluder_zone_id: u32,                // baked
    color: RGBA,                          // y
    b_sphere_world: Sphere,               // baked from object bsphere
    display_seads_rect: Rect<u16>,        // baked
    collide_seads_rect: Rect<u16>,        // baked
    world_transform: Mat4f,               // baked
    collide_seads_id: u32,                // baked
    display_seads_id: u32,                // baked
    world_id: u16,                        // baked
    start: f32,                           // y
    end: f32,                             // y
}

pub type NodeV1_06_63_02PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, NodeBodyV1_06_63_02PC>;

impl Export for NodeV1_06_63_02PC {}
impl Import for NodeV1_06_63_02PC {}
