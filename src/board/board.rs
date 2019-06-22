use crate::positionning::{Pos, Way, Hit};
use crate::dim::Dimensions;
use crate::moves::MovePossibility;
use crate::wall::Wall;

use super::error::{Error, Result};


pub trait Board {
    fn dim(&self) -> Dimensions;

    // Check if position exists on the board.
    fn pos_exists(&self, pos: &Pos) -> bool {
        let dim = self.dim();
        pos.x < dim.columns && pos.y < dim.rows
    }
    
    // Test whether the given position can be used to start a robot on.
    fn is_start_pos(&self, pos: &Pos) -> Result<bool>;
    
    // Evaluate what actions can be done at given position.
    fn moves_from(&self, start: &Pos) -> Result<MovePossibility>;
    
    // Indicates the position of hit with the board content.
    fn hit_from(&self, start: &Pos, way: Way) -> Result<Hit>;

    // Find the hit according only to board dimensions.
    fn side_hit(&self, start: &Pos, way: Way) -> Result<Hit> {
        self.if_exists(start)
            .map(|_| {
                let dim = self.dim();
                assert!(dim.rows >= 1);
                assert!(dim.columns >= 1);

                let pos = match way {
                    Way::Up => Pos::new(start.x, 0),
                    Way::Down => Pos::new(start.x, dim.rows - 1),
                    Way::Left => Pos::new(0, start.y),
                    Way::Right => Pos::new(dim.columns - 1, start.y),
                };

                let distance = start.distance_to(&pos, way);
                Hit { pos, distance }
            })
    }

    fn oob_error(&self, pos: Pos) -> Error {
        let dim = self.dim();
        Error::OutOfBoardPosition{ pos, dim }
    }

    fn if_exists(&self, pos: &Pos) -> Result<()> {
        if self.pos_exists(&pos) { Ok(()) }
        else { Err(self.oob_error(pos.clone())) }
    }
}


pub trait EditableBoard: Board {
    fn reset(&mut self, dim: &Dimensions) -> Result<()>;

    fn put_wall(&mut self, wall: &Wall) -> Result<()>;
}
