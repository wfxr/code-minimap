use std::io::{self, BufRead, BufReader, Read};

pub struct LossyReader<R> {
    inner: BufReader<R>,
}

impl<R: Read> LossyReader<R> {
    pub fn new(inner: R) -> Self {
        Self { inner: BufReader::new(inner) }
    }
}

impl<R: Read> Read for LossyReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<R: Read> BufRead for LossyReader<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt)
    }

    fn read_line(&mut self, buf: &mut String) -> std::io::Result<usize> {
        let mut bytes = Vec::new();
        let len = self.read_until(b'\n', &mut bytes)?;
        buf.push_str(&String::from_utf8_lossy(&bytes));
        Ok(len)
    }
}
