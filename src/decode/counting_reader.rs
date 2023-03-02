use std::io::{IoSliceMut, Read, Result};

pub struct CountingReader<R: Read> {
    inner: R,
    count: usize,
}

impl<R: Read> Read for CountingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let size = self.inner.read(buf)?;
        self.count += size;
        Ok(size)
    }

    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize> {
        let size = self.inner.read_vectored(bufs)?;
        self.count += size;
        Ok(size)
    }
}
