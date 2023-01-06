use std::fmt::Debug;

use tokio::net::TcpStream;

pub struct Client {
    id: u32,
    host: String,
    port: u16
}
impl Client {
    pub fn new(host: String, port: u16) -> Self {
        Client {id: 0, host, port}
    }

    pub async fn connect(&self) -> Result<TcpStream, std::io::Error>{
        let full_addr = format!("{:#?}", self);
        TcpStream::connect(full_addr).await
    }
}

impl Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       if f.alternate() {
        write!(f, "Jade: {} at {}:{}", self.id, self.host, self.port)
       } else {
        write!(f, "{} at {}:{}", self.id, self.host, self.port)
       }
    }
}