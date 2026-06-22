use std::marker::PhantomData;
use std::ops::{Div, Mul, Range, RangeInclusive, Sub};

use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite, binrw};
use derive_more::{Deref, DerefMut};
use num_traits::{CheckedAdd, Float, NumCast, PrimInt, Signed, Unsigned, cast};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::names::Name;

pub type Vec<const SIZE: usize, InnerType = f32> = [InnerType; SIZE];
pub type Vec2<T = f32> = Vec<2, T>;
pub type Vec2f = Vec2;
pub type Vec2i16 = Vec2<i16>;
pub type Vec3<T = f32> = Vec<3, T>;
pub type Vec3f = Vec3;
pub type Vec3i16 = Vec3<i16>;
pub type RGB = Vec3;
pub type Vec4<T = f32> = Vec<4, T>;
pub type Vec4f = Vec4;
pub type Vec4i16 = Vec4<i16>;
pub type Quat = Vec4;
pub type RGBA = Vec4;

pub type Mat<const ROWS: usize, const COLUMNS: usize = ROWS, InnerType = f32> =
    [[InnerType; COLUMNS]; ROWS];
pub type Mat3f = Mat<3>;
pub type Mat4f = Mat<4>;

#[derive(BinRead, BinWrite, Debug, Clone, Copy, Deref, DerefMut, ReferencedNames, JsonSchema)]
pub struct Mat3x4f(
    #[deref]
    #[deref_mut]
    #[schemars(with = "[[LosslessF32Schema; 4]; 3]")]
    pub [[f32; 4]; 3],
);

#[derive(JsonSchema)]
#[serde(untagged)]
#[allow(dead_code)]
enum LosslessF32Schema {
    Float(f32),
    Raw { raw_f32_bits: u32 },
}

#[derive(Clone, Copy)]
struct LosslessF32(f32);

#[derive(Deserialize)]
#[serde(untagged)]
enum LosslessF32Deserialize {
    Float(f32),
    Raw { raw_f32_bits: u32 },
    Null(()),
}

impl From<f32> for LosslessF32 {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<LosslessF32> for f32 {
    fn from(value: LosslessF32) -> Self {
        value.0
    }
}

impl Serialize for LosslessF32 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.0.is_finite() {
            self.0.serialize(serializer)
        } else {
            RawF32Bits {
                raw_f32_bits: self.0.to_bits(),
            }
            .serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for LosslessF32 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(match LosslessF32Deserialize::deserialize(deserializer)? {
            LosslessF32Deserialize::Float(value) => Self(value),
            LosslessF32Deserialize::Raw { raw_f32_bits } => Self(f32::from_bits(raw_f32_bits)),
            LosslessF32Deserialize::Null(()) => Self(f32::NAN),
        })
    }
}

#[derive(Serialize)]
struct RawF32Bits {
    raw_f32_bits: u32,
}

impl Serialize for Mat3x4f {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0
            .map(|row| row.map(LosslessF32::from))
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Mat3x4f {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let matrix = <[[LosslessF32; 4]; 3]>::deserialize(deserializer)?;
        Ok(Self(
            matrix.map(|row| row.map(<f32 as From<LosslessF32>>::from)),
        ))
    }
}

// A fixed precision float with a variable numerator and constant denominator.
#[derive(..BffStruct, Deref, DerefMut)]
#[br(bound(
    for<'a> T: BinRead<Args<'a>: Default> + NumCast,
    F: NumCast + Div<Output = F> + Copy + Float,
))]
#[bw(bound(
    for<'a> T: BinWrite<Args<'a>: Clone + Default> + NumCast,
    F: NumCast + Mul<Output = F> + Copy + Float,
))]
#[serde(transparent)]
pub struct NumeratorFloat<T, const DENOMINATOR: usize, F = f32>(
    #[deref_mut]
    #[deref]
    #[br(map = |x: T| cast::<T, F>(x).unwrap() / cast::<usize, F>(DENOMINATOR).unwrap())]
    #[bw(map = |x: &F| cast::<F, T>((*x * cast::<usize, F>(DENOMINATOR).unwrap()).round()).unwrap())]
    F,
    #[serde(skip)] PhantomData<T>,
);

