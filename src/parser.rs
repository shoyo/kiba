use crate::executor::Request;
use crate::lexer::*;
use log::error;

fn invalid_argc_request(expected: usize, actual: usize) -> Request {
    Request::Invalid {
        error: format!(
            "Unexpected number of arguments. Expected {}, got {}",
            expected, actual
        ),
    }
}

async fn validate_misc_op(op: MiscOp, argv: Vec<&str>) -> Request {
    let argc = argv.len();
    match op {
        MiscOp::Ping => {
            if argc != 0 {
                return invalid_argc_request(0, argc);
            }
            Request::Ping
        }
    }
}

async fn validate_string_op(op: StringOp, argv: Vec<&str>) -> Request {
    let argc = argv.len();
    match op {
        StringOp::Get => {
            if argc != 1 {
                return invalid_argc_request(1, argc);
            }
            Request::Get {
                key: argv[0].to_string(),
            }
        }
        StringOp::Set => {
            if argc != 2 {
                return invalid_argc_request(2, argc);
            }
            Request::Set {
                key: argv[0].to_string(),
                val: argv[1].to_string(),
            }
        }
        StringOp::Incr => {
            if argc != 1 {
                return invalid_argc_request(1, argc);
            }
            Request::Incr {
                key: argv[0].to_string(),
            }
        }
        StringOp::Decr => {
            if argc != 1 {
                return invalid_argc_request(1, argc);
            }
            Request::Decr {
                key: argv[0].to_string(),
            }
        }
        StringOp::IncrBy => {
            if argc != 2 {
                return invalid_argc_request(2, argc);
            }
            let delta = argv[1].to_string().parse::<i64>();
            match delta {
                Ok(d) => Request::IncrBy {
                    key: argv[0].to_string(),
                    delta: d,
                },
                Err(_) => Request::Invalid {
                    error: format!("Value to increment by is a non-integer"),
                },
            }
        }
        StringOp::DecrBy => {
            if argc != 2 {
                return invalid_argc_request(2, argc);
            }
            let delta = argv[1].to_string().parse::<i64>();
            match delta {
                Ok(d) => Request::DecrBy {
                    key: argv[0].to_string(),
                    delta: d,
                },
                Err(_) => Request::Invalid {
                    error: format!("Value to decrement by is a non-integer"),
                },
            }
        }
    }
}

async fn validate_list_op(op: ListOp, argv: Vec<&str>) -> Request {
    let argc = argv.len();
    match op {
        ListOp::LPush => {
            if argc != 2 {
                return invalid_argc_request(2, argc);
            }
            Request::LPush {
                key: argv[0].to_string(),
                val: argv[1].to_string(),
            }
        }
        ListOp::RPush => {
            if argc != 2 {
                return invalid_argc_request(2, argc);
            }
            Request::RPush {
                key: argv[0].to_string(),
                val: argv[1].to_string(),
            }
        }
        ListOp::LPop => {
            if argc != 1 {
                return invalid_argc_request(1, argc);
            }
            Request::LPop {
                key: argv[0].to_string(),
            }
        }
        ListOp::RPop => {
            if argc != 1 {
                return invalid_argc_request(1, argc);
            }
            Request::RPop {
                key: argv[0].to_string(),
            }
        }
    }
}

async fn validate_set_op(op: SetOp, argv: Vec<&str>) -> Request {
    let argc = argv.len();
    match op {
        SetOp::SAdd => {
            if argc != 2 {
                return invalid_argc_request(2, argc);
            }
            Request::SAdd {
                key: argv[0].to_string(),
                val: argv[1].to_string(),
            }
        }
        SetOp::SRem => {
            if argc != 2 {
                return invalid_argc_request(2, argc);
            }
            Request::SRem {
                key: argv[0].to_string(),
                val: argv[1].to_string(),
            }
        }
        SetOp::SIsMember => {
            if argc != 2 {
                return invalid_argc_request(2, argc);
            }
            Request::SIsMember {
                key: argv[0].to_string(),
                val: argv[1].to_string(),
            }
        }
        SetOp::SMembers => {
            if argc != 1 {
                return invalid_argc_request(1, argc);
            }
            Request::SMembers {
                key: argv[0].to_string(),
            }
        }
    }
}

async fn validate_hash_op(op: HashOp, argv: Vec<&str>) -> Request {
    let argc = argv.len();
    match op {
        HashOp::HGet => {
            if argc != 2 {
                return invalid_argc_request(2, argc);
            }
            Request::HGet {
                key: argv[0].to_string(),
                field: argv[1].to_string(),
            }
        }
        HashOp::HSet => {
            if argc != 3 {
                return invalid_argc_request(3, argc);
            }
            Request::HSet {
                key: argv[0].to_string(),
                field: argv[1].to_string(),
                val: argv[2].to_string(),
            }
        }
        HashOp::HDel => {
            if argc != 2 {
                return invalid_argc_request(2, argc);
            }
            Request::HDel {
                key: argv[0].to_string(),
                field: argv[1].to_string(),
            }
        }
    }
}

