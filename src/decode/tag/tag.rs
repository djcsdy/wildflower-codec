use crate::ast::tag::Tag;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tag::display_list::read_place_object_tag;
use std::io::{Read, Result};

pub fn read_tag<R: Read>(reader: &mut R, swf_version: u8) -> Result<Tag> {
    let tag_code_and_length = reader.read_u16()?;
    let tag_code = tag_code_and_length >> 6;
    let mut length = (tag_code_and_length & 0x3f) as usize;
    if length == 0x3f {
        length = reader.read_u32()? as usize
    };

    let mut buffer = vec![0u8; length];
    reader.read_exact(&mut buffer)?;

    let mut slice_reader = SwfSliceReader::new(&buffer, swf_version);
    let tag = match tag_code {
        4 => Tag::PlaceObject(read_place_object_tag(&mut slice_reader)?),
        _ => todo!(),
    };

    Ok(tag)
}
