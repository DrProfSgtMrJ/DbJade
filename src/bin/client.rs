#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

use dbjade::logger::ConfigLogger;
use dbjade::jadeclient;

use clap::Parser;
use log::LevelFilter;
use tokio::io::AsyncWriteExt;
use std::{ops::Deref, str::FromStr};
use dbjade::serverops::ServerOp;
use bincode;

#[derive(Parser, Default, Debug)]
#[clap(
    author = "Jade Dever Matthews",
    version,
    about = "Interact with DB Jade"
)]
struct AppArgs {
    #[clap(default_value = "localhost")]
    /// database host
    host: String,

    #[clap(short, long)]
    #[clap(default_value = "7676")]
    /// database port
    port: u16,

    #[clap(short, long)]
    #[clap(default_value = "debug")]
    /// Level for log: off, error, warn, info, debug, trace
    log_level: String,
}


lazy_static! {
    static ref APP_ARGS: AppArgs = AppArgs::parse();
}

fn ensure_states() {
    // Ensure all statics are valid (a `deref` is enough to lazily initialize them)
    let _ = APP_ARGS.deref();
}

#[tokio::main]
async fn main() {
    // Initialize shared logger
    let lvl_filter = LevelFilter::from_str(&APP_ARGS.log_level).expect("invalid log level");
    let _ = ConfigLogger::init(lvl_filter).expect("Failed to initialize logger");
    ensure_states();

    let host = &APP_ARGS.host;
    let client = jadeclient::Client::new(host.to_string(), APP_ARGS.port);
    info!("Attempting to connected to: {}:{}", host, APP_ARGS.port);
    match client.connect().await {
        Ok(mut stream) => {
            info!("Connected!");
            stream.write(&bincode::serialize(&ServerOp::Dummy).unwrap()).await.map_err(|err| format!("Failed to Send {err}"));
            loop {

            }
        }
        Err(err) => {
            error!("An Error Occured: {}", err)
        }
    }
}