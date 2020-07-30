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

fn parse(tokens: Vec<Token>) -> Request {
    Request { op: Op::Ping }
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
    fn test_parse() {
        assert_eq!(parse(vec![Token::Ping]), Request { op: Op::Ping });
        assert_eq!(
            parse(vec![Token::Get, Token::Operand("foo".to_string())]),
            Request {
                op: Op::Get {
                    key: "foo".to_string()
                }
            }
        );
    }
}
