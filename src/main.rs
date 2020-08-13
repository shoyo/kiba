use kiba::config::parse_config;
use kiba::server::start_server;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    println!("");
    println!("██╗  ██╗██╗██████╗  █████╗ ");
    println!("██║ ██╔╝██║██╔══██╗██╔══██╗");
    println!("█████╔╝ ██║██████╔╝███████║");
    println!("██╔═██╗ ██║██╔══██╗██╔══██║");
    println!("██║  ██╗██║██████╔╝██║  ██║");
    println!("╚═╝  ╚═╝╚═╝╚═════╝ ╚═╝  ╚═╝");
    println!("");
    println!("Kiba Server 0.1 (unstable)");
    println!("===========================");

    let argv: Vec<String> = std::env::args().collect();
    let config;
    match argv.len() {
        1 => {
            info!("Initializing server with default configuration...");
            config = parse_config(None);

        }
        _ => {
            let path = &argv[1];
            info!("Initializing server with configuration file at: {}", &path);
            config = parse_config(Some(path));
        }
    }
    let _ = start_server(config).await;
}
