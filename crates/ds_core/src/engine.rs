// use miniquad::*;

use miniquad::{
    EventHandler,
    conf
};


pub struct Engine {

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
            }, |_ctx| { Box::new(Engine{})});
    }
}

impl EventHandler for Engine {
    fn update(&mut self, _ctx: &mut miniquad::Context) {
        
    }

    fn draw(&mut self, _ctx: &mut miniquad::Context) {
        
    }
}