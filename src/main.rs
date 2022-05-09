extern crate sdl2;
use std::f64::consts::PI;
use std::path::Path;
use std::time::Duration;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;

const CYCLE: i32 = 5000;

struct Powder {
    origin: Point,
    angle: f64,
    dist: usize,
    color: Color,
}

impl Powder {
    pub fn new(origin: Point, min_dist: usize, max_dist: usize) -> Powder {
        let mut rng = rand::thread_rng();

        let color_idx: usize = rng.gen_range(0..3);
        let color = match color_idx {
            0 => Color::RGBA(255, 0, 0, 255),
            1 => Color::RGBA(255, 255, 0, 255),
            2 => Color::RGBA(0, 0, 255, 255),
            _ => panic!(),
        };

        Powder {
            origin,
            angle: rng.gen_range(0..360) as f64,
            dist: rng.gen_range(min_dist..(max_dist + 1)),
            color,
        }
    }

    pub fn cur_pos(&self, frac: f64) -> Point {
        let dist = (self.dist as f64) * frac;

        let ang_radian = self.angle * PI / 180.0;

        let new_x = ((self.origin.x() as f64) + (dist * f64::cos(ang_radian))).round() as i32;
        let new_y = ((self.origin.y() as f64) + (dist * f64::sin(ang_radian))).round() as i32;

        Point::new(new_x, new_y)
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("SDL2", 1024, 1024)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
    canvas.clear();
    canvas.present();

    let timer = sdl_context.timer()?;

    let mut event_pump = sdl_context.event_pump()?;

    let mid_point = Point::new(512, 512);

    let mut powders = Vec::new();
    for _ in 0..200 {
        powders.push(Powder::new(mid_point, 100, 512));
    }

    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                }
                _ => {}
            }
        }

        let ticks = timer.ticks() as i32;

        let ticks_in_cycle = ticks % CYCLE;

        let frac = (ticks_in_cycle as f64) / (CYCLE as f64);

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for p in powders.iter() {
            let cp = p.cur_pos(frac);

            canvas.set_draw_color(p.color);
            canvas.draw_point(cp)?;
        }

        canvas.present();
    }

    Ok(())
}
