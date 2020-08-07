extern crate minesweeperlib;

use minesweeperlib::Error;

#[cfg(not(any(
    feature = "media_layer_sdl2",
    feature = "media_layer_text",
    feature = "media_layer_wasm"
)))]
std::compile_error!("Either feature \"media_layer_sdl2\", feature \"media_layer_text\", or \"media_layer_wasm\" must be enabled for this crate.");

#[cfg(feature = "media_layer_sdl2")]
#[cfg(any(feature = "media_layer_text", feature = "media_layer_wasm"))]
std::compile_error!(
    "Either feature \"media_layer_sdl2\", should not be configured with other media_layers."
);

#[cfg(feature = "media_layer_text")]
#[cfg(any(feature = "media_layer_sdl2", feature = "media_layer_wasm"))]
std::compile_error!(
    "Either feature \"media_layer_text\", should not be configured with other media_layers."
);

#[cfg(feature = "media_layer_wasm")]
#[cfg(any(feature = "media_layer_sdl2", feature = "media_layer_text"))]
std::compile_error!(
    "Either feature \"media_layer_wasm\", should not be configured with other media_layers."
);

#[cfg(feature = "media_layer_sdl2")]
mod sdl2_minesweeper {
    use std::cell::RefCell;
    use std::rc::Rc;

    extern crate sdl2;
    use sdl2::event::Event;
    use sdl2::image::InitFlag;
    use sdl2::keyboard::Keycode;

    extern crate minesweeperlib;
    use crate::minesweeperlib::MessageExchange;
    use crate::minesweeperlib::{ChannelMessage, MouseButton, MouseEvent, MouseHandler};
    use crate::minesweeperlib::{
        Error, Game, Layout, Rect, Renderer, RendererContext, ResourceContainer, Texture,
        TextureManager,
    };

    struct RenderingContext {
        pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        pub canvas: RefCell<sdl2::render::WindowCanvas>,
        pub layout: Layout,
        pub texture_manager: TextureManager,
        pub digits: Vec<String>,
        pub tiles: Vec<String>,
    }

    impl RendererContext for RenderingContext {
        fn render_image(
            &self,
            texture: &Texture,
            src: Option<Rect>,
            dst: Rect,
        ) -> Result<(), String> {
            let taken = texture.get_image_data();
            match taken {
                Some(cache) => {
                    let _result = self.canvas.borrow_mut().copy(&cache.texture, src, dst);
                    texture.set_image_data(Some(cache));
                }
                None => match texture.raw_bytes {
                    Some(png) => {
                        let loader = sdl2::rwops::RWops::from_bytes(png)?;
                        let ops = &loader as &dyn sdl2::image::ImageRWops;
                        let surface = ops.load_png()?;
                        let image = self
                            .texture_creator
                            .create_texture_from_surface(&surface)
                            .unwrap();
                        let _result = self.canvas.borrow_mut().copy(&image, src, dst);
                        texture.set_image_data(Some(ResourceContainer::new(image)));
                    }
                    None => {}
                },
            }
            Ok(())
        }

        fn layout(&self) -> &Layout {
            &self.layout
        }

        fn load(&self, name: &str) -> Result<Rc<Texture>, String> {
            self.texture_manager.load(name)
        }

        fn load_digit(&self, value: u64) -> Result<Rc<Texture>, String> {
            let name = &self.digits[value as usize];
            let image_name = format!("digit_{}", name);
            self.load(&image_name)
        }

        fn load_tile(&self, value: u64) -> Result<Rc<Texture>, String> {
            let name = &self.tiles[value as usize];
            let image_name = format!("tile_{}", name);
            self.load(&image_name)
        }
        fn end_rendering(&self) {
            self.canvas.borrow_mut().present();
        }
    }

    pub struct Minesweeper {
        pub layout: Layout,
        pub game: Game,
        game_sender: std::sync::mpsc::Sender<ChannelMessage>,
        rendering_context: Rc<Box<dyn RendererContext>>,
    }

    impl Minesweeper {
        fn new(canvas: sdl2::render::WindowCanvas) -> Self {
            let layout = Layout::new();
            let texture_creator = canvas.texture_creator();
            let texture_manager = ResourceContainer::new_texture_manager();
            let canvas = RefCell::new(canvas);
            let digits = [
                "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];
            let tiles = [
                "none", "one", "two", "three", "four", "five", "six", "seven", "eight",
            ];
            let rendering_context = RenderingContext {
                texture_creator: texture_creator,
                canvas: canvas,
                layout: layout,
                texture_manager: texture_manager,
                digits: digits.iter().map(|s| s.to_string()).collect(),
                tiles: tiles.iter().map(|s| s.to_string()).collect(),
            };
            let context = Rc::new(Box::new(rendering_context) as Box<dyn RendererContext>);
            let game = Game::new(layout);
            let sender = game.get_sender();
            Self {
                layout: layout,
                game: game,
                game_sender: sender,
                rendering_context: context,
            }
        }
        
        fn render(&mut self) {
            let message = ChannelMessage::Render(Rc::clone(&self.rendering_context));
            self.game_sender.send(message).unwrap();
            while self.game.pull() > 0 {}
            self.rendering_context.end_rendering();
        }

        fn handle_event(&mut self, event: MouseEventData) {
            let message = ChannelMessage::MouseEvent(event);
            self.game_sender.send(message).unwrap();
            // since we're not running threads on the channels, perform a complete pull
            while self.game.pull() > 0 {}
        }
    }

    pub fn main() -> Result<(), Error> {
        let layout = Rc::new(Layout::new());

        // init the video subsystem and creat the game window, even in text mode we do this...
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

        let window = video_subsystem
            .window("minesweeper", layout.width(), layout.height())
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas: sdl2::render::WindowCanvas = window
            .into_canvas()
            .software()
            .build()
            .map_err(|e| e.to_string())?;

        let mut minesweeper = Minesweeper::new(canvas);
        minesweeper.render();
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
                        let mouse_event = MouseEventData {
                            x: x,
                            y: y,
                            mouse_btn: match mouse_btn {
                                sdl2::mouse::MouseButton::Left => minesweeperlib::MouseButton::Left,
                                sdl2::mouse::MouseButton::Right => {
                                    minesweeperlib::MouseButton::Right
                                }
                                _ => MouseButton::Middle,
                            },
                        };
                        minesweeper.handle_event(mouse_event);
                        minesweeper.render();
                    }
                    _ => (),
                },
                None => {
                    minesweeper.render();
                }
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    #[cfg(feature = "media_layer_sdl2")]
    let _result = sdl2_minesweeper::main();

    Ok(())
}
