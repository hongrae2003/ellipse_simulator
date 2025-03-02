mod ball;

use std::{cmp::min, thread::sleep, time::{Duration, Instant}};

use ball::{draw_ball, Ball};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window, EventPump};

use crate::math::{ellipse::{draw_ellipse, mark_focus, Ellipse}, vec2::Vec2};

pub struct Game {
    event_pump: EventPump,
    canvas: Canvas<Window>,
    clock: Instant,
    running: bool,
    ellipse: Ellipse,
    ball: Ball
}

impl Game {
    const WINDOW_WIDTH: u32 = 800;
    const WINDOW_HEIGHT: u32 = 600;
    const FPS: u32 = 60;
    const DELTA_TIME: Duration = Duration::new(0, 1_000_000_000_u32 / Self::FPS);

    pub fn new() -> Game {
        let sdl_context = sdl2::init().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Ellipse Simulator", Self::WINDOW_WIDTH, Self::WINDOW_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let clock = Instant::now();
        let running = true;
        let ellipse = Ellipse::new(Vec2 {x: Self::WINDOW_WIDTH as f32 / 2.0, y: Self::WINDOW_HEIGHT as f32 / 2.0}, 200.0, 150.0);
        let ball = Ball::new(ellipse.focus().0);
        Game { event_pump, canvas, clock, running, ellipse, ball }
    }
    
    pub fn run(&mut self) {
        while self.running {
            self.input();
            self.update();
            self.output();
        }
    }

    pub fn quit(&mut self) {

    }

    fn input(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.running = false;
                },
                _ => {}
            }
        }
    }

    fn tick(&mut self) -> f32 {
        let mut dt = Instant::now() - self.clock;
        if dt < Self::DELTA_TIME {
            sleep(Self::DELTA_TIME - dt);
        }
        dt = Instant::now() - self.clock;
        self.clock = Instant::now();
        min(dt, Duration::new(0, 5_000_000_0u32)).as_secs_f32()
    }
    
    fn update(&mut self) {
        let dt = self.tick();
        // println!("{}", 1.0 / dt);

        self.ball.update(dt);
    }

    fn output(&mut self) {
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();

        draw_ellipse(&mut self.canvas, &self.ellipse);
        mark_focus(&mut self.canvas, &self.ellipse);
        draw_ball(&mut self.canvas, &self.ball);

        self.canvas.present();
    }
}
