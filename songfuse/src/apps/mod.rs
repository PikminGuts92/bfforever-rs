use clap::{Parser, Subcommand};

mod audio;
mod texture;

pub use self::audio::*;
pub use self::texture::*;

// From Cargo.toml
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) trait SubApp {
    fn process(&mut self);
}

#[derive(Parser)]
#[command(name = PKG_NAME, version = VERSION, about = "Use this tool for modding bandfuse", author = PKG_AUTHORS)]
struct Options {
    #[command(subcommand)]
    commands: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    #[command(name = "audio", about = "Encode/decode celt audio")]
    Audio(AudioApp),
    #[command(name = "texture", about = "Decode texture file")]
    Texture(TextureApp),
}

pub struct SongFuseTool {
    options: Options,
}

impl SongFuseTool {
    pub fn new() -> SongFuseTool {
        SongFuseTool {
            options: Options::parse()
        }
    }

    pub fn run(&mut self) {
        match &mut self.options.commands {
            SubCommand::Audio(app) => app.process(),
            SubCommand::Texture(app) => app.process(),
        }
    }
}