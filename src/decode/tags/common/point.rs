use crate::decode::tags::common::twips::Twips;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<N> {
    pub x: Twips<N>,
    pub y: Twips<N>,
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
