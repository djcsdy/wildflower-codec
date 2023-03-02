use crate::ast::tag::Tag;
use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

pub fn read_tag<R: Read>(reader: &mut R) -> Result<Tag> {
    let tag_code_and_length = reader.read_u16()?;
    let tag_code = tag_code_and_length >> 6;
    let mut length = (tag_code_and_length & 0x3f) as u32;
    if length == 0x3f {
        length = reader.read_u32()?
    };

    match tag_code {
        _ => todo!(),
    }
}
