use crate::ast::styles::GradientRecord;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::{Read, Result};

fn read_gradient_record<
    R: Read,
    Color,
    ReadColor: Fn(&mut SwfTagBodyReader<R>) -> Result<Color>,
>(
    reader: &mut SwfTagBodyReader<R>,
    read_color: ReadColor,
) -> Result<GradientRecord<Color>> {
    let ratio = reader.read_u8()?;
    let color = read_color(reader)?;
    Ok(GradientRecord { ratio, color })
}
