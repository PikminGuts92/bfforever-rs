use crate::io::{read_terminated_string_with_size, read_u8, read_u16_be, read_u32_be};
use super::{Texture2D, TextureFormat, XPR2};
use std::fs::File;
use std::io::{Error as IOError, Read, Seek, SeekFrom};
use std::path::Path;
use thiserror::Error as ThisError;

const XPR2_MAGIC: &[u8; 4] = b"XPR2";
const TX2D_MAGIC: &[u8; 4] = b"TX2D";

#[derive(Debug, ThisError)]
pub enum Xpr2ReadError {
    #[error("Unrecognized XPR2 magic value")]
    InvalidMagic,
    #[error("Unsupported texture count of {count}")]
    UnsupportedTextureCount {
        count: u32
    },
    #[error("Unsupported texture type: {tex_type}")]
    UnsupportedTexture {
        tex_type: String,
    },
    #[error("Texture2D error")]
    Texture2DError(Texture2DReadError),
    #[error("IO error")]
    IO(IOError)
}

impl From<IOError> for Xpr2ReadError {
    fn from(value: IOError) -> Self {
        Xpr2ReadError::IO(value)
    }
}

impl From<Texture2DReadError> for Xpr2ReadError {
    fn from(value: Texture2DReadError) -> Self {
        Xpr2ReadError::Texture2DError(value)
    }
}

#[derive(Debug, ThisError)]
pub enum Texture2DReadError {
    #[error("Unsupported texture type: 0x{format:#02X}")]
    UnsupportedTextureFormat {
        format: u8,
    },
    #[error("IO error")]
    IO(IOError)
}

impl From<IOError> for Texture2DReadError {
    fn from(value: IOError) -> Self {
        Texture2DReadError::IO(value)
    }
}

impl XPR2 {
    pub fn from_file<T: AsRef<Path>>(xpr_path: T) -> Result<Self, Xpr2ReadError> {
        let mut file = File::open(xpr_path)?;
        Self::from_stream(&mut file)
    }

    pub fn from_stream<T: Read + Seek>(stream: &mut T) -> Result<Self, Xpr2ReadError> {
        let mut magic_buffer = [0u8; 4];

        // Read magic
        stream.read_exact(&mut magic_buffer)?;
        if magic_buffer.ne(XPR2_MAGIC) {
            return Err(Xpr2ReadError::InvalidMagic);
        }

        stream.seek(SeekFrom::Current(4))?; // Always 2048

        let tex_size = read_u32_be(stream)?;
        let tex_count = read_u32_be(stream)?;

        if tex_count != 1 {
            return  Err(Xpr2ReadError::UnsupportedTextureCount { count: tex_count });
        }

        let mut textures = Vec::new();

        // Read textures
        for _ in 0..tex_count {
            // Read texture magic
            stream.read_exact(&mut magic_buffer)?;
            if magic_buffer.ne(TX2D_MAGIC) {
                let tex_magic = String::from_utf8(magic_buffer.into())
                    .expect("Unable to parse texture type");

                return Err(Xpr2ReadError::UnsupportedTexture { tex_type: tex_magic });
            }

            // Read texture
            let texture = Texture2D::from_stream(stream, tex_size)?;
            textures.push(texture);
        }

        Ok(Self {
            textures
        })
    }
}

impl Texture2D {
    pub(crate) fn from_stream<T: Read + Seek>(stream: &mut T, tex_size: u32) -> Result<Self, Texture2DReadError> {
        let info_offset = read_u32_be(stream)?;
        stream.seek(SeekFrom::Current(4))?; // Always 52

        // Read name
        let max_name_size = read_u32_be(stream)?;
        stream.seek(SeekFrom::Current(4))?; // Always 0
        let name = read_terminated_string_with_size(stream, max_name_size as usize)?;

        stream.seek(SeekFrom::Start(info_offset as u64 + 45))?;

        let img_multiple = read_u16_be(stream)?;
        let data_offset = (img_multiple << 8) + 0x80C;

        let format = match read_u8(stream)? {
            0x52 => TextureFormat::DXT1,
            0x54 => TextureFormat::DXT5,
            f @ _ => {
                return Err(Texture2DReadError::UnsupportedTextureFormat { format: f })
            }
        };
        let height_num = read_u16_be(stream)?;
        let width_num = read_u16_be(stream)?;

        let height = (height_num as u32 + 1) << 3;
        let width = (width_num as u32 + 1) & 0x1FFF;

        let data_size = ((tex_size >> 8) - img_multiple as u32) << 8;

        stream.seek(SeekFrom::Start(data_offset as u64))?;
        let mut data = vec![0u8; data_size as usize];
        stream.read_exact(&mut data)?;

        Ok(Self {
            name,
            format,
            width,
            height,
            data
        })
    }
}