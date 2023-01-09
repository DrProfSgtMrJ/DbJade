use dbjade::serverops::ServerOp;
use tokio::io::{BufReader};
use tokio::net::TcpStream;
use tokio::{net::TcpListener, io::AsyncReadExt};
use tokio::sync::broadcast;

const ADDR: &str = "localhost:7676"; // Your own address : TODO change to be configured
const CHANNEL_NUM: usize = 100;
use std::io;
use std::net::{SocketAddr};
use bincode;


#[tokio::main]
async fn main() {
     // Create listener instance that bounds to certain address
    let listener = TcpListener::bind(ADDR).await.map_err(|err|  panic!("Failed to bind: {err}")).unwrap();
    let (tx, mut rx) = broadcast::channel::<(ServerOp, SocketAddr)>(CHANNEL_NUM);
    

    loop {
        if let Ok((mut socket, addr)) = listener.accept().await {
            let tx = tx.clone();
            let mut rx = tx.subscribe();
            println!("Receieved stream from: {}", addr);
            let mut buf = vec![0, 255];
            tokio::select! {
                result = socket.read(&mut buf) => {
                    match result {
                        Ok(res) => println!("Bytes Read: {res}"),
                        Err(_) => println!(""),
                    }
                    tx.send((ServerOp::Dummy, addr)).unwrap();
                }
                result = rx.recv() =>{
                    match result {
                        Ok(msg, ..) => println!("Hit: {:#?}", msg),
                        Err(err) => eprintln!("Error {err}"),
                    }
                }
            }
        }
    }
}


fn process(mut socket: TcpStream) -> Result<ServerOp, io::Error> {
    Ok(ServerOp::Dummy)
}