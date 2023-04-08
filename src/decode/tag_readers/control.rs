use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::common::string::String;
use crate::decode::tags::control::frame_label::FrameLabelTag;
use std::io::Result;

pub fn read_frame_label_tag(reader: &mut SwfSliceReader) -> Result<FrameLabelTag> {
    let name = String::read(reader)?;
    let named_anchor = reader.bytes_remaining() > 0 && reader.read_u8()? == 1;
    Ok(FrameLabelTag { name, named_anchor })
}
