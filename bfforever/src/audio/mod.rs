mod celt;
pub use celt::*;

pub trait AudioDecoder {
    fn decode(&self) -> Box<[i16]>;
}