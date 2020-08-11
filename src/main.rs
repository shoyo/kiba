use kiba::entities::{Client, Server};
use kiba::executor::{execute, Request, Response};
use kiba::parser::parse_request;
use kiba::store::{StdStore, Store};
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::sync::{mpsc, oneshot};

#[macro_use]
extern crate log;

#[derive(Debug)]
struct Message {
    req: Request,
    pipe: oneshot::Sender<Response>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    println!("==================");
    println!("Kiba Server (v0.1)");
    println!("==================");

    let argv: Vec<String> = std::env::args().collect();
    let mut server;
    match argv.len() {
        1 => {
            info!("Initializing server with default configuration...");
            server = Server::new(None);
        }
        _ => {
            let path = &argv[1];
            info!("Initializing server with configuration file at: {}", &path);
            server = Server::new(Some(&path));
        }
    }
    let mut store: StdStore = Store::new();
    info!("Successfully initialized server");

    let (tx, mut rx) = mpsc::channel(server.cbound);

    let _executor = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let msg: Message = msg; // Make type of `msg` explicit to compiler
            let resp = execute(msg.req, &mut store).await;
            let _ = msg.pipe.send(resp);
        }
    });

    let mut listener = TcpListener::bind(&server.bind).await?;
    info!("Listening on: {}", &server.bind);

    loop {
        let (mut socket, addr) = listener.accept().await?;
        info!(
            "Successfully established inbound TCP connection with: {}",
            &addr
        );
        let mut txc = tx.clone();
        let _task = tokio::spawn(async move {
            loop {
                // let mut buf = [0; 512 * (1 << 20)];
                let mut buf = [0; 512];
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
