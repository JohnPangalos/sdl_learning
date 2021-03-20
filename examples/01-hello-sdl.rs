extern crate sdl2;

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

    let mut _canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for _event in event_pump.poll_iter() {}
        ::std::thread::sleep(Duration::new(2, 0));
        break 'running;
    }

    Ok(())
}
