use std::fmt;
use std::convert::From;
use serde::{Serialize, Deserialize};

/// Server Level Operations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServerOp {
    Init {host: String, port: u16},
    ConnectTo {
        db_name: String
    },
    ListDbs,
    Disconnect,
    Dummy,
}

impl fmt::Display for ServerOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Encode the ServerOp type as a single byte (as long as we don't exceed 255 types)
///
impl From<&ServerOp> for u8 {
    fn from(op: &ServerOp) -> Self {
        match op {
            ServerOp::Init {..} => 1,
            ServerOp::ConnectTo { .. } => 2,
            ServerOp::ListDbs => 3,
            ServerOp::Disconnect => 4,
            ServerOp::Dummy => 5
        }
    }
}