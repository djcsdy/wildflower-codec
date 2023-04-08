use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::read_options::ReadOptions;
use crate::decode::sized_read::SizedRead;
use crate::decode::slice_reader::SwfSliceReader;
use crate::decode::tags::bitmaps::bitmap_data::{BitmapData, ReadBitmapDataOptions};
use crate::decode::tags::bitmaps::bitmap_format::BitmapFormat;
use crate::decode::tags::bitmaps::color_map_data::{ColorMapData, ReadColorMapDataOptions};
use crate::decode::tags::bitmaps::pix15::read_pix15;
use crate::decode::tags::bitmaps::pix24::read_pix24;
use crate::decode::tags::common::rgb::Rgb;
use inflate::DeflateDecoder;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct DefineBitsLosslessTag {
    pub character_id: u16,
    pub bitmap_width: u16,
    pub bitmap_height: u16,
    pub bitmap_data: BitmapData<Rgb>,
}

impl DefineBitsLosslessTag {
    pub fn read<R: SizedRead>(
        ReadOptions {
            reader,
            swf_version,
        }: ReadOptions<R>,
    ) -> Result<Self> {
        let character_id = reader.read_u16()?;
        let bitmap_format = BitmapFormat::read(reader)?;
        let bitmap_width = reader.read_u16()?;
        let bitmap_height = reader.read_u16()?;
        let color_table_size = if bitmap_format == BitmapFormat::ColorMap8 {
            (reader.read_u8()? as usize) + 1
        } else {
            0
        };
        let mut decompressed_bitmap_data = Vec::with_capacity(reader.remaining_bytes() * 2);
        let mut zlib_reader = DeflateDecoder::from_zlib(reader);
        zlib_reader.read_to_end(&mut decompressed_bitmap_data)?;
        let mut bitmap_data_reader = SwfSliceReader::new(&decompressed_bitmap_data, swf_version);
        let bitmap_data = match bitmap_format {
            BitmapFormat::ColorMap8 => {
                BitmapData::ColorMap8(ColorMapData::read(ReadColorMapDataOptions {
                    reader: &mut bitmap_data_reader,
                    read_color: &Rgb::read,
                    color_table_size,
                    bitmap_width,
                    bitmap_height,
                })?)
            }
            BitmapFormat::Rgb15 => BitmapData::read(ReadBitmapDataOptions {
                reader: &mut bitmap_data_reader,
                read_color: &read_pix15,
                bitmap_width,
                bitmap_height,
            })?,
            BitmapFormat::Rgb24 => BitmapData::read(ReadBitmapDataOptions {
                reader: &mut bitmap_data_reader,
                read_color: &read_pix24,
                bitmap_width,
                bitmap_height,
            })?,
        };
        Ok(Self {
            character_id,
            bitmap_width,
            bitmap_height,
            bitmap_data,
        })
    }
}
