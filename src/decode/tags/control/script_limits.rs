use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

/// Overrides the default limits for AVM scripts.
#[derive(Clone, PartialEq, Debug)]
pub struct ScriptLimitsTag {
    pub max_recursion_depth: u16,
    pub script_timeout_seconds: u16,
}

impl ScriptLimitsTag {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let max_recursion_depth = reader.read_u16()?;
        let script_timeout_seconds = reader.read_u16()?;
        Ok(Self {
            max_recursion_depth,
            script_timeout_seconds,
        })
    }
}
