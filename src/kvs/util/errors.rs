//! Primary error structures for kvs.
use std::io;

use failure;

pub enum KvsError {
    Io(io::Error),
    Serde(serde_json::Error),
    //KeyNotFound(String),
    //UnexpectedCommandType(String),
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

/// Use failure crate until obvious error types emerge.
/// Use of failure is discouraged; its use here is
/// particularly lazy, in that, if an error must be
/// returned the call will be `failure::bail!("msg")`.
pub type Result<T> = failure::Fallible<T>;
