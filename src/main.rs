use kiba::ksp::{execute, Request, Response};
use kiba::parser::parse_request;
use kiba::store::{StdStore, Store};
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
struct Message {
    req: Request,
    pipe: oneshot::Sender<Response>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("==================");
    println!("Kiba Server (v0.1)");
    println!("==================");

    let cbound = 128;
    let (tx, mut rx) = mpsc::channel(cbound);

    let _executor = tokio::spawn(async move {
        let mut store: StdStore = Store::new();
        println!("** Initialized data store");

        while let Some(msg) = rx.recv().await {
            let msg: Message = msg; // Make type of `msg` explicit to compiler
            let resp = execute(msg.req, &mut store).await;
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
