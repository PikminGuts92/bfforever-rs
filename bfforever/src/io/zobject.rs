use crate::{HKey, StringKey, SKey};
use std::io::{Error as IOError, Read, Seek, Write};

pub trait ZObjectReader {
    fn read<T: Read + Seek>(&mut self, reader: &mut T) -> Result<(), IOError>;
}

pub trait ZObjectWriter {
    fn write<T: Seek + Write>(writer: &mut T) -> Result<(), IOError>;
}

pub trait ZObjectData {
    fn get_all_string_keys(&self) -> Vec<&dyn StringKey>;
    fn get_hkeys(&self) -> Vec<&HKey>;
    fn get_skeys(&self) -> Vec<&SKey>;
}