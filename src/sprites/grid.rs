use std::collections::BTreeSet;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::config::Layout;
use crate::sprites::GameState;
use crate::sprites::{ChannelMessage, ChannelWiring, Exchange, MessageExchange};
use crate::sprites::{Error, Rect};
use crate::sprites::{MouseEvent, MouseHandler, Renderer, RendererContext};
use crate::sprites::{Sprite, Tile};

pub struct Grid {
    layout: Layout,
    bounding_box: Rect,
    tiles: Vec<Tile>,
    minefield: Minefield,
    exchange: Exchange,
}

impl Grid {
    pub fn new(layout: Layout, wiring: &mut ChannelWiring) -> Self {
        let mut exchange = Exchange::new_from_wiring::<Grid>(wiring);
        let bounding_box = layout.grid();
        let mut minefield = Minefield::new(layout);
        minefield.reset();
        let (mut senders, tiles) = Grid::build_tiles(layout, &minefield, &exchange);
        exchange.replace_senders(&mut senders);

        Self {
            layout: layout,
            bounding_box: bounding_box,
            tiles: tiles,
            minefield: minefield,
            exchange: exchange,
        }
    }

    fn build_tiles(
        layout: Layout,
        minefield: &Minefield,
        exchange: &Exchange,
    ) -> (Vec<Sender<ChannelMessage>>, Vec<Tile>) {
        struct TileChannel {
            sender: Sender<ChannelMessage>,
            receiver: Option<Receiver<ChannelMessage>>,
        }

        impl TileChannel {
            pub fn get_receiver(&mut self) -> Option<Receiver<ChannelMessage>> {
                use std::mem::swap;
                let mut opt: Option<Receiver<ChannelMessage>> = None;
                swap(&mut self.receiver, &mut opt);
                opt
            }

            pub fn clone_sender(&self) -> Sender<ChannelMessage> {
                self.sender.clone()
            }
        }

        impl Default for TileChannel {
            fn default() -> Self {
                let (sender, receiver) = channel();
                Self {
                    sender,
                    receiver: Some(receiver),
                }
            }
        }

        let tile_count = layout.options.tiles();
        let mut tile_channels: Vec<TileChannel> = Vec::new();
        for _ in 0..tile_count {
            tile_channels.push(TileChannel::default());
        }

        // these are the flag_state_listeners
        // the are also the tile sprites
        let mut senders: Vec<Sender<ChannelMessage>> = Vec::new();
        for channel in tile_channels.iter() {
            senders.push(channel.clone_sender());
        }

        let mut tiles: Vec<Tile> = Vec::new();
        for index in 0..tile_count {
            let mut neighbors = exchange.clone_senders();
            let closure = |row, column| {
                let index = layout.options.index(row, column) as usize;
                let channel = &tile_channels[index];
                neighbors.push(channel.clone_sender());
            };
            layout.options.for_each_neighbor(index as u16, closure);
            let bounding_box = layout.grid_tile(index);
            let receiver = tile_channels[index as usize].get_receiver().unwrap();
            let tile_exchange = Exchange::new(neighbors, vec![receiver]);
            let mut tile = Tile::new(tile_exchange, bounding_box);
            let adjacent_mines = minefield.adjacent_mines(index as u16);
            let is_mine = minefield.mine_at(index);
            tile.reset(is_mine, adjacent_mines);
            tiles.push(tile);
        }
        (senders, tiles)
    }
}

impl MessageExchange for Grid {
    fn pull(&mut self) -> u32 {
        let mut count = self.exchange.pull();
        for message in self.exchange.get_messages().iter() {
            match message {
                ChannelMessage::GameStateChanged(GameState::Init) => {
                    self.minefield.reset();
                    for index in 0..self.tiles.len() {
                        let is_mine = self.minefield.mine_at(index as i16);
                        let adjacent_mines = self.minefield.adjacent_mines(index as u16);
                        self.tiles[index].reset(is_mine, adjacent_mines);
                    }
                    self.exchange.push(message.clone());
                }
                ChannelMessage::GameStateChanged(_state) => {
                    self.exchange.push(message.clone());
                }
                ChannelMessage::FlagStateChanged(_exhausted) => {
                    self.exchange.push(message.clone());
                }
                _ => println!("Grid: unhandled message {:#?}", message),
            }
        }
        for tiles in self.tiles.iter_mut() {
            count += tiles.pull();
        }
        count
    }
}

impl Renderer for Grid {
    fn render(&self, context: &dyn RendererContext) -> Result<(), Error> {
        for sprite in self.tiles.iter() {
            sprite.render(context)?;
        }
        Ok(())
    }
}

impl MouseHandler for Grid {
    fn hit_test(&self, event: &MouseEvent) -> bool {
        self.bounding_box.contains_point((event.x, event.y))
    }
    fn handle_event(&mut self, event: &MouseEvent) {
        let column = (event.x - self.bounding_box.left()) / Layout::tile_side() as i32;
        let row = (event.y - self.bounding_box.top()) / Layout::tile_side() as i32;
        let index = self.layout.options.index(row as i16, column as i16) as usize;
        self.tiles[index].handle_event(event);
    }
}

impl Sprite for Grid {}

struct Minefield {
    layout: Layout,
    mines: BTreeSet<i16>,
}

use rand::prelude::*;

impl Minefield {
    fn new(layout: Layout) -> Self {
        let mut obj = Self {
            layout: layout,
            mines: BTreeSet::new(),
        };
        obj.reset();
        obj
    }

    fn mine_at(&self, index: i16) -> bool {
        self.mines.contains(&index)
    }

    fn adjacent_mines(&self, index: u16) -> u8 {
        let mut sum = 0;
        let closure = |row, column| {
            let index = self.layout.options.index(row, column);
            if self.mine_at(index as i16) {
                sum += 1;
            }
        };
        self.layout.options.for_each_neighbor(index, closure);
        sum
    }

    fn reset(&mut self) {
        self.mines.clear();
        self.place_mines();
    }

    fn place_mines(&mut self) {
        let max_index = self.layout.options.tiles();
        let mine_count = self.layout.options.mines() as usize;
        let mut rng = rand::thread_rng();

        while self.mines.len() < mine_count {
            let index = rng.gen_range(0, max_index);
            self.mines.insert(index);
        }
    }
}
