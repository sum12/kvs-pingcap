use std::collections::HashMap;

/// `KvStore` is a  is HashMap backed.
///
/// it stores value in memory and are not persisted to disk
///
/// ```rust
/// use kvs::KvStore;
///
/// let mut kv = KvStore::new();
/// kv.set("alpha".to_owned(), "beta".to_owned());
///
/// let value = kv.get("alpha".to_owned());
/// assert_eq!(value, Some("beta".to_owned()));
///
/// let value = kv.get("nothing".to_owned());
/// assert_eq!(value, None);
/// ```
#[derive(Default, Debug)]
pub struct KvStore(HashMap<String, String>);
#[derive(Debug)]
pub struct Error;
pub type Result<X> = std::result::Result<X, Error>;

impl KvStore {
    pub fn new() -> Self {
        KvStore(HashMap::new())
    }

    /// set a key to a value in store
    ///
    /// any pre-existing keys will be overwritten
    pub fn set(&mut self, k: String, v: String) {
        let _ = self.0.insert(k, v);
    }

    /// return the value of a key from the strore
    ///
    /// returns `None` if key is not present
    pub fn get(&mut self, k: String) -> Option<String> {
        self.0.get(&k).map(|v| v.to_owned())
    }

    /// Remove the given key from the store
    pub fn remove(&mut self, k: String) {
        self.0.remove(&k);
    }
}
