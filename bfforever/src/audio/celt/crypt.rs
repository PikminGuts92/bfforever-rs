use aes::{
    Aes256,
    cipher::{
        block_padding::NoPadding,
        BlockDecrypt,
        BlockEncrypt,
        KeyInit,
    }
};
use super::Celt;

const AES_KEY: [u8; 32] = [
    0x07, 0xc2, 0x30, 0x93, 0x4a, 0x52, 0xf1, 0x72,
    0x1a, 0xa2, 0x77, 0x52, 0xa6, 0x72, 0x43, 0x75,
    0xe8, 0xff, 0xe1, 0x7e, 0x93, 0xef, 0xcc, 0xa5,
    0x14, 0x37, 0xde, 0x7f, 0x31, 0x1c, 0xd2, 0x45
];

pub trait Crypt {
    fn is_encrypted(&self) -> bool;
    fn decrypt(&mut self);
    fn encrypt(&mut self);
}

impl Crypt for Celt {
    fn is_encrypted(&self) -> bool {
        self.header.encrypted
    }

    fn decrypt(&mut self) {
        if !self.is_encrypted() {
            return;
        }

        // Decrypt data
        let cipher = Aes256::new_from_slice(&AES_KEY).unwrap();
        cipher.decrypt_padded::<NoPadding>(&mut self.data).unwrap();

        // Update value
        self.header.encrypted = false;

        // Update offsets
        self.recompute_offsets();
    }

    fn encrypt(&mut self) {
        if self.is_encrypted() {
            return;
        }

        // Encrypt data
        let data_size = self.data.len();
        let cipher = Aes256::new_from_slice(&AES_KEY).unwrap();
        cipher.encrypt_padded::<NoPadding>(&mut self.data, data_size).unwrap();

        // Update value
        self.header.encrypted = true;
    }
}