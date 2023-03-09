use std::cmp::min;
use std::io::{Read, Result};

pub struct MaxLengthReader<R: Read> {
    inner: R,
    count: usize,
    max_length: usize,
}

impl<R: Read> MaxLengthReader<R> {
    pub fn new(inner: R, max_length: usize) -> MaxLengthReader<R> {
        MaxLengthReader {
            inner,
            count: 0,
            max_length,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn remaining(&self) -> usize {
        self.max_length - self.count
    }

    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<R: Read> Read for MaxLengthReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = min(self.remaining(), buf.len());
        let size = self.inner.read(&mut buf[0..len])?;
        self.count += size;
        Ok(size)
    }
}
