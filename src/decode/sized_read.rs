use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{Read, Result};

pub trait SizedRead: Read {
    fn position(&self) -> usize;
    fn length_bytes(&self) -> usize;
    fn remaining_bytes(&self) -> usize {
        self.length_bytes() - self.position()
    }

    fn read_u16_to_end(&mut self) -> Result<Vec<u16>> {
        let mut buffer = Vec::with_capacity(self.remaining_bytes() / 2);
        while self.remaining_bytes() >= 2 {
            buffer.push(self.read_u16()?);
        }
        Ok(buffer)
    }
}
