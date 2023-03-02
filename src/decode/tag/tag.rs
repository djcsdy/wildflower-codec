use crate::ast::tag::Tag;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag::display_list::read_place_object_tag;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::{Read, Result};

pub fn read_tag<R: Read>(reader: &mut R) -> Result<Tag> {
    let tag_code_and_length = reader.read_u16()?;
    let tag_code = tag_code_and_length >> 6;
    let mut length = (tag_code_and_length & 0x3f) as u32;
    if length == 0x3f {
        length = reader.read_u32()?
    };

    let mut tag_body_reader = SwfTagBodyReader::new(reader, length as usize);
    let tag = match tag_code {
        4 => Tag::PlaceObject(read_place_object_tag(&mut tag_body_reader)?),
        _ => todo!(),
    };
    tag_body_reader.skip_to_end()?;

    Ok(tag)
}
