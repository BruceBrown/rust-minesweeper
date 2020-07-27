extern crate minesweeperlib;
use minesweeperlib::Error;

#[cfg(not(any(feature = "media_layer_sdl2", feature = "media_layer_text")))]
std::compile_error!("Either feature \"media_layer_sdl2\" or feature \"media_layer_text\" must be enabled for this crate.");

#[cfg(feature = "media_layer_sdl2")]
#[cfg(feature = "media_layer_text")]
std::compile_error!("Either feature \"media_layer_sdl2\" or feature \"media_layer_text\", but not both, must be enabled for this crate.");

#[cfg(feature = "media_layer_sdl2")]
mod sdl2_details;

#[cfg(feature = "media_layer_text")]
mod text_details;

#[cfg(feature = "media_layer_text")]
mod custom {

    use std::rc::Rc;

    use sdl2::event::Event;
    use sdl2::image::InitFlag;
    use sdl2::keyboard::Keycode;

    extern crate minesweeperlib;
    use crate::minesweeperlib::Game;
    use crate::minesweeperlib::Layout;
    use crate::minesweeperlib::Rect;
    use crate::minesweeperlib::StringCreator;
    use crate::minesweeperlib::{Error, MouseEvent, RendererContext};
    use crate::minesweeperlib::{MouseHandler, Renderer};
    use crate::minesweeperlib::{Texture, TextureCache, TextureManager};

    pub struct MyCanvas {}
    impl MyCanvas {
        pub fn present(&mut self) {}
        pub fn copy<R1, R2>(&mut self, texture: &Texture, _src: R1, _dst: R2) -> Result<(), String>
        where
            R1: Into<Option<Rect>>,
            R2: Into<Option<Rect>>,
        {
            match _dst.into() {
                Some(rect) => println!("{} at {:#?}", texture.texture, rect),
                None => println!("{}", texture.texture),
            }

            Ok(())
        }
    }

    pub struct MyRenderContext<'a> {
        pub layout: Rc<Layout>,
        pub canvas: MyCanvas,
        pub texture_manager: TextureCache<'a>,
        pub digits: [&'a str; 10],
        pub tiles: [&'a str; 9],
    }

    impl<'a> MyRenderContext<'a> {
        fn new(layout: &Rc<Layout>, texture_cache: TextureCache<'a>) -> Self {
            let digits = [
                "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];
            let tiles = [
                "none", "one", "two", "three", "four", "five", "six", "seven", "eight",
            ];
            MyRenderContext {
                layout: Rc::clone(&layout),
                canvas: MyCanvas {},
                texture_manager: texture_cache,
                digits: digits,
                tiles: tiles,
            }
        }
    }

    impl<'a> RendererContext<'a> for MyRenderContext<'a> {
        fn render_image(
            &mut self,
            texture: &Texture<'a>,
            src: Option<Rect>,
            dst: Rect,
        ) -> Result<(), String> {
            self.canvas.copy(texture, src, dst)
        }

        fn layout(&mut self) -> &Layout {
            &self.layout
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

    pub fn main() -> Result<(), Error> {
        let layout = Rc::new(Layout::new());

        // init the video subsystem and creat the game window, even in text mode we do this...
        let sdl_context = sdl2::init()?;
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

        let texture_creator = StringCreator {};
        let texture_manager = TextureManager::new(&texture_creator);
        let texture_cache = TextureCache { texture_manager };

        let mut render_context = MyRenderContext::new(&layout, texture_cache);

        let game = Game::new(&layout);
        game.render(&mut render_context)?;
        render_context.canvas.present();
        /*
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
        */
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    #[cfg(feature = "media_layer_sdl2")]
    let result = sdl2_details::main();

    #[cfg(feature = "media_layer_text")]
    let result = custom::main();

    result
}
