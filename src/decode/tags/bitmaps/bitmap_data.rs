use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::sized_read::SizedRead;
use crate::decode::tags::bitmaps::color_map_data::ColorMapData;
use std::io::Result;

#[derive(Clone, PartialEq, Debug)]
pub enum BitmapData<Color> {
    ColorMap8(ColorMapData<Color>),
    Rgb(Vec<Color>),
}

impl<Color> BitmapData<Color> {
    pub fn read<Read: SizedRead, ReadColor: Fn(&mut Read) -> Result<Color>>(
        ReadBitmapDataOptions {
            reader,
            read_color,
            bitmap_width,
            bitmap_height,
        }: ReadBitmapDataOptions<Read, Color, ReadColor>,
    ) -> Result<Self> {
        let start = reader.position();
        let mut pixel_data = Vec::with_capacity((bitmap_height as usize) * (bitmap_width as usize));
        for _ in 0..bitmap_height {
            for _ in 0..bitmap_width {
                pixel_data.push(read_color(reader)?);
            }
            while (reader.position() - start) & 4 != 0 {
                reader.read_u8()?;
            }
        }
        Ok(Self::Rgb(pixel_data))
    }
}

pub struct ReadBitmapDataOptions<
    'reader,
    'read_color,
    Read: SizedRead,
    Color,
    ReadColor: Fn(&mut Read) -> Result<Color>,
> {
    pub reader: &'reader mut Read,
    pub read_color: &'read_color ReadColor,
    pub bitmap_width: u16,
    pub bitmap_height: u16,
}
