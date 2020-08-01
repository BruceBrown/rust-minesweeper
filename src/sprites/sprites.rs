use snafu::Snafu;
use std::rc::{Rc, Weak};

use crate::config::Layout;
use crate::media_layer::Texture;

pub use super::{Point, Rect, SystemTime};

pub type TraitWrapper<T> = Box<Rc<T>>;
pub type WeakTraitWrapper<T> = Box<Weak<T>>;
pub type WeakTrait<T> = Weak<T>;

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
#[derive(Debug, PartialEq, Clone, Copy)]
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

pub trait GameStateListener {
    fn game_state_changed(&self, state: GameState);
}

pub trait FlagStateListener {
    fn flag_state_changed(&self, exhausted: bool);
}

pub trait TileListener {
    fn reveal(&self, _is_mine: bool, _has_adjacent_mines: bool) {}
    fn clear(&self) {}
    fn flag(&self, _is_flagged: bool) {}
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
    fn handle_event(&self, _event: &MouseEvent) {}
}

pub trait Sprite: Renderer + MouseHandler {}

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
