use std::fmt;

#[derive(Debug)]
pub enum ServerOp {
    ConnectTo {
        db_name: String
    },
    ListDbs,
    Disconnect
}

impl fmt::Display for ServerOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}