mod crypt;
mod decode;
mod io;

pub use crypt::*;
pub use decode::*;
pub use io::IOFile;

#[derive(Clone, Copy)]
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

struct PacketInfo {
    frame_offset: usize,
    data_offset: usize,
    size: usize,
}

pub(crate) struct RawPacket<'a> {
    frame_offset: usize,
    data: &'a[u8],
}

#[derive(Default)]
pub struct Celt {
    header: CeltHeader,
    data: Box<[u8]>,
    packet_map: Vec<PacketInfo>,
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

impl Celt {
    pub const fn get_channels(&self) -> u32 {
        2
    }

    pub fn get_total_samples(&self) -> u32 {
        self.header.total_samples
    }

    pub fn get_bitrate(&self) -> u32 {
        self.header.bitrate
    }

    pub fn get_frame_size(&self) -> u16 {
        self.header.frame_size
    }

    pub fn get_sample_rate(&self) -> u16 {
        self.header.sample_rate
    }

    pub(crate) fn recompute_offsets(&mut self) {
        self.packet_map.clear();

        let (map_data, packet_data) = self.data.split_at(self.header.map_size as usize);

        let mut frame_idx = 0;
        let mut prev_count_part = None;
        let mut silence = true;

        let mut map = Vec::new(); // (frame index, # packets)

        for m in map_data.iter() {
            if let Some(s) = prev_count_part {
                let count = s | (*m as usize);
                if !silence {
                    map.push((frame_idx, count));
                }

                frame_idx += count;
                prev_count_part = None;
                silence = !silence;

                continue;
            }

            let count = *m as usize;
            if (count & 0x80) != 0 {
                prev_count_part = Some((count ^ 0x80) << 8);
            } else {
                if !silence {
                    map.push((frame_idx, count));
                }

                frame_idx += count;
                prev_count_part = None;
                silence = !silence;
            }
        }

        let mut data_index = 0;

        for (frame_start, num_packets) in map.iter() {
            let mut frame_index = *frame_start;

            for _ in 0..*num_packets {
                let packet_size = (((packet_data[data_index] & 0x0F) as usize) << 8) | packet_data[data_index + 1] as usize;

                self.packet_map.push(PacketInfo {
                    frame_offset: frame_index,
                    data_offset: data_index + 2,
                    size: packet_size,
                });

                data_index += packet_size + 2;
                frame_index += 1;
            }
        }
    }

    pub(crate) fn get_raw_packets<'a>(&'a self) -> Vec<RawPacket<'a>> {
        let (_, packet_data) = self.data.split_at(self.header.map_size as usize);

        self.packet_map
            .iter()
            .map(|m| {
                RawPacket {
                    frame_offset: m.frame_offset,
                    data: &packet_data[m.data_offset..(m.data_offset + m.size)],
                }
            })
            .collect()
    }
}