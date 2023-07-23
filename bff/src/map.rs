use std::convert::TryFrom;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

use binrw::BinRead;
use derive_more::Deref;
use indexmap::IndexMap;
use serde::Serialize;

use crate::dynarray::DynArray;

#[derive(Debug, Serialize, BinRead, Deref)]
#[serde(transparent)]
pub struct BffMap<KeyType, ValueType, SizeType = u32>
where
    for<'a> KeyType: Hash + Eq + 'a,

    for<'a> ValueType: 'a,

    for<'a> (KeyType, ValueType): BinRead + Serialize + 'a,
    for<'a> <(KeyType, ValueType) as BinRead>::Args<'a>: Clone + Default,

    Vec<(KeyType, ValueType)>: Copy,

    SizeType: BinRead + Debug + Copy,
    for<'a> <SizeType as BinRead>::Args<'a>: Default,
    usize: TryFrom<SizeType>,
{
    #[deref]
    #[br(map = |pairs: DynArray<(KeyType, ValueType), SizeType>| {
        pairs.into_iter().collect::<IndexMap<_, _>>()
    })]
    map: IndexMap<KeyType, ValueType>,
    #[serde(skip)]
    _phantom: PhantomData<SizeType>,
}

impl<KeyType, ValueType, SizeType> From<IndexMap<KeyType, ValueType>>
    for BffMap<KeyType, ValueType, SizeType>
where
    for<'a> KeyType: Hash + Eq + 'a,

    for<'a> ValueType: 'a,

    for<'a> (KeyType, ValueType): BinRead + Serialize + 'a,
    for<'a> <(KeyType, ValueType) as BinRead>::Args<'a>: Clone + Default,

    Vec<(KeyType, ValueType)>: Copy,

    SizeType: BinRead + Debug + Copy,
    for<'a> <SizeType as BinRead>::Args<'a>: Default,
    usize: TryFrom<SizeType>,
    <usize as TryFrom<SizeType>>::Error: Debug,
{
    fn from(map: IndexMap<KeyType, ValueType>) -> Self {
        Self {
            map,
            _phantom: PhantomData,
        }
    }
}
