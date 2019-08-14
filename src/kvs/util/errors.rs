//! Primary error structures for kvs.
use std::io;

/// Error types for the key-value store.
#[derive(Debug)]
pub enum KvsError {
    /// IO Error indicating that an IO error occurred
    /// during an IO process.
    Io(io::Error),
    /// Serde Error indicating that an error occurred
    /// during the (de)serialization process.
    Serde(serde_json::Error),
    /// Error type indicating that the sought key
    /// could not be found.
    KeyNotFound(String),
    /// Error type indicating that a command found
    /// in the log was unexpected (e.g. a command
    /// named Clear is written to the log, but
    /// this is not a valid Kvs `Command`)
    UnexpectedCommandType(String),
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> KvsError {
        KvsError::Serde(err)
    }
}

/// Custom result type for kvs.
pub type Result<T> = std::result::Result<T, KvsError>;
