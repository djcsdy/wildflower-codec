use crate::decode::tags::styles::fill_style::FillStyle;

#[derive(Clone, PartialEq, Debug)]
pub struct StyleChangeRecord<Color, LineStyle> {
    pub move_delta: (i16, i16),
    pub fill_style_0: u16,
    pub fill_style_1: u16,
    pub line_style: u16,
    pub fill_styles: Option<Vec<FillStyle<Color>>>,
    pub line_styles: Option<Vec<LineStyle>>,
}
