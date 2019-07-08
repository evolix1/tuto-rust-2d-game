use std::rc::Rc;

use rand::seq::SliceRandom;

use crate::config::AppConfig;
use crate::game::GameState;


pub struct Builder {
    config: Rc<AppConfig>,
}


impl Builder {
    pub fn new(config: &Rc<AppConfig>) -> Builder {
        let config = config.clone();
        Builder{ config }
    }


    pub fn build_on(&self, world: &mut GameState) {
        world.board
            .reset(&self.config.board_side_length)
            .expect("valid dimension");

        // TODO error handling
        let mut rng = rand::thread_rng();
        let tile_set = self.config.tile_sets.choose(&mut rng)
            .expect("config has at least one tile sets");
        tile_set.build_rand(&mut world.board)
            .expect("board can be build with tile sets");
    }
}
