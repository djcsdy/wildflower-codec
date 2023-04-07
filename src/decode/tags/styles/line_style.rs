#[derive(Clone, PartialEq, Debug)]
pub struct LineStyle<Color> {
    pub width: u16,
    pub color: Color,
}
