use tokio::net::TcpStream;

pub struct Client {
    host: String,
    port: u16
}
impl Client {
    pub fn new(host: String, port: u16) -> Self {
        Client {host, port}
    }

    pub async fn connect(&self) -> Result<TcpStream, std::io::Error>{
        let full_addr = format!("{}:{}", self.host, self.port);
        TcpStream::connect(full_addr).await
    }
}