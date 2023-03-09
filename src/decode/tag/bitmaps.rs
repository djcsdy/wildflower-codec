use crate::ast::bitmaps::DefineBitsTag;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::{Read, Result};

pub fn read_define_bits_tag<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<DefineBitsTag> {
    let character_id = reader.read_u16()?;
    let mut jpeg_data = Vec::new();
    reader.read_to_end(&mut jpeg_data)?;
    Ok(DefineBitsTag {
        character_id,
        jpeg_data,
    })
}
