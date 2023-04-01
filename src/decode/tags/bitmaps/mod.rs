pub mod define_bits;
pub mod define_bits_jpeg_2;
pub mod jpeg_tables;

use crate::decode::tags::common::fixed_8::Fixed8;
use crate::decode::tags::common::rgb::Rgb;
use crate::decode::tags::common::rgba::Rgba;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineBitsJpeg3Tag {
    pub character_id: u16,
    pub image_data: Vec<u8>,
    pub bitmap_alpha_data: Vec<u8>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DefineBitsJpeg4Tag {
    pub character_id: u16,
    pub deblock_param: Fixed8,
    pub image_data: Vec<u8>,
    pub bitmap_alpha_data: Vec<u8>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DefineBitsLosslessTag {
    pub character_id: u16,
    pub bitmap_width: u16,
    pub bitmap_height: u16,
    pub bitmap_data: BitmapData<Rgb>,
}

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

#[derive(Clone, PartialEq, Debug)]
pub struct ColorMapData<TColor> {
    pub color_table: Vec<TColor>,
    pub pixel_data: Vec<u8>,
}
