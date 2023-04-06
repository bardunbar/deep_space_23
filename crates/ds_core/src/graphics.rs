use miniquad::{
    GraphicsContext,
    fs::Error,
    Buffer, BufferType,
    Bindings,
    Pipeline,
    Shader,
    BufferLayout,
    VertexAttribute,
    VertexFormat,
    Texture
};
use std::sync::{Arc, Mutex};
use glam::{Vec2, Vec3, Quat, vec3};

use crate::texture::Texture2D;

type TextureReference = Arc<Mutex<Option<Texture2D>>>;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Vertex {
    pos: [f32; 3],
    uv: [f32; 2],
    color: [u8; 4],
}

#[derive(Debug, Clone)]
pub struct TextureHandle {
    pub(crate) texture: TextureReference
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
    sprite_batch: SpriteBatch,

}

impl GraphicsSystem {
    pub fn new(ctx: &mut GraphicsContext) -> GraphicsSystem {
        GraphicsSystem { texture_load_queue: Vec::new(), textures: Vec::new(), sprite_batch: SpriteBatch::new(ctx) }
    }

    pub fn update(&mut self, ctx: &mut GraphicsContext) {

        if self.texture_load_queue.len() > 0 {
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

    pub fn batch(&mut self, ctx: &mut GraphicsContext, f: &mut dyn FnMut(&mut SpriteBatch)) {
        self.sprite_batch.begin();

        f(&mut self.sprite_batch);

        self.sprite_batch.end(ctx);
    }


}


pub struct SpriteInfo {
    pub(crate) texture: Texture2D,
    pub(crate) position: Vec2,
}

// https://gamedev.stackexchange.com/questions/21220/how-exactly-does-xnas-spritebatch-work
pub struct SpriteBatch {
    draw_calls: Vec<SpriteInfo>,
    pipeline: Pipeline,
    bindings: Bindings,
}

impl SpriteBatch {
    pub fn new(ctx: &mut GraphicsContext) -> SpriteBatch {

        let shader: Shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta())//.unwrap();
            .unwrap_or_else(|e| panic!("Failed to load shader: {}", e));

        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("position", VertexFormat::Float3),
                VertexAttribute::new("uv", VertexFormat::Float2),
                VertexAttribute::new("color", VertexFormat::Byte4),
            ],
            shader);

        let vertex_buffer = Buffer::stream(ctx, BufferType::VertexBuffer, 4 * std::mem::size_of::<Vertex>());

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![Texture::empty()]
        };

        SpriteBatch {
            draw_calls: Vec::new(),
            pipeline,
            bindings,
        }
    }

    pub fn begin(&mut self) {

    }

    pub fn draw(&mut self, texture_reference: &TextureReference, x: f32, y:f32) {
        if let Some(texture) = *texture_reference.lock().unwrap() {
            let draw_call = SpriteInfo {
                texture: texture.clone(),
                position: Vec2::new(x, y)
            };

            self.draw_calls.push(draw_call);
        }
    }

    pub fn end(&mut self, ctx: &mut GraphicsContext) {

        ctx.apply_pipeline(&self.pipeline);

        let (width, height) = ctx.screen_size();
        let dpi = ctx.dpi_scale();
        // TODO: Do this somewhere else
        let projection = glam::Mat4::orthographic_rh_gl(0., width / dpi, height / dpi, 0., -1., 1.);
        let view = glam::Mat4::look_at_rh(vec3(0., 0., 0.), vec3(0., 0., -1.), vec3(0., 1., 0.));
        let view_proj = projection * view;

        // For now don't bother batching... just render one at a time
        for draw_call in self.draw_calls.iter() {

            let model = glam::Mat4::from_scale_rotation_translation(Vec3::ONE, Quat::IDENTITY, Vec3::new(draw_call.position.x, draw_call.position.y, 0.));

            let texture_width = draw_call.texture.texture.width as f32;
            let texture_height = draw_call.texture.texture.height as f32;

            let vertices: [Vertex; 4] = [
                Vertex { pos: [0.0, 0.0, 0.0], uv: [0.0, 0.0], color: [255, 255, 255, 255]},
                Vertex { pos: [0.0, texture_height, 0.0], uv: [0.0, 1.0], color: [255, 255, 255, 255]},
                Vertex { pos: [texture_width, texture_height, 0.0], uv: [1.0, 1.0], color: [255, 255, 255, 255]},
                Vertex { pos: [texture_width, 0.0, 0.0], uv: [1.0, 0.0], color: [255, 255, 255, 255]},
            ];
            self.bindings.vertex_buffers[0].update(ctx, &vertices);
            self.bindings.images[0] = draw_call.texture.texture;

            ctx.apply_bindings(&self.bindings);

            let mvp = view_proj * model;
            ctx.apply_uniforms(&shader::Uniforms {
                mvp
            });

            ctx.draw(0, 6, 1);

        }

        self.draw_calls.clear();

    }
}

mod shader {

    use miniquad::{ShaderMeta, UniformBlockLayout, UniformType, UniformDesc};

    pub const VERTEX: &str = r#"#version 100
    attribute vec3 position;
    attribute vec2 uv;
    attribute vec4 color;

    varying lowp vec2 texcoord;
    varying lowp vec4 v_color;

    uniform mat4 mvp;
    //uniform mat4 Model;
    //uniform mat4 Projection;

    void main() {
        gl_Position = mvp * vec4(position, 1);
        v_color = color / 255.0;
        texcoord = uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec4 v_color;
    varying lowp vec2 texcoord;

    uniform sampler2D Texture;

    void main() {
        gl_FragColor = v_color * texture2D(Texture, texcoord);
    }"#;


    #[repr(C)]
    pub struct Uniforms {
        pub mvp: glam::Mat4,
    }

    pub fn uniforms() -> Vec<(&'static str, UniformType)> {
        vec![
            ("mvp", UniformType::Mat4),
            //.. Add more uniforms here if and when needed
        ]
    }

    pub fn meta() -> ShaderMeta {
        ShaderMeta { uniforms: UniformBlockLayout {
            uniforms: uniforms()
                .into_iter()
                .map(|(name, kind)| UniformDesc::new(name, kind))
                .collect()
        }, images: vec!["Texture".to_string()] }
    }
}