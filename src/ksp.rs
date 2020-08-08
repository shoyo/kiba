use crate::store::Store;

#[derive(Debug, PartialEq)]
pub enum Request {
    Ping,
    Get {
        key: String,
    },
    Set {
        key: String,
        val: String,
    },
    Incr {
        key: String,
    },
    Decr {
        key: String,
    },
    IncrBy {
        key: String,
        delta: i64,
    },
    DecrBy {
        key: String,
        delta: i64,
    },
    LPush {
        key: String,
        val: String,
    },
    RPush {
        key: String,
        val: String,
    },
    LPop {
        key: String,
    },
    RPop {
        key: String,
    },
    SAdd {
        key: String,
        val: String,
    },
    SRem {
        key: String,
        val: String,
    },
    SIsMember {
        key: String,
        val: String,
    },
    SMembers {
        key: String,
    },
    HGet {
        key: String,
        field: String,
    },
    HSet {
        key: String,
        field: String,
        val: String,
    },
    HDel {
        key: String,
        field: String,
    },
    NoOp,
    Invalid {
        error: String,
    },
}

#[derive(Debug, PartialEq)]
pub struct Response {
    pub body: String,
}

// Response body formats

pub fn f_pong() -> String {
    "PONG".to_string()
}

pub fn f_ok() -> String {
    "OK".to_string()
}

pub fn f_nil() -> String {
    "(nil)".to_string()
}

pub fn f_noop() -> String {
    '\u{0}'.to_string()
}

pub fn f_empty() -> String {
    "(empty list or set)".to_string()
}

pub fn f_int(int: i64) -> String {
    format!("(integer) {}", int)
}

pub fn f_uint(uint: u64) -> String {
    format!("(integer) {}", uint)
}

pub fn f_str(s: String) -> String {
    format!("\"{}\"", s)
}

pub fn f_vec(v: Vec<String>) -> String {
    let mut res = String::new();
    let mut iter = v.iter().enumerate();
    while let Some((idx, item)) = iter.next() {
        res.push_str(&format!("{}) {}", idx + 1, item));
        res.push('\n');
    }
    res
}

pub fn f_err(e: String) -> String {
    format!("(error) {}", e)
}

