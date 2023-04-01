#[derive(Clone, PartialEq, Debug)]
pub struct ColorMapData<Color> {
    pub color_table: Vec<Color>,
    pub pixel_data: Vec<u8>,
}
