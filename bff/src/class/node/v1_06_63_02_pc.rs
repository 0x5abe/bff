use bff_derive::ReferencedNames;
use bilge::prelude::*;
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
use crate::source::classes::node::{NodeFlag, NodeFlags, NodeSourcePart, NodeSourceParts};
use crate::source::flags::{FlagMapping, decode_flags};
use crate::source::part::Named;
use crate::traits::{Export, Import};

#[bitsize(32)]
#[derive(
    BinRead, DebugBits, SerializeBits, BinWrite, DeserializeBits, ReferencedNames, JsonSchemaBits,
)]
struct NodeFlagsV1_06_63_02PC {
    fl_node_scan: u1,
    fl_node_unk_0x2: u1,
    fl_node_unk_0x4: u1,
    fl_node_unk_0x8: u1,
    fl_node_update: u1,
    fl_node_update_lighting: u1,
    fl_node_update_object: u1,
    fl_node_invalidmat: u1,
    fl_node_invalidrot: u1,
    fl_node_animated: u1,
    fl_node_no_omni: u1,
    fl_node_no_occluder: u1,
    fl_node_no_agent: u1,
    fl_node_sequenced: u1,
    fl_node_skinned: u1,
    fl_node_uncollided: u1,
    fl_node_no_seadcollide: u1,
    fl_node_no_seaddisplay: u1,
    fl_node_hide: u1,
    fl_node_user_lock: u1,
    fl_node_vp0_hide: u1,
    fl_node_vp1_hide: u1,
    fl_node_vp2_hide: u1,
    fl_node_vp3_hide: u1,
    fl_node_no_unshared: u1,
    fl_node_unk_0x2000000: u1,
    fl_node_collide: u1,
    fl_node_shadow: u1,
    fl_node_sequenced_abort: u1,
    fl_node_special_vision: u1,
    unknown_0xc0000000: u2,
}

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
    flags: NodeFlagsV1_06_63_02PC,        // y
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

impl TryFrom<Named<'_, NodeV1_06_63_02PC>> for NodeSourceParts {
    type Error = Error;

    fn try_from(named: Named<'_, NodeV1_06_63_02PC>) -> Result<Self, Self::Error> {
        let body = &named.value.body;
        let user_define_name =
            (!body.user_define_name.is_default()).then_some(body.user_define_name);
        let radiosity_bitmap_name =
            (!body.radiosity_bitmap_name.is_default()).then_some(body.radiosity_bitmap_name);

        Ok(Self {
            node: NodeSourcePart {
                head_child_name: body.head_child_name,
                next_node_name: body.next_node_name,
                object_name: body.object_name,
                radiosity_bitmap_name,
                translation: body.translation,
                rotation: body.rotation,
                scale: [body.scale; 3],
                flags: NodeFlags {
                    flags: decode_flags(node_flags_raw(&body.flags), NODE_FLAGS_V1_06_63_02_PC)?,
                },
                color: body.color,
                start: body.start,
                end: body.end,
            },
            user_define_name,
            represented_resources: vec![named.name],
            preserved: Vec::new(),
        })
    }
}

fn node_flags_raw(flags: &NodeFlagsV1_06_63_02PC) -> u32 {
    let mut raw = 0;

    if u8::from(flags.fl_node_scan()) != 0 {
        raw |= 1 << 0;
    }
    if u8::from(flags.fl_node_unk_0x2()) != 0 {
        raw |= 1 << 1;
    }
    if u8::from(flags.fl_node_unk_0x4()) != 0 {
        raw |= 1 << 2;
    }
    if u8::from(flags.fl_node_unk_0x8()) != 0 {
        raw |= 1 << 3;
    }
    if u8::from(flags.fl_node_update()) != 0 {
        raw |= 1 << 4;
    }
    if u8::from(flags.fl_node_update_lighting()) != 0 {
        raw |= 1 << 5;
    }
    if u8::from(flags.fl_node_update_object()) != 0 {
        raw |= 1 << 6;
    }
    if u8::from(flags.fl_node_invalidmat()) != 0 {
        raw |= 1 << 7;
    }
    if u8::from(flags.fl_node_invalidrot()) != 0 {
        raw |= 1 << 8;
    }
    if u8::from(flags.fl_node_animated()) != 0 {
        raw |= 1 << 9;
    }
    if u8::from(flags.fl_node_no_omni()) != 0 {
        raw |= 1 << 10;
    }
    if u8::from(flags.fl_node_no_occluder()) != 0 {
        raw |= 1 << 11;
    }
    if u8::from(flags.fl_node_no_agent()) != 0 {
        raw |= 1 << 12;
    }
    if u8::from(flags.fl_node_sequenced()) != 0 {
        raw |= 1 << 13;
    }
    if u8::from(flags.fl_node_skinned()) != 0 {
        raw |= 1 << 14;
    }
    if u8::from(flags.fl_node_uncollided()) != 0 {
        raw |= 1 << 15;
    }
    if u8::from(flags.fl_node_no_seadcollide()) != 0 {
        raw |= 1 << 16;
    }
    if u8::from(flags.fl_node_no_seaddisplay()) != 0 {
        raw |= 1 << 17;
    }
    if u8::from(flags.fl_node_hide()) != 0 {
        raw |= 1 << 18;
    }
    if u8::from(flags.fl_node_user_lock()) != 0 {
        raw |= 1 << 19;
    }
    if u8::from(flags.fl_node_vp0_hide()) != 0 {
        raw |= 1 << 20;
    }
    if u8::from(flags.fl_node_vp1_hide()) != 0 {
        raw |= 1 << 21;
    }
    if u8::from(flags.fl_node_vp2_hide()) != 0 {
        raw |= 1 << 22;
    }
    if u8::from(flags.fl_node_vp3_hide()) != 0 {
        raw |= 1 << 23;
    }
    if u8::from(flags.fl_node_no_unshared()) != 0 {
        raw |= 1 << 24;
    }
    if u8::from(flags.fl_node_unk_0x2000000()) != 0 {
        raw |= 1 << 25;
    }
    if u8::from(flags.fl_node_collide()) != 0 {
        raw |= 1 << 26;
    }
    if u8::from(flags.fl_node_shadow()) != 0 {
        raw |= 1 << 27;
    }
    if u8::from(flags.fl_node_sequenced_abort()) != 0 {
        raw |= 1 << 28;
    }
    if u8::from(flags.fl_node_special_vision()) != 0 {
        raw |= 1 << 29;
    }

    raw | (u32::from(u8::from(flags.unknown_0xc0000000())) << 30)
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
