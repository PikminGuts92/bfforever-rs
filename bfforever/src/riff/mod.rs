mod errors;
mod io;
use std::io::{Read, Seek, Write};

use io::*;
pub use errors::*;

pub trait Reader : Read + Seek { }

pub struct ChunkInfo {
    pub offset: u64,
    pub size: u64,
}

pub struct RiffReader<T> where T : Reader {
    reader: T,
    big_endian: bool,
    chunks: Vec<ChunkInfo>,
}

impl<T> RiffReader<T> where T : Reader {
    pub fn new(mut reader: T) -> Result<Self, ReadRiffError> {
        let mut riff_reader = Self {
            reader,
            big_endian: false,
            chunks: Vec::new()
        };

        riff_reader.read_endian()?;
        riff_reader.read_chunk_info()?;

        Ok(riff_reader)
    }
}