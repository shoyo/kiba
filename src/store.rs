use std::collections::{HashMap, HashSet, VecDeque};

type Result<T> = std::result::Result<T, OperationalError>;

/// A shared interface for implementations of store.
/// Time and space complexities of each function are not guaranteed and
/// depends on each implementation. (hash vs. btree, vec vs. linked list etc.)
pub trait Store {
    /// Create a new store.
    fn new() -> Self;

    // Strings Operations

    /// Get the value of a key.
    /// If the key does not exist, return None.
    /// Time complexity: O(1)
    fn get(&self, key: String) -> Result<Option<String>>;

    /// Set the value of a key.
    /// If the key already existed, return previous value.
    /// Otherwise, return None.
    /// Time complexity: O(1)
    fn set(&mut self, key: String, val: String) -> Result<Option<String>>;

    /// Increment the value of a key by 1.
    /// Return the updated value.
    /// If the key does not exist, return an error (unlike Redis).
    /// If the value is not/cannot be interpreted as an integer, return an error.
    /// This operation is limited to 64-bit integers.
    /// Time complexity: O(1)
    fn incr(&mut self, key: String) -> Result<i64>;

    /// Decrement the value of a key by 1.
    /// Return the updated value.
    /// If the key does not exist, return an error (unlike Redis).
    /// If the value is not/cannot be interpreted as an integer, return an error.
    /// This operation is limited to 64-bit integers.
    /// Time complexity: O(1)
    fn decr(&mut self, key: String) -> Result<i64>;

    /// Increment the value of a key by a specified amount.
    /// Return the updated value.
    /// If the key does not exist, return an error (unlike Redis).
    /// If the value is not/cannot be interpreted as an integer, return an error.
    /// This operation is limited to 64-bit integers.
    /// Time complexity: O(1)
    fn incrby(&mut self, key: String, delta: i64) -> Result<i64>;

    /// Decrement the value of a key by a specifed amount.
    /// Return the updated value.
    /// If the key does not exist, return an error (unlike Redis).
    /// If the value is not/cannot be interpreted as an integer, return an error.
    /// This operation is limited to 64-bit integers.
    /// Time complexity: O(1)
    fn decrby(&mut self, key: String, delta: i64) -> Result<i64>;

    // Lists Operations

    /// Insert value at the head of list stored at key.
    /// Return the updated length of the list.
    /// If the key does not exist, create an empty list before performing the operation.
    /// Time complexity: O(1)
    fn lpush(&mut self, key: String, val: String) -> Result<u64>;

    /// Insert value at the tail of list stored at key.
    /// Return the updated length of the list.
    /// If the key does not exist, create an empty list before performing the operation.
    /// Time complexity: O(1)
    fn rpush(&mut self, key: String, val: String) -> Result<u64>;

    /// Remove and return the element at the head of list stored at key.
    /// If the list is empty or does not exist, return None.
    /// Time complexity: O(1)
    fn lpop(&mut self, key: String) -> Result<Option<String>>;

    /// Remove and return the element at the head of list stored at key.
    /// If the list is empty or does not exist, return None.
    /// Time complexity: O(1)
    fn rpop(&mut self, key: String) -> Result<Option<String>>;

    // Sets Operations

    /// Insert value in the set stored at key.
    /// Return the updated length of the set.
    /// If the key does not exist, create an empty set before performing the operation.
    /// Time complexity: O(1)
    fn sadd(&mut self, key: String, val: String) -> Result<u64>;

    /// Remove value in the set stored at key.
    /// Return the updated length of the set.
    /// If the key does not exist or the specified value is not a member of the set,
    /// simply ignore and return 0.
    /// Time complexity: O(1)
    fn srem(&mut self, key: String, val: String) -> Result<u64>;

    /// Return if value is a member of the set stored at key.
    /// If the set is empty or does not exist, return false.
    /// Time complexity: O(1)
    fn sismember(&self, key: String, val: String) -> Result<bool>;

    /// Return all members of the set stored at key.
    /// If the set is empty or does not exist, return an empty iterator.
    /// Time complexity: O(N)
    fn smembers(&self, key: String) -> Result<Vec<String>>;

