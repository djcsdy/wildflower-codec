use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct SwfVersion(pub u8);

impl SwfVersion {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Self(reader.read_u8()?))
    }
}
