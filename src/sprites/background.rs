use crate::sprites::MessageExchange;
use crate::sprites::Rect;
use crate::sprites::{Error, MouseHandler, Renderer, RendererContext, Sprite};

// Background sprite is pretty simple
pub struct Background {}

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

impl MouseHandler for Background {}
impl MessageExchange for Background {}
impl Sprite for Background {}
