use crate::decode::bit_read::{bit_read, BitRead, BitReadOptions, BitReadState};
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
        let (state, result) = bit_read(&mut BitReadOptions {
            read_byte: || self.inner.read_u8(),
            state: BitReadState {
                partial_byte: self.partial_byte,
                partial_bit_count: self.partial_bit_count,
            },
            bits,
        });
        self.partial_byte = state.partial_byte;
        self.partial_bit_count = state.partial_bit_count;
        result
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

impl<'buffer> BitReader<&'buffer [u8]> {
    pub fn slice(&mut self, length: usize) -> &'buffer [u8] {
        self.align_byte();
        let (slice, new_buffer) = self.inner.split_at(length);
        self.inner = new_buffer;
        slice
    }
}
