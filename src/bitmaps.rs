use super::common::Rgb;

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

pub struct DefineBitsLosslessTag {
    pub character_id: u16,
    pub bitmap_width: u16,
    pub bitmap_height: u16,
    pub bitmap_data: BitmapData
}

pub enum BitmapData {
    ColorMap8(ColorMapData),
    Rgb15(Vec<Rgb>),
    Rgb24(Vec<Rgb>)
}

pub struct ColorMapData {
    pub color_table_rgb: Vec<Rgb>,
    pub pixel_data: Vec<u8>
}