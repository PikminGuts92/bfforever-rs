#![allow(dead_code)]
#![allow(unused_imports)]

mod apps;
use apps::SongFuseTool;
use simplelog::*;

#[cfg(debug_assertions)]
const LOG_LEVEL: LevelFilter = LevelFilter::Debug;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: LevelFilter = LevelFilter::Info;

fn main() {
    let log_config = ConfigBuilder::new()
        .add_filter_allow_str(env!("CARGO_PKG_NAME"))
        .add_filter_allow_str("bfforever")
        .build();

    // Setup logging
    CombinedLogger::init(
        vec![
            TermLogger::new(LOG_LEVEL, log_config, TerminalMode::Mixed, ColorChoice::Auto),
        ]
    ).unwrap();

    let mut song_fuse = SongFuseTool::new();
    song_fuse.run();
}
