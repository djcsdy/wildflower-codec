use std::io::Read;

pub trait SizedRead: Read {
    fn position(&self) -> usize;
    fn length_bytes(&self) -> usize;
    fn remaining_bytes(&self) -> usize {
        self.length_bytes() - self.position()
    }
}
