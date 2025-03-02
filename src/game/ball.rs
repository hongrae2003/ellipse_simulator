use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window};

use crate::math::{circle::Circle, vec2::Vec2};

pub struct Ball {
    shape: Circle,
    vel: Vec2, // velocity vector must be unit vector
}

impl Ball {
    const RADIUS: f32 = 8.0;
    const VELOCITY: Vec2 = Vec2 { x: 0.6, y: 0.8 };
    const SPEED: f32 = 100.0;
    
    pub fn new(pos: Vec2) -> Ball {
        Ball { shape: Circle{center: pos, rad: Self::RADIUS}, vel: Self::VELOCITY }
    }

    pub fn update(&mut self, dt: f32) {
        self.shape.center += self.vel * Self::SPEED * dt;
    }
}

pub fn draw_ball(canvas: &mut Canvas<Window>, ball: &Ball) {
    canvas.filled_circle(ball.shape.center.x as i16, ball.shape.center.y as i16, ball.shape.rad as i16, Color::GRAY).unwrap();
}