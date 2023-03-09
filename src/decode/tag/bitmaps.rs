use crate::ast::bitmaps::{DefineBitsJpeg2Tag, DefineBitsJpeg3Tag, DefineBitsTag, JpegTablesTag};
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

pub fn read_jpeg_tables_tag<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<JpegTablesTag> {
    let mut jpeg_data = Vec::new();
    reader.read_to_end(&mut jpeg_data)?;
    Ok(JpegTablesTag { jpeg_data })
}

pub fn read_define_bits_jpeg2_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<DefineBitsJpeg2Tag> {
    let character_id = reader.read_u16()?;
    let mut image_data = Vec::new();
    reader.read_to_end(&mut image_data)?;
    Ok(DefineBitsJpeg2Tag {
        character_id,
        image_data,
    })
}

pub fn read_define_bits_jpeg3_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<DefineBitsJpeg3Tag> {
    let character_id = reader.read_u16()?;
    let alpha_data_offset = reader.read_u32()? as usize;
    let mut image_data = vec![0u8; alpha_data_offset];
    reader.read_exact(&mut image_data)?;
    let mut bitmap_alpha_data = Vec::new();
    reader.read_to_end(&mut bitmap_alpha_data)?;
    Ok(DefineBitsJpeg3Tag {
        character_id,
        image_data,
        bitmap_alpha_data,
    })
}
