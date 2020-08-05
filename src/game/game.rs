use crate::config::Layout;
use crate::sprites::Error;
use crate::sprites::{Background, Button, FlagCounter, Grid, Sprite, TimeCounter};
use crate::sprites::{ChannelWiring, MessageExchange};
use crate::sprites::{MouseEvent, MouseHandler, Renderer, RendererContext};

pub struct Game {
    sprites: Vec<Box<dyn Sprite>>,
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

        let mut sprites: Vec<Box<dyn Sprite>> = Vec::new();

        // create the underlying objects, and own via trait
        sprites.push(Box::new(Background {}));
        sprites.push(Box::new(TimeCounter::new(&mut channels)));
        sprites.push(Box::new(FlagCounter::new(layout, &mut channels)));
        sprites.push(Box::new(Button::new(layout, &mut channels)));
        sprites.push(Box::new(Grid::new(layout, &mut channels)));

        // finally create the game object
        Game { sprites: sprites }
    }
}

impl MessageExchange for Game {
    fn pull(&mut self) -> u32 {
        let mut count: u32 = 0;
        for sprite in self.sprites.iter_mut() {
            count += sprite.pull();
        }
        count
    }
}

impl Renderer for Game {
    fn render(&self, context: &dyn RendererContext) -> Result<(), Error> {
        for sprite in self.sprites.iter() {
            sprite.render(context)?;
        }
        Ok(())
    }
}

impl MouseHandler for Game {
    fn hit_test(&self, _event: &MouseEvent) -> bool {
        false
    }

    fn handle_event(&mut self, event: &MouseEvent) {
        for sprite in self.sprites.iter_mut() {
            if sprite.hit_test(event) {
                sprite.handle_event(event);
            }
        }
    }
}
