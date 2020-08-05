use crate::config::Layout;
use crate::sprites::render_digit;
use crate::sprites::Error;
use crate::sprites::GameState;
use crate::sprites::{ChannelMessage, ChannelWiring, Exchange, MessageExchange};
use crate::sprites::{MouseHandler, Renderer, RendererContext, Sprite};

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
}

impl MessageExchange for FlagCounter {
    fn pull(&mut self) -> u32 {
        let count = self.exchange.pull();
        for message in self.exchange.get_messages().iter() {
            match message {
                ChannelMessage::GameStateChanged(GameState::Init) => {
                    self.flags = self.layout.options.mines();
                }
                ChannelMessage::Flagged(true) => {
                    self.flags -= 1;
                    if self.flags == 0 {
                        self.exchange.push(ChannelMessage::FlagStateChanged(false));
                    }
                }
                ChannelMessage::Flagged(false) => {
                    self.flags += 1;
                    if self.flags == 1 {
                        self.exchange.push(ChannelMessage::FlagStateChanged(false));
                    }
                }
                _ => println!("Tile: unhandled message {:#?}", message),
            }
        }
        count
    }
}

impl Renderer for FlagCounter {
    fn render(&self, context: &dyn RendererContext) -> Result<(), Error> {
        let value = self.flags;
        let image = context.load("digit_panel")?;
        let bounding_box = context.layout().flag_digit_panel();
        context.render_image(&image, None, bounding_box)?;

        let ones = value % 10;
        let tens = value / 10 % 10;
        let hundreds = value / 100 % 10;

        render_digit(ones as u64, context.layout().flag_digit(2), context)?;
        render_digit(tens as u64, context.layout().flag_digit(1), context)?;
        render_digit(hundreds as u64, context.layout().flag_digit(0), context)?;
        Ok(())
    }
}

impl MouseHandler for FlagCounter {}
impl Sprite for FlagCounter {}
