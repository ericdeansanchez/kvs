#![warn(missing_docs)]
//! Primary data structures and algorithms for creating and manipulating
//! [`KvStore`](struct.KvStore.html)
use std::collections::{BinaryHeap, HashMap};
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::str;

// Third party crates.
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

// Module declarations.
mod kvio;
mod util;

use kvio::{reader::KvsReader, writer::KvsWriter};

/// Re-exports `util::command_prelude` to be brought in by
/// `use kvs::command_prelude`.
pub use util::command_prelude;
pub use util::errors::{KvsError, Result};

const MAX_STALE_BYTES: u64 = 100;

/// Primary key-value store structure.
///
/// A `KvStore` is essentially a wrapper around a directory. It allows contains
/// The necessary structures to read and write to the store.
/// [`KvsReader`].
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

/// A `KvStore` is a directory. Specifically, a `KvStore` is a directory that
/// contains log files that store sequential commands in JSON format.
impl KvStore {
    /// Opens a `KvStore` given the path to the store's directory.
    ///
    /// # Errors
    ///
    /// This associated function can error under the following conditions:
    ///
    /// * creating the directory, specified by the path, fails
    /// * acquiring the version list fails
    /// * constructing each version's `KvsReader` fails
    /// * loading a log fails
    /// * a writer for this `KvStore` instance cannot be constructed
    ///
    /// # Examples
    /// ```
    /// ```
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

        for &version in version_heap.iter().rev() {
            let mut reader = KvsReader::new(File::open(log_path(&path, version))?)?;
            stale_bytes += Loader::load(version, &mut reader, &mut index)?;
            // If this is the way we are going to go about this, then the readers
            // need to be re-constructed after the initial `load`. It seems that
            // `load`ing exhausts the readers from being able to read again.
            // I am not entirely certain what is going on, but I know that the way
            // the pna example code is written is somewhat incorrect.
            let reader = KvsReader::new(File::open(log_path(&path, version))?)?;
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

    /// Opens a given `KvStore` _without_ generating a new log file.
    ///
    /// # Errors
    ///
    /// This associated function errors similarly to [`KvStore::open`].
    ///
    /// [`KvStore::open`]: #method.open
    pub fn open_with_opts<P: AsRef<Path>>(path: P, _opts: KvOpts) -> Result<KvStore> {
        let path = path.as_ref().to_owned();
        let mut readers = HashMap::new();
        let mut index = HashMap::new();

        // The number of stale bytes that can be compacted.
        let mut stale_bytes = 0u64;

        // Get the version heap.
        let version_heap = version_list(&path)?;

        // Get the current version number. This is the last version generated
        // and is at the top of the version list heap.
        let current_version = *version_heap.peek().unwrap_or(&0) + 1;

        // Load the appropriate logs.
        for &version in version_heap.iter().rev() {
            let mut reader = KvsReader::new(File::open(log_path(&path, version))?)?;
            stale_bytes += Loader::load(version, &mut reader, &mut index)?;
            // If this is the way we are going to go about this, then the readers
            // need to be re-constructed after the initial `load`. It seems that
            // `load`ing exhausts the readers from being able to read again.
            // I am not entirely certain what is going on, but I know that the way
            // the pna example code is written is somewhat incorrect.
            let reader = KvsReader::new(File::open(log_path(&path, version))?)?;
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
                Err(KvsError::UnexpectedCommandType(format!(
                    "no existing command for key: {}",
                    key
                )))
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
            Err(KvsError::KeyNotFound(format!(
                "could not find key: {}",
                key
            )))
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
        let pos = self.writer.pos();
        serde_json::to_writer(&mut self.writer, &cmd)?;
        self.writer.flush()?;
        if let Command::Set { key, .. } = cmd {
            // The call to `insert` returns `None` if the key is not present
            // upon insertion; otherwise, the previous value is returned.
            if let Some(old_cmd) = self
                .index
                .insert(key, (self.version, pos..self.writer.pos()).into())
            {
                // Record the old command's length as stale bytes.
                self.stale_bytes += old_cmd.len;
            }
        }

        if self.stale_bytes > MAX_STALE_BYTES {
            self.compact()?;
        }
        Ok(())
    }

    /// Clears stale command entries from the `KvStore`s logs.
    ///
    /// # Examples
    /// ```rust
    /// ```
    ///
    /// # Panics
    ///
    pub fn compact(&mut self) -> Result<()> {
        let compact_version = self.version + 1;
        self.version += 2;
        self.writer = self.new_log_file(self.version)?;

        let mut compaction_writer = self.new_log_file(compact_version)?;

        let mut new_pos = 0;
        for cmd_pos in &mut self.index.values_mut() {
            let reader = self
                .readers
                .get_mut(&cmd_pos.ver)
                .expect("Cannot find log reader");
            if reader.pos() != cmd_pos.pos {
                reader.seek(SeekFrom::Start(cmd_pos.pos))?;
            }

            let mut entry_reader = reader.take(cmd_pos.len);
            let len = io::copy(&mut entry_reader, &mut compaction_writer)?;
            *cmd_pos = (compact_version, new_pos..new_pos + len).into();
            new_pos += len;
        }

        compaction_writer.flush()?;

        let stale_versions: Vec<_> = self
            .readers
            .keys()
            .filter(|v| **v < compact_version)
            .cloned()
            .collect();

        for stale_gen in stale_versions {
            self.readers.remove(&stale_gen);
            fs::remove_file(log_path(&self.path, stale_gen))?;
        }
        Ok(())
    }

    fn new_log_file(&mut self, gen: u64) -> Result<KvsWriter<File>> {
        new_log_file(&self.path, gen, &mut self.readers)
    }
}

/// Constructs a new log file and returns a `KvsWriter` to it.
///
/// # Errors
///
/// * if `open`ing the file to be consumed by the `KvsWriter` results in an error or
/// * if `open`ing the file to be consumed by the `KvsReader` results in an error
///
/// # Examples
///
/// ```rust
/// ```
fn new_log_file<P: AsRef<Path>>(
    path: P,
    version: u64,
    readers: &mut HashMap<u64, KvsReader<File>>,
) -> Result<KvsWriter<File>> {
    // Construct the log path.
    let path = log_path(path.as_ref(), version);

    // Construct the writer in append mode.
    let writer = KvsWriter::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path)?,
    )?;

    // Finally, insert this log file's reader into the readers map.
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

fn log_path<P: AsRef<Path>>(path: P, version: u64) -> PathBuf {
    path.as_ref().join(format!("{}.log", version))
}

struct Loader;

impl Loader {
    /// Loads the log from disk, into memory.
    fn load(
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
}

/// Structure describing the various options a given `KvStore` can
/// exercise.
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
