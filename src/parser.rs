use crate::ksp::Request;

#[derive(Clone, Debug, PartialEq)]
enum Token {
    MiscOp(MiscOp),
    StringOp(StringOp),
    ListOp(ListOp),
    SetOp(SetOp),
    HashOp(HashOp),
    Operand(String),
}

#[derive(Clone, Debug, PartialEq)]
enum MiscOp {
    Ping,
}

#[derive(Clone, Debug, PartialEq)]
enum StringOp {
    Get,
    Set,
    Incr,
    Decr,
    IncrBy,
    DecrBy,
}

#[derive(Clone, Debug, PartialEq)]
enum ListOp {
    LPush,
    RPush,
    LPop,
    RPop,
}

#[derive(Clone, Debug, PartialEq)]
enum SetOp {
    SAdd,
    SRem,
    SIsMember,
    SMembers,
}

#[derive(Clone, Debug, PartialEq)]
enum HashOp {
    HGet,
    HSet,
    HDel,
}

#[derive(Clone, Debug)]
struct ParserError {
    message: String,
}

fn invalid_argc_error(expected: usize, actual: usize) -> ParserError {
    ParserError {
        message: format!(
            "Unexpected number of arguments. Expected {}, got {}",
            expected, actual
        ),
    }
}

fn not_operand_error() -> ParserError {
    ParserError {
        message: format!("[Internal parser error] Expected operand token"),
    }
}

async fn tokenize(bytes: &[u8]) -> Vec<Token> {
    let mut tokens = Vec::new();
    let text = std::str::from_utf8(bytes).unwrap();
    let mut chunks = text
        .split(|c: char| c.is_whitespace() || c == '\u{0}')
        .filter(|s| !s.is_empty());

    if let Some(chunk) = chunks.next() {
        match chunk.to_uppercase().as_str() {
            "PING" => tokens.push(Token::MiscOp(MiscOp::Ping)),
            "GET" => tokens.push(Token::StringOp(StringOp::Get)),
            "SET" => tokens.push(Token::StringOp(StringOp::Set)),
            "INCR" => tokens.push(Token::StringOp(StringOp::Incr)),
            "DECR" => tokens.push(Token::StringOp(StringOp::Decr)),
            "INCRBY" => tokens.push(Token::StringOp(StringOp::IncrBy)),
            "DECRBY" => tokens.push(Token::StringOp(StringOp::DecrBy)),
            "LPUSH" => tokens.push(Token::ListOp(ListOp::LPush)),
            "RPUSH" => tokens.push(Token::ListOp(ListOp::RPush)),
            "LPOP" => tokens.push(Token::ListOp(ListOp::LPop)),
            "RPOP" => tokens.push(Token::ListOp(ListOp::RPop)),
            "SADD" => tokens.push(Token::SetOp(SetOp::SAdd)),
            "SREM" => tokens.push(Token::SetOp(SetOp::SRem)),
            "SISMEMBER" => tokens.push(Token::SetOp(SetOp::SIsMember)),
            "SMEMBERS" => tokens.push(Token::SetOp(SetOp::SMembers)),
            "HGET" => tokens.push(Token::HashOp(HashOp::HGet)),
            "HSET" => tokens.push(Token::HashOp(HashOp::HSet)),
            "HDEL" => tokens.push(Token::HashOp(HashOp::HDel)),
            _ => tokens.push(Token::Operand(chunk.to_string())),
        }
    }
    while let Some(chunk) = chunks.next() {
        tokens.push(Token::Operand(chunk.to_string()));
    }
    tokens
}

async fn parse_misc_op(op: MiscOp, argc: usize, argv: Vec<Token>) -> Result<Request, ParserError> {
    match op {
        MiscOp::Ping => {
            if argc != 1 {
                return Err(invalid_argc_error(0, argc - 1));
            }
            Ok(Request::Ping)
        }
    }
}

