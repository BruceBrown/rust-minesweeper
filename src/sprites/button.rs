use sdl2::rect::{Point, Rect};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::config::Layout;

use super::sprites::Error;
use crate::sprites::WeakTraitWrapper;
use crate::sprites::{GameState, GameStateListener, TileListener};
use crate::sprites::{MouseEvent, MouseHandler, Renderer, RendererContext, Sprite};

pub struct Button {
    game_state: Cell<GameState>,
    game_state_listeners: RefCell<Vec<WeakTraitWrapper<dyn GameStateListener>>>,
    revealed: Cell<i16>,
    blanks: i16,
    bounding_box: Rect,
}

impl Button {
    pub fn new(layout: &Rc<Layout>) -> Self {
        Self {
            game_state: Cell::new(GameState::Init),
            game_state_listeners: RefCell::new(Vec::new()),
            revealed: Cell::new(0),
            blanks: layout.options.blanks(),
            bounding_box: layout.face(),
        }
    }

    pub fn assign_listeners(&self, listeners: Vec<WeakTraitWrapper<dyn GameStateListener>>) {
        self.game_state_listeners.replace(listeners);
    }

    fn notify_listeners(&self) {
        for listener in self.game_state_listeners.borrow().iter() {
            listener
                .upgrade()
                .unwrap()
                .game_state_changed(self.game_state.get());
        }
    }
}

impl<'a> Renderer<'_> for Button {
    fn render(&self, context: &mut dyn RendererContext) -> Result<(), Error> {
        let name = match self.game_state.get() {
            GameState::Init => "face_playing",
            GameState::Playing => "face_playing",
            GameState::Win => "face_win",
            GameState::Lose => "face_lose",
        };
        let image = context.load(name)?;
        context.canvas().copy(&image, None, self.bounding_box)?;
        Ok(())
    }
}
impl MouseHandler for Button {
    fn hit_test(&self, event: &MouseEvent) -> bool {
        self.bounding_box
            .contains_point(Point::new(event.x, event.y))
    }
    fn handle_event(&self, event: &MouseEvent) {
        if event.mouse_btn == sdl2::mouse::MouseButton::Left {
            self.game_state.set(GameState::Init);
            self.revealed.set(0);
            self.notify_listeners();
        }
    }
}

impl<'a> Sprite<'_> for Button {}

impl TileListener for Button {
    fn reveal(&self, is_mine: bool, _has_adjacent_mines: bool) {
        if is_mine {
            self.game_state.set(GameState::Lose);
            self.notify_listeners();
        } else {
            if self.game_state.get() == GameState::Init {
                self.game_state.set(GameState::Playing);
                self.notify_listeners();
            }
            let revealed = self.revealed.get() + 1;
            self.revealed.set(revealed);
            if revealed == self.blanks {
                self.game_state.set(GameState::Win);
                self.notify_listeners();
            }
        }
    }
}
