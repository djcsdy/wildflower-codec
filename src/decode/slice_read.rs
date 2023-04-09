use std::io::Read;

pub trait SliceRead: Read + Sized {
    fn slice(&mut self, length: usize) -> Self;
    fn remaining_slice(&mut self) -> Self;
}
