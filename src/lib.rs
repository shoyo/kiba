use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

struct MiniKV<K, V> {
    store: HashMap<K, V>,
}

impl<K: Hash + Eq, V> MiniKV<K, V> {
    pub fn new() -> MiniKV<K, V> {
        MiniKV {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: K, val: V) -> Option<V> {
        self.store.insert(key, val)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.store.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut store: MiniKV<String, u32> = MiniKV::new();
        store.set("foo".to_string(), 32);
        let result = store.get(&"bar".to_string());
    }
}
