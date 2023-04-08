use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::common::string::String;
use crate::decode::tags::control::frame_label::FrameLabelTag;
use crate::decode::tags::metadata::file_attributes::FileAttributesTag;
use crate::decode::tags::metadata::FileAttributesFlags;
use std::io::{Read, Result};

pub fn read_frame_label_tag(reader: &mut SwfSliceReader) -> Result<FrameLabelTag> {
    let name = String::read(reader)?;
    let named_anchor = reader.bytes_remaining() > 0 && reader.read_u8()? == 1;
    Ok(FrameLabelTag { name, named_anchor })
}

pub fn read_file_attributes_tag<R: Read>(reader: &mut R) -> Result<FileAttributesTag> {
    let flags = FileAttributesFlags::from_bits_truncate(reader.read_u32()?);
    Ok(FileAttributesTag { flags })
}
