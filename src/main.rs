use tokio::net::TcpListener;
use tokio::sync::mpsc;
use server::ops::ServerOp;

const ADDR: &str = "localhost:7676"; // Your own address : TODO change to be configured
const CHANNEL_NUM: usize = 10;


#[tokio::main]
async fn main() {
     // Create listener instance that bounds to certain address
    let listener = TcpListener::bind(ADDR).await.expect("Failed to connect");
    let (tx, mut rx) = mpsc::channel::<ServerOp>(CHANNEL_NUM);

    loop {
        match listener.accept().await {
            Ok((mut socket, addr)) => {
                println!("Received stream from: {}", addr);
            }
            Err(err) => {
                println!("Received Error {}", err);
            }
        }
    }

}
