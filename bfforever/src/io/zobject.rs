use crate::{HKey, StringKey, SKey};

pub trait ZObjectReader {
    fn read() -> Self;
}

pub trait ZObjectWriter {
    fn write();
}

pub trait ZObjectData {
    fn get_all_string_keys(&self) -> Vec<&dyn StringKey>;
    fn get_hkeys(&self) -> Vec<&HKey>;
    fn get_skeys(&self) -> Vec<&SKey>;
}