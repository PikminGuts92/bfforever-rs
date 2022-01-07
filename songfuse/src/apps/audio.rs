use crate::apps::{SubApp};
use bfforever::audio::*;
use clap::Parser;
use log::debug;

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
        let mut celt_audio = Celt::from_path(&self.input_path);

        print!("Wrote output to \"{}\"", &self.output_path);
    }
}