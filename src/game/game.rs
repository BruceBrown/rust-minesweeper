use crate::config::Layout;
use crate::sprites::Error;
use crate::sprites::{Background, Button, FlagCounter, Grid, Sprite, TimeCounter};
use crate::sprites::{ChannelMessage, ChannelWiring, Exchange, MessageExchange};
use crate::sprites::{Renderer, RendererContext};

pub struct Game {
    sprites: Vec<Box<dyn Sprite>>,
    sender: std::sync::mpsc::Sender<ChannelMessage>,
    exchange: Exchange,
}

impl Game {
    pub fn new(layout: Layout) -> Game {
        // get all the channel wiring setup
        let mut channels = ChannelWiring::default();
        channels.wire::<Grid, Button>();
        channels.wire::<Grid, FlagCounter>();

        channels.wire::<Button, TimeCounter>();
        channels.wire::<Button, FlagCounter>();
        channels.wire::<Button, Grid>();

        channels.wire::<FlagCounter, Grid>();

        channels.wire::<Game, Background>();
        channels.wire::<Game, TimeCounter>();
        channels.wire::<Game, FlagCounter>();
        channels.wire::<Game, Button>();
        channels.wire::<Game, Grid>();
        struct Minesweeper;
        channels.wire::<Minesweeper, Game>();

        let mut sprites: Vec<Box<dyn Sprite>> = Vec::new();

        // create the underlying objects, and own via trait
        sprites.push(Box::new(Background::new(&mut channels)));
        sprites.push(Box::new(TimeCounter::new(&mut channels)));
        sprites.push(Box::new(FlagCounter::new(layout, &mut channels)));
        sprites.push(Box::new(Button::new(layout, &mut channels)));
        sprites.push(Box::new(Grid::new(layout, &mut channels)));

        // finally create the game object
        let (senders, _) = channels.channels::<Minesweeper>();
        let sender = senders.unwrap().pop().unwrap();
        Game {
            sprites: sprites,
            sender: sender,
            exchange: Exchange::new_from_wiring::<Game>(&mut channels),
        }
    }

    pub fn get_sender(&self) -> std::sync::mpsc::Sender<ChannelMessage> {
        self.sender.clone()
    }
}

impl MessageExchange for Game {
    fn pull(&mut self) -> u32 {
        let mut count = self.exchange.pull();
        for message in self.exchange.get_messages().iter() {
            match message {
                ChannelMessage::Render(_) => self.exchange.push_message(message.clone()),
                ChannelMessage::MouseEvent(data) => self.exchange.push_message(message.clone()),
                _ => (),
            }
        }

        for sprite in self.sprites.iter_mut() {
            count += sprite.pull();
        }
        count
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Layout, BEGINNER_LAYOUT};
    use crate::media_layer::Texture;
    use crate::sprites::Rect;
    use std::rc::Rc;

    struct TestRendering {}
    impl RendererContext for TestRendering {
        fn render_image(
            &self,
            texture: &Texture,
            src: Option<Rect>,
            dst: Rect,
        ) -> Result<(), String> {
            Ok(())
        }
        fn layout(&self) -> &Layout {
            &BEGINNER_LAYOUT
        }
        fn load(&self, name: &str) -> Result<Rc<Texture>, String> {
            Err("image not found".to_string())
        }
        fn load_digit(&self, value: u64) -> Result<Rc<Texture>, String> {
            Err("image not found".to_string())
        }
        fn load_tile(&self, value: u64) -> Result<Rc<Texture>, String> {
            Err("image not found".to_string())
        }
        fn end_rendering(&self) {}
    }

    #[test]
    fn test_construction() {
        let layout = BEGINNER_LAYOUT;
        let mut game = Game::new(layout);
        let sender = game.get_sender();
        sender.send(ChannelMessage::TestMessage);
        game.pull();
    }

    #[test]
    fn test_render() {
        let context = TestRendering {};
        let rendering_context = Rc::new(Box::new(context) as Box<dyn RendererContext>);

        let layout = BEGINNER_LAYOUT;
        let mut game = Game::new(layout);
        let sender = game.get_sender();
        let message = ChannelMessage::Render(Rc::clone(&rendering_context));
        sender.send(message);
        game.pull();
    }
}
