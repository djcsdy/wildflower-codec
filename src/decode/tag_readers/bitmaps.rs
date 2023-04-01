use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::bitmaps::define_bits::DefineBitsTag;
use crate::decode::tags::bitmaps::{
    BitmapData, ColorMapData, DefineBitsJpeg2Tag, DefineBitsJpeg3Tag, DefineBitsJpeg4Tag,
    DefineBitsLossless2Tag, DefineBitsLosslessTag, JpegTablesTag,
};
use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::common::rgb::Rgb;
use crate::decode::tags::common::rgba::Rgba;
use inflate::DeflateDecoder;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::io::ErrorKind::InvalidData;
use std::io::{Error, Read, Result};

pub fn read_define_bits_tag<R: Read>(reader: &mut R) -> Result<DefineBitsTag> {
    let character_id = reader.read_u16()?;
    let jpeg_data = reader.read_u8_to_end()?;
    Ok(DefineBitsTag {
        character_id,
        jpeg_data,
    })
}

pub fn read_jpeg_tables_tag(reader: &mut SwfSliceReader) -> Result<JpegTablesTag> {
    let jpeg_data = reader.read_u8_to_end()?;
    Ok(JpegTablesTag { jpeg_data })
}

pub fn read_define_bits_jpeg_2_tag(reader: &mut SwfSliceReader) -> Result<DefineBitsJpeg2Tag> {
    let character_id = reader.read_u16()?;
    let image_data = reader.read_u8_to_end()?;
    Ok(DefineBitsJpeg2Tag {
        character_id,
        image_data,
    })
}

