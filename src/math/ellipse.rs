use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window};

use super::vec2::Vec2;

// x^ 2 / a^2 + y^2 / b^2 = 1
pub struct Ellipse {
    center: Vec2,
    a: f32,
    b: f32
}

impl Ellipse {
    pub fn new(center: Vec2, a: f32, b: f32) -> Ellipse {
        Ellipse {center, a, b}
    }

    pub fn focus(&self) -> (Vec2, Vec2) {
        if self.a >= self.b {
            let c = Vec2 {x: f32::sqrt(self.a * self.a - self.b * self.b), y: 0.0};
            (self.center + c, self.center - c)
        } else {
            let c = Vec2 {x: 0.0, y: f32::sqrt(self.b * self.b - self.a * self.a)};
            (self.center + c, self.center - c)
        }
    }
}

pub fn draw_ellipse(canvas: &mut Canvas<Window>, ellipse: &Ellipse) {
    canvas.aa_ellipse(ellipse.center.x as i16, ellipse.center.y as i16, ellipse.a as i16, ellipse.b as i16, Color::BLACK).unwrap();
}

pub fn mark_focus(canvas: &mut Canvas<Window>, ellipse: &Ellipse) {
    let f = ellipse.focus();
    canvas.filled_circle(f.0.x as i16, f.0.y as i16, 2, Color::BLACK).unwrap();
    canvas.filled_circle(f.1.x as i16, f.1.y as i16, 2, Color::BLACK).unwrap();
}
