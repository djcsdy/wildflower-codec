#[derive(Clone, PartialEq, Debug)]
pub struct GradientRecord<Color> {
    pub ratio: u8,
    pub color: Color,
}
