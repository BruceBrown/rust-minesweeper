use crate::config::Layout;
use crate::sprites::GameState;
use crate::sprites::{Error, Rect};
use crate::sprites::{MouseButton, MouseEventData, Renderer, RendererContext, Sprite};

use crate::sprites::{ChannelMessage, ChannelWiring, Exchange, MessageExchange};

pub struct Button {
    game_state: GameState,
    revealed: i16,
    blanks: i16,
    bounding_box: Rect,
    exchange: Exchange,
}

impl Button {
    pub fn new(layout: Layout, wiring: &mut ChannelWiring) -> Self {
        Self {
            game_state: GameState::Init,
            revealed: 0,
            blanks: layout.options.blanks(),
            bounding_box: layout.face(),
            exchange: Exchange::new_from_wiring::<Button>(wiring),
        }
    }

    fn update_game_state(&mut self, new_state: GameState) {
        self.game_state = new_state;
        // let everyone know it changed
        let message = ChannelMessage::GameStateChanged(new_state);
        self.exchange.push_message(message);
    }

    fn render(&self, context: &Box<dyn RendererContext>) -> Result<(), Error> {
        let name = match self.game_state {
            GameState::Init => "face_playing",
            GameState::Playing => "face_playing",
            GameState::Win => "face_win",
            GameState::Lose => "face_lose",
        };
        let image = context.load(name)?;
        context.render_image(&image, None, self.bounding_box)?;
        Ok(())
    }
}

impl MessageExchange for Button {
    fn pull(&mut self) -> u32 {
        let count = self.exchange.pull();
        for message in self.exchange.get_messages().iter() {
            match message {
                ChannelMessage::Revealed(true, _) => self.update_game_state(GameState::Lose),
                ChannelMessage::Revealed(false, _) => {
                    if self.game_state == GameState::Init {
                        self.update_game_state(GameState::Playing);
                    }
                    self.revealed += 1;
                    if self.revealed == self.blanks {
                        self.update_game_state(GameState::Win);
                    }
                }
                ChannelMessage::Render(context) => self.render(&context).unwrap(),
                ChannelMessage::MouseEvent(event) => {
                    if self.bounding_box.contains_point((event.x, event.y)) {
                        if event.mouse_btn == MouseButton::Left {
                            self.revealed = 0;
                            self.update_game_state(GameState::Init);
                        }
                    }
                }
                _ => (),
            }
        }
        count
    }
}

impl Sprite for Button {}
