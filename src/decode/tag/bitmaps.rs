use crate::ast::bitmaps::{
    BitmapData, ColorMapData, DefineBitsJpeg2Tag, DefineBitsJpeg3Tag, DefineBitsLosslessTag,
    DefineBitsTag, JpegTablesTag,
};
use crate::ast::common::Rgb;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::tag_body_reader::SwfTagBodyReader;
use inflate::DeflateDecoder;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Result};

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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
enum BitmapFormat {
    ColorMap8 = 3,
    Rgb15 = 4,
    Rgb24 = 5,
}

pub fn read_define_bits_lossless_tag<R: Read>(
    reader: &mut SwfTagBodyReader<R>,
) -> Result<DefineBitsLosslessTag> {
    let character_id = reader.read_u16()?;
    let bitmap_format = reader
        .read_u8()?
        .try_into()
        .map_err(|_| Error::from(InvalidData))?;
    let bitmap_width = reader.read_u16()?;
    let bitmap_height = reader.read_u16()?;
    let color_table_size = if bitmap_format == BitmapFormat::ColorMap8 {
        (reader.read_u8()? as usize) + 1
    } else {
        0
    };
    let swf_version = reader.swf_version();
    let mut zlib_reader =
        SwfTagBodyReader::new(DeflateDecoder::from_zlib(reader), swf_version, usize::MAX);
    let bitmap_data = match bitmap_format {
        BitmapFormat::ColorMap8 => {
            BitmapData::ColorMap8(read_colormap_data(ReadColorMapDataOptions {
                reader: &mut zlib_reader,
                read_color: &SwfTagBodyReader::read_rgb,
                color_table_size,
                bitmap_width,
                bitmap_height,
            })?)
        }
        BitmapFormat::Rgb15 => read_bitmap_data(ReadBitmapDataOptions {
            reader: &mut zlib_reader,
            read_color: &read_pix15,
            bitmap_width,
            bitmap_height,
        })?,
        BitmapFormat::Rgb24 => read_bitmap_data(ReadBitmapDataOptions {
            reader: &mut zlib_reader,
            read_color: &read_pix24,
            bitmap_width,
            bitmap_height,
        })?,
    };
    Ok(DefineBitsLosslessTag {
        character_id,
        bitmap_width,
        bitmap_height,
        bitmap_data,
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

struct ReadBitmapDataOptions<
    'read_bitmap_data,
    R: Read,
    Color,
    ReadColor: Fn(&mut SwfTagBodyReader<R>) -> Result<Color>,
> {
    reader: &'read_bitmap_data mut SwfTagBodyReader<R>,
    read_color: ReadColor,
    bitmap_width: u16,
    bitmap_height: u16,
}

fn read_bitmap_data<R: Read, Color, ReadColor: Fn(&mut SwfTagBodyReader<R>) -> Result<Color>>(
    options: ReadBitmapDataOptions<R, Color, ReadColor>,
) -> Result<BitmapData<Color>> {
    let start = options.reader.count();
    let mut pixel_data =
        Vec::with_capacity((options.bitmap_height as usize) * (options.bitmap_width as usize));
    for _ in 0..options.bitmap_height {
        for _ in 0..options.bitmap_width {
            pixel_data.push((options.read_color)(options.reader)?);
        }
        while (options.reader.count() - start) & 4 != 0 {
            options.reader.read_u8()?;
        }
    }
    Ok(BitmapData::Rgb(pixel_data))
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
