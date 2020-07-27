//#![feature(proc_macro_diagnostic)]
mod config;
mod game;
mod sprites;

/**
 * The library implements most of the game logic. There is very little that needs to be exposed to the from end.
 * A rendering context is passed around which is used in generating the UI updates.
 */
pub use crate::config::Layout;
pub use crate::game::Game;
pub use crate::sprites::manager::TextureManager;
pub use crate::sprites::{Canvas, TextureCache};
pub use crate::sprites::{Error, MouseEvent, MouseHandler};
pub use crate::sprites::{RenderContext, Renderer};

#[cfg(feature = "media_layer_text")]
pub use crate::sprites::manager::StringCreator;

// need these if implementing render
pub use crate::sprites::Rect;
pub use crate::sprites::RendererContext;
pub use crate::sprites::Texture;

//Eventually, we should be able to put out a diagnostic
/*
use proc_macro::Diagnostic;

Diagnostic::new()
    .note("building for text rendering")
    .emit();
*/
