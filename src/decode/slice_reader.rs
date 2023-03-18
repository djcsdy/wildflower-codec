use crate::ast::common::String;
use crate::decode::bit_read::BitRead;
use crate::decode::bit_reader::BitReader;
use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{IoSliceMut, Read, Result};

pub struct SwfSliceReader<'buffer> {
    inner: BitReader<&'buffer [u8]>,
    swf_version: u8,
    length: usize,
}

impl<'buffer> SwfSliceReader<'buffer> {
    pub fn new(buffer: &'buffer [u8], swf_version: u8) -> Self {
        SwfSliceReader {
            inner: BitReader::new(buffer),
            swf_version,
            length: buffer.len(),
        }
    }

    pub fn swf_version(&self) -> u8 {
        self.swf_version
    }

    pub fn count(&self) -> usize {
        self.inner.inner().len() - self.length
    }

    pub fn remaining(&self) -> usize {
        self.inner.inner().len()
    }

    pub fn slice(&mut self, length: usize) -> Self {
        Self::new(self.inner.slice(length), self.swf_version)
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

impl<'buffer> Read for SwfSliceReader<'buffer> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.inner.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize> {
        self.inner.read_vectored(bufs)
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