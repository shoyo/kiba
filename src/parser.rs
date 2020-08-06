use crate::kvsp::Request;

#[derive(Debug, PartialEq)]
enum Token {
    Ping,
    StringOp(StringOp)
    ListOp(ListOp),
    SetOp(SetOp),
    HashOp(HashOp),
    Operand(String),
}

#[derive(Debug, PartialEq)]
enum StringOp {
    Get,
    Set,
    Incr,
    Decr,
    IncrBy,
    DecrBy,
}

#[derive(Debug, PartialEq)]
enum ListOp {
    LPush,
    RPush,
    LPop,
    RPop,
}

#[derive(Debug, PartialEq)]
enum SetOp {
    SAdd,
    SRem,
    SIsMember,
    SMembers,
}

#[derive(Debug, PartialEq)]
enum HashOp {
    HGet
    HSet,
    HDel,
}

#[derive(Debug)]
struct ParserError {
    message: String,
}

async fn tokenize(bytes: &[u8]) -> Vec<Token> {
    let mut tokens = Vec::new();
    let text = std::str::from_utf8(bytes).unwrap();
    let mut chunks = text
        .split(|c: char| c.is_whitespace() || c == '\u{0}')
        .filter(|s| !s.is_empty());

    while let Some(chunk) = chunks.next() {
        match chunk.to_uppercase().as_str() {
            "PING" => tokens.push(Token::Ping),
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
            "HGET" => tokens.push(Token::HashOp(SetOp::HGet)),
            "HSET" => tokens.push(Token::HashOp(SetOp::HSet)),
            "HDEL" => tokens.push(Token::HashOp(SetOp::HDel)),
            _ => tokens.push(Token::Operand(chunk.to_string())),
        }
    }
    tokens
}

async fn parse_tokens(tokens: Vec<Token>) -> Result<Request, ParserError> {
    let argc = tokens.len();
    if argc == 0 {
        return Ok(Request::NoOp);
    }
}

async fn parse_tokens(tokens: Vec<Token>) -> Result<Request, ParserError> {
    let argc = tokens.len();
    if argc == 0 {
        return Ok(Request::NoOp);
    }
    let op = &tokens[0];
    match op {
        Token::Ping => {
            if argc != 1 {
                return Err(ParserError {
                    message: format!("Ping op expected no operands, got {}", argc - 1),
                });
            }
            return Ok(Request::Ping);
        }
        Token::Get => {
            if argc != 2 {
                return Err(ParserError {
                    message: format!("Get op expected exactly 1 operand, got {}", argc - 1),
                });
            }
            match &tokens[1] {
                Token::Operand(k) => {
                    return Ok(Request::Get { key: k.to_string() });
                }
                _ => {
                    return Err(ParserError {
                        message: format!("Get operands cannot be op types"),
                    })
                }
            }
        }
        Token::Set => {
            if argc != 3 {
                return Err(ParserError {
                    message: format!("Set op expected 2 operands, got {}", argc - 1),
                });
            }
            let key;
            match &tokens[1] {
                Token::Operand(k) => key = k.to_string(),
                _ => {
                    return Err(ParserError {
                        message: format!("Set operands cannot be op types"),
                    })
                }
            }
            let val;
            match &tokens[2] {
                Token::Operand(v) => val = v.to_string(),
                _ => {
                    return Err(ParserError {
                        message: format!("Set operands cannot be op types"),
                    })
                }
            }
            return Ok(Request::Set { key: key, val: val });
        }
        _ => {
            return Err(ParserError {
                message: format!("Invalid op token"),
            })
        }
    }
}

pub async fn parse_request(bytes: &[u8]) -> Request {
    let tokens = tokenize(bytes).await;
    match parse_tokens(tokens).await {
        Ok(req) => return req,
        Err(e) => return Request::Invalid { error: e.message },
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
