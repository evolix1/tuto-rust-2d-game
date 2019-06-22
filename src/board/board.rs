use crate::positionning::{Pos, Way, Hit, SideLength};
use crate::moves::MovePossibility;
use crate::wall::Wall;

use super::error::{Error, Result};


pub trait Board {
    fn side_length(&self) -> SideLength;

    // Check if position exists on the board.
    fn pos_exists(&self, pos: &Pos) -> bool {
        let side = self.side_length().0;
        pos.x < side && pos.y < side
    }
    
    // Test whether the given position can be used to start a robot on.
    fn is_start_pos(&self, pos: &Pos) -> Result<bool>;
    
    // Evaluate what actions can be done at given position.
    fn moves_from(&self, start: &Pos) -> Result<MovePossibility>;
    
    // Indicates the position of hit with the board content.
    fn hit_from(&self, start: &Pos, way: Way) -> Result<Hit>;

    // Find the hit according only to board SideLength.
    fn side_hit(&self, start: &Pos, way: Way) -> Result<Hit> {
        self.if_exists(start)
            .map(|_| {
                let side = self.side_length().0;
                assert!(side >= 1);

                let pos = match way {
                    Way::Up => Pos::new(start.x, 0),
                    Way::Down => Pos::new(start.x, side - 1),
                    Way::Left => Pos::new(0, start.y),
                    Way::Right => Pos::new(side - 1, start.y),
                };

                let distance = start.distance_to(&pos, way);
                Hit { pos, distance }
            })
    }

    fn oob_error(&self, pos: Pos) -> Error {
        let side_length = self.side_length();
        Error::OutOfBoardPosition{ pos, side_length }
    }

    fn if_exists(&self, pos: &Pos) -> Result<()> {
        if self.pos_exists(&pos) { Ok(()) }
        else { Err(self.oob_error(pos.clone())) }
    }
}


pub trait EditableBoard: Board {
    fn reset(&mut self, side_length: &SideLength) -> Result<()>;

    fn put_wall(&mut self, wall: &Wall) -> Result<()>;
}
