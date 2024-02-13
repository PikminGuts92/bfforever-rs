use crate::{HKey, SKey};

pub struct Song {
    pub title: SKey,
    pub artist: SKey,
    pub description: SKey,
    pub album: SKey,
    pub texture_path: HKey,
    pub legend_tag: HKey,
    pub era_tag: HKey,

    pub year: u32,
    pub guitar_intensity: f32,
    pub bass_intensity: f32,
    pub vox_intensity: f32,

    pub metadata_tags: Vec<HKey>,
    pub genre_tags: Vec<HKey>,
    pub labels: Vec<SKey>,

    pub song_length: f32,
    // Empty 4 bytes

    pub preview_path: HKey,
    pub video_path: HKey,
    // Empty 8 bytes

    pub instrument_tags: Vec<HKey>,

    pub backing_audio_path: HKey,
    pub bass_audio_path: HKey,
    pub drums_audio_path: HKey,
    pub lead_guitar_audio_path: HKey,
    pub rhythm_guitar_audio_path: HKey,
    pub vox_audio_path: HKey,
}