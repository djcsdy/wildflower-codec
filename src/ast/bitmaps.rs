use super::common::{Fixed8, Rgb, Rgba};

pub struct DefineBitsTag {
    pub character_id: u16,
    pub jpeg_data: Vec<u8>,
}

pub struct JpegTablesTag {
    pub jpeg_data: Vec<u8>,
}

pub struct DefineBitsJpeg2Tag {
    pub character_id: u16,
    pub image_data: Vec<u8>,
}

pub struct DefineBitsJpeg3Tag {
    pub character_id: u16,
    pub image_data: Vec<u8>,
    pub bitmap_alpha_data: Vec<u8>,
}

pub struct DefineBitsJpeg4Tag {
    pub character_id: u16,
    pub deblock_param: Fixed8,
    pub image_data: Vec<u8>,
    pub bitmap_alpha_data: Vec<u8>
}

pub struct DefineBitsLosslessTag {
    pub character_id: u16,
    pub bitmap_width: u16,
    pub bitmap_height: u16,
    pub bitmap_data: BitmapData<Rgb>,
}

pub struct DefineBitsLossless2Tag {
    pub character_id: u16,
    pub bitmap_width: u16,
    pub bitmap_height: u16,
    pub bitmap_data: BitmapData<Rgba>,
}

pub enum BitmapData<TColor> {
    ColorMap8(ColorMapData<TColor>),
    Rgb15(Vec<TColor>),
    Rgb24(Vec<TColor>),
}

pub struct ColorMapData<TColor> {
    pub color_table: Vec<TColor>,
    pub pixel_data: Vec<u8>,
}
