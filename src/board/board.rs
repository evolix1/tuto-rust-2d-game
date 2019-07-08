use crate::positionning::{LogicalPos, PosExtra, Way, Hit, SideLength};
use crate::moves::MovePossibility;
use crate::wall::{Wall, Side};

use super::error::*;


pub trait Board {
    fn side_length(&self) -> SideLength;

    // Check if position exists on the board.
    fn pos_exists(&self, pos: &LogicalPos) -> bool {
        let side = self.side_length().0;
        pos.x < side && pos.y < side
    }

    // Test whether the given position can be used to start a robot on.
    fn is_start_pos(&self, pos: &LogicalPos) -> Result<bool> {
        self.moves_from(pos)
            .map(|moves| !moves.forbidden)
    }

    // Evaluate what actions can be done at given position.
    fn moves_from(&self, start: &LogicalPos) -> Result<MovePossibility>;

    // Indicates the position of hit with the board content.
    fn hit_from(&self, start: &LogicalPos, way: Way) -> Result<Hit>;

    // Find the hit according only to board SideLength.
    fn side_hit(&self, start: &LogicalPos, way: Way) -> Result<Hit> {
        self.if_exists(start)
            .map(|_| {
                let side = self.side_length().0;
                assert!(side >= 1);

                let pos = match way {
                    Way::Up => LogicalPos{ x: start.x, y: 0 },
                    Way::Down => LogicalPos{ x: start.x, y: side - 1 },
                    Way::Left => LogicalPos{ x: 0, y: start.y },
                    Way::Right => LogicalPos{ x: side - 1, y: start.y },
                };

                let distance = start.distance_to(&pos, way);
                Hit { pos, distance }
            })
    }

    fn oob_error(&self, pos: LogicalPos) -> ErrorKind {
        let side_length = self.side_length();
        ErrorKind::OutOfBoardPosition(pos, side_length)
    }

    fn if_exists(&self, pos: &LogicalPos) -> Result<()> {
        if self.pos_exists(&pos) { Ok(()) }
        else { bail!(self.oob_error(pos.clone())) }
    }
}


pub trait EditableBoard: Board {
    fn reset(&mut self, side_length: &SideLength) -> Result<()>;

    fn put_wall(&mut self, wall: &Wall) -> Result<()>;
    
    fn forbid_cell(&mut self, pos: &LogicalPos) -> Result<()> {
        let side = self.side_length().0;
        
        if pos.y > 0 {
            let top_pos = LogicalPos{ y: pos.y - 1, ..*pos };
            self.put_wall(&Wall { pos: top_pos, side: Side::Down })?;
        }
        
        if pos.y + 1 < side {
            let bottom_pos = LogicalPos{ y: pos.y + 1, ..*pos };
            self.put_wall(&Wall { pos: bottom_pos, side: Side::Up })?;
        }
        
        if pos.x > 0 {
            let left_pos = LogicalPos{ x: pos.x - 1, ..*pos };
            self.put_wall(&Wall { pos: left_pos, side: Side::Right })?;
        }
        
        if pos.x + 1 < side {
            let right_pos = LogicalPos{ x: pos.x + 1, ..*pos };
            self.put_wall(&Wall { pos: right_pos, side: Side::Left })?;
        }

        Ok(())
    }
}
