pub mod color_transform;
pub mod fixed_16;
pub mod fixed_8;
pub mod matrix;
pub mod rectangle;
pub mod rgb;
pub mod rgba;
pub mod string;

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
