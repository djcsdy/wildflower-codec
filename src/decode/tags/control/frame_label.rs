use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::sized_read::SizedRead;
use crate::decode::tags::common::string;
use std::io::Result;

/// Labels the current frame with the specified name.
#[derive(Clone, PartialEq, Debug)]
pub struct FrameLabelTag {
    pub name: string::String,

    pub named_anchor: bool,
}

impl FrameLabelTag {
    pub fn read<R: SizedRead>(reader: &mut R) -> Result<Self> {
        let name = string::String::read(reader)?;
        let named_anchor = reader.remaining_bytes() > 0 && reader.read_u8()? == 1;
        Ok(Self { name, named_anchor })
    }
}