// A fixed precision normal float between -1 and 1. (x / x.max_value()) * 2 + -1.
#[derive(..BffStruct, Deref, DerefMut)]
#[br(bound(
    for<'a> T: BinRead<Args<'a>: Default> + NumCast + Div<F, Output = F> + Unsigned + PrimInt,
    F: NumCast + Signed + Copy + Float,
))]
#[bw(bound(
    for<'a> T: BinWrite<Args<'a>: Clone + Default> + NumCast + Div<F, Output = F> + Unsigned + PrimInt,
    F: NumCast + Signed + Copy + Float,
))]
#[serde(transparent)]
pub struct SignedNormalFloat<T, F = f32>(
    #[deref_mut]
    #[deref]
    #[br(map = |x: T| (x / cast::<T, F>(T::max_value()).unwrap()).mul_add(cast::<f32, F>(2.).unwrap(), cast::<f32, F>(-1.).unwrap()))]
    #[bw(map = |x: &F| cast::<F, T>(((*x + cast::<f32, F>(1.).unwrap()) / cast::<f32, F>(2.).unwrap() * cast::<T, F>(T::max_value()).unwrap()).round()).unwrap())]
    F,
    #[serde(skip)] PhantomData<T>,
);

// Range whose first element is first and last element is last. [first, last].
// We intentionally use the names first and last instead of begin and end to avoid confusion with
// C++ iterators.
#[binrw]
#[derive(Debug, Serialize, Deref, DerefMut, Deserialize, ReferencedNames, JsonSchema)]
#[bw(bound(
    for<'a> T: BinWrite<Args<'a>: Clone + Default> + Copy,
))]
#[serde(rename = "range_inclusive")]
pub struct RangeFirstLast<T = u16> {
    #[br(temp)]
    #[bw(calc = *inner.start())]
    start: T,
    #[br(temp)]
    #[bw(calc = *inner.end())]
    end: T,
    #[br(calc = start..=end)]
    #[bw(ignore)]
    inner: RangeInclusive<T>,
}

// Range whose first element is first and contains size elements. [first, first + size).
#[binrw]
#[derive(Debug, Serialize, Deref, DerefMut, Deserialize, ReferencedNames, JsonSchema)]
#[br(bound(
    for<'a> T: BinRead<Args<'a>: Default> + Copy + CheckedAdd<Output = T>,
))]
#[bw(bound(
    for<'a> T: BinWrite<Args<'a>: Clone + Default> + Copy + Sub<Output = T>,
))]
#[serde(rename = "range")]
pub struct RangeBeginSize<T = u16> {
    #[br(temp)]
    #[bw(calc = inner.start)]
    first: T,
    #[br(temp)]
    #[bw(calc = inner.end - inner.start)]
    size: T,
    #[br(try_calc = first.checked_add(&size).map(|last| first..last).ok_or("Addition overflow in RangeBeginSize"))]
    #[bw(ignore)]
    inner: Range<T>,
}

#[derive(..BffStruct)]
pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32,
}

#[derive(..BffStruct)]
pub struct BffBox {
    pub matrix: Mat3x4f,
    pub vec: Vec3f,
    pub scale: f32,
}

#[derive(..BffStruct)]
pub struct Segment {
    pub origin: Vec3f,
    pub length: f32,
    pub direction: Vec3f,
    pub pad: f32,
}

#[derive(..BffStruct)]
pub struct Cylindre {
    pub seg: Segment,
    pub radius: f32,
}

#[derive(..BffStruct)]
pub struct DynSphere {
    pub sphere: Sphere,
    pub flags: u32,
    pub name: Name,
}

#[derive(..BffStruct)]
pub struct DynBox {
    pub matrix: Mat4f,
    pub flags: u32,
    pub name: Name,
}

#[derive(..BffStruct)]
#[br(bound(for<'a> T: BinRead<Args<'a>: Clone + Default> + 'a))]
#[bw(bound(for<'a> T: BinWrite<Args<'a>: Clone + Default> + 'a))]
pub struct Rect<T = i32> {
    pub top_left: Vec2<T>,
    pub bottom_right: Vec2<T>,
}
