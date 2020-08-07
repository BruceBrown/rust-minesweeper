use crate::sprites::render_digit;
use crate::sprites::Error;
use crate::sprites::GameState;
use crate::sprites::{Renderer, RendererContext, Sprite};

use crate::sprites::SystemTime;

use crate::sprites::{ChannelMessage, ChannelWiring, Exchange, MessageExchange};

pub struct TimeCounter {
    elapsed: u64,
    running: bool,
    start: SystemTime,
    exchange: Exchange,
}

impl TimeCounter {
    pub fn new(wiring: &mut ChannelWiring) -> Self {
        Self {
            elapsed: 0,
            running: false,
            start: SystemTime::now(),
            exchange: Exchange::new_from_wiring::<TimeCounter>(wiring),
        }
    }

    fn render(&self, context_: &Box<dyn RendererContext>) -> Result<(), Error> {
        let elapsed = if self.running {
            self.start
                .elapsed()
                /*
                .context(StartTimeInvalid {
                    start: self.start.get(),
                })?
                */
                .unwrap()
                .as_secs()
        } else {
            self.elapsed
        };
        let image = context_.load("digit_panel")?;
        let bounding_box = context_.layout().timer_digit_panel();
        context_.render_image(&image, None, bounding_box)?;

        let ones = elapsed % 10;
        let tens = elapsed / 10 % 10;
        let hundreds = elapsed / 100 % 10;

        render_digit(ones, context_.layout().timer_digit(2), context_)?;
        render_digit(tens, context_.layout().timer_digit(1), context_)?;
        render_digit(hundreds, context_.layout().timer_digit(0), context_)?;
        Ok(())
    }
}

impl MessageExchange for TimeCounter {
    fn pull(&mut self) -> u32 {
        let count = self.exchange.pull();
        for message in self.exchange.get_messages().iter() {
            match message {
                ChannelMessage::GameStateChanged(GameState::Init) => {
                    self.running = false;
                    self.elapsed = 0;
                }
                ChannelMessage::GameStateChanged(GameState::Playing) => {
                    self.running = true;
                    self.start = SystemTime::now();
                }
                ChannelMessage::GameStateChanged(GameState::Win) => {
                    self.running = false;
                    self.elapsed = self.start.elapsed().unwrap().as_secs();
                }
                ChannelMessage::GameStateChanged(GameState::Lose) => {
                    self.running = false;
                    self.elapsed = self.start.elapsed().unwrap().as_secs();
                }
                ChannelMessage::Render(context) => self.render(&context).unwrap(),
                _ => (),
            }
        }
        count
    }
}

impl Sprite for TimeCounter {}
