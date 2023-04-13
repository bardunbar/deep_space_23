
#[derive(Debug, Clone, Copy)]
pub struct Rect2D {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect2D {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect2D {
        Rect2D { x, y, w, h }
    }
}