use clap::Parser;
use log::{info, warn, error};
use network::Network;
use std::net::TcpStream;
use std::{io, process};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specify address
    #[arg(long, default_value = "127.0.0.1")]
    server_address: String,

    /// Specify port
    #[arg(short, long, default_value = "8787")]
    port: String,

    /// Use debug version
    #[arg(long)]
    debug: bool,
}

fn main() -> io::Result<()> {

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let args = Args::parse();

    let server = Network::new(args.server_address, args.port);
    let listener_result = server.start_server();
    let listener = match listener_result {
        Ok(r) => r,
        Err(error) => {
            error!("Serevr failed to start! {}", error);
            process::exit(1);
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => match handle_client(&mut s) {
                Ok(_) => {}
                Err(err) => warn!("Data received from the client has a probleme! {}", err),
            },
            Err(err) => {
                warn!("Something went wrong with a stream ! {}", err)
            }
        }
    }

    Ok(())
}

fn handle_client(stream: &mut TcpStream) -> Result<(), io::Error> {
    info!("Incoming connection {stream:?}");

    let _fragment_request = Network::get_work_request(stream)?;

    Network::send_work(stream)?;

    Ok(())
}
