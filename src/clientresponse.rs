use std::fmt;
use std::convert::From;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientResponse {
    Connected { id: u32 },
    ListDbs{
        names: Vec<String>
    },
    Dummy,
}
impl fmt::Display for ClientResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Encode the ClientResponse type as a single byte (as long as we don't exceed 255 types)
///
impl From<&ClientResponse> for u8 {
    fn from(op: &ClientResponse) -> Self {
        match op {
            ClientResponse::Connected {..} => 1,
            ClientResponse::ListDbs {..} => 2,
            ClientResponse::Dummy => 3,
        }
    }
}