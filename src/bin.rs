use minikv::HashMiniKV;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

/// Serialization
///
/// MISP (MInikv Serialization Protocol)

struct MispRequest {
    op: Ops,
}

enum Ops {
    Ping,
    Get(OpGet),
    Set(OpSet),
}

struct OpGet {
    key: String,
}

struct OpSet {
    key: String,
    val: String,
}

struct MispResponse {
    ret: String,
}

enum Token {
    Ping,
    Get,
    Set,
    Operand,
}

fn parse(tokens: Vec<String>) -> MispRequest {
    MispRequest { op: Ops::Ping }
}

fn tokenize(bytes: &[u8]) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut start = 0;
    let mut idx = 0;
    let mut is_ws = true;
    while idx < bytes.len() {
        if bytes[idx] == b' ' {
            if !is_ws {
                match &bytes[start..idx] {
                    b"PING" | b"ping" => tokens.push(Token::Ping),
                    b"GET" | b"get" => tokens.push(Token::Get),
                    b"SET" | b"set" => tokens.push(Token::Set),
                    _ => tokens.push(Token::Operand),
                }
                start = idx;
            }
            is_ws = true;
        } else {
            is_ws = false;
        }
        idx += 1;
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
        println!("foo");
    }
}
