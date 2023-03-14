use crate::ast::shape_morphing::{MorphFocalGradient, MorphGradient, MorphGradientRecord};
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use std::io::{Read, Result};

fn read_morph_gradient<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<MorphGradient> {
    let num_gradients = reader.read_u8()? as usize;
    let mut gradient_records = Vec::with_capacity(num_gradients);
    for _ in 0..num_gradients {
        gradient_records.push(read_morph_gradient_record(reader)?);
    }
    Ok(MorphGradient { gradient_records })
}

fn read_morph_focal_gradient<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<MorphFocalGradient> {
    let morph_gradient = read_morph_gradient(reader)?;
    let start_focal_point = reader.read_fixed8()?;
    let end_focal_point = reader.read_fixed8()?;
    Ok(MorphFocalGradient {
        gradient_records: morph_gradient.gradient_records,
        start_focal_point,
        end_focal_point,
    })
}

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
