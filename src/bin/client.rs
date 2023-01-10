#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

use dbjade::serverops::ServerOp;
use dbjade::{logger::ConfigLogger, clientresponse::ClientResponse};
use dbjade::{jadeclient, CHANNEL_NUM};

use clap::Parser;
use log::LevelFilter;
use tokio::io::AsyncWriteExt;
use std::{ops::Deref, str::FromStr};
use dbjade::connection::Connection;
use tokio::{net::TcpListener, net::TcpStream};
use tokio::sync::mpsc;
use std::net::{SocketAddr};
use std::io;

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
    let mut client = jadeclient::Client::new(host.to_string(), APP_ARGS.port);
    info!("Attempting to connected to: {}:{}", host, APP_ARGS.port);
    match client.connect().await {
        Ok(result) => {
            info!("Connected!");
            let listener = TcpListener::bind("localhost:0").await.map_err(|err|  panic!("Failed to bind: {err}")).unwrap();
            let (tx, mut rx) = mpsc::channel::<(ClientResponse, SocketAddr)>(CHANNEL_NUM);
            let localaddr = listener.local_addr().expect("Failed to get local address");
            info!("Listening on: {localaddr}");
            let init = ServerOp::Init { host: localaddr.ip().to_string(), port: localaddr.port() };
            let mut connection = Connection::new(result);
            if let Ok(_) = connection.write::<ServerOp>(&init).await {
                info!("Sending init");
            }
            tokio::spawn(async move {
                    loop {
                        if let Ok((socket, addr)) = listener.accept().await {
                            info!("Receieved stream from: {}", addr);
                            let tx = tx.clone();
                            let mut connection = Connection::new(socket);
                            if let Ok(Some(result)) = connection.read::<ClientResponse>().await {
                                tx.send((result, addr)).await.expect("Failed to send over data through channel");
                            }
                        }
                    }
            });

            while let Some(clientresponse) = rx.recv().await {
                match clientresponse {
                    (ClientResponse::Connected {id}, ..) => {
                        info!("Assigning Id: {id}");
                        client.set_id(id);
                    },
                    (ClientResponse::ListDbs { names}, ..) => {
                        for dbname in names {
                            info!("Name: {dbname}");
                        }
                    }
                    (ClientResponse::Dummy, ..) => {
                        info!("Just a dummy");
                    }
                }
            }
        }
        Err(err) => {
            error!("An Error Occured: {}", err)
        }
    }

}

pub async fn send_init(localaddr: SocketAddr, serveraddr: String) -> Result<(), io::Error> {
    let init = ServerOp::Init { host: localaddr.ip().to_string(), port: localaddr.port() };
    let stream = TcpStream::connect(serveraddr).await?;
    let mut connection = Connection::new(stream);
    connection.write(&init).await
}

pub async fn send_dummy(serveraddr: String) -> Result<(), io::Error> {
    let dummy = ServerOp::Dummy;
    let stream = TcpStream::connect(serveraddr).await?;
    let mut connection = Connection::new(stream);
    connection.write(&dummy).await
}