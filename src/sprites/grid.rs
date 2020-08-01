use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::{Rc, Weak};

use crate::config::Layout;
use crate::sprites::{Error, Rect};
use crate::sprites::{
    FlagStateListener, GameState, GameStateListener, Sprite, TileListener, TraitWrapper,
    WeakTraitWrapper,
};
use crate::sprites::{Tile, TileSprite};

use crate::sprites::{MouseEvent, MouseHandler};
use crate::sprites::{Renderer, RendererContext};

pub struct Grid {
    layout: Layout,
    bounding_box: Rect,
    tile_sprites: Vec<TraitWrapper<dyn TileSprite>>,
    flag_state_listeners: Vec<WeakTraitWrapper<dyn FlagStateListener>>,
    minefield: RefCell<Minefield>,
}

impl Grid {
    pub fn new(layout: Layout) -> Self {
        let bounding_box = layout.grid();
        let minefield = RefCell::new(Minefield::new(layout));
        minefield.borrow_mut().reset();

        Self {
            layout: layout,
            bounding_box: bounding_box,
            //tile_listeners: Vec::new(),
            tile_sprites: Vec::new(),
            flag_state_listeners: Vec::new(),
            minefield: minefield,
        }
    }

    pub fn assign_listeners(&mut self, listeners: &Vec<WeakTraitWrapper<dyn TileListener>>) {
        let tile_count = self.layout.options.tiles();
        let mut tiles: Vec<TraitWrapper<Tile>> = Vec::new();
        // first build the vector of tiles, we need this to get trait vectors
        for index in 0..tile_count {
            let bounding_box = self.layout.tile(self.bounding_box, index);
            let tile = Box::new(Rc::new(Tile::new(self.layout, bounding_box)));

            let adjacent_mines = self.minefield.borrow().adjacent_mines(index as u16);
            let is_mine = self.minefield.borrow().mine_at(index);
            tile.reset(is_mine, adjacent_mines);

            tiles.push(tile);
        }
        // from there, build the vector of sprite traits
        let mut tile_sprites: Vec<TraitWrapper<dyn TileSprite>> = Vec::new();
        for tile in tiles.iter() {
            let tile_sprite = Rc::clone(&tile) as Rc<dyn TileSprite>;
            tile_sprites.push(Box::new(tile_sprite));
        }
        self.tile_sprites = tile_sprites;

        // now the FlagStateListeners (which are also tiles)
        let mut flag_state_listeners: Vec<WeakTraitWrapper<dyn FlagStateListener>> = Vec::new();
        for tile in tiles.iter() {
            let listener = Rc::downgrade(&tile) as Weak<dyn FlagStateListener>;
            flag_state_listeners.push(Box::new(listener));
        }
        self.flag_state_listeners = flag_state_listeners;

        // finally, build the adjacency network
        for index in 0..tiles.len() {
            let mut all = listeners.clone();
            let closure = |row, column| {
                let index = self.layout.options.index(row, column) as usize;
                let tile = &tiles[index];
                let listener = Rc::downgrade(&tile) as Weak<dyn TileListener>;
                all.push(Box::new(listener));
            };
            self.layout.options.for_each_neighbor(index as u16, closure);
            tiles[index as usize].assign_listeners(all);
        }
    }
}

impl Renderer for Grid {
    fn render(&self, context: &dyn RendererContext) -> Result<(), Error> {
        for sprite in self.tile_sprites.iter() {
            sprite.render(context)?;
        }
        Ok(())
    }
}

impl MouseHandler for Grid {
    fn hit_test(&self, event: &MouseEvent) -> bool {
        self.bounding_box.contains_point((event.x, event.y))
    }
    fn handle_event(&self, event: &MouseEvent) {
        let column = (event.x - self.bounding_box.left()) / Layout::tile_side() as i32;
        let row = (event.y - self.bounding_box.top()) / Layout::tile_side() as i32;
        let index = self.layout.options.index(row as i16, column as i16) as usize;
        self.tile_sprites[index].handle_event(event);
    }
}

impl Sprite for Grid {}

impl GameStateListener for Grid {
    fn game_state_changed(&self, state: GameState) {
        if state == GameState::Init {
            self.minefield.borrow_mut().reset();
            for index in 0..self.tile_sprites.len() {
                let minefield = self.minefield.borrow();
                let is_mine = minefield.mine_at(index as i16);
                let adjacent_mines = minefield.adjacent_mines(index as u16);
                self.tile_sprites[index].reset(is_mine, adjacent_mines);
            }
        }
        for sprite in self.tile_sprites.iter() {
            sprite.game_state_changed(state);
        }
    }
}

impl FlagStateListener for Grid {
    fn flag_state_changed(&self, exhausted: bool) {
        for listener in self.flag_state_listeners.iter() {
            listener.upgrade().unwrap().flag_state_changed(exhausted);
        }
    }
}

struct Minefield {
    layout: Layout,
    mines: BTreeSet<i16>,
}

use rand::prelude::*;

impl Minefield {
    pub fn new(layout: Layout) -> Self {
        let mut obj = Self {
            layout: layout,
            mines: BTreeSet::new(),
        };
        obj.reset();
        obj
    }

    pub fn mine_at(&self, index: i16) -> bool {
        self.mines.contains(&index)
    }

    pub fn adjacent_mines(&self, index: u16) -> u8 {
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

    pub fn reset(&mut self) {
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
