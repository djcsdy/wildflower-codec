use crate::ast::control::{FrameLabelTag, SetBackgroundColorTag};
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::{Read, Result};

pub fn read_set_background_color_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<SetBackgroundColorTag> {
    let color = reader.read_rgb()?;
    Ok(SetBackgroundColorTag { color })
}

pub fn read_frame_label_tag<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<FrameLabelTag> {
    let name = reader.read_string()?;
    let named_anchor = reader.remaining() > 0 && reader.read_u8()? == 1;
    Ok(FrameLabelTag { name, named_anchor })
}
