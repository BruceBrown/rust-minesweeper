mod config;
mod game;
mod sprites;

use sprites::{Error, MouseEvent, MouseHandler, Renderer};
use std::rc::Rc;

extern crate sdl2;
extern crate snafu;

use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;

fn main() -> Result<(), Error> {
    let layout = Rc::new(config::Layout::new());

    // init the video subsystem and creat the game window
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("minesweeper", layout.width(), layout.height())
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let texture_manager = sprites::TextureManager::new(&texture_creator);
    let digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let tiles = [
        "none", "one", "two", "three", "four", "five", "six", "seven", "eight",
    ];
    let mut render_context = sprites::RenderContext {
        layout: Rc::clone(&layout),
        canvas: canvas,
        texture_manager: texture_manager,
        digits: digits,
        tiles: tiles,
    };

    let game = game::Game::new(&layout);
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
