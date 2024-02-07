mod io;

#[allow(unused_imports)] pub use io::*;
use super::DXGI_Encoding;
use std::io::Error as IOError;
use std::path::Path;

use crate::texture::decode_dx_image;

#[derive(Debug)]
pub enum TextureFormat {
    DXT1,
    DXT5,
    Raw
}

#[derive(Debug)]
pub struct Texture2D {
    pub name: String,
    pub format: TextureFormat,

    pub width: u32,
    pub height: u32,

    pub data: Vec<u8>
}

#[derive(Debug)]
pub struct XPR2 {
    pub textures: Vec<Texture2D>
}

impl Texture2D {
    pub fn save<T: AsRef<Path>>(&self, file_path: T) -> Result<(), IOError> {
        use crate::io::create_missing_dirs;
        use image::RgbaImage;

        let rgba = self.unpack_rgba();
        let image = RgbaImage::from_vec(self.width, self.height, rgba).unwrap();

        create_missing_dirs(&file_path)?;
        image.save(&file_path).unwrap();
        Ok(())
    }

    fn unpack_rgba(&self) -> Vec<u8> {
        let mut rgba = vec![0u8; (self.width * self.height * 4) as usize];

        let (encoding, bpb, bpp) = match self.format {
            TextureFormat::DXT1 => (DXGI_Encoding::DXGI_FORMAT_BC1_UNORM, 8, 4),
            TextureFormat::DXT5 => (DXGI_Encoding::DXGI_FORMAT_BC3_UNORM, 16, 8),
            TextureFormat::Raw => {
                let rgba_length = rgba.len();
                rgba.copy_from_slice(&self.data[..rgba_length]);
                return rgba;
            }
        };

        let untiled_img = untile_texture(&self.data, self.width, self.width, self.height, self.height, 4, 4, bpb);

        let img_size = ((self.width * self.height) * bpp) / 8;
        let data = &untiled_img[..img_size as usize];

        decode_dx_image(&data, &mut rgba, self.width, encoding, true);
        rgba
    }
}

fn untile_texture(src: &[u8], tiled_width: u32, original_width: u32, tiled_height: u32, original_height: u32, block_size_x: u32, block_size_y: u32, bytes_per_block: u32) -> Vec<u8> {
    let dst_size = ((tiled_height * tiled_width) * ((bytes_per_block * 8) / (block_size_x * block_size_y))) / 8;
    let mut dst = vec![0u8; dst_size as usize];

    let tiled_block_width = tiled_width / block_size_x;          // Width of image in blocks
    let original_block_width = original_width / block_size_x;    // Width of image in blocks
    let _tiled_block_height = tiled_height / block_size_y;       // Height of image in blocks
    let original_block_height = original_height / block_size_y;  // Height of image in blocks
    let log_bpb = bytes_per_block.ilog2();

    let mut sx_offset = 0;
    if (tiled_block_width >= original_block_width * 2) && (original_width == 16) {
        sx_offset = original_block_width;
    }

    // Iterate image blocks
    for dy in 0..original_block_height
    {
        for dx in 0..original_block_width
        {
            let swz_addr = get_tiled_offset(dx + sx_offset, dy, tiled_block_width, log_bpb);  // Do once for whole block
            let sy = swz_addr / tiled_block_width;
            let sx = swz_addr % tiled_block_width;

            let dst_offset = ((dy * original_block_width + dx) * bytes_per_block) as usize;
            let src_offset = ((sy * tiled_block_width + sx) * bytes_per_block) as usize;

            dst[dst_offset..(dst_offset + bytes_per_block as usize)].copy_from_slice(&src[src_offset..(src_offset + bytes_per_block as usize)]);
        }
    }

    dst
}

fn get_tiled_offset(x: u32, y: u32, width: u32, log_bpb: u32) -> u32 {
    // Width <= 8192 && (x < width)

    let aligned_width = align(width, 32);
    // Top bits of coordinates
    let macro_part = ((x >> 5) + (y >> 5) * (aligned_width >> 5)) << (log_bpb + 7);
    // Lower bits of coordinates (result is 6-bit value)
    let micro_part = ((x & 7) + ((y & 0xE) << 2)) << log_bpb;
    // Mix micro/macro + add few remaining x/y bits
    let offset = macro_part + ((micro_part & !0xF) << 1) + (micro_part & 0xF) + ((y & 1) << 4);

    // Mix bits again
    return (((offset & !0x1FF) << 3) +              // Upper bits (offset bits [*-9])
        ((y & 16) << 7) +                           // Next 1 bit
        ((offset & 0x1C0) << 2) +                   // Next 3 bits (offset bits [8-6])
        (((((y & 8) >> 2) + (x >> 3)) & 3) << 6) +  // Next 2 bits
        (offset & 0x3F)                             // Lower 6 bits (offset bits [5-0])
        ) >> log_bpb;
}

fn align(ptr: u32, alignment: u32) -> u32 {
    (ptr + alignment - 1) & !(alignment - 1)
}