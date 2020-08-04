use std::cmp::Eq;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// A shared interface for implementations of store.
/// Time and space complexities of each function are not guaranteed and
/// depends on each implementation. (hash vs. btree, vec vs. linked list etc.)
trait Store {
    /// Create a new store.
    fn new() -> Self;

    // Strings Operations

    /// Get the value of a key.
    /// If the key does not exist, return None.
    fn get(&self, key: &str) -> Result<&str>;

    /// Set the value of a key.
    /// If the key already existed, return previous value.
    /// Otherwise, return None.
    fn set(&mut self, key: &str, val: &str) -> Result<Option<&str>>;

    /// Increment the value of a key by 1.
    /// Return the updated value.
    /// If the key does not exist, return an error (unlike Redis).
    /// If the value is not/cannot be interpreted as an integer, return an error.
    /// This operation is limited to 64-bit integers.
    fn incr(&mut self, key: &str) -> Result<i64>;

    /// Decrement the value of a key by 1.
    /// Return the updated value.
    /// If the key does not exist, return an error (unlike Redis).
    /// If the value is not/cannot be interpreted as an integer, return an error.
    /// This operation is limited to 64-bit integers.
    fn decr(&mut self, key: &str) -> Result<i64>;

    /// Increment the value of a key by a specified amount.
    /// Return the updated value.
    /// If the key does not exist, return an error (unlike Redis).
    /// If the value is not/cannot beinterpreted as an integer, return an error.
    /// This operation is limited to 64-bit integers.
    /// Time complexity: O(1)
    fn incrby(&mut self, key: &str, by: i64) -> Result<i64>;

    /// Decrement the value of a key by a specifed amount.
    /// Return the updated value.
    /// If the key does not exist, return an error (unlike Redis).
    /// If the value is not/cannot beinterpreted as an integer, return an error.
    /// This operation is limited to 64-bit integers.
    fn decrby(&mut self, key: &str, by: i64) -> Result<i64>;

    // Lists Operations

    /// Insert value at the head of list stored at key.
    /// Return the updated length of the list.
    /// If the key does not exist, create an empty list before performing the operation.
    fn lpush(&mut self, key: &str, val: &str) -> Result<u64>;

    /// Insert value at the tail of list stored at key.
    /// Return the updated length of the list.
    /// If the key does not exist, create an empty list before performing the operation.
    fn rpush(&mut self, key: &str, val: &str) -> Result<u64>;

    /// Remove and return the element at the head of list stored at key.
    /// If the key does not exist, return an error.
    fn lpop(&mut self, key: &str, val: &str) -> Result<u64>;

    /// Remove and return the element at the head of list stored at key.
    /// If the key does not exist, return an error.
    fn rpop(&mut self, key: &str, val: &str) -> Result<u64>;

    // Sets Operations

    /// Insert value in the set stored at key.
    /// Return the updated length of the set.
    /// If the key does not exist, create an empty set before performing the operation.
    fn sadd(&mut self, key: &str, val: &str) -> Result<u64>;

    /// Remove value in the set stored at key.
    /// Return the updated length of the set.
    /// If the key does not exist, create an empty set before performing the operation.
    fn srem(&mut self, key: &str, val: &str) -> Result<u64>;

    /// Return if value is a member of the set stored at key.
    fn sismember(&self, key: &str, val: &str) -> Result<bool>;

    /// Return all members of the set stored at key.
    fn smembers(&self, key: &str) -> Result<Vec<&str>>;

    // Hashes Operations

    /// Get the value related to field in the hash stored at key.
    /// If the field does not exist, return None.
    /// If the key does not exist, return an error.
    fn hget(&self, key: &str, field: &str) -> Result<&str>;

    /// Set the field of the hash stored at key to value.
    /// If the field already existed, return previous value.
    /// Otherwise, return None.
    /// If the key does not exist, create an empty hash before performing the operation.
    fn hset(&mut self, key: &str, field: &str, val: &str) -> Result<Option<&str>>;

    /// Remove field from the hash stored at key.
    /// Return the number of fields that were deleted.
    /// If the field does not exist, do nothing (and return 0).
    /// If the key does not exist, return an error.
    fn hdel(&mut self, key: &str, field: &str) -> Result<u64>;
}

#[derive(Debug)]
pub struct StdStore {
    strings: HashMap<String, String>,
    lists: HashMap<String, Vec<String>>,
    hashes: HashMap<String, HashMap<String, String>>,
    sets: HashMap<String, HashSet<String, String>>,
}

impl Store for StdStore {
    fn new() -> Self {
        Store {
            strings: HashMap::new(),
            lists: HashMap::new(),
            hashes: HashMap::new(),
            sets: HashMap::new(),
        }
    }

    // Strings Operations

    pub fn get(&self, key: &str) -> Result<Option<&str>> {
        match self.strings.get(&key) {
            Some(val) => Ok(Some(val)),
            None => Ok(None),
        }
    }

    pub fn set(&mut self, key: &str, val: &str) -> Result<Option<&str>> {
        // If some contraints are not satisfied (e.g. out of memory)
        // return an error
        match self.strings.insert(key, val) {
            Some(val) => Ok(Some(val)),
            None => Ok(None),
        }
    }

    pub fn incr(&mut self, key: &str) -> Result<i64> {}

    pub fn decr(&mut self, key: &str) -> Result<i64> {}

    pub fn incrby(&mut self, key: &str, by: i64) -> Result<i64> {}

    pub fn decrby(&mut self, key: &str, by: i64) -> Result<i64> {}

    /// Lists Operations

    pub fn lpush(&mut self, key: &str, val: &str) -> Result<u64> {}

    pub fn rpush(&mut self, key: &str, val: &str) -> Result<u64> {}

    pub fn lpop(&mut self, key: &str, val: &str) -> Result<u64> {}

    pub fn rpop(&mut self, key: &str, val: &str) -> Result<u64> {}

    /// Sets Operations

    pub fn sadd(&mut self, key: &str, val: &str) -> Result<u64> {}

    pub fn srem(&mut self, key: &str, val: &str) -> Result<u64> {}

    pub fn sismember(&self, key: &str, val: &str) -> Result<bool> {}

    pub fn smembers(&self, key: &str) -> Result<Vec<&str>> {}

    /// Hashes Operations

    pub fn hset(&mut self, key: &str, hkey: &str, hval: &str) -> Result<Option<&str>> {}

    pub fn hget(&self, key: &str, hkey: &str) -> Result<&str> {}

    pub fn hdel(&mut self, key: &str, hkey: &str) -> Result<()> {}
}

#[derive(Debug, Clone)]
struct OperationalError;

impl fmt::Display for OperationalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Operational Error occured")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
