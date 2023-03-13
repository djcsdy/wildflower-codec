use crate::ast::shape_morphing::MorphGradientRecord;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::{Read, Result};

fn read_morph_gradient_record<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<MorphGradientRecord> {
    let start_ratio = reader.read_u8()?;
    let start_color = reader.read_rgba()?;
    let end_ratio = reader.read_u8()?;
    let end_color = reader.read_rgba()?;
    Ok(MorphGradientRecord {
        start_ratio,
        start_color,
        end_ratio,
        end_color,
    })
}
