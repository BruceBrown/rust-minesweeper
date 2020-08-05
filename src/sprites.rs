pub mod sprites;
pub use sprites::render_digit;
pub use sprites::StartTimeInvalid;

pub use sprites::Error;

pub mod channel_wiring;

pub mod message_exchange;
pub use message_exchange::{ChannelMessage, ChannelWiring, Exchange, MessageExchange};

pub use sprites::GameState;
pub use sprites::{MouseButton, MouseEvent, MouseHandler, Renderer, RendererContext, Sprite};

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
pub use tile::Tile;

#[cfg(feature = "media_layer_wasm")]
mod util;

#[cfg(feature = "media_layer_wasm")]
pub use util::{Point, Rect, SystemTime};

#[cfg(feature = "media_layer_sdl2")]
pub use sdl2::rect::{Point, Rect};

#[cfg(feature = "media_layer_sdl2")]
pub type SystemTime = std::time::SystemTime;
