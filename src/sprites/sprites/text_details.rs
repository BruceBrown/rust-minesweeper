use std::option::Option;
use std::rc::Rc;

use crate::sprites::manager::{Texture, TextureManager};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rect {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}
impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    /// Returns the x-position of the left side of this rectangle.
    pub fn left(&self) -> i32 {
        self.x
    }

    /// Returns the x-position of the right side of this rectangle.
    pub fn right(&self) -> i32 {
        self.x + self.width as i32
    }

    /// Returns the y-position of the top side of this rectangle.
    pub fn top(&self) -> i32 {
        self.y
    }

    /// Returns the y-position of the bottom side of this rectangle.
    pub fn bottom(&self) -> i32 {
        self.y + self.height as i32
    }

    pub fn contains_point<P>(&self, point: P) -> bool
    where
        P: Into<(i32, i32)>,
    {
        let (x, y) = point.into();
        let inside_x = x >= self.left() && x < self.right();
        inside_x && (y >= self.top() && y < self.bottom())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Point {
        Point::new(x, y)
    }
}

pub struct Canvas {}
impl Canvas {
    pub fn present(&mut self) {}
    pub fn copy<R1, R2>(&mut self, texture: &Texture, _src: R1, _dst: R2) -> Result<(), String>
    where
        R1: Into<Option<Rect>>,
        R2: Into<Option<Rect>>,
    {
        println!("{}", texture.texture);
        Ok(())
    }
}

pub struct TextureCache<'a> {
    pub texture_manager: TextureManager<'a>,
}

impl<'a> TextureCache<'a> {
    pub fn load(&mut self, name: &str) -> Result<Rc<Texture<'a>>, String> {
        self.texture_manager.load(name)
    }
}
