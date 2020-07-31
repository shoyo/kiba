use kiva::{HashStore, Store};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

#[derive(Debug, PartialEq)]
struct Request {
    op: Op,
}

#[derive(Debug, PartialEq)]
enum Op {
    Ping,
    Get { key: String },
    Set { key: String, val: String },
    Invalid { error: String },
}

#[derive(Debug, PartialEq)]
struct Response {
    body: String,
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

async fn parse_tokens(tokens: Vec<Token>) -> Result<Request, ParserError> {
    let op = &tokens[0];
    let argc = tokens.len();
    match op {
        Token::Ping => {
            if argc != 1 {
                return Err(ParserError(format!(
                    "Ping op expected no operands, got {}",
                    argc - 1
                )));
            }
            return Ok(Request { op: Op::Ping });
        }
        Token::Get => {
            if argc != 2 {
                return Err(ParserError(format!(
                    "Get op expected exactly 1 operand, got {}",
                    argc - 1
                )));
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
                return Err(ParserError(format!(
                    "Set op expected 2 operands, got {}",
                    argc - 1
                )));
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

async fn tokenize(bytes: &[u8]) -> Vec<Token> {
    let mut tokens = Vec::new();
    let text = std::str::from_utf8(bytes).unwrap();
    let mut chunks = text
        .split(|c: char| c.is_whitespace() || c == '\u{0}')
        .filter(|s| !s.is_empty());

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

async fn parse_request(bytes: &[u8]) -> Result<Request, ParserError> {
    let tokens = tokenize(bytes).await;
    let req = parse_tokens(tokens).await?;
    println!("{:?}", req);
    Ok(req)
}

async fn exec_request(req: Request, store: &mut HashStore<String, String>) -> Response {
    match req.op {
        Op::Ping => {
            return Response {
                body: "PONG".to_string(),
            }
        }
        Op::Get { key } => match store.get(&key).unwrap() {
            Some(val) => {
                return Response {
                    body: format!("\"{}\"", val),
                }
            }
            None => {
                return Response {
                    body: "(nil)".to_string(),
                }
            }
        },
        Op::Set { key, val } => {
            let _ = store.set(key, val);
            return Response {
                body: "OK".to_string(),
            };
        }
        Op::Invalid { error } => {
            return Response {
                body: format!("ERROR: {}", error),
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // TEMP: initialiize new kv-store for each connection
    let mut store: HashStore<String, String> = Store::new();

    loop {
        let mut buf = [0; 128];
        let r = stream.read(&mut buf[..]).await?;
        println!("{}", r);

        let req;
        match parse_request(&buf).await {
            Ok(request) => req = request,
            Err(e) => {
                req = Request {
                    op: Op::Invalid {
                        error: e.0.to_string(),
                    },
                }
            }
        }
        let resp = exec_request(req, &mut store).await;

        println!("{:?}", resp);

        stream.write_all(resp.body.as_bytes()).await?;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("==================");
    println!("Kiva Server (v0.1)");
    println!("==================");

    let url = "127.0.0.1:6464";
    let mut listener = TcpListener::bind(url).await?;

    println!("** Listening on: {}", url);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!(
            "** Successfully established inbound TCP connection with: {}",
            &addr
        );
        handle_connection(socket).await?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kiva::{HashStore, Store};

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
            Request { op: Op::Ping }
        );
        assert_eq!(
            parse_tokens(vec![Token::Get, Token::Operand("foo".to_string())])
                .await
                .unwrap(),
            Request {
                op: Op::Get {
                    key: "foo".to_string()
                }
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
            Request {
                op: Op::Set {
                    key: "foo".to_string(),
                    val: "bar".to_string()
                }
            }
        );
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

    #[tokio::test]
    async fn test_response() {
        let mut store: HashStore<String, String> = Store::new();
        assert_eq!(
            exec_request(Request { op: Op::Ping }, &mut store).await,
            Response {
                body: "PONG".to_string()
            }
        )
    }
}
