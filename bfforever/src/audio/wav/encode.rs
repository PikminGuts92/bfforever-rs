use crate::audio::AudioEncoder;
use crate::io::create_new_file;
use wav::{Header, WAV_FORMAT_PCM, write as wav_write};

pub struct WavEncoder<'a> {
    data: &'a Box<[i16]>,
    channels: u16,
    sample_rate: u32
}

impl<'a> WavEncoder<'a> {
    pub fn new(data: &'a Box<[i16]>, channels: u16, sample_rate: u32) -> Self {
        WavEncoder {
            data,
            channels,
            sample_rate
        }
    }
}

impl<'a> AudioEncoder for WavEncoder<'a> {
    fn encode_to_file<T: AsRef<std::path::Path>>(&self, out_path: T) {
        let mut out_file = create_new_file(out_path).unwrap();

        let header = Header::new(WAV_FORMAT_PCM, self.channels, self.sample_rate, 16);
        let bit_data = self.data.to_owned().to_vec().into(); // Ugh... I hate this so much...

        wav_write(header, &bit_data, &mut out_file).unwrap();
    }
}