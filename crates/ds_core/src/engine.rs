
use ds_math::Rect2D;
use miniquad::{
    EventHandler,
    GraphicsContext,
    conf
};

use crate::{graphics::{GraphicsSystem, TextureHandle}, display_object};


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
            sb.draw(&display_object::DisplaySprite::new(self.test_handle.clone(), 128.0, 128.0));

            sb.draw(&display_object::DisplaySprite::new_rect(self.test_handle.clone(), 256.0, 256.0,
                Rect2D::new(0.0, 0.0, 64.0, 64.0)));

            sb.draw(&display_object::DisplaySprite::new_rect(self.test_handle.clone(), 192.0, 192.0,
                Rect2D::new(64.0, 0.0, 64.0, 64.0)));
        });

        ctx.end_render_pass();

        ctx.commit_frame();
    }
}