use crate::apps::{SubApp};
use bfforever::audio::*;
use clap::Parser;
use log::debug;

const SUPPORTED_EXTS: [&'static str; 1] = [
    ".wav",
    //".ogg",
];

#[derive(Parser)]
pub struct AudioApp {
    #[clap(help = "Path to input audio file (clt/wav)", required = true)]
    pub input_path: String,
    #[clap(help = "Path to output audio file (clt/wav)", required = true)]
    pub output_path: String,
}

impl SubApp for AudioApp {
    fn process(&mut self) {
        // clt -> clt - decrypt
        // clt -> wav - decode
        // wav -> clt - encode
        debug!("Processing audio: {}", &self.input_path);

        // Assume input is celt
        let mut celt_audio = Celt::open(&self.input_path);
        celt_audio.decrypt();

        if self.output_path.ends_with(".clt") {
            // Save as decrypted .clt
            celt_audio.save(&self.output_path);
        } else {
            // Decode and save as .wav
            let samples = celt_audio.decode();
            let channels = celt_audio.get_channels() as u16;
            let sample_rate = celt_audio.get_sample_rate() as u32;

            let wav_encoder = WavEncoder::new(&samples, channels, sample_rate);
            wav_encoder.encode_to_file(&self.output_path);
        }

        print!("Wrote output to \"{}\"", &self.output_path);
    }
}