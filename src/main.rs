extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture};
use sdl2::surface::Surface;
use sdl2::video::Window;
use std::path::Path;
use std::time::Duration;

fn load_texture<'a>(canvas: Canvas<Window>, path: &'a str) -> &Texture<'a> {
    let img = Surface::load_bmp(Path::new(path)).unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(img).unwrap();
    return &texture;
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 640, 480)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let press_img = Surface::load_bmp(Path::new("resources/press.bmp")).unwrap();
    let up_img = Surface::load_bmp(Path::new("resources/up.bmp")).unwrap();
    let left_img = Surface::load_bmp(Path::new("resources/left.bmp")).unwrap();
    let down_img = Surface::load_bmp(Path::new("resources/down.bmp")).unwrap();
    let right_img = Surface::load_bmp(Path::new("resources/right.bmp")).unwrap();

    let texture_creator = canvas.texture_creator();
    let press_texture = texture_creator
        .create_texture_from_surface(press_img)
        .unwrap();
    let up_texture = texture_creator.create_texture_from_surface(up_img).unwrap();
    let left_texture = texture_creator
        .create_texture_from_surface(left_img)
        .unwrap();
    let down_texture = texture_creator
        .create_texture_from_surface(down_img)
        .unwrap();
    let right_texture = texture_creator
        .create_texture_from_surface(right_img)
        .unwrap();

    canvas.clear();
    canvas.copy(&press_texture, None, None).unwrap();
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
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    canvas.clear();
                    canvas.copy(&up_texture, None, None).unwrap();
                    canvas.present();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    canvas.clear();
                    canvas.copy(&left_texture, None, None).unwrap();
                    canvas.present();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    canvas.clear();
                    canvas.copy(&down_texture, None, None).unwrap();
                    canvas.present();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    canvas.clear();
                    canvas.copy(&right_texture, None, None).unwrap();
                    canvas.present();
                }
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
