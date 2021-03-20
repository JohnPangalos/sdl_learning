extern crate sdl2;

use sdl2::surface::Surface;
use std::path::Path;

use std::time::Duration;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Example 1", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let img = Surface::load_bmp(Path::new("resources/hello_world.bmp"))?;
    let mut event_pump = sdl_context.event_pump()?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(img)
        .map_err(|e| e.to_string())?;

    canvas.clear();
    canvas.copy(&texture, None, None)?;
    canvas.present();

    'running: loop {
        for _event in event_pump.poll_iter() {}
        ::std::thread::sleep(Duration::new(2, 0));
        break 'running;
    }

    Ok(())
}
