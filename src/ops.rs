use bytes::Bytes;
use std::fmt;

#[derive(Debug)]
pub enum Op {
    Connect,
    Disconnect
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}