mod crypt;
mod io;

pub use crypt::*;
pub use io::IOFile;

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
