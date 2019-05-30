use crate::positionning::{Pos, Way, Hit};

use super::{Error, Result};
use super::dim::Dimensions;
use super::moves::MovePossibility;


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
        if self.pos_exists(start) {
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
            Ok(Hit { pos, distance })
        } 
        else {
            return Err(Error::OutOfBoardPosition);
        }
    }
}


pub trait EditableBoard: Board {
    fn reset(&mut self, dim: Dimensions) -> Result<()>;
    
    fn put_wall(&mut self, pos: &Pos, way: Way) -> Result<()>;
    
    fn put_walls(&mut self, pos: &Pos, ways: &[Way]) -> Result<()> {
        for way in ways {
            self.put_wall(pos, *way)?;
        }

        Ok(())
    }
}
