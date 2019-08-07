use std::io::{self, BufReader, Read, Seek, SeekFrom};

use crate::util::errors::Result;

pub struct KvsReader<R: Read + Seek> {
    reader: BufReader<R>,
    pos: u64,
}

impl<R: Read + Seek> KvsReader<R> {
    pub fn new(mut buf: R) -> Result<Self> {
        let pos = buf.seek(SeekFrom::Current(0))?;
        Ok(KvsReader {
            reader: BufReader::new(buf),
            pos,
        })
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