async fn parse_string_op(
    op: StringOp,
    argc: usize,
    argv: Vec<Token>,
) -> Result<Request, ParserError> {
    match op {
        StringOp::Get => {
            if argc != 2 {
                return Err(invalid_argc_error(1, argc - 1));
            }
            if let Token::Operand(k) = &argv[1] {
                return Ok(Request::Get { key: k.to_string() });
            }
            Err(not_operand_error())
        }
        StringOp::Set => {
            if argc != 3 {
                return Err(invalid_argc_error(2, argc - 1));
            }
            if let Token::Operand(k) = &argv[1] {
                if let Token::Operand(v) = &argv[2] {
                    return Ok(Request::Set {
                        key: k.to_string(),
                        val: v.to_string(),
                    });
                }
            }
            Err(not_operand_error())
        }
        StringOp::Incr => {
            if argc != 2 {
                return Err(invalid_argc_error(1, argc - 1));
            }
            if let Token::Operand(k) = &argv[1] {
                return Ok(Request::Incr { key: k.to_string() });
            }
            Err(not_operand_error())
        }
        StringOp::Decr => {
            if argc != 2 {
                return Err(invalid_argc_error(1, argc - 1));
            }
            if let Token::Operand(k) = &argv[1] {
                return Ok(Request::Decr { key: k.to_string() });
            }
            Err(not_operand_error())
        }
        StringOp::IncrBy => {
            if argc != 3 {
                return Err(invalid_argc_error(2, argc - 1));
            }
            let Token::Operand(k) = &argv[1];
            let Token::Operand(d) = &argv[2];
            let parse_d = d.to_string().parse::<i64>();
            match parse_d {
                Ok(delta) => Ok(Request::IncrBy {
                    key: k.to_string(),
                    delta: delta,
                }),
                Err(_) => Err(ParserError {
                    message: format!("Value is a non-integer or out of range"),
                }),
            }
        }
        StringOp::DecrBy => {
            if argc != 3 {
                return Err(invalid_argc_error(2, argc - 1));
            }
            let Token::Operand(k) = &argv[1];
            let Token::Operand(d) = &argv[2];
            let parse_d = d.to_string().parse::<i64>();
            match parse_d {
                Ok(delta) => Ok(Request::DecrBy {
                    key: k.to_string(),
                    delta: delta,
                }),
                Err(_) => Err(ParserError {
                    message: format!("Value is a non-integer or out of range"),
                }),
            }
        }
    }
}

async fn parse_list_op(op: ListOp, argc: usize, argv: Vec<Token>) -> Result<Request, ParserError> {
    match op {
        ListOp::LPush => {
            if argc != 3 {
                return Err(invalid_argc_error(2, argc - 1));
            }
            let Token::Operand(k) = &argv[1];
            let Token::Operand(v) = &argv[2];
            Ok(Request::LPush {
                key: k.to_string(),
                val: v.to_string(),
            })
        }
        ListOp::RPush => {
            if argc != 3 {
                return Err(invalid_argc_error(2, argc - 1));
            }
            let Token::Operand(k) = &argv[1];
            let Token::Operand(v) = &argv[2];
            Ok(Request::RPush {
                key: k.to_string(),
                val: v.to_string(),
            })
        }
        ListOp::LPop => {
            if argc != 2 {
                return Err(invalid_argc_error(1, argc - 1));
            }
            let Token::Operand(k) = &argv[1];
            Ok(Request::LPop { key: k.to_string() })
        }
        ListOp::RPop => {
            if argc != 2 {
                return Err(invalid_argc_error(1, argc - 1));
            }
            let Token::Operand(k) = &argv[1];
            Ok(Request::RPop { key: k.to_string() })
        }
    }
}

async fn parse_set_op(op: SetOp, argc: usize, argv: Vec<Token>) -> Result<Request, ParserError> {
    match op {
        SetOp::SAdd => {
            if argc != 3 {
                return Err(invalid_argc_error(2, argc - 1));
            }
            let Token::Operand(k) = &argv[1];
            let Token::Operand(v) = &argv[2];
            Ok(Request::SAdd {
                key: k.to_string(),
                val: v.to_string(),
            })
        }
        SetOp::SRem => {
            if argc != 3 {
                return Err(invalid_argc_error(2, argc - 1));
            }
            let Token::Operand(k) = &argv[1];
            let Token::Operand(v) = &argv[2];
            Ok(Request::SRem {
                key: k.to_string(),
                val: v.to_string(),
            })
        }
        SetOp::SIsMember => {
            if argc != 3 {
                return Err(invalid_argc_error(2, argc - 1));
            }
            let Token::Operand(k) = &argv[1];
            let Token::Operand(v) = &argv[2];
            Ok(Request::SIsMember {
                key: k.to_string(),
                val: v.to_string(),
            })
        }
        SetOp::SMembers => {
            if argc != 2 {
                return Err(invalid_argc_error(1, argc - 1));
            }
            let Token::Operand(k) = &argv[1];
            Ok(Request::SMembers { key: k.to_string() })
        }
    }
}

