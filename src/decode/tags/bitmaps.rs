use super::common::*;

#[derive(Clone, PartialEq, Debug)]
pub struct DefineBitsTag {
    pub character_id: u16,
    pub jpeg_data: Vec<u8>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct JpegTablesTag {
    pub jpeg_data: Vec<u8>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DefineBitsJpeg2Tag {
    pub character_id: u16,
    pub image_data: Vec<u8>,
}

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