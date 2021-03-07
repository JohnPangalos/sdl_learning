extern crate sdl2;
use crate::sdl2::image::LoadTexture;

use sdl2::render::Texture;
// use sdl2::surface::{Surface, SurfaceRef};
// use sdl2::video::WindowContext;

struct LTexture<'a> {
    texture: Texture<'a>,
    width: i32,
    height: i32,
}

impl LTexture<'_> {
    pub fn new<'a>(width: i32, height: i32, texture: Texture) -> Self {
        Self {
            width: width,
            height: height,
            texture: texture,
        }
    }
}

pub fn main() -> Result<(), String> {
    Ok(())
}
