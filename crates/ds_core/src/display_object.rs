use crate::{
    graphics::{
        Vertex,
        TextureHandle,
    },
    texture::Texture2D,
};
use ds_math::Rect2D;

use glam::{Mat4, Vec3, Quat};

pub trait DisplayObject {
    fn get_verts(&self) -> &Vec<Vertex>;
    fn get_transform(&self) -> Mat4;
    fn get_texture(&self) -> Texture2D;
}

pub struct DisplaySprite {
    texture_handle: TextureHandle,
    x: f32,
    y: f32,
    verts: Vec<Vertex>,
}

impl DisplaySprite {
    pub fn new(texture_handle: TextureHandle, x: f32, y: f32) -> DisplaySprite {
        let width = texture_handle.width() as f32;
        let height = texture_handle.height() as f32;

        DisplaySprite { texture_handle, x, y, verts: vec![
            Vertex { pos: [0.0, 0.0, 0.0], uv: [0.0, 0.0], color: [255, 255, 255, 255]},
            Vertex { pos: [0.0, height, 0.0], uv: [0.0, 1.0], color: [255, 255, 255, 255]},
            Vertex { pos: [width, height, 0.0], uv: [1.0, 1.0], color: [255, 255, 255, 255]},
            Vertex { pos: [width, 0.0, 0.0], uv: [1.0, 0.0], color: [255, 255, 255, 255]},
        ] }
    }

    pub fn new_rect(texture_handle: TextureHandle, x: f32, y: f32, rect: Rect2D) -> DisplaySprite {
        let texture_width = texture_handle.width() as f32;
        let texture_height = texture_handle.height() as f32;

        let (x0, y0) = (rect.x, rect.y);
        let (x1, y1) = (rect.x + rect.w, rect.y + rect.h);

        let (u0, v0) = (x0 / texture_width, y0 / texture_height);
        let (u1, v1) = (x1 / texture_width, y1 / texture_height);

        DisplaySprite { texture_handle, x, y, verts: vec![
            Vertex { pos: [0.0, 0.0, 0.0], uv: [u0, v0], color: [255, 255, 255, 255]},
            Vertex { pos: [0.0, rect.h, 0.0], uv: [u0, v1], color: [255, 255, 255, 255]},
            Vertex { pos: [rect.w, rect.h, 0.0], uv: [u1, v1], color: [255, 255, 255, 255]},
            Vertex { pos: [rect.w, 0.0, 0.0], uv: [u1, v0], color: [255, 255, 255, 255]},
        ] }
    }
}

impl DisplayObject for DisplaySprite {
    fn get_texture(&self) -> Texture2D {
        self.texture_handle.get_texture().unwrap_or(Texture2D::empty())
    }

    fn get_transform(&self) -> Mat4 {
        glam::Mat4::from_scale_rotation_translation(Vec3::ONE, Quat::IDENTITY, Vec3::new(self.x, self.y, 0.))
    }

    fn get_verts(&self) -> &Vec<Vertex> {
        // if
        &self.verts
    }
}