async fn validate_meta_op(op: MetaOp, _argv: Vec<&str>) -> Request {
    match op {
        MetaOp::NoOp => Request::NoOp,
        MetaOp::Quit => Request::Quit,
        MetaOp::Unrecognized => Request::Invalid {
            error: format!("Unrecognized operator"),
        },
    }
}

async fn parse(tokens: LexerResult<'_>) -> Request {
    match tokens.op {
        Operator::MiscOp(op) => validate_misc_op(op, tokens.argv).await,
        Operator::StringOp(op) => validate_string_op(op, tokens.argv).await,
        Operator::ListOp(op) => validate_list_op(op, tokens.argv).await,
        Operator::SetOp(op) => validate_set_op(op, tokens.argv).await,
        Operator::HashOp(op) => validate_hash_op(op, tokens.argv).await,
        Operator::MetaOp(op) => validate_meta_op(op, tokens.argv).await,
    }
}

pub async fn parse_request(bytes: &[u8]) -> Request {
    let text = match std::str::from_utf8(bytes) {
        Ok(txt) => txt,
        Err(_) => {
            error!("Input bytestream could not be converted into valid UTF-8");
            std::process::exit(1);
        }
    };
    let mut lexer = Lexer::new(text);
    let tokens = lexer.tokenize();
    parse(tokens).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_request_misc() {
        assert_eq!(parse_request(b"PING").await, Request::Ping);
        assert_eq!(
            parse_request("\u{0}PING\u{0}\u{0}\u{0}".as_bytes()).await,
            Request::Ping
        );
        assert_eq!(
            parse_request(b"PING extra args").await,
            Request::Invalid {
                error: "Unexpected number of arguments. Expected 0, got 2".to_string()
            }
        );
    }

    #[tokio::test]
    async fn test_parse_request_strings() {
        assert_eq!(
            parse_request(b"GET foo").await,
            Request::Get {
                key: "foo".to_string()
            }
        );
        assert_eq!(
            parse_request(b"GET").await,
            Request::Invalid {
                error: "Unexpected number of arguments. Expected 1, got 0".to_string()
            }
        );
        // Mixed case
        assert_eq!(
            parse_request(b"gEt foo").await,
            Request::Get {
                key: "foo".to_string()
            }
        );
        // Quotations containing whitespace
        assert_eq!(
            parse_request(b"get \"foo bar\"").await,
            Request::Get {
                key: "foo bar".to_string()
            }
        );
        // Operator and operand in quotes
        assert_eq!(
            parse_request(b"\"GET\" \"foo\"").await,
            Request::Get {
                key: "foo".to_string()
            }
        );
        // No closing quotation mark
        assert_eq!(
            parse_request(b"GET \"foo bar").await,
            Request::Get {
                key: "foo bar".to_string()
            }
        );
        // Backslash-quote to include quote
        assert_eq!(
            parse_request(b"GET \\\"foo").await,
            Request::Get {
                key: "\\\"foo".to_string()
            }
        );
        assert_eq!(
            parse_request(b"set foo bar").await,
            Request::Set {
                key: "foo".to_string(),
                val: "bar".to_string()
            }
        );
        assert_eq!(
            parse_request(b"set \"foo\" \"bar\"").await,
            Request::Set {
                key: "foo".to_string(),
                val: "bar".to_string()
            }
        );
        assert_eq!(
            parse_request(b"set foo \"bar").await,
            Request::Set {
                key: "foo".to_string(),
                val: "bar".to_string()
            }
        );
        assert_eq!(
            parse_request(b"set foo \"").await,
            Request::Set {
                key: "foo".to_string(),
                val: "".to_string()
            }
        );
        assert_eq!(
            parse_request(b"SET foo").await,
            Request::Invalid {
                error: "Unexpected number of arguments. Expected 2, got 1".to_string()
            }
        );
        assert_eq!(
            parse_request(b"GET SET").await,
            Request::Get {
                key: "SET".to_string()
            }
        );
        assert_eq!(
            parse_request(b"INCR foo").await,
            Request::Incr {
                key: "foo".to_string()
            }
        );
        assert_eq!(
            parse_request(b"INCR").await,
            Request::Invalid {
                error: "Unexpected number of arguments. Expected 1, got 0".to_string()
            }
        );
        assert_eq!(
            parse_request(b"deCR foo").await,
            Request::Decr {
                key: "foo".to_string()
            }
        );
        assert_eq!(
            parse_request(b"DECR foo bar baz").await,
            Request::Invalid {
                error: "Unexpected number of arguments. Expected 1, got 3".to_string()
            }
        );
        assert_eq!(
            parse_request(b"INCRBY foo 10").await,
            Request::IncrBy {
                key: "foo".to_string(),
                delta: 10
            }
        );
        assert_eq!(
            parse_request(b"INCRBY   foo    10.1").await,
            Request::Invalid {
                error: "Value to increment by is a non-integer".to_string()
            }
        );
        assert_eq!(
            parse_request(b"DECRBY foo 20").await,
            Request::DecrBy {
                key: "foo".to_string(),
                delta: 20
            }
        );
        assert_eq!(
            parse_request(b"DECRBY foo bar").await,
            Request::Invalid {
                error: "Value to decrement by is a non-integer".to_string()
            }
        );
    }

    #[tokio::test]
    async fn test_parse_request_lists() {
        assert_eq!(
            parse_request(b"LPUSH foo apples").await,
            Request::LPush {
                key: "foo".to_string(),
                val: "apples".to_string()
            }
        );
        assert_eq!(
            parse_request(b"LPUSH foo \"apples\"").await,
            Request::LPush {
                key: "foo".to_string(),
                val: "apples".to_string()
            }
        );
        assert_eq!(
            parse_request(b"LPUSH foo").await,
            Request::Invalid {
                error: "Unexpected number of arguments. Expected 2, got 1".to_string()
            }
        );
        assert_eq!(
            parse_request(b"RPUSH foo apples").await,
            Request::RPush {
                key: "foo".to_string(),
                val: "apples".to_string()
            }
        );
        assert_eq!(
            parse_request(b"RPUSH foo").await,
            Request::Invalid {
                error: "Unexpected number of arguments. Expected 2, got 1".to_string()
            }
        );
        assert_eq!(
            parse_request(b"lpop foo").await,
            Request::LPop {
                key: "foo".to_string(),
            }
        );
        assert_eq!(
            parse_request(b"LPop foo apples").await,
            Request::Invalid {
                error: "Unexpected number of arguments. Expected 1, got 2".to_string()
            }
        );
        assert_eq!(
            parse_request(b"RPop foo").await,
            Request::RPop {
                key: "foo".to_string(),
            }
        );
        assert_eq!(
            parse_request(b"RPOP foo apples").await,
            Request::Invalid {
                error: "Unexpected number of arguments. Expected 1, got 2".to_string()
            }
        );
    }

    #[tokio::test]
    async fn test_parse_request_sets() {
        assert_eq!(
            parse_request(b"SADD foo apples").await,
            Request::SAdd {
                key: "foo".to_string(),
                val: "apples".to_string(),
            }
        );
        assert_eq!(
            parse_request(b"SAdd foo bar baz").await,
            Request::Invalid {
                error: "Unexpected number of arguments. Expected 2, got 3".to_string()
            }
        );
        assert_eq!(
            parse_request(b"SREM foo apples").await,
            Request::SRem {
                key: "foo".to_string(),
                val: "apples".to_string(),
            }
        );
        assert_eq!(
            parse_request(b"SREM foo bananas oranges").await,
            Request::Invalid {
                error: "Unexpected number of arguments. Expected 2, got 3".to_string(),
            }
        );
        assert_eq!(
            parse_request(b"SISMEMBER foo apples").await,
            Request::SIsMember {
                key: "foo".to_string(),
                val: "apples".to_string(),
            }
        );
        assert_eq!(
            parse_request(b"SISMEMBER foo apples oranges").await,
            Request::Invalid {
                error: "Unexpected number of arguments. Expected 2, got 3".to_string(),
            }
        );
        assert_eq!(
            parse_request(b"SMEMBERS foo").await,
            Request::SMembers {
                key: "foo".to_string(),
            }
        );
        assert_eq!(
            parse_request(b"SMEMBERS foo apples oranges").await,
            Request::Invalid {
                error: "Unexpected number of arguments. Expected 1, got 3".to_string()
            }
        );
    }

    #[tokio::test]
    async fn test_parse_request_hashes() {
        assert_eq!(
            parse_request(b"HGET foo name").await,
            Request::HGet {
                key: "foo".to_string(),
                field: "name".to_string()
            }
        );
        assert_eq!(
            parse_request(b"HGET foo name address").await,
            Request::Invalid {
                error: "Unexpected number of arguments. Expected 2, got 3".to_string()
            }
        );
        assert_eq!(
            parse_request(b"HSET foo name Joe").await,
            Request::HSet {
                key: "foo".to_string(),
                field: "name".to_string(),
                val: "Joe".to_string()
            }
        );
        assert_eq!(
            parse_request(b"HSET foo name").await,
            Request::Invalid {
                error: "Unexpected number of arguments. Expected 3, got 2".to_string()
            }
        );
        assert_eq!(
            parse_request(b"HDel foo name").await,
            Request::HDel {
                key: "foo".to_string(),
                field: "name".to_string()
            }
        );
        assert_eq!(
            parse_request(b"HDel foo name John").await,
            Request::Invalid {
                error: "Unexpected number of arguments. Expected 2, got 3".to_string()
            }
        );
    }

    #[tokio::test]
    async fn test_parse_request_meta() {
        assert_eq!(
            parse_request(b"NOTACOMMAND foo bar").await,
            Request::Invalid {
                error: "Unrecognized operator".to_string()
            }
        );
        assert_eq!(parse_request(b"").await, Request::NoOp);
        assert_eq!(parse_request(b"   ").await, Request::NoOp);
        assert_eq!(parse_request("\u{0}".as_bytes()).await, Request::NoOp);
    }
}
