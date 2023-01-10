use dbjade::{CHANNEL_NUM};
use dbjade::clientresponse::ClientResponse;
use dbjade::connection::Connection;
use dbjade::serverops::ServerOp;
use tokio::net::TcpStream;
use tokio::{net::TcpListener};
use tokio::sync::mpsc;

const ADDR: &str = "localhost:7676"; // Your own address : TODO change to be configured
use std::net::{SocketAddr};
    

#[tokio::main]
async fn main() {
     // Create listener instance that bounds to certain address
    let listener = TcpListener::bind(ADDR).await.map_err(|err|  panic!("Failed to bind: {err}")).unwrap();
    let (tx, mut rx) = mpsc::channel::<(ServerOp, SocketAddr)>(CHANNEL_NUM);
    
    tokio::spawn(async move {
        loop {
            if let Ok((socket, addr)) = listener.accept().await {
                println!("Receieved stream from: {}", addr);
                let tx = tx.clone();
                let mut connection = Connection::new(socket);
                if let Ok(Some(result)) = connection.read::<ServerOp>().await {
                    tx.send((result, addr)).await.expect("Failed to send over data through channel");
                }
            }
        }
    });

    while let Some(serverop) = rx.recv().await {
        match serverop {
            (ServerOp::Init, addr) => {
                let stream = TcpStream::connect(addr).await.expect("Failed to Connect to client");
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
            (ServerOp::ConnectTo { .. }, ..) => println!("Received a ConnectTo message"),
            (ServerOp::ListDbs, ..) => println!("Received a ListDbs message"),
            (ServerOp::Disconnect, ..) => println!("Received a Disconnect message"),
        }
    }
}
