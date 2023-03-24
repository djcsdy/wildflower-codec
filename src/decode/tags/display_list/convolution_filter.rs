use crate::decode::tags::common::rgba::Rgba;

#[derive(Clone, PartialEq, Debug)]
pub struct ConvolutionFilter {
    pub divisor: f32,
    pub bias: f32,
    pub matrix: Vec<Vec<f32>>,
    pub default_color: Rgba,
    pub clamp: bool,
    pub preserve_alpha: bool,
}
