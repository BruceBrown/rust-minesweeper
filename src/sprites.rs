pub mod sprites;
pub use sprites::render_digit;
pub use sprites::StartTimeInvalid;

pub use sprites::Error;

pub use sprites::{FlagStateListener, GameState, GameStateListener, TileListener};
pub use sprites::{MouseButton, MouseEvent, MouseHandler, Renderer, RendererContext, Sprite};
pub use sprites::{TraitWrapper, WeakTrait, WeakTraitWrapper};

pub mod background;
pub use background::Background;

pub mod button;
pub use button::Button;

pub mod flagcounter;
pub use flagcounter::FlagCounter;

pub mod grid;
pub use grid::Grid;

pub mod timecounter;
pub use timecounter::TimeCounter;

pub mod tile;
pub use tile::{Tile, TileSprite};

#[cfg(feature = "media_layer_wasm")]
mod util;

#[cfg(feature = "media_layer_wasm")]
pub use util::{Point, Rect, SystemTime};

#[cfg(feature = "media_layer_sdl2")]
pub use sdl2::rect::{Point, Rect};

#[cfg(feature = "media_layer_sdl2")]
pub type SystemTime = std::time::SystemTime;
