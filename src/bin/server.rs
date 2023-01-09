use std::net::TcpStream;

use tokio::net::TcpListener;
use tokio::sync::mpsc;
use dbjade::ops::ServerOp;

const ADDR: &str = "localhost:7676"; // Your own address : TODO change to be configured
const CHANNEL_NUM: usize = 10;
use std::net::{SocketAddr};


#[tokio::main]
async fn main() {
     // Create listener instance that bounds to certain address
    let listener = TcpListener::bind(ADDR).await.map_err(|err|  panic!("Failed to connect: {err}")).unwrap();
    let (tx, mut rx) = mpsc::channel::<(ServerOp, SocketAddr)>(CHANNEL_NUM);
    
    loop {
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                let tx = tx.clone();
                tokio::spawn(async move {
                    if let Err(err) = tx.send((ServerOp::Dummy, addr)).await {
                        eprintln!("Cannont send data. {err}");
                    }
                });
                println!("Received stream from: {}", addr);
            }
            Err(err) => {
                println!("Received Error {}", err);
            }
        }
    }
}
