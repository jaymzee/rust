extern crate sdl2;

use sdl2::gfx::primitives::DrawRenderer;
//use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init()
        .expect("initialize sdl context");
    let video_subsystem = sdl_context.video()
        .expect("initialize video subsystem");
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered().build()
        .expect("create window");
    let mut canvas = window.into_canvas().build()
        .expect("create canvas");
    let mut event_pump = sdl_context.event_pump()
        .expect("create event pump");
    let mut i: u8 = 0;

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        canvas.line(i as i16, 0, 500, 500, Color::RGB(255, 255, 0)).unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
