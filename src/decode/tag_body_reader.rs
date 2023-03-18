use crate::ast::common::String;
use crate::decode::bit_read::BitRead;
use crate::decode::bit_reader::BitReader;
use crate::decode::max_length_reader::MaxLengthReader;
use crate::decode::read_ext::SwfTypesReadExt;
use std::io::{IoSliceMut, Read, Result};

pub struct SwfTagBodyReader<R: Read> {
    inner: BitReader<MaxLengthReader<R>>,
    swf_version: u8,
}

impl<R: Read> SwfTagBodyReader<R> {
    pub fn new(inner: R, swf_version: u8, max_length: usize) -> SwfTagBodyReader<R> {
        SwfTagBodyReader {
            inner: BitReader::new(MaxLengthReader::new(inner, max_length)),
            swf_version,
        }
    }

    pub fn into_inner(self) -> R {
        self.inner.into_inner().into_inner()
    }

    pub fn swf_version(&self) -> u8 {
        self.swf_version
    }

    pub fn count(&self) -> usize {
        self.inner.inner().count()
    }

    pub fn remaining(&self) -> usize {
        self.inner.inner().remaining()
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
        self.inner.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize> {
        self.inner.read_vectored(bufs)
    }
}

impl<R: Read> BitRead for SwfTagBodyReader<R> {
    fn align_byte(&mut self) {
        self.inner.align_byte()
    }

    fn read_ub(&mut self, bits: u8) -> Result<u32> {
        self.inner.read_ub(bits)
    }
}