    // Hashes Operations

    /// Get the value related to field in the hash stored at key.
    /// If the key or field does not exist, return None.
    /// Time complexity: O(1)
    fn hget(&self, key: String, field: String) -> Result<Option<String>>;

    /// Set the field of the hash stored at key to value.
    /// If the field already existed, return previous value.
    /// Otherwise, return None.
    /// If the key does not exist, create an empty hash before performing the operation.
    /// Time complexity: O(1)
    fn hset(&mut self, key: String, field: String, val: String) -> Result<Option<String>>;

    /// Remove field from the hash stored at key.
    /// Return the number of fields that were deleted.
    /// If the key or field does not exist, do nothing (and return 0).
    /// Time complexity: O(1)
    fn hdel(&mut self, key: String, field: String) -> Result<u64>;
}

#[derive(Debug)]
pub struct StdStore {
    strings: HashMap<String, String>,
    lists: HashMap<String, VecDeque<String>>,
    hashes: HashMap<String, HashMap<String, String>>,
    sets: HashMap<String, HashSet<String>>,
}

impl StdStore {
    fn update_int(&mut self, key: String, delta: i64) -> Result<i64> {
        match self.strings.get_mut(&key) {
            Some(val) => match val.to_string().parse::<i64>() {
                Ok(int) => {
                    let check = int.checked_add(delta);
                    match check {
                        Some(sum) => {
                            *val = sum.to_string();
                            Ok(sum)
                        }
                        None => {
                            return Err(OperationalError {
                                message: format!(
                                    "Operation would cause integer to go out-of-bounds"
                                ),
                            })
                        }
                    }
                }
                Err(_) => {
                    return Err(OperationalError {
                        message: format!(
                            "Value stored at key cannot be represented as a 64-bit integer"
                        ),
                    })
                }
            },
            None => {
                return Err(OperationalError {
                    message: format!("Specified key does not exist"),
                })
            }
        }
    }
}

impl Store for StdStore {
    fn new() -> Self {
        StdStore {
            strings: HashMap::new(),
            lists: HashMap::new(),
            hashes: HashMap::new(),
            sets: HashMap::new(),
        }
    }

    // Strings Operations

    fn get(&self, key: String) -> Result<Option<String>> {
        match self.strings.get(&key) {
            Some(val) => Ok(Some(val.to_string())),
            None => Ok(None),
        }
    }

    fn set(&mut self, key: String, val: String) -> Result<Option<String>> {
        match self.strings.insert(key, val) {
            Some(val) => Ok(Some(val)),
            None => Ok(None),
        }
    }

    fn incr(&mut self, key: String) -> Result<i64> {
        self.update_int(key, 1)
    }

    fn decr(&mut self, key: String) -> Result<i64> {
        self.update_int(key, -1)
    }

    fn incrby(&mut self, key: String, delta: i64) -> Result<i64> {
        self.update_int(key, delta)
    }

    fn decrby(&mut self, key: String, delta: i64) -> Result<i64> {
        self.update_int(key, -delta)
    }

    /// Lists Operations

    fn lpush(&mut self, key: String, val: String) -> Result<u64> {
        match self.lists.get_mut(&key) {
            Some(list) => {
                list.push_front(val);
                Ok(list.len() as u64)
            }
            None => {
                let mut list = VecDeque::new();
                list.push_front(val);
                self.lists.insert(key, list);
                Ok(1)
            }
        }
    }

    fn rpush(&mut self, key: String, val: String) -> Result<u64> {
        match self.lists.get_mut(&key) {
            Some(list) => {
                list.push_back(val);
                Ok(list.len() as u64)
            }
            None => {
                let mut list = VecDeque::new();
                list.push_back(val);
                self.lists.insert(key, list);
                Ok(1)
            }
        }
    }

    fn lpop(&mut self, key: String) -> Result<Option<String>> {
        match self.lists.get_mut(&key) {
            Some(list) => Ok(list.pop_front()),
            None => Ok(None),
        }
    }

