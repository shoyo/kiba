use std::collections::{HashMap, HashSet};

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
    /// If the value is not/cannot beinterpreted as an integer, return an error.
    /// This operation is limited to 64-bit integers.
    /// Time complexity: O(1)
    fn incrby(&mut self, key: String, by: i64) -> Result<i64>;

    /// Decrement the value of a key by a specifed amount.
    /// Return the updated value.
    /// If the key does not exist, return an error (unlike Redis).
    /// If the value is not/cannot beinterpreted as an integer, return an error.
    /// This operation is limited to 64-bit integers.
    fn decrby(&mut self, key: String, by: i64) -> Result<i64>;

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
    /// If the key does not exist, return an error.
    fn lpop(&mut self, key: String, val: String) -> Result<u64>;

    /// Remove and return the element at the head of list stored at key.
    /// If the key does not exist, return an error.
    fn rpop(&mut self, key: String, val: String) -> Result<u64>;

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
    lists: HashMap<String, Vec<String>>,
    hashes: HashMap<String, HashMap<String, String>>,
    sets: HashMap<String, HashSet<String, String>>,
}

impl StdStore {
    fn update_int(&mut self, key: String, delta: i64, err: String) -> Result<i64> {
        match self.strings.get(&key) {
            Some(val) => match val.to_string().parse::<i64>() {
                Ok(int) => {
                    self.strings.insert(key, (int + delta).to_string());
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
        self.update_int(key, delta, format!("Cannot decrement non-integer values"))
    }

    /// Lists Operations

    fn lpush(&mut self, key: String, val: String) -> Result<u64> {
        Ok(0)
    }

    fn rpush(&mut self, key: String, val: String) -> Result<u64> {
        Ok(0)
    }

    fn lpop(&mut self, key: String, val: String) -> Result<u64> {
        Ok(0)
    }

    fn rpop(&mut self, key: String, val: String) -> Result<u64> {
        Ok(0)
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
    fn it_works() {
        assert_eq!(true, true);
    }
}
