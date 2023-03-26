use miniquad::GraphicsContext;
use image::{
    ImageFormat
};

use crate::error::Error;

// use crate::file::{
//     FileError,
//     load_bytes
// };

#[derive(Debug, Clone, Copy)]
pub struct Texture2D {
    pub(crate) texture: miniquad::Texture, 
}

impl Texture2D {
    pub(crate) fn from_bytes(bytes: &[u8], ctx: &mut GraphicsContext) -> Result<Texture2D, Error> {
        match image::load_from_memory_with_format(bytes, ImageFormat::Png) {
            Ok(loaded_image) => {
                let rgb = loaded_image.to_rgba8();
                let width = rgb.width() as u16;
                let height = rgb.height() as u16;
                let bytes = rgb.into_raw();

                let texture = miniquad::Texture::from_rgba8(ctx, width, height, &bytes);
                Ok(Texture2D { texture })
            },
            Err(e) => {
                Err(Error::error(&e.to_string()))
            }
        }
    }

    pub fn width(&self) -> u32 {
        self.texture.width
    }

    pub fn height(&self) -> u32 {
        self.texture.height
    }
}