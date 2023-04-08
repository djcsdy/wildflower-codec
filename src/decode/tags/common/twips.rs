use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Twips<N>(pub N);

impl<N> Twips<N> {
    pub fn new(twips: N) -> Twips<N> {
        Twips(twips)
    }
}

impl<N: Display> Display for Twips<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}twpx", self.0)
    }
}

impl<N: Display> Debug for Twips<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}
