use std::cell::{Cell, RefCell};

use crate::config::Layout;

use crate::sprites::Error;
use crate::sprites::WeakTraitWrapper;
use crate::sprites::{FlagStateListener, GameState, GameStateListener, TileListener};
use crate::sprites::{MouseHandler, Renderer, RendererContext, Sprite};

use crate::sprites::render_digit;

pub struct FlagCounter {
    layout: Layout,
    flags: Cell<i16>,
    flag_state_listeners: RefCell<Vec<WeakTraitWrapper<dyn FlagStateListener>>>,
}

impl FlagCounter {
    pub fn new(layout: Layout) -> FlagCounter {
        FlagCounter {
            layout: layout,
            flags: Cell::new(layout.options.mines()),
            flag_state_listeners: RefCell::new(Vec::new()),
        }
    }

    pub fn assign_listeners(&self, listeners: Vec<WeakTraitWrapper<dyn FlagStateListener>>) {
        self.flag_state_listeners.replace(listeners);
    }

    fn notify_listeners(&self, exhausted: bool) {
        for listener in self.flag_state_listeners.borrow().iter() {
            listener.upgrade().unwrap().flag_state_changed(exhausted);
        }
    }
}

impl Renderer for FlagCounter {
    fn render(&self, context: &dyn RendererContext) -> Result<(), Error> {
        let value = self.flags.get();
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

impl GameStateListener for FlagCounter {
    fn game_state_changed(&self, state: GameState) {
        match state {
            GameState::Init => {
                self.flags.set(self.layout.options.mines());
            }
            _ => {}
        }
    }
}

impl Sprite for FlagCounter {}

impl TileListener for FlagCounter {
    fn flag(&self, flagged: bool) {
        let mut flags = self.flags.get();
        if flagged {
            flags -= 1;
            self.flags.set(flags);
            if flags == 0 {
                self.notify_listeners(flagged);
            }
        } else {
            flags += 1;
            self.flags.set(flags);
            if flags == 1 {
                self.notify_listeners(flagged);
            }
        }
    }
}
