use crate::decode::bit_read::BitRead;
use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{BufRead, IoSliceMut, Read, Result};

pub struct BitReader<R: Read> {
    inner: R,
    partial_byte: u8,
    partial_bit_count: u8,
}

impl<R: Read> BitReader<R> {
    pub fn new(inner: R) -> Self {
        BitReader {
            inner,
            partial_byte: 0,
            partial_bit_count: 0,
        }
    }

    pub fn inner(&self) -> &R {
        &self.inner
    }

    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<R: Read> Read for BitReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.align_byte();
        self.inner.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize> {
        self.align_byte();
        self.inner.read_vectored(bufs)
    }
}

impl<R: Read> BitRead for BitReader<R> {
    fn align_byte(&mut self) {
        self.partial_byte = 0;
        self.partial_bit_count = 0;
    }

    fn read_ub(&mut self, bits: u8) -> Result<u32> {
        if bits > 32 {
            panic!();
        }

        if bits <= self.partial_bit_count {
            self.partial_bit_count = self.partial_bit_count - bits;
            Ok((self.partial_byte as u32) >> self.partial_bit_count)
        } else {
            let mut result = self.partial_byte as u32;
            let mut bits_remaining = bits - self.partial_bit_count;
            while bits_remaining > 8 {
                result = (result << 8) | self.inner.read_u8()? as u32;
                bits_remaining = bits_remaining - 8;
            }

            self.partial_byte = self.inner.read_u8()?;
            self.partial_bit_count = 8 - bits_remaining;

            Ok((result << bits_remaining) | ((self.partial_byte as u32) >> self.partial_bit_count))
        }
    }
}

impl<R: BufRead> BufRead for BitReader<R> {
    fn fill_buf(&mut self) -> Result<&[u8]> {
        self.align_byte();
        self.inner.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.align_byte();
        self.inner.consume(amt)
    }
}
