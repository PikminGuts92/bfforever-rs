mod celt;
mod wav;

pub use celt::*;
use std::path::{Path, PathBuf};
pub use self::wav::*;

pub trait AudioDecoder {
    fn decode(&self) -> Box<[i16]>;
}

pub trait AudioEncoder {
    fn encode_to_file<T: AsRef<Path>>(&self, out_path: T);
}