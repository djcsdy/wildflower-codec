use crate::ast::common::String;
use crate::decode::bit_read::BitRead;
use crate::decode::max_length_reader::MaxLengthReader;
use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{IoSliceMut, Read, Result};

pub struct SwfTagBodyReader<R: Read> {
    inner: MaxLengthReader<R>,
    swf_version: u8,
    partial_byte: u8,
    partial_bit_count: u8,
}

impl<R: Read> SwfTagBodyReader<R> {
    pub fn new(inner: R, swf_version: u8, max_length: usize) -> SwfTagBodyReader<R> {
        SwfTagBodyReader {
            inner: MaxLengthReader::new(inner, max_length),
            swf_version,
            partial_byte: 0,
            partial_bit_count: 0,
        }
    }

    pub fn into_inner(self) -> R {
        self.inner.into_inner()
    }

    pub fn swf_version(&self) -> u8 {
        self.swf_version
    }

    pub fn count(&self) -> usize {
        self.inner.count()
    }

    pub fn remaining(&self) -> usize {
        self.inner.remaining()
    }

    pub fn skip_to_end(&mut self) -> Result<()> {
        let mut buf = [0; 4096];
        while self.remaining() > 0 {
            self.read(&mut buf)?;
        }
        Ok(())
    }

    pub fn slice(&mut self, length: usize) -> SwfTagBodyReader<&mut Self> {
        SwfTagBodyReader::new(self, self.swf_version, length)
    }

    pub fn read_null_terminated_bytes(&mut self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        let mut byte = self.read_u8()?;
        while byte != 0 {
            bytes.push(byte);
            byte = self.read_u8()?;
        }
        Ok(bytes)
    }

    pub fn read_string(&mut self) -> Result<String> {
        Ok(String::from_bytes(self.read_null_terminated_bytes()?))
    }

    pub fn read_fixed_string(&mut self, byte_length: usize) -> Result<String> {
        let mut buffer = vec![0u8; byte_length];
        self.read_exact(&mut buffer)?;
        Ok(String::from_bytes(buffer))
    }

    pub fn read_u8_to_end(&mut self) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        self.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    pub fn read_u16_to_end(&mut self) -> Result<Vec<u16>> {
        let mut buffer = Vec::new();
        while self.remaining() > 0 {
            buffer.push(self.read_u16()?);
        }
        Ok(buffer)
    }
}

impl<R: Read> Read for SwfTagBodyReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.align_byte();
        self.inner.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize> {
        self.align_byte();
        self.inner.read_vectored(bufs)
    }
}

impl<R: Read> BitRead for SwfTagBodyReader<R> {
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
                result = (result << 8) | self.read_u8()? as u32;
                bits_remaining = bits_remaining - 8;
            }

            self.partial_byte = self.read_u8()?;
            self.partial_bit_count = 8 - bits_remaining;

            Ok((result << bits_remaining) | ((self.partial_byte as u32) >> self.partial_bit_count))
        }
    }
}
