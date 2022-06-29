use thiserror::Error as ThisError;
use std::io::Error as IOError;

#[derive(Debug, ThisError)]
pub enum ReadRiffError {
    #[error("Invalid magic of {magic:?}")]
    InvalidMagic {
        magic: [u8; 4]
    },
    #[error("Chunk index {index} is out of range")]
    InvalidChunkIndex {
        index: usize
    },
    #[error("IO Error: {io_error}")]
    IOError {
        io_error: IOError
    },
}

impl From<IOError> for ReadRiffError {
    fn from(err: IOError) -> Self {
        ReadRiffError::IOError { io_error: err }
    }
}