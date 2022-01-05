use clap::{Parser, Subcommand};

mod audio_decrypt;
pub use self::audio_decrypt::*;

// From Cargo.toml
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) trait SubApp {
    fn process(&mut self);
}

#[derive(Parser)]
#[clap(name = PKG_NAME, version = VERSION, about = "Use this tool for modding bandfuse", author = PKG_AUTHORS)]
struct Options {
    #[clap(subcommand)]
    commands: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    #[clap(name = "audio", about = "Encode/decode celt audio")]
    Audio(AudioApp),
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
        }
    }
}