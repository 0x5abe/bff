use std::fmt::Debug;

use crate::BffResult;
use crate::error::{UnknownSourceFlagsError, UnsupportedSourceFlagError};

#[derive(Copy, Clone, Debug)]
pub struct FlagMapping<F> {
    pub raw: u32,
    pub source: F,
}

pub fn decode_flags<F>(raw: u32, mapping: &[FlagMapping<F>]) -> BffResult<Vec<F>>
where
    F: Copy,
{
    let mut flags = Vec::new();
    let mut unknown = raw;

    for &FlagMapping {
        raw: raw_flag,
        source,
    } in mapping
    {
        if raw & raw_flag != 0 {
            flags.push(source);
            unknown &= !raw_flag;
        }
    }

    if unknown != 0 {
        return Err(UnknownSourceFlagsError::new(raw, unknown).into());
    }

    Ok(flags)
}

pub fn encode_flags<F>(flags: &[F], mapping: &[FlagMapping<F>]) -> BffResult<u32>
where
    F: Copy + Debug + Eq,
{
    let mut raw = 0;

    for &flag in flags {
        let Some(mapping) = mapping.iter().find(|mapping| mapping.source == flag) else {
            return Err(UnsupportedSourceFlagError::new(format!("{flag:?}")).into());
        };

        raw |= mapping.raw;
    }

    Ok(raw)
}
