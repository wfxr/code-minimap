use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::Path,
};

pub struct LossyReader {
    reader: BufReader<File>,
}

impl LossyReader {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        Ok(Self { reader })
    }
}

impl Read for LossyReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }
}

impl BufRead for LossyReader {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.reader.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt)
    }

    fn read_line(&mut self, buf: &mut String) -> std::io::Result<usize> {
        let mut append_buf = Vec::new();
        let res = self.read_until(0x0a, &mut append_buf);
        if let Err(err) = res {
            return Err(err);
        }
        buf.push_str(&String::from_utf8_lossy(&append_buf));
        Ok(buf.len())
    }
}
