use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::error::Error;
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
use crate::source::classes::node::{NodeFlag, NodeFlags, NodeSourcePart};
use crate::source::flags::{FlagMapping, decode_flags};
use crate::source::part::Named;
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

impl TryFrom<Named<'_, NodeV1_06_63_02PC>> for NodeSourcePart {
    type Error = Error;

    fn try_from(named: Named<'_, NodeV1_06_63_02PC>) -> Result<Self, Self::Error> {
        let body = &named.value.body;
        let radiosity_bitmap_name =
            (!body.radiosity_bitmap_name.is_default()).then_some(body.radiosity_bitmap_name);

        Ok(Self {
            name: named.name,
            head_child_name: body.head_child_name,
            next_node_name: body.next_node_name,
            object_name: body.object_name,
            radiosity_bitmap_name,
            translation: body.translation,
            rotation: body.rotation,
            scale: [body.scale; 3],
            flags: NodeFlags {
                flags: decode_flags(body.flags, NODE_FLAGS_V1_06_63_02_PC)?,
            },
            color: body.color,
            start: body.start,
            end: body.end,
        })
    }
}

const NODE_FLAGS_V1_06_63_02_PC: &[FlagMapping<NodeFlag>] = &[
    FlagMapping {
        raw: 1 << 0,
        source: NodeFlag::Scan,
    },
    FlagMapping {
        raw: 1 << 1,
        source: NodeFlag::Unknown0x2,
    },
    FlagMapping {
        raw: 1 << 2,
        source: NodeFlag::Unknown0x4,
    },
    FlagMapping {
        raw: 1 << 3,
        source: NodeFlag::Unknown0x8,
    },
    FlagMapping {
        raw: 1 << 4,
        source: NodeFlag::Update,
    },
    FlagMapping {
        raw: 1 << 5,
        source: NodeFlag::UpdateLighting,
    },
    FlagMapping {
        raw: 1 << 6,
        source: NodeFlag::UpdateObject,
    },
    FlagMapping {
        raw: 1 << 7,
        source: NodeFlag::InvalidMatrix,
    },
    FlagMapping {
        raw: 1 << 8,
        source: NodeFlag::InvalidRotation,
    },
    FlagMapping {
        raw: 1 << 9,
        source: NodeFlag::Animated,
    },
    FlagMapping {
        raw: 1 << 10,
        source: NodeFlag::NoOmni,
    },
    FlagMapping {
        raw: 1 << 11,
        source: NodeFlag::NoOccluder,
    },
    FlagMapping {
        raw: 1 << 12,
        source: NodeFlag::NoAgent,
    },
    FlagMapping {
        raw: 1 << 13,
        source: NodeFlag::Sequenced,
    },
    FlagMapping {
        raw: 1 << 14,
        source: NodeFlag::Skinned,
    },
    FlagMapping {
        raw: 1 << 15,
        source: NodeFlag::Uncollided,
    },
    FlagMapping {
        raw: 1 << 16,
        source: NodeFlag::NoSeadCollide,
    },
    FlagMapping {
        raw: 1 << 17,
        source: NodeFlag::NoSeadDisplay,
    },
    FlagMapping {
        raw: 1 << 18,
        source: NodeFlag::Hide,
    },
    FlagMapping {
        raw: 1 << 19,
        source: NodeFlag::UserLock,
    },
    FlagMapping {
        raw: 1 << 20,
        source: NodeFlag::Vp0Hide,
    },
    FlagMapping {
        raw: 1 << 21,
        source: NodeFlag::Vp1Hide,
    },
    FlagMapping {
        raw: 1 << 22,
        source: NodeFlag::Vp2Hide,
    },
    FlagMapping {
        raw: 1 << 23,
        source: NodeFlag::Vp3Hide,
    },
    FlagMapping {
        raw: 1 << 24,
        source: NodeFlag::NoUnshared,
    },
    FlagMapping {
        raw: 1 << 25,
        source: NodeFlag::Unknown0x2000000,
    },
    FlagMapping {
        raw: 1 << 26,
        source: NodeFlag::Collide,
    },
    FlagMapping {
        raw: 1 << 27,
        source: NodeFlag::Shadow,
    },
    FlagMapping {
        raw: 1 << 28,
        source: NodeFlag::SequencedAbort,
    },
    FlagMapping {
        raw: 1 << 29,
        source: NodeFlag::SpecialVision,
    },
];