pub async fn execute(req: Request, store: &mut impl Store) -> Response {
    match req {
        Request::Ping => Response { body: f_pong() },
        Request::Get { key } => match store.get(key).unwrap() {
            Some(val) => Response { body: f_str(val) },
            None => Response { body: f_nil() },
        },
        Request::Set { key, val } => {
            let _ = store.set(key, val);
            Response { body: f_ok() }
        }
        Request::Incr { key } => match store.incr(key) {
            Ok(val) => Response { body: f_int(val) },
            Err(e) => Response {
                body: f_err(e.message),
            },
        },
        Request::Decr { key } => match store.decr(key) {
            Ok(val) => Response { body: f_int(val) },
            Err(e) => Response {
                body: f_err(e.message),
            },
        },
        Request::IncrBy { key, delta } => match store.incrby(key, delta) {
            Ok(val) => Response { body: f_int(val) },
            Err(e) => Response {
                body: f_err(e.message),
            },
        },
        Request::DecrBy { key, delta } => match store.decrby(key, delta) {
            Ok(val) => Response { body: f_int(val) },
            Err(e) => Response {
                body: f_err(e.message),
            },
        },
        Request::LPush { key, val } => {
            let len = store.lpush(key, val).unwrap();
            Response { body: f_uint(len) }
        }
        Request::RPush { key, val } => {
            let len = store.rpush(key, val).unwrap();
            Response { body: f_uint(len) }
        }
        Request::LPop { key } => match store.lpop(key).unwrap() {
            Some(val) => Response { body: f_str(val) },
            None => Response { body: f_nil() },
        },
        Request::RPop { key } => match store.rpop(key).unwrap() {
            Some(val) => Response { body: f_str(val) },
            None => Response { body: f_nil() },
        },
        Request::SAdd { key, val } => {
            let len = store.sadd(key, val).unwrap();
            Response { body: f_uint(len) }
        }
        Request::SRem { key, val } => {
            let len = store.srem(key, val).unwrap();
            Response { body: f_uint(len) }
        }
        Request::SIsMember { key, val } => match store.sismember(key, val).unwrap() {
            true => Response { body: f_uint(1) },
            false => Response { body: f_uint(0) },
        },
        Request::SMembers { key } => {
            let members = store.smembers(key).unwrap();
            match members.len() {
                0 => Response { body: f_empty() },
                _ => Response {
                    body: f_vec(members),
                },
            }
        }
        Request::HGet { key, field } => match store.hget(key, field).unwrap() {
            Some(val) => Response { body: f_str(val) },
            None => Response { body: f_nil() },
        },
        Request::HSet { key, field, val } => match store.hset(key, field, val).unwrap() {
            Some(_) => Response { body: f_uint(0) },
            None => Response { body: f_uint(1) },
        },
        Request::HDel { key, field } => {
            let del = store.hdel(key, field).unwrap();
            Response { body: f_uint(del) }
        }
        Request::NoOp => return Response { body: f_noop() },
        Request::Invalid { error } => return Response { body: f_err(error) },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::{StdStore, Store};

    #[tokio::test]
    async fn test_execute() {
        let mut store: StdStore = Store::new();

        // PING
        assert_eq!(
            execute(Request::Ping, &mut store).await,
            Response {
                body: "PONG".to_string()
            }
        );

        // SET AND GET
        assert_eq!(
            execute(
                Request::Set {
                    key: "foo".to_string(),
                    val: "bar".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "OK".to_string()
            }
        );
        assert_eq!(
            execute(
                Request::Get {
                    key: "foo".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "\"bar\"".to_string()
            }
        );
        assert_eq!(
            execute(
                Request::Get {
                    key: "baz".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "(nil)".to_string()
            }
        );

        // INCR, DECR, INCRBY, DECRBY
        assert_eq!(
            execute(
                Request::Incr {
                    key: "foo".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "(error) Cannot increment non-integer values".to_string()
            }
        );
        assert_eq!(
            execute(
                Request::Incr {
                    key: "baz".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "(error) Specified key does not exist".to_string()
            }
        );
        let _ = store.set("cnt".to_string(), 1.to_string());
        assert_eq!(
            execute(
                Request::Incr {
                    key: "cnt".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "(integer) 2".to_string()
            }
        );
        assert_eq!(
            execute(
                Request::Decr {
                    key: "cnt".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "(integer) 1".to_string()
            }
        );
        assert_eq!(
            execute(
                Request::IncrBy {
                    key: "cnt".to_string(),
                    delta: 10
                },
                &mut store
            )
            .await,
            Response {
                body: "(integer) 11".to_string()
            }
        );
        assert_eq!(
            execute(
                Request::DecrBy {
                    key: "cnt".to_string(),
                    delta: 20
                },
                &mut store
            )
            .await,
            Response {
                body: "(integer) -9".to_string()
            }
        );

        // List operations
        assert_eq!(
            execute(
                Request::LPush {
                    key: "letters".to_string(),
                    val: "a".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "(integer) 1".to_string()
            }
        );
        assert_eq!(
            execute(
                Request::RPush {
                    key: "letters".to_string(),
                    val: "b".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "(integer) 2".to_string()
            }
        );
        assert_eq!(
            execute(
                Request::RPop {
                    key: "letters".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "\"b\"".to_string()
            }
        );
        assert_eq!(
            execute(
                Request::LPop {
                    key: "letters".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "\"a\"".to_string()
            }
        );
        assert_eq!(
            execute(
                Request::LPop {
                    key: "letters".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "(nil)".to_string()
            }
        );

        // Set operations
        assert_eq!(
            execute(
                Request::SRem {
                    key: "words".to_string(),
                    val: "the".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "(integer) 0".to_string()
            }
        );
        assert_eq!(
            execute(
                Request::SAdd {
                    key: "words".to_string(),
                    val: "the".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "(integer) 1".to_string()
            }
        );
        assert_eq!(
            execute(
                Request::SAdd {
                    key: "words".to_string(),
                    val: "of".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "(integer) 2".to_string()
            }
        );
        assert_eq!(
            execute(
                Request::SIsMember {
                    key: "words".to_string(),
                    val: "of".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "(integer) 1".to_string()
            }
        );
        assert_eq!(
            execute(
                Request::SIsMember {
                    key: "words".to_string(),
                    val: "at".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "(integer) 0".to_string()
            }
        );

        // Hash operations
        assert_eq!(
            execute(
                Request::HGet {
                    key: "user1".to_string(),
                    field: "name".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "(nil)".to_string()
            }
        );
        assert_eq!(
            execute(
                Request::HSet {
                    key: "user1".to_string(),
                    field: "name".to_string(),
                    val: "Jane Doe".to_string(),
                },
                &mut store
            )
            .await,
            Response {
                body: "(integer) 1".to_string()
            }
        );
        assert_eq!(
            execute(
                Request::HSet {
                    key: "user1".to_string(),
                    field: "name".to_string(),
                    val: "John Smith".to_string(),
                },
                &mut store
            )
            .await,
            Response {
                body: "(integer) 0".to_string()
            }
        );
        assert_eq!(
            execute(
                Request::HGet {
                    key: "user1".to_string(),
                    field: "name".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "\"John Smith\"".to_string()
            }
        );
        assert_eq!(
            execute(
                Request::HDel {
                    key: "user1".to_string(),
                    field: "address".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "(integer) 0".to_string()
            }
        );
        assert_eq!(
            execute(
                Request::HDel {
                    key: "user1".to_string(),
                    field: "name".to_string()
                },
                &mut store
            )
            .await,
            Response {
                body: "(integer) 1".to_string()
            }
        );
    }
}
