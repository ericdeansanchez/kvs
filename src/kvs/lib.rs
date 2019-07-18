use std::collections::HashMap;

pub mod util;

// re-exports
pub use util::command_prelude;

pub struct KvStore {
    hm: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.hm.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) -> Option<String> {
        self.hm.remove(&key)
    }

    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        self.hm.insert(key, value)
    }
}

impl Default for KvStore {
    fn default() -> Self {
        KvStore { hm: HashMap::new() }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
