use dbjade::serverops::ServerOp;
use tokio::io::{BufReader};
use tokio::net::TcpStream;
use tokio::{net::TcpListener, io::AsyncReadExt};
use tokio::sync::mpsc;

const ADDR: &str = "localhost:7676"; // Your own address : TODO change to be configured
const CHANNEL_NUM: usize = 100;
use std::io;
use std::net::{SocketAddr};
use bincode;


#[tokio::main]
async fn main() {
     // Create listener instance that bounds to certain address
    let listener = TcpListener::bind(ADDR).await.map_err(|err|  panic!("Failed to bind: {err}")).unwrap();
    let (tx, mut rx) = mpsc::channel::<(ServerOp, SocketAddr)>(CHANNEL_NUM);
    
    tokio::spawn(async move {
        loop {
            if let Ok((mut socket, addr)) = listener.accept().await {
                let tx = tx.clone();
                println!("Receieved stream from: {}", addr);
                let mut buf = vec![0, 255];
                if let Ok(result) = tx.send((ServerOp::Dummy, addr)).await {
                    println!("Sent dummy data");
                }
            }
        }
    });

    while let Some(serverop) = rx.recv().await {
        match serverop {
            (ServerOp::Dummy, ..) => println!("Received a Dummy message"),
            (ServerOp::ConnectTo { .. }, ..) => println!("Received a ConnectTo message"),
            (ServerOp::ListDbs, ..) => println!("Received a ListDbs message"),
            (ServerOp::Disconnect, ..) => println!("Received a Disconnect message"),
        }
    }
}


fn process(mut socket: TcpStream) -> Result<ServerOp, io::Error> {
    Ok(ServerOp::Dummy)
}