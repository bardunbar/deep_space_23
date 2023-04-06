
use miniquad::{
    EventHandler,
    GraphicsContext,
    conf
};

use crate::graphics::{GraphicsSystem, TextureHandle};


pub struct Engine {
    graphics: GraphicsSystem,
    test_handle: TextureHandle,
}

impl Engine {
    pub fn start() {
        miniquad::start(
            conf::Conf {
                window_title: "Deep Space 23".to_string(),
                window_width: 1024,
                window_height: 768,
                fullscreen: false,
                ..Default::default()
            }, |ctx| { Box::new(Engine::new(ctx))});
    }

    fn new(ctx: &mut GraphicsContext) -> Engine {

        let mut graphics = GraphicsSystem::new(ctx);

        let handle = graphics.load_texture("assets/ds_tileset.png");

        Engine {
            graphics,
            test_handle: handle,
        }
    }
}

impl EventHandler for Engine {
    fn update(&mut self, ctx: &mut miniquad::Context) {

        self.graphics.update(ctx);

    }

    fn draw(&mut self, ctx: &mut miniquad::Context) {

        ctx.begin_default_pass(Default::default());

        self.graphics.batch(ctx, &mut |sb| {
            sb.draw(&self.test_handle.texture, 0.0, 0.0);
        });

        ctx.end_render_pass();

        ctx.commit_frame();
    }
}