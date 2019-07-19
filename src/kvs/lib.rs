#![warn(missing_docs)]
//! Primary data structures and algorithms for creating and manipulating
//! [`KvStore`](struct.KvStore.html)
use std::collections::HashMap;

// Module declarations.
mod util;

/// Re-exports `util::command_prelude` to be brought in by
/// `use kvs::command_prelude`.
pub use util::command_prelude;

/// Primary key-value store structure. This structure is a wrapper around a
/// [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html).
pub struct KvStore {
    hm: HashMap<String, String>,
}

impl KvStore {
    /// Constructs a new `KvStore` with an empty hash map.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kvs::KvStore;
    ///
    /// let mut kvs = KvStore::new();
    /// assert!(kvs.is_empty());
    ///
    /// kvs.set("hello".to_owned(), "mars".to_owned());
    /// assert!(!kvs.is_empty());
    /// assert_eq!(kvs.size(), 1);
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Gets a string value if the given key has been [`set`]; otherwise this
    /// method returns `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kvs::KvStore;
    /// // get will return None if the given key does not exist within the
    /// // `KvStore`.
    /// let mut kvs = KvStore::new();
    /// assert_eq!(kvs.get("hello".to_owned()), None);
    ///
    /// kvs.set("hello".to_owned(), "mars".to_owned());
    /// assert_eq!(kvs.get("hello".to_owned()), Some("mars".to_owned()));
    /// ```
    /// [`set`]: #method.set
    pub fn get(&self, key: String) -> Option<String> {
        self.hm.get(&key).cloned()
    }

    /// Removes a key, along with its corresponding value, from the `KvStore`
    /// If the given key is in the `KvStore`, then the removed value will be
    /// return. Otherwise, if the key does not exist, then `None` is returned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kvs::KvStore;
    ///
    /// let mut kvs = KvStore::new();
    /// assert_eq!(kvs.remove("hello".to_owned()), None);
    ///
    /// kvs.set("hello".to_owned(), "mars".to_owned());
    /// assert_eq!(kvs.remove("hello".to_owned()), Some("mars".to_owned()));
    /// ```
    pub fn remove(&mut self, key: String) -> Option<String> {
        self.hm.remove(&key)
    }

    /// Sets a key-value pair in the `KvStore` by inserting this entry-pair into
    /// the underlying map. If the given key has not already been set, then this
    /// method returns `None`. Otherwise, the given key's value is updated, and
    /// the old value is returned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kvs::KvStore;
    ///
    /// let mut kvs = KvStore::new();
    /// assert_eq!(kvs.set("hello".to_owned(), "mars".to_owned()), None);
    /// assert_eq!(kvs.set("hello".to_owned(), "jupiter".to_owned()), Some("mars".to_owned()));
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        self.hm.insert(key, value)
    }

    /// Returns true if the `KvStore` representation is empty, false otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kvs::KvStore;
    ///
    /// let mut kvs = KvStore::new();
    /// assert!(kvs.is_empty());
    ///
    /// kvs.set("hello".to_owned(), "mars".to_owned());
    /// assert!(!kvs.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.hm.is_empty()
    }

    /// Returns the number of entries in the `KvStore`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use kvs::KvStore;
    ///
    /// let mut kvs = KvStore::new();
    /// assert_eq!(kvs.size(), 0);
    ///
    /// kvs.set("hello".to_owned(), "mars".to_owned());
    /// assert_eq!(kvs.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.hm.len()
    }
}

impl Default for KvStore {
    fn default() -> Self {
        KvStore { hm: HashMap::new() }
    }
}
