#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

use server::logger::ConfigLogger;

use clap::Parser;
use log::LevelFilter;
use std::{ops::Deref, str::FromStr};

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

fn main() {
    // Initialize shared logger
    let lvl_filter = LevelFilter::from_str(&APP_ARGS.log_level).expect("invalid log level");
    let _logger = ConfigLogger::init(lvl_filter);

    info!("starting up");
    ensure_states();
}