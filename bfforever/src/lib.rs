#![allow(dead_code)]

pub mod audio;
pub(crate) mod io;
pub(crate) mod riff;
pub mod texture;
pub mod zobject;

#[derive(Default)]
pub enum Localization {
    #[default]
    English,  // enUS
    Japanese, // jaJP
    German,   // deDE
    Italian,  // itIT
    Spanish,  // esES
    French,   // frFR
}

pub trait StringKey {
    fn get_key(&self) -> u64;
    fn set_key(&mut self, key: u64);
}

#[derive(Default)]
pub struct SKey {
    pub value: u64,
}

impl StringKey for SKey {
    fn get_key(&self) -> u64 {
        self.value
    }

    fn set_key(&mut self, key: u64) {
        self.value = key;
    }
}

#[derive(Default)]
pub struct HKey {
    pub value: u64,
}

impl StringKey for HKey {
    fn get_key(&self) -> u64 {
        self.value
    }

    fn set_key(&mut self, key: u64) {
        self.value = key;
    }
}