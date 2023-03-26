use miniquad::{GraphicsContext, fs::Error};
use std::sync::{Arc, Mutex};

use crate::texture::Texture2D;

type TextureReference = Arc<Mutex<Option<Texture2D>>>;

#[derive(Debug, Clone)]
pub struct TextureHandle {
    texture: TextureReference
}

impl TextureHandle {
    pub fn width(&self) -> u32 {
        match  *self.texture.lock().unwrap() {
            Some(texture) => {
                texture.width()
            },
            None => { 0 }
        }
    }

    pub fn height(&self) -> u32 {
        match *self.texture.lock().unwrap() {
            Some(texture) => {
                texture.height()
            },
            None => { 0 }
        }
    } 
}

pub struct TextureLoadData {
    contents: Arc<Mutex<Option<Result<Vec<u8>, Error>>>>,
    texture_index: usize,
}

pub struct GraphicsSystem {
    texture_load_queue: Vec<TextureLoadData>,
    textures: Vec<TextureReference>,
}

impl GraphicsSystem {
    pub fn new() -> GraphicsSystem {
        GraphicsSystem { texture_load_queue: Vec::new(), textures: Vec::new() }
    }

    pub fn update(&mut self, ctx: &mut GraphicsContext) {

        let mut finished = Vec::new();
        for i in (0..self.texture_load_queue.len()).rev() {
            if let Some(data) = self.texture_load_queue.get(i) {
                if let Some(result) = &*data.contents.lock().unwrap() {
                    finished.push(i);

                    match result {
                        Ok(bytes) => {
                            if let Ok(texture) = Texture2D::from_bytes(bytes, ctx) {
                                *self.textures[data.texture_index].lock().unwrap() = Some(texture);
                            }
                        },
                        Err(_) => {

                        }
                    }
                }
            }
        }

        for i in finished.iter() {
            self.texture_load_queue.remove(*i);
        }

    }

    pub fn load_texture(&mut self, path: &str) -> TextureHandle {

        let contents = Arc::new(Mutex::new(None));

        {
            let contents = contents.clone();
            
            crate::file::load_bytes_async(path, move |result| {       
                *contents.lock().unwrap() = Some(result);
            });
        }

        let index = self.textures.len();
        let texture_reference = Arc::new(Mutex::new(None));
        self.textures.push(texture_reference.clone());

        self.texture_load_queue.push(TextureLoadData { contents, texture_index: index });

        TextureHandle { texture: texture_reference }
    }
}
