use crate::config::parse_config;
use crate::store::{StdStore, Store};
use log::error;
use tokio::net::TcpStream;
use tokio::prelude::*;

/// Server's representation of a client
pub struct Client {
    /// Unique identifier for client assigned by server
    id: u64,

    /// A TCP stream between the client and server
    socket: TcpStream,

    /// Address of client's remote socket
    addr: String,
}

/// Server hosting a data store.
/// Accepts connections from clients and serves requests to mutate the data store.
pub struct Server {
    /// Network interface to listen for client connections
    pub bind: String,

    /// Collection of all connected clients
    pub clients: Vec<Client>,

    /// Limit to the number of simultaneous connections
    pub cbound: usize,
}

impl Server {
    pub fn new(conf_path: Option<&str>) -> Self {
        let mut server = Self {
            bind: "127.0.0.1:6464".to_string(),
            clients: Vec::new(),
            cbound: 128,
        };
        match conf_path {
            Some(path) => {
                let conf = parse_config(&path);
                // TODO: Handle hasher and list settings
                if let Some(bind) = conf.get("bind") {
                    server.bind = bind.to_string();
                }
                if let Some(cbound) = conf.get("cbound") {
                    match cbound.parse::<usize>() {
                        Ok(cb) => server.cbound = cb,
                        Err(_) => {
                            error!(
                                "Channel size `cbound` must be a valid integer, found \"{}\"",
                                cbound
                            );
                            std::process::exit(1);
                        }
                    }
                }
                server
            }
            None => server,
        }
    }
}
