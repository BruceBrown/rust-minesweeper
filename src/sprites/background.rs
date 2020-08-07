use crate::sprites::Rect;
use crate::sprites::{ChannelMessage, ChannelWiring, Exchange, MessageExchange};
use crate::sprites::{Error, Renderer, RendererContext, Sprite};

// Background sprite is pretty simple
pub struct Background {
    exchange: Exchange,
}

impl Background {
    pub fn new(wiring: &mut ChannelWiring) -> Self {
        Self {
            exchange: Exchange::new_from_wiring::<Background>(wiring),
        }
    }
}

impl Renderer for Background {
    fn render(&self, context: &dyn RendererContext) -> Result<(), Error> {
        let base = context.layout().options.level();
        let name = format!("bg_{}", base);
        let image = context.load(&name)?;
        let w = context.layout().width();
        let h = context.layout().height();
        let rect = Rect::new(0, 0, w, h);
        context.render_image(&image, None, rect)?;
        Ok(())
    }
}

impl MessageExchange for Background {
    fn pull(&mut self) -> u32 {
        let mut count = self.exchange.pull();
        for message in self.exchange.get_messages().iter() {
            match message {
                ChannelMessage::Render(context) => {
                    let base = context.layout().options.level();
                    let name = format!("bg_{}", base);
                    let image = context.load(&name).unwrap();
                    let w = context.layout().width();
                    let h = context.layout().height();
                    let rect = Rect::new(0, 0, w, h);
                    context.render_image(&image, None, rect).unwrap();
                }
                _ => (),
            }
        }
        count
    }
}

impl Sprite for Background {}
