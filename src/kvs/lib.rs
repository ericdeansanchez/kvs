#![warn(missing_docs)]
//! Primary data structures and algorithms for creating and manipulating
//! [`KvStore`](struct.KvStore.html)
use std::collections::{BinaryHeap, HashMap};
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::str;

use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

// Module declarations.
mod kvio;
mod util;

use kvio::{reader::KvsReader, writer::KvsWriter};

/// Re-exports `util::command_prelude` to be brought in by
/// `use kvs::command_prelude`.
pub use util::command_prelude;
pub use util::errors::Result;

/// Primary key-value store structure. This structure is a wrapper around a
/// [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html).
pub struct KvStore {
    /// A mapping between key-strings and their corresponding CommandPosition.
    index: HashMap<String, CommandPosition>,
    /// The path to this store's directory.
    path: PathBuf,
    /// A mapping between a given version number and its corresponding reader.
    readers: HashMap<u64, KvsReader<File>>,
    /// The number of 'stale bytes' the current store contains.
    stale_bytes: u64,
    /// The writer of a log.
    writer: KvsWriter<File>,
    /// The version number of a log.
    version: u64,
}

impl KvStore {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<KvStore> {
        let path = path.as_ref().to_owned();
        fs::create_dir_all(&path)?;
        let mut readers = HashMap::new();
        let mut index = HashMap::new();

        // The number of stale bytes that can be compacted.
        let mut stale_bytes = 0u64;

        // Get the version heap.
        let version_heap = version_list(&path)?;

        // Get the current version number. This is the last version generated
        // and is at the top of the version list heap.
        let current_version = version_heap.peek().unwrap_or(&0) + 1;

        for &version in version_heap.iter() {
            let f = File::open(log_path(&path, version))?;
            let mut reader = KvsReader::new(f)?;
            stale_bytes += load_log(version, &mut reader, &mut index)?;
            readers.insert(version, reader);
        }
        let writer = new_log_file(&path, current_version, &mut readers)?;
        Ok(KvStore {
            path,
            readers,
            writer,
            version: current_version,
            index,
            stale_bytes,
        })
    }

    pub fn open_with_opts<P: AsRef<Path>>(path: P, opts: KvOpts) -> Result<KvStore> {
        let path = path.as_ref().to_owned();
        let mut readers = HashMap::new();
        let mut index = HashMap::new();

        // The number of stale bytes that can be compacted.
        let mut stale_bytes = 0u64;

        // Get the version heap.
        let version_heap = version_list(&path)?;

        // Get the current version number. This is the last version generated
        // and is at the top of the version list heap.
        let current_version = *version_heap.peek().unwrap_or(&0);

        for &version in version_heap.iter() {
            let f = File::open(log_path(&path, version))?;
            let mut reader = KvsReader::new(f)?;
            stale_bytes += load_log(version, &mut reader, &mut index)?;
            readers.insert(version, reader);
        }

        let writer = new_log_file(&path, current_version, &mut readers)?;
        Ok(KvStore {
            path,
            readers,
            writer,
            version: current_version,
            index,
            stale_bytes,
        })
    }

