use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct ColorMapData<Color> {
    pub color_table: Vec<Color>,
    pub pixel_data: Vec<u8>,
}

impl<Color> ColorMapData<Color> {
    pub fn read<Reader: Read, ReadColor: Fn(&mut Reader) -> Result<Color>>(
        options: ReadColorMapDataOptions<Reader, Color, ReadColor>,
    ) -> Result<Self> {
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
        Ok(Self {
            color_table,
            pixel_data,
        })
    }
}

pub struct ReadColorMapDataOptions<
    'reader,
    'read_color,
    Reader: Read,
    Color,
    ReadColor: Fn(&mut Reader) -> Result<Color>,
> {
    pub reader: &'reader mut Reader,
    pub read_color: &'read_color ReadColor,
    pub color_table_size: usize,
    pub bitmap_width: u16,
    pub bitmap_height: u16,
}
