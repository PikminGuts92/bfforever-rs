use crate::HKey;

mod song;

#[allow(unused_imports)] pub use song::*;

pub enum ZObject {
    Song(Song),
}

pub struct ZObjectChunk {
    pub full_path: HKey,
    pub dir_path: HKey,
    pub object: ZObject,
}