    /// Gets a string value if the given key has been [`set`]; otherwise this
    /// method returns `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    /// [`set`]: #method.set
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(cmd_pos) = self.index.get(&key) {
            let reader = self
                .readers
                .get_mut(&cmd_pos.ver)
                .expect("Cannot find log reader");

            reader.seek(SeekFrom::Start(cmd_pos.pos))?;

            let cmd_reader = reader.take(cmd_pos.len);
            if let Command::Set { value, .. } = serde_json::from_reader(cmd_reader)? {
                Ok(Some(value))
            } else {
                failure::bail!("bailed: KvsError::UnexpectedCommandType")
            }
        } else {
            Ok(None)
        }
    }

    /// Removes a key, along with its corresponding value, from the `KvStore`
    /// If the given key is in the `KvStore`, then the removed value will be
    /// return. Otherwise, if the key does not exist, then `None` is returned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.index.contains_key(&key) {
            let cmd = Command::Remove { key };
            serde_json::to_writer(&mut self.writer, &cmd)?;
            self.writer.flush()?;
            if let Command::Remove { key } = cmd {
                let old_cmd = self.index.remove(&key).expect("key not found");
                self.stale_bytes += old_cmd.len;
            }
            Ok(())
        } else {
            failure::bail!("bailed: Err(KvsError::KeyNotFound)")
        }
    }

    /// Sets a key-value pair in the `KvStore` by inserting this entry-pair into
    /// the underlying map. If the given key has not already been set, then this
    /// method returns `None`. Otherwise, the given key's value is updated, and
    /// the old value is returned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::Set { key, value };
        let pos = self.writer.pos;
        serde_json::to_writer(&mut self.writer, &cmd)?;
        self.writer.flush()?;
        if let Command::Set { key, .. } = cmd {
            if let Some(old_cmd) = self
                .index
                .insert(key, (self.version, pos..self.writer.pos).into())
            {
                self.stale_bytes += old_cmd.len;
            }
        }
        Ok(())
    }
}

fn new_log_file<P: AsRef<Path>>(
    path: P,
    version: u64,
    readers: &mut HashMap<u64, KvsReader<File>>,
) -> Result<KvsWriter<File>> {
    let path = log_path(path.as_ref(), version);
    let writer = KvsWriter::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path)?,
    )?;
    readers.insert(version, KvsReader::new(File::open(&path)?)?);
    Ok(writer)
}

fn version_list<P: AsRef<Path>>(path: P) -> Result<BinaryHeap<u64>> {
    Ok(fs::read_dir(path.as_ref())?
        .flat_map(|res| -> Result<_> { Ok(res?.path()) })
        .filter(|path| path.is_file() && path.extension() == Some("log".as_ref()))
        .flat_map(|path| {
            path.file_name()
                .and_then(OsStr::to_str)
                .map(|s| s.trim_end_matches(".log"))
                .map(str::parse::<u64>)
        })
        .flatten()
        .collect())
}

fn log_path<P: AsRef<Path>>(dir: P, version: u64) -> PathBuf {
    dir.as_ref().join(format!("{}.log", version))
}

fn load_log(
    version: u64,
    reader: &mut KvsReader<File>,
    index: &mut HashMap<String, CommandPosition>,
) -> Result<u64> {
    let mut pos = reader.seek(SeekFrom::Start(0))?;
    let mut stream = Deserializer::from_reader(reader).into_iter::<Command>();
    let mut stale_bytes = 0u64;

    while let Some(cmd) = stream.next() {
        // Update the new position to the number of bytes successfully
        // deserialized into a `Command`.
        let new_pos = stream.byte_offset() as u64;
        match cmd? {
            Command::Set { key, .. } => {
                // If a given key is present in the map, then `insert` is updating
                // a value that is already present in the map. The old value,
                // in this case the old `CommandPosition`, is returned.
                //
                // This old `CommandPosition`'s length represents a number of stale bytes
                // that can be compacted.
                if let Some(old_cmd) = index.insert(key, (version, pos..new_pos).into()) {
                    stale_bytes += old_cmd.len;
                }
            }
            Command::Remove { key } => {
                // If a given key is present in the map, then `remove` will return
                // the value. In this case, the old `CommandPosition` is returned.
                //
                // The removed `CommandPosition`'s length represents a number of
                // stale bytes that can be compacted.
                if let Some(old_cmd) = index.remove(&key) {
                    stale_bytes += old_cmd.len;
                }
                // The removal command's length (in bytes) can also be safely
                // compacted.
                stale_bytes += new_pos - pos;
            }
        }
        pos = new_pos;
    }
    Ok(stale_bytes)
}

pub struct KvOpts;

#[derive(Debug)]
struct CommandPosition {
    ver: u64,
    pos: u64,
    len: u64,
}

impl From<(u64, Range<u64>)> for CommandPosition {
    fn from((ver, range): (u64, Range<u64>)) -> Self {
        CommandPosition {
            ver,
            pos: range.start,
            len: range.end - range.start,
        }
    }
}

/// Struct representation of a command.
#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}
