use crate::config::Config;
use crate::executor::{execute, Request, Response};
use crate::parser::parse_request;
use crate::store::{StdStore, Store};
use log::*;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::sync::{mpsc, oneshot};

/// Server's representation of a client
pub struct ClientConnection {
    /// Unique identifier for client assigned by server
    id: u64,

    /// A TCP stream between the client and server
    socket: TcpStream,

    /// Address of client's remote socket
    addr: SocketAddr,
}

impl ClientConnection {
    fn new(id: u64, socket: TcpStream, addr: SocketAddr) -> Self {
        Self { id, socket, addr }
    }
}

/// Message sent between a server's threads to mutate the data store
#[derive(Debug)]
struct Message {
    /// Request contains the mutation to be executed by the executor thread
    req: Request,

    /// A single-use channel to pass a response back from the executor thread
    pipe: oneshot::Sender<Response>,
}

pub async fn start_server(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut store: StdStore = Store::new();
    debug!("Initialized data store");

    let (tx, mut rx) = mpsc::channel(config.cbound);
    debug!("Initialized executor thread channel");

    let _executor = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let msg: Message = msg; // Make type of `msg` explicit to compiler
            let resp = execute(msg.req, &mut store).await;
            let _ = msg.pipe.send(resp);
        }
    });

    let mut listener = match TcpListener::bind(&config.bind).await {
        Ok(l) => l,
        Err(_) => {
            error!("An invalid URL was provided: {}", &config.bind);
            std::process::exit(1);
        }
    };
    info!("Ready to accept connections at: {}", &config.bind);

    // TODO: Consider tracking client connections
    let _clients: Vec<&ClientConnection> = Vec::new();
    let mut client_id: u64 = 0;

    loop {
        let (socket, addr) = listener.accept().await?;

        let mut client = ClientConnection::new(client_id, socket, addr);
        client_id += 1;

        info!(
            "Successfully established inbound TCP connection with: {}",
            &client.addr
        );

        let mut txc = tx.clone();
        let _task = tokio::spawn(async move {
            loop {
                // let mut buf = [0; 512 * (1 << 20)];
                let mut buf = [0; 512];
                let _ = client.socket.read(&mut buf[..]).await;

                let req = parse_request(&buf).await;
                info!("Received a request from client {} ({}):", client.id, &client.addr);
                info!("  -> \"{:?}\"", &req);

                if req == Request::Quit {
                    info!("Received a QUIT request from client {} ({})", client.id, &client.addr);
                    break;
                }

                let (send_pipe, recv_pipe) = oneshot::channel();
                let msg = Message {
                    req: req,
                    pipe: send_pipe,
                };

                let _ = txc.send(msg).await;

                let resp = recv_pipe.await.unwrap();
                let _ = client.socket.write_all(resp.body.as_bytes()).await;
            }
        });
    }
}