    fn rpop(&mut self, key: String) -> Result<Option<String>> {
        match self.lists.get_mut(&key) {
            Some(list) => Ok(list.pop_back()),
            None => Ok(None),
        }
    }

    /// Sets Operations

    fn sadd(&mut self, key: String, val: String) -> Result<u64> {
        match self.sets.get_mut(&key) {
            Some(set) => {
                set.insert(val);
                Ok(set.len() as u64)
            }
            None => {
                let mut set = HashSet::new();
                set.insert(val);
                self.sets.insert(key, set);
                Ok(1)
            }
        }
    }

    fn srem(&mut self, key: String, val: String) -> Result<u64> {
        match self.sets.get_mut(&key) {
            Some(set) => {
                set.remove(&val);
                Ok(set.len() as u64)
            }
            None => Ok(0),
        }
    }

    fn sismember(&self, key: String, val: String) -> Result<bool> {
        match self.sets.get(&key) {
            Some(set) => Ok(set.contains(&val)),
            None => Ok(false),
        }
    }

    fn smembers(&self, key: String) -> Result<Vec<String>> {
        match self.sets.get(&key) {
            Some(set) => Ok(set.iter().map(|v| v.to_owned()).collect()),
            None => Ok(vec![]),
        }
    }

    /// Hashes Operations

    fn hget(&self, key: String, field: String) -> Result<Option<String>> {
        match self.hashes.get(&key) {
            Some(hash) => match hash.get(&field) {
                Some(val) => Ok(Some(val.to_string())),
                None => Ok(None),
            },
            None => Ok(None),
        }
    }

    fn hset(&mut self, key: String, field: String, val: String) -> Result<Option<String>> {
        match self.hashes.get_mut(&key) {
            Some(hash) => Ok(hash.insert(field, val)),
            None => {
                let mut hash = HashMap::new();
                hash.insert(field, val);
                self.hashes.insert(key, hash);
                Ok(None)
            }
        }
    }

