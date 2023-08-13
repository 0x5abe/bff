use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_381_67_09_pc;

use v1_381_67_09_pc::RotShapeDataV1_381_67_09PC;

bff_class!(RotShapeData {
    (V1_381_67_09, PC) => RotShapeDataV1_381_67_09PC,
});
