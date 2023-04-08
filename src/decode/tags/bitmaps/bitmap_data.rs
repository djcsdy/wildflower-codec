use crate::decode::tags::bitmaps::color_map_data::ColorMapData;

#[derive(Clone, PartialEq, Debug)]
pub enum BitmapData<Color> {
    ColorMap8(ColorMapData<Color>),
    Rgb(Vec<Color>),
}