    fn hdel(&mut self, key: String, field: String) -> Result<u64> {
        match self.hashes.get_mut(&key) {
            Some(hash) => match hash.remove(&field) {
                Some(_) => Ok(1),
                None => Ok(0),
            },
            None => Ok(0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OperationalError {
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_std_get_set() {
        let mut store: StdStore = Store::new();
        assert_eq!(store.get("foo".to_string()).unwrap(), None);
        assert_eq!(
            store.set("foo".to_string(), "bar".to_string()).unwrap(),
            None
        );
        assert_eq!(
            store.get("foo".to_string()).unwrap(),
            Some("bar".to_string())
        );
        assert_eq!(
            store.set("foo".to_string(), "baz".to_string()).unwrap(),
            Some("bar".to_string())
        );
        assert_eq!(
            store.get("foo".to_string()).unwrap(),
            Some("baz".to_string())
        );
    }

    #[test]
    fn test_std_incr_decr() {
        let mut store: StdStore = Store::new();
        let _ = store.set("foo".to_string(), 5.to_string());
        let _ = store.set("bar".to_string(), "test".to_string());
        let _ = store.set("baz".to_string(), (3.14).to_string());

        // Valid operations
        assert_eq!(store.incr("foo".to_string()).unwrap(), 6);
        assert_eq!(store.incrby("foo".to_string(), 10).unwrap(), 16);
        assert_eq!(store.decr("foo".to_string()).unwrap(), 15);
        assert_eq!(store.decrby("foo".to_string(), 10).unwrap(), 5);

        // Invalid operations
        assert_eq!(store.incr("dne".to_string()).is_ok(), false);
        assert_eq!(store.incr("bar".to_string()).is_ok(), false);
        assert_eq!(store.incr("baz".to_string()).is_ok(), false);

        // Overflow operations
        let _ = store.set("x".to_string(), i64::MAX.to_string());
        assert_eq!(store.incrby("x".to_string(), 1).is_ok(), false);
        let _ = store.set("y".to_string(), i64::MIN.to_string());
        assert_eq!(store.decrby("y".to_string(), 1).is_ok(), false);
        assert_eq!(
            store
                .set("z".to_string(), "99999999999999999999999".to_string())
                .unwrap(),
            None
        );
        assert_eq!(store.incr("z".to_string()).is_ok(), false);
    }

    #[test]
    fn test_std_lists() {
        let mut store: StdStore = Store::new();
        // NOTE: Implementation details regarding push and pop
        //
        // When popping from a non-existent key, no list is initialized
        // and None is simply returned (no error is thrown).
        // When pushing to a non-existent key, an empty list is first
        // initialized and then the push operation is performed.
        // Empty lists (after successive pop operations) are NOT destroyed.

        // Popping from empty list
        assert_eq!(store.rpop("foo".to_string()).unwrap(), None);
        assert_eq!(store.lpop("foo".to_string()).unwrap(), None);

        // Pushing
        assert_eq!(store.lpush("foo".to_string(), "b".to_string()).unwrap(), 1);
        assert_eq!(store.lpush("foo".to_string(), "a".to_string()).unwrap(), 2);
        assert_eq!(store.rpush("foo".to_string(), "c".to_string()).unwrap(), 3);

        // Popping from non-empty list
        assert_eq!(
            store.lpop("foo".to_string()).unwrap(),
            Some("a".to_string())
        );
        assert_eq!(
            store.rpop("foo".to_string()).unwrap(),
            Some("c".to_string())
        );
        assert_eq!(
            store.lpop("foo".to_string()).unwrap(),
            Some("b".to_string())
        );
        assert_eq!(store.rpop("foo".to_string()).unwrap(), None);
    }

    #[test]
    fn test_std_sets() {
        let mut store: StdStore = Store::new();

        // Add items to set
        assert_eq!(
            store.sadd("foo".to_string(), "item1".to_string()).unwrap(),
            1
        );
        assert_eq!(
            store.sadd("foo".to_string(), "item2".to_string()).unwrap(),
            2
        );
        assert_eq!(
            store.sadd("foo".to_string(), "item3".to_string()).unwrap(),
            3
        );
        assert_eq!(
            store.sadd("foo".to_string(), "item4".to_string()).unwrap(),
            4
        );

        // Check membership of set
        assert_eq!(
            store
                .sismember("foo".to_string(), "item1".to_string())
                .unwrap(),
            true
        );
        assert_eq!(
            store
                .sismember("foo".to_string(), "item5".to_string())
                .unwrap(),
            false
        );

        // Remove item from set
        assert_eq!(
            store.srem("foo".to_string(), "item1".to_string()).unwrap(),
            3
        );
        assert_eq!(
            store
                .sismember("foo".to_string(), "item1".to_string())
                .unwrap(),
            false
        );

        // Get members of set (not rigorous)
        let actual = store.smembers("foo".to_string()).unwrap();
        assert_eq!(actual.len(), 3);

        let mut expected = HashSet::new();
        expected.insert("item2".to_string());
        expected.insert("item3".to_string());
        expected.insert("item4".to_string());

        for item in actual.iter() {
            assert!(expected.contains(item));
        }
    }

    #[test]
    fn test_std_hashes() {
        let mut store: StdStore = Store::new();
        assert_eq!(
            store.hget("foo".to_string(), "name".to_string()).unwrap(),
            None
        );
        assert_eq!(
            store
                .hset(
                    "foo".to_string(),
                    "name".to_string(),
                    "John Doe".to_string()
                )
                .unwrap(),
            None
        );
        assert_eq!(
            store
                .hset(
                    "foo".to_string(),
                    "name".to_string(),
                    "John Smith".to_string()
                )
                .unwrap(),
            Some("John Doe".to_string())
        );
        assert_eq!(
            store.hget("foo".to_string(), "name".to_string()).unwrap(),
            Some("John Smith".to_string())
        );
        assert_eq!(
            store.hdel("bar".to_string(), "name".to_string()).unwrap(),
            0
        );
        assert_eq!(
            store.hdel("foo".to_string(), "name".to_string()).unwrap(),
            1
        );
        assert_eq!(
            store.hget("foo".to_string(), "name".to_string()).unwrap(),
            None
        );
    }
}
