use serde::Serialize;
use std::io::Cursor;
use bytes::BytesMut;
use serde::de::DeserializeOwned;
use tokio::io::{BufWriter, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::{io::AsyncReadExt};
use std::io;

#[derive(Debug)]
pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    /// Create a new `Connection`, backed by `socket`. Read and write buffers
    /// are initialized.
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    } 

     /// Write a serializable value into the stream
     pub async fn write<T: Serialize>(&mut self, value: &T) -> Result<(), io::Error> {
        let mut buf = bincode::serialize(value).expect("Failied to serialz");
        self.stream.write_all(&mut buf).await?;
        self.stream.flush().await?;
        Ok(())
    }

     /// Reads from the socket until a complete message is received, or an error occurs
     pub async fn read<T: DeserializeOwned>(&mut self) -> Result<Option<T>, io::Error> {
        loop {
            if let Some(frame) = self.parse()? {
                return Ok(Some(frame));
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                return if self.buffer.is_empty() {
                    Ok(None)
                } else {
                    Ok(None)
                };
            }
        }
    }

    /// Attempts to deserialize a T from the internal buffer.
    fn parse<T: DeserializeOwned>(&mut self) -> Result<Option<T>, io::Error> {
        let mut buf = Cursor::new(&self.buffer[..]);
        match bincode::deserialize_from(&mut buf) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Ok(None),
        }
    }
}