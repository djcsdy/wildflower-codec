#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct ActionOffset(usize);

impl From<usize> for ActionOffset {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
