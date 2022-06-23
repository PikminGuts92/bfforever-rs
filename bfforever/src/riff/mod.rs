mod errors;
mod io;
use std::io::{Read, Seek, Write};

use io::*;
pub use errors::*;

pub trait Reader : Read + Seek { }

struct ChunkInfo {
    pub id: [u8; 4],
    pub offset: u64,
    pub size: u64,
}

pub struct RiffReader<T> where T : Reader {
    reader: T,
    big_endian: bool,
    chunks: Vec<ChunkInfo>,
}

impl<T> RiffReader<T> where T : Reader {
    pub fn new(reader: T) -> Result<Self, ReadRiffError> {
        let mut riff_reader = Self {
            reader,
            big_endian: false,
            chunks: Vec::new()
        };

        riff_reader.read_endian()?;
        riff_reader.read_chunk_info()?;

        Ok(riff_reader)
    }

    pub fn read_chunks(&mut self) -> Result<(), ReadRiffError> {
        
        
        Ok(())
    }
}