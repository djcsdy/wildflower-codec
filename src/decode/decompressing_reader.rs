use crate::decode::decompressing_reader::DecompressingReader::{Deflate, Uncompressed};
use inflate::DeflateDecoderBuf;
use std::io::{BufRead, IoSliceMut, Read, Result};

pub enum DecompressingReader<R: BufRead> {
    Uncompressed(R),
    Deflate(DeflateDecoderBuf<R>),
}

impl<R: BufRead> DecompressingReader<R> {
    pub fn uncompressed(inner: R) -> DecompressingReader<R> {
        Uncompressed(inner)
    }

    pub fn deflate(inner: R) -> DecompressingReader<R> {
        Deflate(DeflateDecoderBuf::new(inner))
    }
}

impl<R: BufRead> Read for DecompressingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self {
            Uncompressed(inner) => inner.read(buf),
            Deflate(inner) => inner.read(buf),
        }
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize> {
        match self {
            Uncompressed(inner) => inner.read_vectored(bufs),
            Deflate(inner) => inner.read_vectored(bufs),
        }
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        match self {
            Uncompressed(inner) => inner.read_to_end(buf),
            Deflate(inner) => inner.read_to_end(buf),
        }
    }

    fn read_to_string(&mut self, buf: &mut String) -> Result<usize> {
        match self {
            Uncompressed(inner) => inner.read_to_string(buf),
            Deflate(inner) => inner.read_to_string(buf),
        }
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        match self {
            Uncompressed(inner) => inner.read_exact(buf),
            Deflate(inner) => inner.read_exact(buf),
        }
    }
}
