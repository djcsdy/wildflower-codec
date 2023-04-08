use crate::decode::decompressing_reader::DecompressingReader::{Uncompressed, Zlib};
use inflate::DeflateDecoderBuf;
use std::io::{BufRead, IoSliceMut, Read, Result};

pub enum DecompressingReader<R: BufRead> {
    Uncompressed(R),
    Zlib(DeflateDecoderBuf<R>),
}

impl<R: BufRead> DecompressingReader<R> {
    pub fn uncompressed(inner: R) -> DecompressingReader<R> {
        Uncompressed(inner)
    }

    pub fn deflate(inner: R) -> DecompressingReader<R> {
        Zlib(DeflateDecoderBuf::from_zlib(inner))
    }
}

impl<R: BufRead> Read for DecompressingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self {
            Uncompressed(inner) => inner.read(buf),
            Zlib(inner) => inner.read(buf),
        }
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize> {
        match self {
            Uncompressed(inner) => inner.read_vectored(bufs),
            Zlib(inner) => inner.read_vectored(bufs),
        }
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        match self {
            Uncompressed(inner) => inner.read_to_end(buf),
            Zlib(inner) => inner.read_to_end(buf),
        }
    }

    fn read_to_string(&mut self, buf: &mut String) -> Result<usize> {
        match self {
            Uncompressed(inner) => inner.read_to_string(buf),
            Zlib(inner) => inner.read_to_string(buf),
        }
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        match self {
            Uncompressed(inner) => inner.read_exact(buf),
            Zlib(inner) => inner.read_exact(buf),
        }
    }
}
