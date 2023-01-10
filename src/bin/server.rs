
#[macro_use]
extern crate log;

use dbjade::{CHANNEL_NUM};
use dbjade::clientresponse::ClientResponse;
use dbjade::connection::Connection;
use dbjade::serverops::ServerOp;
use tokio::net::TcpStream;
use tokio::{net::TcpListener};
use tokio::sync::mpsc;
use dbjade::{logger::ConfigLogger};
use log::LevelFilter;

const ADDR: &str = "localhost:7676";
// Your own address : TODO change to be configured
use std::net::{SocketAddr};
use std::str::FromStr;

#[tokio::main]
async fn main() {
     // Create listener instance that bounds to certain address
    let listener = TcpListener::bind(ADDR).await.map_err(|err|  panic!("Failed to bind: {err}")).unwrap();
    let (tx, mut rx) = mpsc::channel::<(ServerOp, SocketAddr)>(CHANNEL_NUM);
    let localaddr = listener.local_addr().expect("Failed to get local address");
    let lvl_filter = LevelFilter::from_str("debug").expect("invalid log level");
    let _ = ConfigLogger::init(lvl_filter).expect("Failed to initialize logger");
    info!("Listening on: {localaddr}");
    tokio::spawn(async move {
        loop {
            if let Ok((socket, addr)) = listener.accept().await {
                info!("Receieved stream from: {:#?}", addr);
                let tx = tx.clone();
                let mut connection = Connection::new(socket);
                if let Ok(Some(result)) = connection.read::<ServerOp>().await {
                    info!("Got Op");
                    tx.send((result, addr)).await.expect("Failed to send over data through channel");
                }
            }
        }
    });

    while let Some(serverop) = rx.recv().await {
        info!("Receieving Server Op: ");
        match serverop {
            (ServerOp::Init {host, port}, _addr) => {
                info!("Assigning Id");
                let full_addr = format!("{}:{}", host, port);
                let stream = TcpStream::connect(full_addr).await.expect("Failed to Connect to client");
                let mut connection = Connection::new(stream);
                let clientresp = ClientResponse::Connected { id: 200 };
                connection.write(&clientresp).await.expect("Failed to write back");
            }
            (ServerOp::Dummy, addr) => {
                let stream = TcpStream::connect(addr).await.expect("Failed to Connect to client");
                let mut connection = Connection::new(stream);
                let clientresp = ClientResponse::Dummy;
                connection.write(&clientresp).await.expect("Failed to write back");
            }
            (ServerOp::ConnectTo { .. }, ..) => info!("Received a ConnectTo message"),
            (ServerOp::ListDbs, ..) => info!("Received a ListDbs message"),
            (ServerOp::Disconnect, ..) => info!("Received a Disconnect message"),
        }
    }
}
