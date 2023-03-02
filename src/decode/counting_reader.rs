use std::cmp::min;
use std::io::{Read, Result};

pub struct CountingReader<R: Read> {
    inner: R,
    count: usize,
    max_length: Option<usize>,
}

impl<R: Read> CountingReader<R> {
    pub fn new(inner: R) -> CountingReader<R> {
        CountingReader {
            inner,
            count: 0,
            max_length: None,
        }
    }

    pub fn take(inner: R, max_length: usize) -> CountingReader<R> {
        CountingReader {
            inner,
            count: 0,
            max_length: Some(max_length),
        }
    }

    pub fn remaining(&self) -> Option<usize> {
        self.max_length.map(|max_length| max_length - self.count)
    }

    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<R: Read> Read for CountingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = self
            .remaining()
            .map_or(buf.len(), |remaining| min(remaining, buf.len()));
        let size = self.inner.read(&mut buf[0..len])?;
        self.count += size;
        Ok(size)
    }
}
