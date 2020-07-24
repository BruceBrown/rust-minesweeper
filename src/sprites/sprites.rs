use sdl2::render::{Texture, WindowCanvas};
use snafu::Snafu;
use std::rc::Rc;

use crate::config::Layout;

use std::rc::Weak;
pub type TraitWrapper<T> = Box<Rc<T>>;
pub type WeakTraitWrapper<T> = Box<Weak<T>>;
pub type WeakTrait<T> = Weak<T>;

pub fn render_digit(
    digit: u64,
    bounding_box: sdl2::rect::Rect,
    context: &mut dyn RendererContext,
) -> Result<(), String> {
    let image = context.load_digit(digit)?;
    context.canvas().copy(&image, None, bounding_box)?;
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

// common traits

pub trait RendererContext<'a> {
    fn layout(&mut self) -> &Layout;
    fn canvas(&mut self) -> &mut WindowCanvas;
    fn load(&mut self, name: &str) -> Result<Rc<Texture<'a>>, String>;
    fn load_digit(&mut self, value: u64) -> Result<Rc<Texture<'a>>, String>;
    fn load_tile(&mut self, value: u64) -> Result<Rc<Texture<'a>>, String>;
}

pub trait Renderer<'a> {
    fn render(&self, _context: &mut dyn RendererContext) -> Result<(), Error> {
        Ok(())
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

pub struct MouseEvent {
    pub x: i32,
    pub y: i32,
    pub mouse_btn: sdl2::mouse::MouseButton,
}

pub trait MouseHandler {
    fn hit_test(&self, _event: &MouseEvent) -> bool {
        false
    }
    fn handle_event(&self, _event: &MouseEvent) {}
}

pub trait Sprite<'a>: Renderer<'a> + MouseHandler {}

// the rendering context
use super::manager::TextureManager;
use sdl2::video::WindowContext;

pub struct RenderContext<'a> {
    pub layout: Rc<Layout>,
    pub canvas: WindowCanvas,
    pub texture_manager: TextureManager<'a, WindowContext>,
    pub digits: [&'a str; 10],
    pub tiles: [&'a str; 9],
}

impl<'a> RendererContext<'a> for RenderContext<'a> {
    fn layout(&mut self) -> &Layout {
        &self.layout
    }

    fn canvas(&mut self) -> &mut WindowCanvas {
        &mut self.canvas
    }

    fn load(&mut self, name: &str) -> Result<Rc<Texture<'a>>, String> {
        self.texture_manager.load(name)
    }

    fn load_digit(&mut self, value: u64) -> Result<Rc<Texture<'a>>, String> {
        let name = self.digits[value as usize];
        let image_name = format!("digit_{}", name);
        self.load(&image_name)
    }

    fn load_tile(&mut self, value: u64) -> Result<Rc<Texture<'a>>, String> {
        let name = self.tiles[value as usize];
        let image_name = format!("tile_{}", name);
        self.load(&image_name)
    }
}
