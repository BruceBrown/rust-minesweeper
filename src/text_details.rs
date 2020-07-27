use std::rc::Rc;

use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;

extern crate minesweeperlib;
use crate::minesweeperlib::Game;
use crate::minesweeperlib::Layout;
use crate::minesweeperlib::StringCreator;
use crate::minesweeperlib::{Canvas, TextureCache, TextureManager};
use crate::minesweeperlib::{Error, MouseEvent, RenderContext};
use crate::minesweeperlib::{MouseHandler, Renderer};

pub fn main() -> Result<(), Error> {
    let layout = Rc::new(Layout::new());

    // init the video subsystem and creat the game window, even in text mode we do this...
    let sdl_context = sdl2::init()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let texture_creator = StringCreator {};
    let texture_manager = TextureManager::new(&texture_creator);
    let canvas = Canvas {};

    let digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let tiles = [
        "none", "one", "two", "three", "four", "five", "six", "seven", "eight",
    ];
    let mut render_context = RenderContext {
        layout: Rc::clone(&layout),
        canvas: canvas,
        texture_manager: TextureCache { texture_manager },
        digits: digits,
        tiles: tiles,
    };

    let game = Game::new(&layout);
    game.render(&mut render_context)?;
    render_context.canvas.present();

    let mut event_pump: sdl2::EventPump = sdl_context.event_pump()?;
    'running: loop {
        match event_pump.wait_event_timeout(100) {
            Some(event) => match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'running,
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    let mouse_event = MouseEvent {
                        x: x,
                        y: y,
                        mouse_btn: mouse_btn,
                    };
                    game.handle_event(&mouse_event);
                }
                _ => (),
            },
            None => {
                game.render(&mut render_context)?;
                render_context.canvas.present();
            }
        }
    }

    Ok(())
}