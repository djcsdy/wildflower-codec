pub mod color_transform;
pub mod color_transform_with_alpha;
pub mod fixed_16;
pub mod fixed_8;
pub mod matrix;
pub mod rectangle;
pub mod rgb;
pub mod rgba;
pub mod string;

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
