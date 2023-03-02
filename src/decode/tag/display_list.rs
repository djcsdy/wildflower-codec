use crate::ast::display_list::PlaceObjectTag;
use crate::decode::field_reader::SwfFieldReader;
use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

pub fn read_place_object_tag<R: Read>(reader: &mut SwfFieldReader<R>) -> Result<PlaceObjectTag> {
    let character_id = reader.read_u16()?;
    let depth = reader.read_u16()?;
    let matrix = reader.read_matrix()?;
    let color_transform = if reader.remaining() > 0 {
        Some(reader.read_color_transform()?)
    } else {
        None
    };

    Ok(PlaceObjectTag {
        character_id,
        depth,
        matrix,
        color_transform,
    })
}