pub fn read_define_bits_jpeg_3_tag(reader: &mut SwfSliceReader) -> Result<DefineBitsJpeg3Tag> {
    let character_id = reader.read_u16()?;
    let alpha_data_offset = reader.read_u32()? as usize;
    let mut image_data = vec![0u8; alpha_data_offset];
    reader.read_exact(&mut image_data)?;
    let bitmap_alpha_data = reader.read_u8_to_end()?;
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

pub fn read_define_bits_lossless_tag(reader: &mut SwfSliceReader) -> Result<DefineBitsLosslessTag> {
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
    let mut decompressed_bitmap_data = Vec::with_capacity(reader.bytes_remaining() * 2);
    let mut zlib_reader = DeflateDecoder::from_zlib(reader);
    zlib_reader.read_to_end(&mut decompressed_bitmap_data)?;
    let mut bitmap_data_reader = SwfSliceReader::new(&decompressed_bitmap_data, swf_version);
    let bitmap_data = match bitmap_format {
        BitmapFormat::ColorMap8 => {
            BitmapData::ColorMap8(read_colormap_data(ReadColorMapDataOptions {
                reader: &mut bitmap_data_reader,
                read_color: &Rgb::read,
                color_table_size,
                bitmap_width,
                bitmap_height,
            })?)
        }
        BitmapFormat::Rgb15 => read_bitmap_data(&mut ReadBitmapDataOptions {
            reader: &mut bitmap_data_reader,
            read_color: &read_pix15,
            bitmap_width,
            bitmap_height,
        })?,
        BitmapFormat::Rgb24 => read_bitmap_data(&mut ReadBitmapDataOptions {
            reader: &mut bitmap_data_reader,
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
    'reader,
    'buffer,
    'read_color,
    Color,
    ReadColor: Fn(&mut SwfSliceReader<'buffer>) -> Result<Color>,
> {
    reader: &'reader mut SwfSliceReader<'buffer>,
    read_color: &'read_color ReadColor,
    color_table_size: usize,
    bitmap_width: u16,
    bitmap_height: u16,
}

fn read_colormap_data<
    'reader,
    'buffer,
    'read_color,
    Color,
    ReadColor: Fn(&mut SwfSliceReader<'buffer>) -> Result<Color>,
>(
    options: ReadColorMapDataOptions<'reader, 'buffer, 'read_color, Color, ReadColor>,
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
    'reader,
    'buffer,
    'read_color,
    Color,
    ReadColor: Fn(&mut SwfSliceReader<'buffer>) -> Result<Color>,
> {
    reader: &'reader mut SwfSliceReader<'buffer>,
    read_color: &'read_color ReadColor,
    bitmap_width: u16,
    bitmap_height: u16,
}

fn read_bitmap_data<
    'options,
    'reader,
    'buffer,
    'read_color,
    Color,
    ReadColor: Fn(&mut SwfSliceReader<'buffer>) -> Result<Color>,
>(
    options: &'options mut ReadBitmapDataOptions<'reader, 'buffer, 'read_color, Color, ReadColor>,
) -> Result<BitmapData<Color>> {
    let start = options.reader.position();
    let mut pixel_data =
        Vec::with_capacity((options.bitmap_height as usize) * (options.bitmap_width as usize));
    for _ in 0..options.bitmap_height {
        for _ in 0..options.bitmap_width {
            pixel_data.push((options.read_color)(options.reader)?);
        }
        while (options.reader.position() - start) & 4 != 0 {
            options.reader.read_u8()?;
        }
    }
    Ok(BitmapData::Rgb(pixel_data))
}

fn read_pix15(reader: &mut SwfSliceReader) -> Result<Rgb> {
    reader.read_bit()?;
    let red = reader.read_ub8(5)? << 3;
    let green = reader.read_ub8(5)? << 3;
    let blue = reader.read_ub8(5)? << 3;
    Ok(Rgb { red, green, blue })
}

fn read_pix24(reader: &mut SwfSliceReader) -> Result<Rgb> {
    reader.read_u8()?;
    let red = reader.read_u8()?;
    let green = reader.read_u8()?;
    let blue = reader.read_u8()?;
    Ok(Rgb { red, green, blue })
}

pub fn read_define_bits_lossless_2_tag(
    reader: &mut SwfSliceReader,
) -> Result<DefineBitsLossless2Tag> {
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
    let mut decompressed_bitmap_data = Vec::with_capacity(reader.bytes_remaining() * 2);
    let mut zlib_reader = DeflateDecoder::from_zlib(reader);
    zlib_reader.read_to_end(&mut decompressed_bitmap_data)?;
    let mut bitmap_data_reader = SwfSliceReader::new(&decompressed_bitmap_data, swf_version);
    let bitmap_data = match bitmap_format {
        BitmapFormat::ColorMap8 => {
            BitmapData::ColorMap8(read_colormap_data(ReadColorMapDataOptions {
                reader: &mut bitmap_data_reader,
                read_color: &Rgba::read,
                color_table_size,
                bitmap_width,
                bitmap_height,
            })?)
        }
        BitmapFormat::Rgb15 => return Err(Error::from(InvalidData)),
        BitmapFormat::Rgb24 => read_bitmap_data(&mut ReadBitmapDataOptions {
            reader: &mut bitmap_data_reader,
            read_color: &Rgba::read_argb,
            bitmap_width,
            bitmap_height,
        })?,
    };
    Ok(DefineBitsLossless2Tag {
        character_id,
        bitmap_width,
        bitmap_height,
        bitmap_data,
    })
}

pub fn read_define_bits_jpeg_4_tag(reader: &mut SwfSliceReader) -> Result<DefineBitsJpeg4Tag> {
    let character_id = reader.read_u16()?;
    let alpha_data_offset = reader.read_u32()? as usize;
    let deblock_param = Fixed8::read(reader)?;
    let mut image_data = vec![0u8; alpha_data_offset];
    reader.read_exact(&mut image_data)?;
    let bitmap_alpha_data = reader.read_u8_to_end()?;
    Ok(DefineBitsJpeg4Tag {
        character_id,
        deblock_param,
        image_data,
        bitmap_alpha_data,
    })
}
