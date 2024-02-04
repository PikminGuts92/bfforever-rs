use crate::io::create_new_file;
use nom::IResult;
use nom::bytes::streaming::tag;
use nom::number::streaming::{le_u16, le_u32};
use nom::sequence::tuple;
use std::fs::File;
use std::io::{Read, Write};
use std::mem::size_of;
use std::path::Path;
use super::{Celt, CeltHeader, Crypt};

pub struct ByteWriter<'a>(&'a mut [u8]);

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

pub trait IOFile {
    fn open<T: AsRef<Path>>(celt_path: T) -> Self;
    fn save<T: AsRef<Path>>(&self, celt_path: T);
}

impl CeltHeader {
    pub fn parse_data<'a>(data: &'a [u8]) -> IResult<&'a [u8], CeltHeader> {
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

    pub fn write_to_slice(&self, data: &mut [u8; 40]) {
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

impl IOFile for Celt {
    fn open<T: AsRef<Path>>(celt_path: T) -> Celt {
        let mut celt_file = File::open(celt_path).unwrap();
        //let file_size = celt_file.metadata().unwrap().len();
        // TODO: Check file size before reading?

        let mut header_data = [0u8; 40];
        celt_file.read_exact(&mut header_data).unwrap();

        let header = CeltHeader::from_data(&header_data);

        let actual_map_size = header.packets_start_offset - header.map_start_offset; // Multiple of 4
        let mut data_size = (actual_map_size + header.packets_size) as usize;
        let rem = data_size % 16;

        if rem > 0 {
            // Pad size to fit 16-byte block
            data_size += 16 - rem;
        }

        // Read data
        let mut data = vec![0u8; data_size].into_boxed_slice();
        celt_file.read_exact(&mut data).unwrap();

        let mut celt = Celt {
            header,
            data,
            ..Default::default()
        };

        if !celt.is_encrypted() {
            celt.recompute_offsets();
        }

        celt
    }

    fn save<T: AsRef<Path>>(&self, celt_path: T) {
        let celt_path = celt_path.as_ref();

        let mut celt_file = create_new_file(celt_path).unwrap();

        let mut header_data = [0u8; 40];
        self.header.write_to_slice(&mut header_data);

        celt_file.write(&header_data).unwrap();
        celt_file.write(&self.data).unwrap();

        // TODO: Return result
    }
}