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
        options: &mut ReadBitmapDataOptions<Read, Color, ReadColor>,
    ) -> Result<Self> {
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
