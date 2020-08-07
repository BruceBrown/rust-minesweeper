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

mod config;
mod game;
mod media_layer;
mod sprites;

/**
 * The library contains most of the game logic. There is very little that needs to be exposed to the from end.
 * A rendering context is passed around which is used in generating the UI updates.
 */
pub use crate::config::Layout;
pub use crate::game::Game;
pub use crate::sprites::{Error, MouseButton, MouseEventData};
pub use crate::sprites::{Renderer, RendererContext};

pub use crate::media_layer::{ResourceContainer, Texture, TextureManager};

pub use crate::sprites::Rect;
pub use crate::sprites::{ChannelMessage, MessageExchange};

#[cfg(feature = "media_layer_text")]
mod text {
    use super::*;
    pub struct Minesweeper {
        pub layout: Rc<Layout>,
        pub game: Game,
        pub texture_manager: TextureManager,
        pub digits: Vec<String>,
        pub tiles: Vec<String>,
    }
    impl Minesweeper {
        fn new() -> Self {
            let layout = Rc::new(config::BEGINNER_LAYOUT);

            let digits = [
                "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];
            let tiles = [
                "none", "one", "two", "three", "four", "five", "six", "seven", "eight",
            ];
            Self {
                layout: layout.clone(),
                game: Game::new(&layout),
                texture_manager: TextureManager::new(),
                digits: digits.iter().map(|s| s.to_string()).collect(),
                tiles: tiles.iter().map(|s| s.to_string()).collect(),
            }
        }
    }
}

#[cfg(feature = "media_layer_wasm")]
mod utils;

#[cfg(feature = "media_layer_wasm")]
mod wasm {
    extern crate web_sys;

    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    use super::*;
    use wasm_bindgen::prelude::*;

    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    #[cfg(feature = "wee_alloc")]
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

    use media_layer::TextureManager;

    use std::cell::RefCell;
    use std::rc::Rc;

    // A macro to provide `println!(..)`-style syntax for `console.log` logging.
    macro_rules! log {
        ( $( $t:tt )* ) => {
            web_sys::console::log_1(&format!( $( $t )* ).into());
        }
    }

    struct RenderingContext {
        canvas: Rc<web_sys::CanvasRenderingContext2d>,
        layout: Layout,
         texture_manager: TextureManager,
        digits: Vec<String>,
        tiles: Vec<String>,
    }

    impl RenderingContext {

        fn render_from_cache(&self, resource: &ResourceContainer, left: i32, top: i32) {
            self.canvas
                .put_image_data(resource.get_image_data(), left as f64, top as f64);
        }

    }
    impl RendererContext for RenderingContext {
        fn render_image(
            &self,
            texture: &Texture,
            _src: Option<Rect>,
            dst: Rect,
        ) -> Result<(), String> {
            match texture.image_data.take() {
                Some(web_image) => {
                    self.render_from_cache(&web_image, dst.left(), dst.top());
                    texture.image_data.set(Some(web_image));
                }
                None => match texture.raw_bytes() {
                    Some(png) => {
                        let resource = ResourceContainer::new(png, dst.width(), dst.height());
                        self.render_from_cache(&resource, dst.left(), dst.top());
                        texture.image_data.set(Some(resource));
                    }
                    None => log!("unable to load png from memory"),
                },
            };
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

        fn end_rendering(&self) {}
    }

    pub struct Minesweeper {
        layout: Layout,
        game: RefCell<Game>,
        game_sender: std::sync::mpsc::Sender<ChannelMessage>,
        rendering_context: Rc<Box<dyn RendererContext>>,
    }

    impl Minesweeper {
        pub fn new(canvas: &Rc<web_sys::CanvasRenderingContext2d>) -> Self {
            let layout = config::BEGINNER_LAYOUT;

            let digits = [
                "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];
            let tiles = [
                "none", "one", "two", "three", "four", "five", "six", "seven", "eight",
            ];
            

            let rendering_context = RenderingContext {
                canvas: canvas.clone(),
                layout: layout,
                texture_manager: ResourceContainer::new_texture_manager(),
                digits: digits.iter().map(|s| s.to_string()).collect(),
                tiles: tiles.iter().map(|s| s.to_string()).collect(),
            };
            let context = Rc::new(Box::new(rendering_context) as Box<dyn RendererContext>);
            let game = Game::new(layout);
            let sender = game.get_sender();
            Self {
                layout: layout,
                game: RefCell::new(game),
                game_sender: sender,
                rendering_context: context,
            }
        }

        pub fn width(&self) -> u32 {
            self.layout.width()
        }

        pub fn height(&self) -> u32 {
            self.layout.height()
        }

        fn render(&self) {
            let message = ChannelMessage::Render(Rc::clone(&self.rendering_context));
            self.game_sender.send(message).unwrap();
            while self.game.borrow_mut().pull() > 0 {}
            self.rendering_context.end_rendering();
        }

        pub fn handle_event(&self, event: MouseEventData) {
            let message = ChannelMessage::MouseEvent(event);
            self.game_sender.send(message).unwrap();
            while self.game.borrow_mut().pull() > 0 {}
        }
    }


    fn window() -> web_sys::Window {
        web_sys::window().expect("no global `window` exists")
    }

    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        window()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    fn document() -> web_sys::Document {
        window()
            .document()
            .expect("should have a document on window")
    }

    fn body() -> web_sys::HtmlElement {
        document().body().expect("document should have a body")
    }

    fn make_minesweeper() -> Result<Rc<Minesweeper>, JsValue> {
        // create the canvas and supress default right click
        let canvas = document()
            .create_element("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()?;
        body().append_child(&canvas)?;
        canvas.set_width(640);
        canvas.set_height(480);
        canvas.style().set_property("border", "solid")?;
        canvas.set_attribute("oncontextmenu", "event.preventDefault();")?;

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
        let context = Rc::new(context);
        // create the game
        let minesweeper = wasm::Minesweeper::new(&context);
        canvas.set_width(minesweeper.width());
        canvas.set_height(minesweeper.height());
        let minesweeper = Rc::new(minesweeper);
        minesweeper.render();
        // setup the mouse down hook
        {
            let minesweeper = minesweeper.clone();
            let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                let button = match event.button() {
                    0 => MouseButton::Left,
                    1 => MouseButton::Middle,
                    2 => MouseButton::Right,
                    _ => MouseButton::Middle,
                };
                let mouse_event = MouseEventData {
                    x: event.offset_x(),
                    y: event.offset_y(),
                    mouse_btn: button,
                };

                minesweeper.handle_event(mouse_event);
                minesweeper.render();
            }) as Box<dyn FnMut(_)>);
            canvas
                .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        // setup request  animation frame loop
        let closure_option = Rc::new(RefCell::new(None));
        let cloned_option = closure_option.clone();
        {
            let minesweeper = minesweeper.clone();
            let closure = Closure::wrap(Box::new(move || {
                minesweeper.render();
                // Schedule ourself for another requestAnimationFrame callback.
                request_animation_frame(closure_option.borrow().as_ref().unwrap());
            }) as Box<dyn FnMut()>);
            cloned_option.replace(Some(closure));
        }
        request_animation_frame(cloned_option.borrow().as_ref().unwrap());
        Ok(minesweeper)
    }

    #[wasm_bindgen(start)]
    pub fn start() -> Result<(), JsValue> {
        log!("Starting the game");
        utils::set_panic_hook();
        let _ = make_minesweeper()?;
        Ok(())
    }
}
