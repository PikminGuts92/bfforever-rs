use std::io::SeekFrom;
use super::*;

const MAGIC_RIFF: &[u8; 4] = b"RIFF";
const MAGIC_RIFF_R: &[u8; 4] = b"FFIR"; // Big endian

impl<T> RiffReader<T> where T : Reader {
    pub fn read_endian(&mut self) -> Result<(), ReadRiffError> {
        let mut buffer = [0u8; 4];
        self.reader.read(&mut buffer)?;

        self.big_endian = match &buffer {
            MAGIC_RIFF => false,
            MAGIC_RIFF_R => true,
            _ => return Err(ReadRiffError::InvalidMagic { magic: buffer.to_owned() }),
        };

        Ok(())
    }

    pub fn read_chunk_info(&mut self) -> Result<(), ReadRiffError> {
        let total_size = self.read_u32()? as u64;
        let end_pos = total_size + self.reader.stream_position()?;

        loop {
            let chunk_pos = self.reader.stream_position()?;

            if chunk_pos >= end_pos {
                break;
            }

            let mut id_buf = [0u8; 4];
            self.reader.read_exact(&mut id_buf)?;
            let chunk_size = self.read_u32()? as u64;

            if self.big_endian {
                // Reverse bytes
                id_buf.reverse();
            }

            // Skip chunk data
            self.reader.seek(SeekFrom::Current(chunk_size as i64))?;

            self.chunks.push(ChunkInfo {
                id: id_buf,
                offset: chunk_pos,
                size: chunk_size
            });
        }

        Ok(())
    }

    pub fn read_u32(&mut self) -> Result<u32, ReadRiffError> {
        let mut buffer = [0u8; 4];
        self.reader.read(&mut buffer)?;

        Ok(match self.big_endian {
            true => u32::from_be_bytes(buffer),
            _ => u32::from_le_bytes(buffer)
        })
    }
}