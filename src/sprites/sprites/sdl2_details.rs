use std::rc::Rc;

pub use sdl2::rect::{Point, Rect};

use sdl2::video::WindowContext;

use crate::sprites::manager::{Texture, TextureManager};

pub struct Canvas {
    pub canvas: sdl2::render::WindowCanvas,
}

impl Canvas {
    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn copy<R1, R2>(&mut self, texture: &Texture, src: R1, dst: R2) -> Result<(), String>
    where
        R1: Into<Option<Rect>>,
        R2: Into<Option<Rect>>,
    {
        self.canvas.copy(&texture.texture, src, dst)
    }
}

pub struct TextureCache<'a> {
    pub texture_manager: TextureManager<'a, WindowContext>,
}

impl<'a> TextureCache<'a> {
    pub fn load(&mut self, name: &str) -> Result<Rc<Texture<'a>>, String> {
        self.texture_manager.load(name)
    }
}
