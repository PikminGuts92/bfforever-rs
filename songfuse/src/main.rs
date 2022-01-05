mod apps;
use apps::SongFuseTool;

fn main() {
    let mut song_fuse = SongFuseTool::new();
    song_fuse.run();
}
