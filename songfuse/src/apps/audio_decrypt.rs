use crate::apps::{SubApp};
use clap::Parser;

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
        print!("Wrote output to \"{}\"", &self.output_path);
    }
}