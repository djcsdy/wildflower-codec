pub mod fixed_16;
pub mod fixed_8;
pub mod rectangle;
pub mod rgb;
pub mod rgba;
pub mod string;

use fixed_16::Fixed16;
use fixed_8::Fixed8;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Twips<N>(N);

impl<N> Twips<N> {
    pub fn new(twips: N) -> Twips<N> {
        Twips(twips)
    }
}

impl<N: Display> Display for Twips<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}twpx", self.0)
    }
}

impl<N: Display> Debug for Twips<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<N> {
    x: Twips<N>,
    y: Twips<N>,
}

impl<N: Display> Display for Point<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})twpx", self.x.0, self.y.0)
    }
}

impl<N: Display> Debug for Point<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

/// A 2×3 matrix, used for 2D affine transformations.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Matrix {
    pub scale_x: Fixed16,
    pub scale_y: Fixed16,
    pub rotate_skew_0: Fixed16,
    pub rotate_skew_1: Fixed16,
    pub translate_x: i32,
    pub translate_y: i32,
}

/// A simple color transformation.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ColorTransform {
    pub red_multiplication_term: Fixed8,
    pub green_multiplication_term: Fixed8,
    pub blue_multiplication_term: Fixed8,
    pub red_addition_term: i16,
    pub green_addition_term: i16,
    pub blue_addition_term: i16,
}

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
