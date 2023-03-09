use crate::ast::bitmaps::{
    ColorMapData, DefineBitsJpeg2Tag, DefineBitsJpeg3Tag, DefineBitsTag, JpegTablesTag,
};
use crate::ast::common::Rgb;
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

struct ReadColorMapDataOptions<
    'read_color_map_data,
    R: Read,
    Color,
    ReadColor: Fn(&mut SwfTagBodyReader<R>) -> Result<Color>,
> {
    reader: &'read_color_map_data mut SwfTagBodyReader<R>,
    read_color: ReadColor,
    color_table_size: usize,
    bitmap_width: u16,
    bitmap_height: u16,
}

fn read_colormap_data<R: Read, Color, ReadColor: Fn(&mut SwfTagBodyReader<R>) -> Result<Color>>(
    options: ReadColorMapDataOptions<R, Color, ReadColor>,
) -> Result<ColorMapData<Color>> {
    let mut color_table = Vec::with_capacity(options.color_table_size);
    for _ in 0..options.color_table_size {
        color_table.push((options.read_color)(options.reader)?);
    }
    let mut pixel_data =
        Vec::with_capacity((options.bitmap_width as usize) * (options.bitmap_height as usize));
    for _ in 0..options.bitmap_height {
        for _ in 0..((options.bitmap_width + 3) & 4) {
            pixel_data.push(options.reader.read_u8()?);
        }
    }
    Ok(ColorMapData {
        color_table,
        pixel_data,
    })
}

fn read_pix15<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<Rgb> {
    reader.read_bit()?;
    let red = reader.read_ub8(5)? << 3;
    let green = reader.read_ub8(5)? << 3;
    let blue = reader.read_ub8(5)? << 3;
    Ok(Rgb { red, green, blue })
}

fn read_pix24<R: Read>(reader: &mut SwfTagBodyReader<R>) -> Result<Rgb> {
    reader.read_u8()?;
    let red = reader.read_u8()?;
    let green = reader.read_u8()?;
    let blue = reader.read_u8()?;
    Ok(Rgb { red, green, blue })
}
