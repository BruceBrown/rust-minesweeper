use std::rc::Rc;

use crate::config::Layout;
use crate::sprites::{Background, Button, FlagCounter, Grid, TimeCounter};
use crate::sprites::{MouseEvent, MouseHandler, Renderer, RendererContext, Sprite};

pub struct Game {
    sprites: Vec<TraitWrapper<dyn Sprite>>,
}

use crate::sprites::Error;
use crate::sprites::{FlagStateListener, GameStateListener, TileListener};
use crate::sprites::{TraitWrapper, WeakTrait, WeakTraitWrapper};

impl Game {
    pub fn new(layout: Layout) -> Game {
        // create the underlying objects
        let bg = Rc::new(Background {});
        let timer = Rc::new(TimeCounter::new());
        let flag_counter = Rc::new(FlagCounter::new(layout));
        let button = Rc::new(Button::new(layout));
        let mut grid = Rc::new(Grid::new(layout));

        let tile_listeners: Vec<WeakTraitWrapper<dyn TileListener>> = vec![
            Box::new(Rc::downgrade(&button) as WeakTrait<dyn TileListener>),
            Box::new(Rc::downgrade(&flag_counter) as WeakTrait<dyn TileListener>),
        ];
        Rc::get_mut(&mut grid)
            .unwrap()
            .assign_listeners(&tile_listeners);

        let game_state_listeners: Vec<WeakTraitWrapper<dyn GameStateListener>> = vec![
            Box::new(Rc::downgrade(&timer) as WeakTrait<dyn GameStateListener>),
            Box::new(Rc::downgrade(&flag_counter) as WeakTrait<dyn GameStateListener>),
            Box::new(Rc::downgrade(&grid) as WeakTrait<dyn GameStateListener>),
        ];
        button.assign_listeners(game_state_listeners);

        let flag_state_listeners: Vec<WeakTraitWrapper<dyn FlagStateListener>> = vec![Box::new(
            Rc::downgrade(&grid) as WeakTrait<dyn FlagStateListener>,
        )];
        flag_counter.assign_listeners(flag_state_listeners);

        let sprites: Vec<TraitWrapper<dyn Sprite>> = vec![
            Box::new(bg),
            Box::new(timer),
            Box::new(flag_counter),
            Box::new(button),
            Box::new(grid),
        ];

        // finally create the game object
        Game { sprites: sprites }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            sprites: Vec::new(),
        }
    }
}

impl Renderer for Game {
    fn render(&self, context: &dyn RendererContext) -> Result<(), Error> {
        for sprite in self.sprites.iter() {
            sprite.render(context)?;
        }
        Ok(())
    }
}

impl MouseHandler for Game {
    fn hit_test(&self, _event: &MouseEvent) -> bool {
        false
    }

    fn handle_event(&self, event: &MouseEvent) {
        for sprite in self.sprites.iter() {
            if sprite.hit_test(event) {
                sprite.handle_event(event);
            }
        }
    }
}
