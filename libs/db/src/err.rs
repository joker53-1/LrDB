use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum DBError {
    #[error("unsupport protocol version {0}")]
    ProtocolVersion(u8),

    #[error("stdio error: {0:?}")]
    Io(#[from] io::Error),
}
