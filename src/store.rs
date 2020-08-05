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
    fn get(&self, key: String) -> Result<Option<String>>;

    /// Set the value of a key.
    /// If the key already existed, return previous value.
    /// Otherwise, return None.
    fn set(&mut self, key: String, val: String) -> Result<Option<String>>;

    /// Increment the value of a key by 1.
    /// Return the updated value.
    /// If the key does not exist, return an error (unlike Redis).
    /// If the value is not/cannot be interpreted as an integer, return an error.
    /// This operation is limited to 64-bit integers.
    fn incr(&mut self, key: String) -> Result<i64>;

    /// Decrement the value of a key by 1.
    /// Return the updated value.
    /// If the key does not exist, return an error (unlike Redis).
    /// If the value is not/cannot be interpreted as an integer, return an error.
    /// This operation is limited to 64-bit integers.
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
    fn decrby(&mut self, key: String, delta: i64) -> Result<i64>;

    // Lists Operations

    /// Insert value at the head of list stored at key.
    /// Return the updated length of the list.
    /// If the key does not exist, create an empty list before performing the operation.
    fn lpush(&mut self, key: String, val: String) -> Result<u64>;

    /// Insert value at the tail of list stored at key.
    /// Return the updated length of the list.
    /// If the key does not exist, create an empty list before performing the operation.
    fn rpush(&mut self, key: String, val: String) -> Result<u64>;

    /// Remove and return the element at the head of list stored at key.
    /// If the list is empty, return None.
    fn lpop(&mut self, key: String) -> Result<Option<String>>;

    /// Remove and return the element at the head of list stored at key.
    /// If the list is empty, return None.
    fn rpop(&mut self, key: String) -> Result<Option<String>>;

    // Sets Operations

    /// Insert value in the set stored at key.
    /// Return the updated length of the set.
    /// If the key does not exist, create an empty set before performing the operation.
    fn sadd(&mut self, key: String, val: String) -> Result<u64>;

    /// Remove value in the set stored at key.
    /// Return the updated length of the set.
    /// If the key does not exist, create an empty set before performing the operation.
    fn srem(&mut self, key: String, val: String) -> Result<u64>;

    /// Return if value is a member of the set stored at key.
    fn sismember(&self, key: String, val: String) -> Result<bool>;

    /// Return all members of the set stored at key.
    fn smembers(&self, key: String) -> Result<Vec<&str>>;

    // Hashes Operations

    /// Get the value related to field in the hash stored at key.
    /// If the field does not exist, return None.
    /// If the key does not exist, return an error.
    fn hget(&self, key: String, field: String) -> Result<&str>;

    /// Set the field of the hash stored at key to value.
    /// If the field already existed, return previous value.
    /// Otherwise, return None.
    /// If the key does not exist, create an empty hash before performing the operation.
    fn hset(&mut self, key: String, field: String, val: String) -> Result<Option<&str>>;

    /// Remove field from the hash stored at key.
    /// Return the number of fields that were deleted.
    /// If the field does not exist, do nothing (and return 0).
    /// If the key does not exist, return an error.
    fn hdel(&mut self, key: String, field: String) -> Result<u64>;
}

#[derive(Debug)]
pub struct StdStore {
    strings: HashMap<String, String>,
    lists: HashMap<String, VecDeque<String>>,
    hashes: HashMap<String, HashMap<String, String>>,
    sets: HashMap<String, HashSet<String, String>>,
}

impl StdStore {
    fn update_int(&mut self, key: String, delta: i64, err: String) -> Result<i64> {
        match self.strings.get_mut(&key) {
            Some(val) => match val.to_string().parse::<i64>() {
                Ok(int) => {
                    *val = (int + delta).to_string();
                    return Ok(int + delta);
                }
                Err(_) => return Err(OperationalError { message: err }),
            },
            None => {
                return Err(OperationalError {
                    message: format!("Cannot increment non-integer values"),
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
        self.update_int(key, 1, format!("Cannot increment non-integer values"))
    }

    fn decr(&mut self, key: String) -> Result<i64> {
        self.update_int(key, -1, format!("Cannot decrement non-integer values"))
    }

    fn incrby(&mut self, key: String, delta: i64) -> Result<i64> {
        self.update_int(key, delta, format!("Cannot increment non-integer values"))
    }

    fn decrby(&mut self, key: String, delta: i64) -> Result<i64> {
        self.update_int(key, -delta, format!("Cannot decrement non-integer values"))
    }

    /// Lists Operations

    fn lpush(&mut self, key: String, val: String) -> Result<u64> {
        match self.lists.get_mut(&key) {
            Some(list) => {
                list.push_front(val);
                return Ok(list.len() as u64);
            }
            None => {
                let mut list = VecDeque::new();
                list.push_front(val);
                let len = list.len() as u64;
                self.lists.insert(key, list);
                return Ok(len);
            }
        }
    }

    fn rpush(&mut self, key: String, val: String) -> Result<u64> {
        match self.lists.get_mut(&key) {
            Some(list) => {
                list.push_back(val);
                return Ok(list.len() as u64);
            }
            None => {
                let mut list = VecDeque::new();
                list.push_back(val);
                let len = list.len() as u64;
                self.lists.insert(key, list);
                return Ok(len);
            }
        }
    }

    fn lpop(&mut self, key: String) -> Result<Option<String>> {
        match self.lists.get_mut(&key) {
            Some(list) => return Ok(list.pop_front()),
            None => return Ok(None),
        }
    }

    fn rpop(&mut self, key: String) -> Result<Option<String>> {
        match self.lists.get_mut(&key) {
            Some(list) => return Ok(list.pop_back()),
            None => return Ok(None),
        }
    }

    /// Sets Operations

    fn sadd(&mut self, key: String, val: String) -> Result<u64> {
        Ok(0)
    }

    fn srem(&mut self, key: String, val: String) -> Result<u64> {
        Ok(0)
    }

    fn sismember(&self, key: String, val: String) -> Result<bool> {
        Ok(true)
    }

    fn smembers(&self, key: String) -> Result<Vec<&str>> {
        Ok(vec![])
    }

    /// Hashes Operations

    fn hget(&self, key: String, field: String) -> Result<&str> {
        Ok("foo")
    }

    fn hset(&mut self, key: String, field: String, val: String) -> Result<Option<&str>> {
        Ok(None)
    }

    fn hdel(&mut self, key: String, field: String) -> Result<u64> {
        Ok(0)
    }
}

#[derive(Debug, Clone)]
pub struct OperationalError {
    message: String,
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

    fn test_std_incr_decr() {
        let mut store: StdStore = Store::new();
        store.set("foo".to_string(), 5.to_string());
        store.set("bar".to_string(), "test".to_string());
        store.set("baz".to_string(), (3.14).to_string());

        // Valid operations
        assert_eq!(store.incr("foo".to_string()).unwrap(), 6);
        assert_eq!(store.incrby("foo".to_string(), 10).unwrap(), 16);
        assert_eq!(store.decr("foo".to_string()).unwrap(), 15);
        assert_eq!(store.decrby("foo".to_string(), 10).unwrap(), 5);

        // Invalid operations
        assert_eq!(store.incr("dne".to_string()).is_ok(), false);
        assert_eq!(store.incr("bar".to_string()).is_ok(), false);
        assert_eq!(store.incr("baz".to_string()).is_ok(), false);
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
        assert!(false);
    }

    #[test]
    fn test_std_hashes() {
        assert!(false);
    }
}
