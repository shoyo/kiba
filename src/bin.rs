use minikv::HashMiniKV;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

#[derive(Debug, PartialEq)]
struct Request {
    op: Op,
}

#[derive(Debug, PartialEq)]
enum Op {
    Ping,
    Get { key: String },
    Set { key: String, val: String },
}

#[derive(Debug, PartialEq)]
struct Response {
    ret: String,
}

#[derive(Debug, PartialEq)]
enum Token {
    Ping,
    Get,
    Set,
    Operand(String),
}

#[derive(Debug)]
struct ParserError(String);

fn parse(tokens: Vec<Token>) -> Result<Request, ParserError> {
    let op = &tokens[0];
    let argc = tokens.len();
    match op {
        Token::Ping => {
            if argc != 1 {
                return Err(ParserError(format!("Ping op cannot have operands")));
            }
            return Ok(Request { op: Op::Ping });
        }
        Token::Get => {
            if argc != 2 {
                return Err(ParserError(format!("Get op must have exactly 1 operand")));
            }
            match &tokens[1] {
                Token::Operand(k) => {
                    return Ok(Request {
                        op: Op::Get { key: k.to_string() },
                    });
                }
                _ => return Err(ParserError(format!("Get operands cannot be op types"))),
            }
        }
        Token::Set => {
            if argc != 3 {
                return Err(ParserError(format!("Set op must have exactly 2 operands")));
            }
            let key;
            match &tokens[1] {
                Token::Operand(k) => key = k.to_string(),
                _ => return Err(ParserError(format!("Set operands cannot be op types"))),
            }
            let val;
            match &tokens[2] {
                Token::Operand(v) => val = v.to_string(),
                _ => return Err(ParserError(format!("Set operands cannot be op types"))),
            }
            return Ok(Request {
                op: Op::Set { key: key, val: val },
            });
        }
        _ => return Err(ParserError(format!("Invalid op token"))),
    }
}

fn tokenize(bytes: &[u8]) -> Vec<Token> {
    let mut tokens = Vec::new();
    let text = std::str::from_utf8(bytes).unwrap();
    let mut chunks = text.split_whitespace();

    while let Some(chunk) = chunks.next() {
        match chunk.to_uppercase().as_str() {
            "PING" => tokens.push(Token::Ping),
            "GET" => tokens.push(Token::Get),
            "SET" => tokens.push(Token::Set),
            _ => tokens.push(Token::Operand(chunk.to_string())),
        }
    }
    tokens
}

fn handle_request(mut stream: TcpStream) {
    let mut buf = [0; 128];
    stream.read(&mut buf).unwrap();
    println!("{:?}\n", String::from_utf8_lossy(&buf[..]));
}

fn main() -> std::io::Result<()> {
    let host = "127.0.0.1";
    let port = "6464";
    let uri = format!("{}:{}", host, port);

    let listener = TcpListener::bind(uri)?;

    for stream in listener.incoming() {
        handle_request(stream?);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize(b"PING    "), vec![Token::Ping]);
        assert_eq!(
            tokenize(b"SET foo bar"),
            vec![
                Token::Set,
                Token::Operand("foo".to_string()),
                Token::Operand("bar".to_string())
            ]
        );
        assert_eq!(
            tokenize(b"  GET    baz       "),
            vec![Token::Get, Token::Operand("baz".to_string())]
        );
        assert_eq!(
            tokenize(b" set time now"),
            vec![
                Token::Set,
                Token::Operand("time".to_string()),
                Token::Operand("now".to_string()),
            ]
        );
        assert_eq!(
            tokenize(b"is invalid request"),
            vec![
                Token::Operand("is".to_string()),
                Token::Operand("invalid".to_string()),
                Token::Operand("request".to_string())
            ]
        );
    }

    #[test]
    fn test_valid_parse() {
        assert_eq!(parse(vec![Token::Ping]).unwrap(), Request { op: Op::Ping });
        assert_eq!(
            parse(vec![Token::Get, Token::Operand("foo".to_string())]).unwrap(),
            Request {
                op: Op::Get {
                    key: "foo".to_string()
                }
            }
        );
        assert_eq!(
            parse(vec![
                Token::Set,
                Token::Operand("foo".to_string()),
                Token::Operand("bar".to_string())
            ])
            .unwrap(),
            Request {
                op: Op::Set {
                    key: "foo".to_string(),
                    val: "bar".to_string()
                }
            }
        );
    }

    #[test]
    #[should_panic]
    fn test_invalid_ping() {
        parse(vec![Token::Ping, Token::Operand("foo".to_string())]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_get() {
        parse(vec![Token::Get]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_set() {
        parse(vec![
            Token::Set,
            Token::Operand("baz".to_string()),
            Token::Operand("bar".to_string()),
            Token::Operand("foo".to_string()),
        ])
        .unwrap();
    }
}
