extern crate sdl2;
use crate::sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
// use sdl2::surface::{Surface, SurfaceRef};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::WindowContext;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

struct LTexture<'a> {
    texture: Texture<'a>,
    width: u32,
    height: u32,
}

impl<'a> LTexture<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
        path: String,
    ) -> Result<Self, String> {
        let texture = texture_creator.load_texture(path)?;
        Ok(Self {
            width: texture.query().width,
            height: texture.query().height,
            texture: texture,
        })
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Example 10", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    let background = LTexture::new(&texture_creator, "resources/background.png".to_string())?;
    let foo = LTexture::new(&texture_creator, "resources/foo.png".to_string())?;

    canvas.set_draw_color(Color::WHITE);
    canvas.clear();

    canvas.set_viewport(Rect::new(0, 0, WIDTH, HEIGHT));
    canvas.copy(&background.texture, None, None)?;

    canvas.set_viewport(Rect::new(240, 190, foo.width, foo.height));
    canvas.copy(&foo.texture, None, None)?;
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
