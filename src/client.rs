use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, Result};

const ADDR: &str = "localhost:7676"; // Your own address : TODO change to be configured

struct Client {
    host: String,
    port: String
}
impl Client {
    pub fn new(host: String, port: String) -> Self {
        Client {host, port}
    }

    pub async fn connect(&self) -> Result<TcpStream, std::io::Error>{
        let full_addr = format!("{}:{}", self.host, self.port);
        TcpStream::connect(full_addr).await
    }
}