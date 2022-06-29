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

impl ChunkInfo {
    pub fn get_id_as_str<'a>(&'a self) -> Option<&'a str> {
        std::str::from_utf8(&self.id).ok()
    }
}

pub struct RiffReader<T> where T : Reader {
    reader: T,
    big_endian: bool,
    fourcc: Option<[u8; 4]>,
    chunks: Vec<ChunkInfo>,
}

impl<T> RiffReader<T> where T : Reader {
    pub fn new(reader: T) -> Result<Self, ReadRiffError> {
        RiffReader::read(reader, true)
    }

    pub fn new_without_fourcc(reader: T) -> Result<Self, ReadRiffError> {
        // BF riffs lack fourcc
        RiffReader::read(reader, false)
    }

    fn read(reader: T, read_fourcc: bool) -> Result<Self, ReadRiffError> {
        let mut riff_reader = Self {
            reader,
            big_endian: false,
            fourcc: None,
            chunks: Vec::new()
        };

        riff_reader.read_endian()?;

        if read_fourcc {
            let mut fourcc = riff_reader.read_bytes()?;

            if riff_reader.big_endian {
                // Reverse bytes
                fourcc.reverse();
            }

            riff_reader.fourcc = Some(fourcc);
        }

        riff_reader.read_chunk_info()?;
        Ok(riff_reader)
    }

    pub fn read_chunks(&mut self) -> Result<(), ReadRiffError> {
        
        
        Ok(())
    }
}