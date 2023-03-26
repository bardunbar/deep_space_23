
use miniquad::{
    EventHandler,
    conf
};

use crate::graphics::GraphicsSystem;


pub struct Engine {
    graphics: GraphicsSystem
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
            }, |_ctx| { Box::new(Engine::new())});
    }

    fn new() -> Engine {
        Engine { 
            graphics: GraphicsSystem::new() 
        }
    }
}

impl EventHandler for Engine {
    fn update(&mut self, ctx: &mut miniquad::Context) {
        
        self.graphics.update(ctx);

    }

    fn draw(&mut self, _ctx: &mut miniquad::Context) {
        
    }
}