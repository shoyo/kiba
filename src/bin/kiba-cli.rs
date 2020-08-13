use std::io::prelude::*;
use tokio::net::TcpStream;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("");
    println!("██╗  ██╗██╗██████╗  █████╗ ");
    println!("██║ ██╔╝██║██╔══██╗██╔══██╗");
    println!("█████╔╝ ██║██████╔╝███████║");
    println!("██╔═██╗ ██║██╔══██╗██╔══██║");
    println!("██║  ██╗██║██████╔╝██║  ██║");
    println!("╚═╝  ╚═╝╚═╝╚═════╝ ╚═╝  ╚═╝");
    println!("");
    println!("Kiba CLI 0.1 (unstable)");
    println!("===========================");

    let url = "127.0.0.1:6464";
    let mut stream = TcpStream::connect(url).await?;

    println!(
        "** Successfully established outbound TCP connection with: {}",
        url
    );

    loop {
        let mut wbuf = String::new();
        print!("kiba> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut wbuf)
            .expect("Failed to read input");

        stream.write_all(wbuf.as_bytes()).await?;

        // let mut rbuf = [0; 512 * (1 << 20)];
        let mut rbuf = [0; 512];
        stream.read(&mut rbuf[..]).await?;

        println!("{}\n", String::from_utf8_lossy(&rbuf));
    }
}
