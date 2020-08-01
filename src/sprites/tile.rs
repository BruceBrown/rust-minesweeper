use std::cell::{Cell, RefCell};

use crate::config::Layout;
use crate::sprites::{Error, Rect};
use crate::sprites::{
    FlagStateListener, GameState, GameStateListener, Sprite, TileListener, WeakTraitWrapper,
};

use crate::sprites::{MouseButton, MouseEvent, MouseHandler};
use crate::sprites::{Renderer, RendererContext};

pub trait TileSprite: Sprite {
    fn reset(&self, is_mine: bool, adjacent_mines: u8);
    fn game_state_changed(&self, state: GameState);
}

pub struct Tile {
    is_revealed: Cell<bool>,
    is_mine: Cell<bool>,
    is_flagged: Cell<bool>,
    adjacent_mines: Cell<u8>,
    adjacent_flags: Cell<u8>,
    flag_remaining: Cell<bool>,
    is_game_over: Cell<bool>,
    bounding_box: Rect,
    listeners: RefCell<Vec<WeakTraitWrapper<dyn TileListener>>>,
}

impl Tile {
    pub fn new(_layout: Layout, bounding_box: Rect) -> Tile {
        Tile {
            is_revealed: Cell::new(false),
            is_mine: Cell::new(false),
            is_flagged: Cell::new(false),
            adjacent_mines: Cell::new(0),
            adjacent_flags: Cell::new(0),
            flag_remaining: Cell::new(true),
            is_game_over: Cell::new(false),
            bounding_box: bounding_box,
            listeners: RefCell::new(Vec::new()),
        }
    }

    pub fn assign_listeners(&self, listeners: Vec<WeakTraitWrapper<dyn TileListener>>) {
        self.listeners.replace(listeners);
    }

    fn try_clear(&self) {
        if self.adjacent_flags.get() == self.adjacent_mines.get() {
            for listener in self.listeners.borrow().iter() {
                listener.upgrade().unwrap().clear();
            }
        }
    }

    fn try_reveal(&self) {
        if self.is_game_over.get() || self.is_flagged.get() || self.is_revealed.get() {
        } else {
            self.is_revealed.set(true);
            let has_adjacent_mines = self.adjacent_mines.get() > 0;
            let is_mine = self.is_mine.get();
            for listener in self.listeners.borrow().iter() {
                listener
                    .upgrade()
                    .unwrap()
                    .reveal(is_mine, has_adjacent_mines);
            }
        }
    }

    fn try_toggle_flag(&self) {
        if self.is_game_over.get() || self.is_revealed.get() {
            return;
        }
        let mut is_flagged = self.is_flagged.get();
        if !is_flagged && !self.flag_remaining.get() {
            return;
        }
        is_flagged = !is_flagged;
        self.is_flagged.set(is_flagged);
        for listener in self.listeners.borrow().iter() {
            listener.upgrade().unwrap().flag(is_flagged);
        }
    }

    fn handle_game_state_changed(&self, state: GameState) {
        match state {
            GameState::Init => {
                self.is_flagged.set(false);
                self.is_revealed.set(false);
                self.adjacent_flags.set(0);
                self.is_game_over.set(false);
                self.flag_remaining.set(true);
            }
            GameState::Win => {
                self.is_game_over.set(true);
            }
            GameState::Lose => {
                self.is_game_over.set(true);
            }
            _ => {}
        }
    }
}

impl Renderer for Tile {
    fn render(&self, context: &dyn RendererContext) -> Result<(), Error> {
        if self.is_revealed.get() {
            if self.is_mine.get() {
                let image = context.load("tile_mine")?;
                context.render_image(&image, None, self.bounding_box)?;
            } else {
                let image = context.load_tile(self.adjacent_mines.get() as u64)?;
                context.render_image(&image, None, self.bounding_box)?;
            }
        } else if self.is_flagged.get() {
            let image = context.load("tile_flag")?;
            context.render_image(&image, None, self.bounding_box)?;
        } else {
            let image = context.load("tile")?;
            context.render_image(&image, None, self.bounding_box)?;
        }
        Ok(())
    }
}

impl MouseHandler for Tile {
    fn hit_test(&self, event: &MouseEvent) -> bool {
        self.bounding_box.contains_point((event.x, event.y))
    }
    fn handle_event(&self, event: &MouseEvent) {
        match event.mouse_btn {
            MouseButton::Left => {
                if self.is_revealed.get() {
                    self.try_clear();
                } else {
                    self.try_reveal();
                }
            }
            MouseButton::Right => {
                self.try_toggle_flag();
            }
            _ => {}
        }
    }
}

impl FlagStateListener for Tile {
    fn flag_state_changed(&self, exhausted: bool) {
        self.flag_remaining.set(!exhausted);
    }
}

impl GameStateListener for Tile {
    fn game_state_changed(&self, state: GameState) {
        self.handle_game_state_changed(state);
    }
}

impl Sprite for Tile {}

impl TileSprite for Tile {
    fn reset(&self, is_mine: bool, adjacent_mines: u8) {
        self.is_mine.set(is_mine);
        self.adjacent_mines.set(adjacent_mines);
    }
    fn game_state_changed(&self, state: GameState) {
        self.handle_game_state_changed(state);
    }
}

impl TileListener for Tile {
    fn reveal(&self, is_mine: bool, has_adjacent_mines: bool) {
        if !is_mine && !has_adjacent_mines {
            self.try_reveal();
        }
    }

    fn clear(&self) {
        self.try_reveal();
    }

    fn flag(&self, is_flagged: bool) {
        let mut adjacent_flags = self.adjacent_flags.get();
        if is_flagged {
            adjacent_flags += 1;
        } else {
            adjacent_flags -= 1;
        }
        self.adjacent_flags.set(adjacent_flags);
    }
}
