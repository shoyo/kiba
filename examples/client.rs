use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    println!("====================");
    println!("MiniKV Client (v0.1)");
    println!("====================");

    let url = "127.0.0.1:6464";
    let mut stream = TcpStream::connect(url).unwrap();

    println!("** Successfully established TCP connection with outbound server");
    println!("** Listening on: {}", url);

    loop {
        let mut wbuf = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut wbuf)
            .expect("Failed to read input");
        let _ = stream.write(wbuf.as_bytes());

        let mut rbuf = [0, 128];
        let _ = stream.read(&mut rbuf);

        println!("{}", String::from_utf8_lossy(&rbuf));
    }
}
