use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window};

use crate::math::{circle::Circle, vec2::Vec2};

pub struct Ball {
    shape: Circle,
}

impl Ball {
    const RADIUS: f32 = 10.0;
    pub fn new(pos: Vec2) -> Ball {
        Ball { shape: Circle{center: pos, rad: Self::RADIUS} }
    }
}

pub fn draw_ball(canvas: &mut Canvas<Window>, ball: &Ball) {
    canvas.filled_circle(ball.shape.center.x as i16, ball.shape.center.y as i16, ball.shape.rad as i16, Color::BLUE).unwrap();
}