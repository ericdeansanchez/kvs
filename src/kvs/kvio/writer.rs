use std::io::{self, BufWriter, Seek, SeekFrom, Write};

use crate::util::errors::Result;

pub struct KvsWriter<W: Write + Seek> {
    pub writer: BufWriter<W>,
    pub pos: u64,
}

impl<W: Write + Seek> KvsWriter<W> {
    pub fn new(mut buf: W) -> Result<Self> {
        let pos = buf.seek(SeekFrom::Current(0))?;
        Ok(KvsWriter {
            writer: BufWriter::new(buf),
            pos,
        })
    }
}

impl<W: Write + Seek> Write for KvsWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}
