use std::io;

use thiserror::Error as ThisError;
use std::error::Error as StdError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("stdio error: {0:?}")]
    Io(#[from] io::Error),

    #[error("runtime error: {0:?}")]
    Runtime(#[from] Box<dyn StdError + Send + Sync>),

    // #[error("protocal error: {0:?}")]
    // Protocol(#[from] ProtocolError),
}
