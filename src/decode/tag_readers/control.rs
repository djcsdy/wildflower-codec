use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::sized_read::SizedRead;
use crate::decode::tags::common::string::String;
use crate::decode::tags::control::frame_label::FrameLabelTag;
use std::io::Result;

pub fn read_frame_label_tag<R: SizedRead>(reader: &mut R) -> Result<FrameLabelTag> {
    let name = String::read(reader)?;
    let named_anchor = reader.remaining_bytes() > 0 && reader.read_u8()? == 1;
    Ok(FrameLabelTag { name, named_anchor })
}