async fn parse_hash_op(op: HashOp, argc: usize, argv: Vec<Token>) -> Result<Request, ParserError> {
    match op {
        HashOp::HGet => {
            if argc != 3 {
                return Err(invalid_argc_error(2, argc - 1));
            }
            let Token::Operand(k) = &argv[1];
            let Token::Operand(f) = &argv[2];
            Ok(Request::HGet {
                key: k.to_string(),
                field: f.to_string(),
            })
        }
        HashOp::HSet => {
            if argc != 4 {
                return Err(invalid_argc_error(3, argc - 1));
            }
            let Token::Operand(k) = &argv[1];
            let Token::Operand(f) = &argv[2];
            let Token::Operand(v) = &argv[3];
            Ok(Request::HSet {
                key: k.to_string(),
                field: f.to_string(),
                val: v.to_string(),
            })
        }
        HashOp::HDel => {
            if argc != 3 {
                return Err(invalid_argc_error(2, argc - 1));
            }
            let Token::Operand(k) = &argv[1];
            let Token::Operand(f) = &argv[2];
            Ok(Request::HDel {
                key: k.to_string(),
                field: f.to_string(),
            })
        }
    }
}

async fn parse_tokens(tokens: Vec<Token>) -> Result<Request, ParserError> {
    let argc = tokens.len();
    if argc == 0 {
        return Ok(Request::NoOp);
    }
    match &tokens[0] {
        Token::MiscOp(op) => parse_misc_op(op.clone(), argc, tokens).await,
        Token::StringOp(op) => parse_string_op(op.clone(), argc, tokens).await,
        Token::ListOp(op) => parse_list_op(op.clone(), argc, tokens).await,
        Token::SetOp(op) => parse_set_op(op.clone(), argc, tokens).await,
        Token::HashOp(op) => parse_hash_op(op.clone(), argc, tokens).await,
        _ => Err(ParserError {
            message: format!("Invalid op token"),
        }),
    }
}

pub async fn parse_request(bytes: &[u8]) -> Request {
    let tokens = tokenize(bytes).await;
    match parse_tokens(tokens).await {
        Ok(req) => req,
        Err(e) => Request::Invalid { error: e.message },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tokenize() {
        assert_eq!(tokenize(b"PING    ").await, vec![Token::Ping]);
        assert_eq!(
            tokenize("SET foo bar\u{0}\u{0}\u{0}".as_bytes()).await,
            vec![
                Token::Set,
                Token::Operand("foo".to_string()),
                Token::Operand("bar".to_string())
            ]
        );
        assert_eq!(
            tokenize(b"  GET    baz       ").await,
            vec![Token::Get, Token::Operand("baz".to_string())]
        );
        assert_eq!(
            tokenize(b" set time now").await,
            vec![
                Token::Set,
                Token::Operand("time".to_string()),
                Token::Operand("now".to_string()),
            ]
        );
        assert_eq!(
            tokenize(b"is invalid request").await,
            vec![
                Token::Operand("is".to_string()),
                Token::Operand("invalid".to_string()),
                Token::Operand("request".to_string())
            ]
        );
        assert_eq!(tokenize(b" ").await, vec![]);
    }

    #[tokio::test]
    async fn test_valid_parse_tokens() {
        assert_eq!(
            parse_tokens(vec![Token::Ping]).await.unwrap(),
            Request::Ping
        );
        assert_eq!(
            parse_tokens(vec![Token::Get, Token::Operand("foo".to_string())])
                .await
                .unwrap(),
            Request::Get {
                key: "foo".to_string()
            }
        );
        assert_eq!(
            parse_tokens(vec![
                Token::Set,
                Token::Operand("foo".to_string()),
                Token::Operand("bar".to_string())
            ])
            .await
            .unwrap(),
            Request::Set {
                key: "foo".to_string(),
                val: "bar".to_string()
            }
        );
        assert_eq!(parse_tokens(vec![]).await.unwrap(), Request::NoOp);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_ping() {
        parse_tokens(vec![Token::Ping, Token::Operand("foo".to_string())])
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_get() {
        parse_tokens(vec![Token::Get]).await.unwrap();
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_set() {
        parse_tokens(vec![
            Token::Set,
            Token::Operand("baz".to_string()),
            Token::Operand("bar".to_string()),
            Token::Operand("foo".to_string()),
        ])
        .await
        .unwrap();
    }
}
