/// A pointer into an ActionList buffer.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct ActionPointer(usize);

impl ActionPointer {
    pub const START: Self = ActionPointer(0);

    pub fn index_buffer<'buffer, Buffer: Into<&'buffer [u8]>>(
        &self,
        buffer: Buffer,
    ) -> &'buffer [u8] {
        &buffer.into()[self.0..]
    }
}
