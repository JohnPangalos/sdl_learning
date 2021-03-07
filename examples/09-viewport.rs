extern crate sdl2;
use crate::sdl2::image::LoadTexture;

use sdl2::event::Event;
use sdl2::image::{init, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::time::Duration;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let _img_ref = init(InitFlag::PNG).unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("resources/viewport.png")?;
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();

    canvas.set_viewport(Rect::new(0, 0, WIDTH / 2, HEIGHT / 2));
    canvas.copy(&texture, None, None)?;

    canvas.set_viewport(Rect::new((WIDTH / 2) as i32, 0, WIDTH / 2, HEIGHT / 2));
    canvas.copy(&texture, None, None)?;

    canvas.set_viewport(Rect::new(0, (HEIGHT / 2) as i32, WIDTH, HEIGHT / 2));
    canvas.copy(&texture, None, None)?;
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
