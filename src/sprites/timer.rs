use snafu::ResultExt;
use std::cell::Cell;

use crate::sprites::render_digit;
use crate::sprites::Error;
use crate::sprites::StartTimeInvalid;
use crate::sprites::{GameState, GameStateListener};
use crate::sprites::{MouseHandler, Renderer, RendererContext, Sprite};

pub struct Timer {
    elapsed: Cell<u64>,
    running: Cell<bool>,
    start: Cell<std::time::SystemTime>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            elapsed: Cell::new(0),
            running: Cell::new(false),
            start: Cell::new(std::time::SystemTime::now()),
        }
    }
}

impl GameStateListener for Timer {
    fn game_state_changed(&self, state: GameState) {
        match state {
            GameState::Init => {
                self.running.set(false);
                self.elapsed.set(0);
            }
            GameState::Playing => {
                self.running.set(true);
                self.start.set(std::time::SystemTime::now());
            }
            GameState::Win => {
                self.running.set(false);
                self.elapsed
                    .set(self.start.get().elapsed().unwrap().as_secs());
            }
            GameState::Lose => {
                self.running.set(false);
                self.elapsed
                    .set(self.start.get().elapsed().unwrap().as_secs());
            }
        }
    }
}

impl<'a> Renderer<'_> for Timer {
    fn render(&self, context_: &mut dyn RendererContext) -> Result<(), Error> {
        let elapsed = if self.running.get() {
            self.start
                .get()
                .elapsed()
                .context(StartTimeInvalid {
                    start: self.start.get(),
                })?
                .as_secs()
        } else {
            self.elapsed.get()
        };
        let image = context_.load("digit_panel")?;
        let bounding_box = context_.layout().timer_digit_panel();
        context_.canvas().copy(&image, None, bounding_box)?;

        let ones = elapsed % 10;
        let tens = elapsed / 10 % 10;
        let hundreds = elapsed / 100 % 10;

        render_digit(ones, context_.layout().timer_digit(2), context_)?;
        render_digit(tens, context_.layout().timer_digit(1), context_)?;
        render_digit(hundreds, context_.layout().timer_digit(0), context_)?;
        Ok(())
    }
}

impl MouseHandler for Timer {}

impl<'a> Sprite<'_> for Timer {}
