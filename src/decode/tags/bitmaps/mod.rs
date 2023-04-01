pub mod color_map_data;
pub mod define_bits;
pub mod define_bits_jpeg_2;
pub mod define_bits_jpeg_3;
pub mod define_bits_jpeg_4;
pub mod define_bits_lossless;
pub mod jpeg_tables;

use crate::decode::tags::common::rgba::Rgba;
use color_map_data::ColorMapData;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineBitsLossless2Tag {
    pub character_id: u16,
    pub bitmap_width: u16,
    pub bitmap_height: u16,
    pub bitmap_data: BitmapData<Rgba>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum BitmapData<TColor> {
    ColorMap8(ColorMapData<TColor>),
    Rgb(Vec<TColor>),
}
