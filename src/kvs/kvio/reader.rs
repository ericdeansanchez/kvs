use std::io::{self, BufReader, Read, Seek, SeekFrom};

use crate::util::errors::Result;

#[derive(Debug)]
pub struct KvsReader<R: Read + Seek> {
    reader: BufReader<R>,
    pos: u64,
}

impl<R: Read + Seek> KvsReader<R> {
    pub fn new(mut inner: R) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Start(0))?;
        Ok(KvsReader {
            reader: BufReader::new(inner),
            pos,
        })
    }

    pub fn pos(&self) -> u64 {
        self.pos
    }
}

impl<R: Read + Seek> Read for KvsReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<R: Read + Seek> Seek for KvsReader<R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}
