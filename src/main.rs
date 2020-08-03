use kiva::kvsp::{Request, Response};
use kiva::parser::parse_request;
use kiva::store::{HashStore, Store};
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
struct Message {
    req: Request,
    pipe: oneshot::Sender<Response>,
}

async fn exec_request(req: Request, store: &mut HashStore<String, String>) -> Response {
    match req {
        Request::Ping => {
            return Response {
                body: "PONG".to_string(),
            }
        }
        Request::Get { key } => match store.get(&key).unwrap() {
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
        Request::Set { key, val } => {
            let _ = store.set(key, val);
            return Response {
                body: "OK".to_string(),
            };
        }
        Request::NoOp => {
            return Response {
                body: "\u{0}".to_string(),
            }
        }
        Request::Invalid { error } => {
            return Response {
                body: format!("ERROR: {}", error),
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("==================");
    println!("Kiva Server (v0.1)");
    println!("==================");

    let cbound = 128;
    let (tx, mut rx) = mpsc::channel(cbound);

    let _manager = tokio::spawn(async move {
        let mut store: HashStore<String, String> = Store::new();
        println!("** Initialized data store");

        while let Some(msg) = rx.recv().await {
            let msg: Message = msg; // Make type of `msg` explicit to compiler
            let resp = exec_request(msg.req, &mut store).await;
            let _ = msg.pipe.send(resp);
        }
    });

    let url = "127.0.0.1:6464";
    let mut listener = TcpListener::bind(url).await?;
    println!("** Listening on: {}", url);

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!(
            "** Successfully established inbound TCP connection with: {}",
            &addr
        );
        let mut txc = tx.clone();
        let _task = tokio::spawn(async move {
            loop {
                let mut buf = [0; 1024];
                let _ = socket.read(&mut buf[..]).await;

                let req = parse_request(&buf).await;

                let (send_pipe, recv_pipe) = oneshot::channel();
                let msg = Message {
                    req: req,
                    pipe: send_pipe,
                };

                let _ = txc.send(msg).await;

                let resp = recv_pipe.await.unwrap();
                let _ = socket.write_all(resp.body.as_bytes()).await;
            }
        });
    }
}
