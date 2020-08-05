use crate::sprites::{ChannelMessage, Exchange, MessageExchange};
use crate::sprites::{Error, Rect};
use crate::sprites::{GameState, Sprite};
use crate::sprites::{MouseButton, MouseEvent, MouseHandler, Renderer, RendererContext};

pub struct Tile {
    is_revealed: bool,
    is_mine: bool,
    is_flagged: bool,
    adjacent_mines: u8,
    adjacent_flags: u8,
    flag_remaining: bool,
    is_game_over: bool,
    bounding_box: Rect,
    exchange: Exchange,
}

impl Tile {
    pub fn new(exchange: Exchange, bounding_box: Rect) -> Tile {
        Tile {
            is_revealed: false,
            is_mine: false,
            is_flagged: false,
            adjacent_mines: 0,
            adjacent_flags: 0,
            flag_remaining: true,
            is_game_over: false,
            bounding_box: bounding_box,
            exchange: exchange,
        }
    }

    pub fn reset(&mut self, is_mine: bool, adjacent_mines: u8) {
        self.is_mine = is_mine;
        self.adjacent_mines = adjacent_mines;
    }

    fn try_clear(&self) {
        if self.adjacent_flags == self.adjacent_mines {
            self.exchange.push(ChannelMessage::Clear);
        }
    }

    fn try_reveal(&mut self) {
        if self.is_game_over || self.is_flagged || self.is_revealed {
        } else {
            self.is_revealed = true;
            let has_adjacent_mines = self.adjacent_mines > 0;
            let is_mine = self.is_mine;
            self.exchange
                .push(ChannelMessage::Revealed(is_mine, has_adjacent_mines));
        }
    }

    fn try_toggle_flag(&mut self) {
        if self.is_game_over || self.is_revealed {
            return;
        }
        if !self.is_flagged && !self.flag_remaining {
            return;
        }
        self.is_flagged = !self.is_flagged;
        self.exchange.push(ChannelMessage::Flagged(self.is_flagged));
    }

    fn handle_game_state_changed(&mut self, state: &GameState) {
        match state {
            GameState::Init => {
                self.is_flagged = false;
                self.is_revealed = false;
                self.adjacent_flags = 0;
                self.is_game_over = false;
                self.flag_remaining = true;
            }
            GameState::Win => {
                self.is_game_over = true;
            }
            GameState::Lose => {
                self.is_game_over = true;
            }
            _ => {}
        }
    }
}

impl Renderer for Tile {
    fn render(&self, context: &dyn RendererContext) -> Result<(), Error> {
        if self.is_revealed {
            if self.is_mine {
                let image = context.load("tile_mine")?;
                context.render_image(&image, None, self.bounding_box)?;
            } else {
                let image = context.load_tile(self.adjacent_mines as u64)?;
                context.render_image(&image, None, self.bounding_box)?;
            }
        } else if self.is_flagged {
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
    fn handle_event(&mut self, event: &MouseEvent) {
        match event.mouse_btn {
            MouseButton::Left => {
                if self.is_revealed {
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
impl MessageExchange for Tile {
    fn pull(&mut self) -> u32 {
        let count = self.exchange.pull();
        for message in self.exchange.get_messages().iter() {
            match message {
                ChannelMessage::FlagStateChanged(exhausted) => self.flag_remaining = !exhausted,
                ChannelMessage::GameStateChanged(state) => self.handle_game_state_changed(&state),
                ChannelMessage::Revealed(false, false) => self.try_reveal(),
                ChannelMessage::Clear => self.try_reveal(),
                ChannelMessage::Flagged(true) => self.adjacent_flags += 1,
                ChannelMessage::Flagged(false) => self.adjacent_flags -= 1,
                _ => (),
            }
        }
        count
    }
}

impl Sprite for Tile {}
