mod utils;

use blue_box::{
    fractal::fractal::Fractal,
    network::{
        client::Client,
        communication_types::{FragmentResult, FragmentTask, PixelData},
    },
};
use clap::Parser;
use env_logger::Env;
use log::{debug, error, info, warn};
use std::{io::{self, Write}, process};

use crate::utils::start_util;

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
    start_util::start_message();

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    // INFO: Log levels:
    //  error
    //  warn
    //  info
    //  debug
    //  trace

    let args = Args::parse();

    let server = Client::new(args.server_address, args.port);

    let stream_result = server.connect_to_server();

    let mut stream = match stream_result {
        Ok(rep) => rep,
        Err(err) => {
            // TODO: Try serveral times
            error!("The server unreachable! {}", err);
            process::exit(1)
        }
    };

    let mut data: Vec<u8> = Vec::new();
    let fractal_task_request = Client::ask_for_work(&mut stream, "Groupe 7".to_string(), 100);
    let fragment_task: FragmentTask = match fractal_task_request {
        Ok((fragment, data_in)) => {
            // INFO: I don't know if the data returned by the server at the same time as the fragment_task is important.
            data = data_in;
            fragment
        }
        Err(err) => {
            error!("Something went wrong: {}", err);
            process::exit(1)
        }
    };

    // INFO: From here, you have everything you need to calculate the fractals

    let mut fragment_result: FragmentResult = FragmentResult {
        id: fragment_task.id.clone(),
        resolution: fragment_task.resolution.clone(),
        range: fragment_task.range.clone(),
        pixels: PixelData {
            offset: fragment_task.id.count,
            count: fragment_task.resolution.nx as u32 * fragment_task.resolution.ny as u32,
        },
    };

    // Fractal::run(&fragment_task, &mut fragment_result, &mut data);

    let zn: f32 = 0.018979378;
    let count: f32 = 1.0;
    data.write_all(&zn.to_be_bytes());
    data.write_all(&count.to_be_bytes());

    for &num in &data{
        print!("{:#x}, ", num);
    }

    // WARN: The server do not get the response
    let response = Client::send_work_done(&mut stream, fragment_result, data);
    match response {
        Ok(_) => info!("Result sent to server"),
        Err(err) => warn!("Can't send result to server: {err}"),
    }

    Ok(())
}

