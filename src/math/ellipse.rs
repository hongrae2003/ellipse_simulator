use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window};

use super::vec2::Vec2;

// x^ 2 / a^2 + y^2 / b^2 = 1
pub struct Ellipse {
    pos: Vec2,
    a: f32,
    b: f32
}

impl Ellipse {
    pub fn new(pos: Vec2, a: f32, b: f32) -> Ellipse {
        Ellipse {pos, a, b}
    }
}

pub fn draw_ellipse(canvas: &mut Canvas<Window>, ellipse: &Ellipse) {
    canvas.aa_ellipse(ellipse.pos.x as i16, ellipse.pos.y as i16, ellipse.a as i16, ellipse.b as i16, Color::BLACK).unwrap();
}
