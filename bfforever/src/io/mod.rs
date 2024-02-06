mod file;

pub use file::*;
use std::fs::{create_dir_all, File, remove_file};
use std::io::{Error as IOError, Read, Seek};
use std::path::Path;

pub fn create_new_file<T: AsRef<Path>>(file_path: T) -> Result<File, IOError> {
    let file_path = file_path.as_ref();

    /*if !file_path.is_file() {
        // TODO: Throw error?
    }*/

    // Create directory
    if let Some(output_dir) = file_path.parent() {
        if !output_dir.exists() {
            create_dir_all(&output_dir)?;
        }
    }

    // Delete old file
    if file_path.exists() {
        // TODO: Investigate better approach to guarantee deletion
        remove_file(file_path)?;
    }

    File::create(file_path)
}

pub fn read_u8<T: Read + Seek>(stream: &mut T) -> Result<u8, IOError> {
    let mut b = [0u8; std::mem::size_of::<u8>()];
    stream.read(&mut b)?;

    Ok(b[0])
}

pub fn read_u16_be<T: Read + Seek>(stream: &mut T) -> Result<u16, IOError> {
    let mut b = [0u8; std::mem::size_of::<u16>()];
    stream.read_exact(&mut b)?;

    Ok(u16::from_be_bytes(b))
}

pub fn read_u32_be<T: Read + Seek>(stream: &mut T) -> Result<u32, IOError> {
    let mut b = [0u8; std::mem::size_of::<u32>()];
    stream.read_exact(&mut b)?;

    Ok(u32::from_be_bytes(b))
}

pub fn read_terminated_string_with_size<T: Read + Seek>(stream: &mut T, n: usize) -> Result<String, IOError> {
    let mut str_buffer = vec![0u8; n];
    stream.read_exact(&mut str_buffer)?;

    // Remove terminating chars
    let str_length = str_buffer
        .iter()
        .enumerate()
        .find(|(_, c)| c.eq(&&b'\0'))
        .map(|(i, _)| i)
        .unwrap_or_default();
    str_buffer.truncate(str_length);

    let str = String::from_utf8(str_buffer).unwrap(); // TODO: Handle safely
    Ok(str)
}