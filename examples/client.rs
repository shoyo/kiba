use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6464")?;

    let cmds = ["PING", "SET foo \"bar\"", "GET foo", "GET bar"];

    stream.write(b"SET foo\"bar\"");

    //    for cmd in &cmds {
    //        stream.write(&cmd.as_bytes());
    //    }

    Ok(())
}
