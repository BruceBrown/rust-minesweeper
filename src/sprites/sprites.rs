use snafu::Snafu;
use std::rc::Rc;

use crate::config::Layout;
use crate::media_layer::Texture;

pub use super::{Point, Rect, SystemTime};

pub fn render_digit(
    digit: u64,
    bounding_box: Rect,
    context: &dyn RendererContext,
) -> Result<(), String> {
    let image = context.load_digit(digit)?;
    context.render_image(&image, None, bounding_box)?;
    Ok(())
}

// common enums
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GameState {
    Init,
    Playing,
    Win,
    Lose,
}

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum Error {
    #[snafu(display("The start time {:#?} is invalid: {:#?}", start, source))]
    StartTimeInvalid {
        start: std::time::SystemTime,
        //backtrace: Backtrace,
        source: std::time::SystemTimeError,
    },
    #[snafu(display("error: {}", desc))]
    Any { desc: String },
}

impl std::convert::From<String> for Error {
    fn from(source: String) -> Self {
        Error::Any { desc: source }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MouseEvent {
    pub x: i32,
    pub y: i32,
    pub mouse_btn: MouseButton,
}

pub trait MouseHandler {
    fn hit_test(&self, _event: &MouseEvent) -> bool {
        false
    }
    fn handle_event(&mut self, _event: &MouseEvent) {}
}

use crate::sprites::MessageExchange;
pub trait Sprite: Renderer + MouseHandler + MessageExchange {}

pub trait RendererContext {
    fn render_image(&self, texture: &Texture, src: Option<Rect>, dst: Rect) -> Result<(), String>;
    fn layout(&self) -> &Layout;
    fn load(&self, name: &str) -> Result<Rc<Texture>, String>;
    fn load_digit(&self, value: u64) -> Result<Rc<Texture>, String>;
    fn load_tile(&self, value: u64) -> Result<Rc<Texture>, String>;
}

pub trait Renderer {
    fn render(&self, _context: &dyn RendererContext) -> Result<(), Error> {
        Ok(())
    }
}
