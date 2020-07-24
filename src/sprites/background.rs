use super::sprites::Error;
use super::sprites::{MouseHandler, Renderer, RendererContext, Sprite};

// Background sprite is pretty simple
pub struct Background {}

impl<'a> Renderer<'_> for Background {
    fn render(&self, context: &mut dyn RendererContext) -> Result<(), Error> {
        let base = context.layout().options.level();
        let name = format!("bg_{}", base);
        let image = context.load(&name)?;
        let q = image.query();
        let rect = sdl2::rect::Rect::new(0, 0, q.width, q.height);
        context.canvas().copy(&image, None, rect)?;
        Ok(())
    }
}

impl MouseHandler for Background {}

impl<'a> Sprite<'_> for Background {}
