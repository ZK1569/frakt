use clap::Parser;
use env_logger::Env;
use frakt_trc::Fractal;
use log::{info, error, debug};
use network::{models::commmunication::{FragmentTask, FragmentResult, PixelData}, Network};
use std::{io, process};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specify server address
    #[arg(long, default_value = "127.0.0.1")]
    server_address: String,

    /// Specify server port
    #[arg(short, long, default_value = "8787")]
    port: String,

    /// Use debug version
    #[arg(long)]
    debug: bool,

    /// Use GPU
    #[arg(long)]
    gpu_use: bool,
}

fn main() -> io::Result<()> {

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    // INFO: Log levels: 
    //  error
    //  warn
    //  info
    //  debug
    //  trace

    info!("--- Server Start ---");

    let args = Args::parse();

    let server = Network::new(args.server_address, args.port);

    let stream_result = server.connect_to_server();

    let mut stream = match stream_result {
        Ok(rep) => rep,
        Err(err) => {
            // TODO: Try serveral times
            error!("The server unreachable! {}", err);
            process::exit(1)
        }
    };

    let fractal_task_request = Network::ask_for_work(&mut stream, "Groupe 7".to_string(), 100);
    let fragment_task: FragmentTask = match fractal_task_request {
        Ok(fragment) => fragment,
        Err(err) => {
            error!("Something went wrong: {}", err);
            process::exit(1)
        }
    };

    debug!("FragmentTask from server : {:?}", fragment_task);

    let mut fragment_result = FragmentResult {
        id: fragment_task.id,
        resolution: fragment_task.resolution,
        range: fragment_task.range,
        pixels: PixelData {
            offset: 0,
            count: 0
        }
    };

    Fractal::calculate_iterations(&mut fragment_result);

    Ok(())
}
