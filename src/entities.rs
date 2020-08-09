use crate::store::StdStore;
use tokio::net::TcpStream;

/// A client which makes queries to the server
struct Client {
    /// Unique identifier for client assigned by server
    id: u64,

    /// A TCP stream between the client and server
    socket: TcpStream,

    /// Address of client's remote socket
    addr: String,
}

/// A server hosting a database.
/// Accepts connections from clients and serves requests to mutate the data store.
struct Server {
    /// Network interface to listen for client connections
    bind: String,

    /// Collection of all connected clients
    clients: Vec<Client>,

    /// Data store
    store: StdStore,
}
