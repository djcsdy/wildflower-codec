use crate::decode::bit_read::BitRead;
use crate::decode::bit_reader::BitReader;
use crate::decode::read_ext::SwfTypesReadExt;
use crate::decode::sized_read::SizedRead;
use std::io::{IoSliceMut, Read, Result};

pub struct SwfSliceReader<'buffer> {
    buffer: &'buffer [u8],
    inner: BitReader<&'buffer [u8]>,
}

impl<'buffer> SwfSliceReader<'buffer> {
    pub fn new(buffer: &'buffer [u8]) -> Self {
        SwfSliceReader {
            buffer,
            inner: BitReader::new(buffer),
        }
    }

    pub fn seek(&mut self, position: usize) {
        self.inner = BitReader::new(&self.buffer[position..]);
    }

    pub fn slice(&mut self, length: usize) -> Self {
        Self::new(self.inner.slice(length))
    }

    pub fn remaining_slice(mut self) -> Self {
        Self::new(self.inner.slice(self.remaining_bytes()))
    }

    pub fn read_u16_to_end(&mut self) -> Result<Vec<u16>> {
        let mut buffer = Vec::new();
        while self.remaining_bytes() > 0 {
            buffer.push(self.read_u16()?);
        }
        Ok(buffer)
    }
}

impl<'buffer> Read for SwfSliceReader<'buffer> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.inner.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize> {
        self.inner.read_vectored(bufs)
    }
}

impl<'buffer> SizedRead for SwfSliceReader<'buffer> {
    fn position(&self) -> usize {
        self.inner.inner().len() - self.buffer.len()
    }

    fn length_bytes(&self) -> usize {
        self.buffer.len()
    }

    fn remaining_bytes(&self) -> usize {
        self.inner.inner().len()
    }
}

impl<'buffer> BitRead for SwfSliceReader<'buffer> {
    fn align_byte(&mut self) {
        self.inner.align_byte()
    }

    fn read_ub(&mut self, bits: u8) -> Result<u32> {
        self.inner.read_ub(bits)
    }
}
