use inflate::DeflateDecoderBuf;
use std::io::{BufRead, IoSliceMut, Read};

pub enum TransparentDecompressingReader<R: BufRead> {
    Uncompressed(R),
    Deflate(DeflateDecoderBuf<R>),
}

impl<R: BufRead> Read for TransparentDecompressingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            TransparentDecompressingReader::Uncompressed(inner) => inner.read(buf),
            TransparentDecompressingReader::Deflate(inner) => inner.read(buf),
        }
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> std::io::Result<usize> {
        match self {
            TransparentDecompressingReader::Uncompressed(inner) => inner.read_vectored(bufs),
            TransparentDecompressingReader::Deflate(inner) => inner.read_vectored(bufs),
        }
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        match self {
            TransparentDecompressingReader::Uncompressed(inner) => inner.read_to_end(buf),
            TransparentDecompressingReader::Deflate(inner) => inner.read_to_end(buf),
        }
    }

    fn read_to_string(&mut self, buf: &mut String) -> std::io::Result<usize> {
        match self {
            TransparentDecompressingReader::Uncompressed(inner) => inner.read_to_string(buf),
            TransparentDecompressingReader::Deflate(inner) => inner.read_to_string(buf),
        }
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        match self {
            TransparentDecompressingReader::Uncompressed(inner) => inner.read_exact(buf),
            TransparentDecompressingReader::Deflate(inner) => inner.read_exact(buf),
        }
    }
}
