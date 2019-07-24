//! Primary error structures for kvs.
use failure;

/// Use failure crate until obvious error types emerge.
/// Use of failure is discouraged; its use here is
/// particularly lazy, in that, if an error must be
/// returned the call will be `failure::bail!("msg")`.
pub type Result<T> = failure::Fallible<T>;
