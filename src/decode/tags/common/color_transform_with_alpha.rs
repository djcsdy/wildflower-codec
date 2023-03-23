use crate::decode::tags::common::fixed_8::Fixed8;

/// A simple transformation of an RGBA color-with-alpha.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ColorTransformWithAlpha {
    pub red_multiplication_term: Fixed8,
    pub green_multiplication_term: Fixed8,
    pub blue_multiplication_term: Fixed8,
    pub alpha_multiplication_term: Fixed8,
    pub red_addition_term: i16,
    pub green_addition_term: i16,
    pub blue_addition_term: i16,
    pub alpha_addition_term: i16,
}
