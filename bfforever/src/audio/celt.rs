use nom::{Err, IResult, Needed};
use nom::bytes::streaming::tag;
use nom::number::streaming::{le_u8, le_u16, le_u32};
use nom::sequence::{pair, terminated, tuple};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

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
}

impl Celt {
    pub fn from_path<T: AsRef<Path>>(celt_path: T) -> Celt {
        let mut celt_file = File::open(celt_path).unwrap();
        //let file_size = celt_file.metadata().unwrap().len();
        // TODO: Check file size before reading?

        let mut header_data = [0u8; 40];
        celt_file.read_exact(&mut header_data).unwrap();

        let header = CeltHeader::from_data(&header_data);

        Celt {

        }
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
