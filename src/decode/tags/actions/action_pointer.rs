use crate::decode::tags::actions::action_offset::ActionOffset;
use std::ops::Add;

/// A pointer into an ActionList buffer.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct ActionPointer(usize);

impl ActionPointer {
    pub const START: Self = ActionPointer(0);

    pub fn index_buffer<'buffer, Buffer: AsRef<[u8]>>(
        &self,
        buffer: &'buffer Buffer,
    ) -> &'buffer [u8] {
        &buffer.as_ref()[self.0..]
    }
}

impl Add<ActionOffset> for ActionPointer {
    type Output = ActionPointer;

    fn add(self, rhs: ActionOffset) -> Self::Output {
        ActionPointer(self.0 + rhs.offset())
    }
}
