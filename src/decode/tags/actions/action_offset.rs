#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct ActionOffset(usize);

impl ActionOffset {
    pub fn offset(&self) -> usize {
        self.0
    }
}

impl From<usize> for ActionOffset {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
