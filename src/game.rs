use std::{cmp::min, thread::sleep, time::{Duration, Instant}};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window, EventPump};

pub struct Game {
    event_pump: EventPump,
    canvas: Canvas<Window>,
    clock: Instant,
    running: bool,
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
        let window = video_subsystem.window("Hello SDL", Self::WINDOW_WIDTH, Self::WINDOW_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let clock = Instant::now();
        let running = true;
        Game { event_pump, canvas, clock, running }
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
    }

    fn output(&mut self) {
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();
        self.canvas.present();
    }
}
