use crate::config::Layout;
use crate::sprites::render_digit;
use crate::sprites::Error;
use crate::sprites::GameState;
use crate::sprites::{ChannelMessage, ChannelWiring, Exchange, MessageExchange};
use crate::sprites::{Renderer, RendererContext, Sprite};
use std::rc::Rc;

pub struct FlagCounter {
    layout: Layout,
    flags: i16,
    exchange: Exchange,
}

impl FlagCounter {
    pub fn new(layout: Layout, wiring: &mut ChannelWiring) -> FlagCounter {
        FlagCounter {
            layout: layout,
            flags: layout.options.mines(),
            exchange: Exchange::new_from_wiring::<FlagCounter>(wiring),
        }
    }

    fn render(&self, context: &Box<dyn RendererContext>) -> Result<(), Error> {
        let value = self.flags;
        let image = context.load("digit_panel")?;
        let bounding_box = context.layout().flag_digit_panel();
        context.render_image(&image, None, bounding_box)?;

        let ones = value % 10;
        let tens = value / 10 % 10;
        let hundreds = value / 100 % 10;

        render_digit(ones as u64, context.layout().flag_digit(2), &context)?;
        render_digit(tens as u64, context.layout().flag_digit(1), &context)?;
        render_digit(hundreds as u64, context.layout().flag_digit(0), &context)?;
        Ok(())
    }
}

impl MessageExchange for FlagCounter {
    fn pull(&mut self) -> u32 {
        let count = self.exchange.pull();
        for message in self.exchange.get_messages().iter() {
            match message {
                ChannelMessage::GameStateChanged(GameState::Init) => {
                    self.flags = self.layout.options.mines()
                }
                ChannelMessage::Flagged(true) => {
                    self.flags -= 1;
                    if self.flags == 0 {
                        self.exchange
                            .push_message(ChannelMessage::FlagStateChanged(true));
                    }
                }
                ChannelMessage::Flagged(false) => {
                    self.flags += 1;
                    if self.flags == 1 {
                        self.exchange
                            .push_message(ChannelMessage::FlagStateChanged(false));
                    }
                }
                ChannelMessage::Render(context) => self.render(&context).unwrap(),
                _ => (),
            }
        }
        count
    }
}

impl Sprite for FlagCounter {}
