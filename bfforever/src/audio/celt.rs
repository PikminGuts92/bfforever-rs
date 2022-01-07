use aes::{Aes256, Block, ParBlocks};
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, NewBlockCipher,
    generic_array::GenericArray,
};
use block_modes::{BlockMode, Ecb};
use block_modes::block_padding::NoPadding;
use core::num;
use nom::{Err, IResult, Needed};
use nom::bytes::streaming::tag;
use nom::number::streaming::{le_u8, le_u16, le_u32};
use nom::sequence::{pair, terminated, tuple};
use std::fs::{copy, create_dir_all, File, read, remove_dir_all, remove_file, write};
use std::io::{Read, Write};
use std::mem::size_of;
use std::path::{Path, PathBuf};

const AES_KEY: [u8; 32] = [
    0x07, 0xc2, 0x30, 0x93, 0x4a, 0x52, 0xf1, 0x72,
    0x1a, 0xa2, 0x77, 0x52, 0xa6, 0x72, 0x43, 0x75,
    0xe8, 0xff, 0xe1, 0x7e, 0x93, 0xef, 0xcc, 0xa5,
    0x14, 0x37, 0xde, 0x7f, 0x31, 0x1c, 0xd2, 0x45
];

type Aes256Ecb = Ecb<Aes256, NoPadding>;

struct CeltHeader {
    pub version: u16,
    pub encrypted: bool,
    pub total_samples: u32,
    pub bitrate: u32,

    pub frame_size: u16,
    pub look_ahead: u16,
    pub sample_rate: u16,
    pub unknown: u16,

    pub map_start_offset: u32,
    pub map_size: u32,
    pub packets_start_offset: u32,
    pub packets_size: u32,
}

pub struct Celt {
    header: CeltHeader,
    data: Box<[u8]>,
}

impl CeltHeader {
    fn from_data(data: &[u8]) -> CeltHeader {
        // TODO: Handle errors
        let (_, header) = CeltHeader::parse_data(data).unwrap();
        header
    }

    fn parse_data<'a>(data: &'a [u8]) -> IResult<&'a [u8], CeltHeader> {
        let (d, _) = tag("BFAD")(data)?;
        let (d, (
            version,
            enc_num,
            total_samples,
            bitrate,
            frame_size,
            look_ahead,
            sample_rate,
            unknown,
            map_start_offset,
            map_size,
            packets_start_offset,
            packets_size
        )) = tuple((
            le_u16,
            le_u16,
            le_u32,
            le_u32,
            le_u16,
            le_u16,
            le_u16,
            le_u16,
            le_u32,
            le_u32,
            le_u32,
            le_u32,
        ))(d)?;

        Ok((d, CeltHeader {
            version,
            encrypted: enc_num != 0,
            total_samples,
            bitrate,
            frame_size,
            look_ahead,
            sample_rate,
            unknown,
            map_start_offset,
            map_size,
            packets_start_offset,
            packets_size
        }))
    }

    fn write_to_slice(&self, data: &mut [u8; 40]) {
        // Write magic
        data[..4].copy_from_slice(b"BFAD");

        // Write everything else
        ByteWriter::new(&mut data[4..])
            .write_u16(self.version)
            .write_u16(self.encrypted.into())
            .write_u32(self.total_samples)
            .write_u32(self.bitrate)
            .write_u16(self.frame_size)
            .write_u16(self.look_ahead)
            .write_u16(self.sample_rate)
            .write_u16(self.unknown)
            .write_u32(self.map_start_offset)
            .write_u32(self.map_size)
            .write_u32(self.packets_start_offset)
            .write_u32(self.packets_size);
    }
}

struct ByteWriter<'a>(&'a mut [u8]);

impl<'a> ByteWriter<'a> {
    fn new(data: &'a mut [u8]) -> ByteWriter<'a> {
        ByteWriter(data)
    }

    fn write_u16(self, num: u16) -> ByteWriter<'a> {
        let ByteWriter(data) = self;

        data[..size_of::<u16>()].copy_from_slice(&num.to_le_bytes());
        ByteWriter(&mut data[size_of::<u16>()..])
    }

    fn write_u32(self, num: u32) -> ByteWriter<'a> {
        let ByteWriter(data) = self;

        data[..size_of::<u32>()].copy_from_slice(&num.to_le_bytes());
        ByteWriter(&mut data[size_of::<u32>()..])
    }
}

impl Celt {
    pub fn from_path<T: AsRef<Path>>(celt_path: T) -> Celt {
        let mut celt_file = File::open(celt_path).unwrap();
        //let file_size = celt_file.metadata().unwrap().len();
        // TODO: Check file size before reading?

        let mut header_data = [0u8; 40];
        celt_file.read_exact(&mut header_data).unwrap();

        let header = CeltHeader::from_data(&header_data);

        let mut data_size = (header.map_size + header.packets_size) as usize;
        let rem = data_size % 16;

        if rem > 0 {
            // Pad size to fit 16-byte block
            data_size += 16 - rem;
        }

        // Read data
        let mut data = vec![0u8; data_size].into_boxed_slice();
        celt_file.read_exact(&mut data).unwrap();

        Celt {
            header,
            data
        }
    }

    pub fn save<T: AsRef<Path>>(&self, celt_path: T) {
        let celt_path = celt_path.as_ref();

        if !celt_path.is_file() {
            // TODO: Throw error?
        }

        // Create directory
        if let Some(output_dir) = celt_path.parent() {
            if !output_dir.exists() {
                create_dir_all(&output_dir).unwrap();
            }
        }

        // Delete old file
        if celt_path.exists() {
            // TODO: Investigate better approach to guarantee deletion
            remove_file(celt_path).unwrap();
        }

        let mut celt_file = File::create(celt_path).unwrap();

        let mut header_data = [0u8; 40];
        self.header.write_to_slice(&mut header_data);

        celt_file.write(&header_data).unwrap();
        celt_file.write(&self.data).unwrap();

        // TODO: Return result
    }

    pub fn is_encrypted(&self) -> bool {
        self.header.encrypted
    }

    pub fn decrypt(&mut self) {
        if !self.is_encrypted() {
            return;
        }

        // Decrypt data
        let cipher = Aes256Ecb::new_from_slices(&AES_KEY, &[0u8; 16]).unwrap();
        cipher.decrypt(&mut self.data).unwrap();

        // Update value
        self.header.encrypted = false;
    }
}

impl Default for CeltHeader {
    fn default() -> CeltHeader {
        CeltHeader {
            version: 2,
            encrypted: false,
            total_samples: 0,
            bitrate: 96000,
            frame_size: 960,
            look_ahead: 312,
            sample_rate: 48000,
            unknown: 1,
            map_start_offset: 40,
            map_size: 0,
            packets_start_offset: 40,
            packets_size: 0
        }
    }
}
