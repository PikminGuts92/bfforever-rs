use audiopus::coder::Decoder;
use audiopus::{Channels, MutSignals, SampleRate};
use audiopus::packet::Packet;
use crate::audio::AudioDecoder;
use super::{Celt, CeltHeader};

impl AudioDecoder for Celt {
    fn decode(&self) -> Box<[i16]> {
        let packets = self.get_raw_packets();
        /*let total_samples = self.get_total_samples();
        let bitrate = self.get_bitrate();
        let frame_Size = self.get_frame_size();
        let sample_rate = self.get_sample_rate();*/

        let CeltHeader { total_samples, frame_size, sample_rate, .. } = self.header;
        let channels = self.get_channels() as usize;
        let calc_frame_size = (frame_size as u32 * channels as u32) as usize;

        let mut samples = vec![0i16; (total_samples * channels as u32) as usize].into_boxed_slice();

        let sample_rate = match sample_rate {
             8000 => SampleRate::Hz8000,
            12000 => SampleRate::Hz12000,
            16000 => SampleRate::Hz16000,
            24000 => SampleRate::Hz24000,
            48000 => SampleRate::Hz48000,
                _ => panic!("Unsupported sample rate of {}Hz", sample_rate), // TODO: Switch to result error?
        };

        let channels = match channels {
            1 => Channels::Mono,
            2 => Channels::Stereo,
            _ => Channels::Auto,
        };

        let mut decoder = Decoder::new(sample_rate, channels).unwrap();

        for raw_packet in packets.iter() {
            let data_start = calc_frame_size * raw_packet.frame_offset;

            let buffer = &mut samples[data_start..(data_start + calc_frame_size)];
            decoder.decode(Some(raw_packet.data), buffer, false).unwrap();
        }

        samples
    }
}