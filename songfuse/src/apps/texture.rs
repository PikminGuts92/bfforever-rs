use crate::apps::SubApp;
use bfforever::texture::*;
use clap::Parser;

#[derive(Parser)]
pub struct TextureApp {
    #[arg(help = "Path to input texture file (xpr)", required = true)]
    pub input_path: String,
    #[arg(help = "Path to output texture file (png)", required = true)]
    pub output_path: String,
}

impl SubApp for TextureApp {
    fn process(&mut self) {
        let xpr = XPR2::from_file(&self.input_path).unwrap();
        let tex = &xpr.textures[0];

        tex.save(&self.output_path).unwrap();

        print!("Wrote output to \"{}\"", &self.output_path);
    }
}