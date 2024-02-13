mod errors;
mod io;

use std::io::{Cursor, Read, Seek, SeekFrom};

#[allow(unused_imports)] use io::*;
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

    pub fn get_chunk_offset(&self) -> u64 {
        self.offset
    }

    pub fn get_data_offset(&self) -> u64 {
        self.offset + 8
    }

    pub fn get_chunk_size(&self) -> u64 {
        self.size + 8
    }

    pub fn get_data_size(&self) -> u64 {
        self.size
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

        riff_reader.read_magic()?;

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

    pub fn read_chunk(&mut self, index: usize) -> Result<Cursor<Vec<u8>>, ReadRiffError> {
        let (offset, size) = {
            let info = self
                .chunks
                .get(index)
                .ok_or(ReadRiffError::InvalidChunkIndex { index })?;

            (info.get_data_offset(), info.get_data_size())
        };

        self.reader.seek(SeekFrom::Start(offset))?;

        // Read data
        // TODO: Return new object with lazy read functionality?
        let mut buffer = vec![0u8; size as usize];
        self.reader.read_exact(&mut buffer).unwrap();

        Ok(Cursor::new(buffer))
    }

    pub fn has_fourcc(&self) -> bool {
        self.fourcc.is_some()
    }

    pub fn get_fourcc<'a>(&'a self) -> Option<&'a [u8; 4]> {
        self.fourcc
            .as_ref()
    }

    pub fn get_fourcc_as_str<'a>(&'a self) -> Option<&'a str> {
        self.fourcc
            .as_ref()
            .and_then(|fcc| std::str::from_utf8(fcc).ok())
    }

    pub fn get_chunk_count(&self) -> usize {
        self.chunks.len()
    }
}