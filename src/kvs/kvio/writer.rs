use std::io::{self, BufWriter, Seek, SeekFrom, Write};

use crate::util::errors::Result;

pub struct KvsWriter<W: Write + Seek> {
    writer: BufWriter<W>,
    pos: u64,
}

impl<W: Write + Seek> KvsWriter<W> {
    pub fn new(mut inner: W) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(KvsWriter {
            writer: BufWriter::new(inner),
            pos,
        })
    }

    pub fn pos(&self) -> u64 {
        self.pos
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
