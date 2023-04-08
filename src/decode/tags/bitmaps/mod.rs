pub mod color_map_data;
pub mod define_bits;
pub mod define_bits_jpeg_2;
pub mod define_bits_jpeg_3;
pub mod define_bits_jpeg_4;
pub mod define_bits_lossless;
pub mod define_bits_lossless_2;
pub mod jpeg_tables;

use color_map_data::ColorMapData;

#[derive(Clone, PartialEq, Debug)]
pub enum BitmapData<Color> {
    ColorMap8(ColorMapData<Color>),
    Rgb(Vec<Color>),
}
