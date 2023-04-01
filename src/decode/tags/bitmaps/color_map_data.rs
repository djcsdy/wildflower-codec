#[derive(Clone, PartialEq, Debug)]
pub struct ColorMapData<TColor> {
    pub color_table: Vec<TColor>,
    pub pixel_data: Vec<u8>,
}
