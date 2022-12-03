use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct KvStore(HashMap<String, String>);

impl KvStore {
    pub fn new() -> Self {
        KvStore(HashMap::new())
    }

    pub fn set(&mut self, k: String, v: String) {
        let _ = self.0.insert(k, v);
    }

    pub fn get(&mut self, k: String) -> Option<String> {
        self.0.get(&k).map(|v| v.to_owned())
    }

    pub fn remove(&mut self, k: String) {
        self.0.remove(&k);
    }
}
