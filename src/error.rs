use std::{error, io};

// error enum for the whole project
// try to make functions return this
#[derive(Debug)]
pub enum TeleportError {
    Network(Box<dyn error::Error + Send>),
    Disk(io::Error),
    Protocol(&'static str),
    Rpc(bitcoincore_rpc::Error),
    Socks(tokio_socks::Error),
}

impl From<Box<dyn error::Error + Send>> for TeleportError {
    fn from(e: Box<dyn error::Error + Send>) -> TeleportError {
        TeleportError::Network(e)
    }
}

impl From<io::Error> for TeleportError {
    fn from(e: io::Error) -> TeleportError {
        TeleportError::Disk(e)
    }
}

impl From<bitcoincore_rpc::Error> for TeleportError {
    fn from(e: bitcoincore_rpc::Error) -> TeleportError {
        TeleportError::Rpc(e)
    }
}

impl From<tokio_socks::Error> for TeleportError {
    fn from(e: tokio_socks::Error) -> TeleportError {
        TeleportError::Socks(e)
    }
}
