use crate::ast::control::SetBackgroundColorTag;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::{Read, Result};

pub fn read_set_background_color_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<SetBackgroundColorTag> {
    let color = reader.read_rgb()?;
    Ok(SetBackgroundColorTag { color })
}
