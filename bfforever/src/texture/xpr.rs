pub enum TextureFormat {
    DXT1,
    DXT5
}

pub struct Texture2D {
    pub name: String,
    pub width: u32,
    pub height: u32,

    pub data: Vec<u8>
}

pub struct XPR2 {
    pub textures: Vec<Texture2D>
}