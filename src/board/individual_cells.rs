use std::collections::HashMap;

use crate::positionning::{LogicalPos, PosExtra, Way, Hit, SideLength};
use crate::moves::MovePossibility;
use crate::wall::{Wall, Side};

use super::error::*;
use super::board::{Board, EditableBoard};


#[derive(Debug)]
pub struct BoardByIndividualCells {
    side_length: SideLength,
    cells: HashMap<LogicalPos, MovePossibility>,
}


impl BoardByIndividualCells {

    #[allow(dead_code)]
    pub fn new() -> BoardByIndividualCells {
        BoardByIndividualCells {
            side_length: SideLength(0),
            cells: HashMap::new(),
        }
    }

}


impl Board for BoardByIndividualCells {
    fn side_length(&self) -> SideLength {
        self.side_length.clone()
    }


    fn is_start_pos(&self, _pos: &LogicalPos) -> Result<bool> {
        Ok(true)
    }


    fn moves_from(&self, start: &LogicalPos) -> Result<MovePossibility> {
        self.if_exists(start)
            .map(|_| {
                let mut moves = self.cells
                    .get(start)
                    .cloned()
                    .unwrap_or_else(MovePossibility::all);

                moves.up &= start.y > 0;
                moves.down &= start.y + 1 < self.side_length.0;
                moves.left &= start.x > 0;
                moves.right &= start.x + 1 < self.side_length.0;
                
                moves.forbidden = !(moves.left | moves.right | moves.up | moves.down);

                moves
            })
    }


    fn hit_from(&self, start: &LogicalPos, way: Way) -> Result<Hit> {
        let edge = self.side_hit(start, way)?;

        // Gather all positions for `start` to `edge`.
        let hit =
            start.direct_path_to(&edge.pos)
            .unwrap_or(Vec::new())
            .into_iter()
            // Keep only positions that block our way
            .filter(|pos| match self.moves_from(&pos) {
                Err(_) => false,
                Ok(moves) => !moves.can_go(&way),
            })
            // Compute hit
            .map(|pos| {
                let distance = start.distance_to(&pos, way);
                Hit{ pos, distance }
            })
            // Keep the closest position
            .filter(|hit| hit.distance >= 0)
                .min_by_key(|hit| hit.distance)
                .unwrap_or(edge);

        Ok(hit)
    }
}


impl EditableBoard for BoardByIndividualCells {

    fn reset(&mut self, side_length: &SideLength) -> Result<()> {
        if side_length.0 >= 2 && side_length.0 >= 2 {
            self.cells.clear();
            self.side_length = side_length.clone();
            Ok(())
        }
        else {
            bail!(ErrorKind::InvalidDimensionToBuildBoard)
        }
    }


    fn put_wall(&mut self, wall: &Wall) -> Result<()> {
        self.if_exists(&wall.pos)
            .map(|_| { match wall.side {
                Side::Up => {
                    if wall.pos.y != 0
                    {
                        self.cells
                            .entry(wall.pos.clone())
                            .or_insert_with(MovePossibility::all)
                            .up = false;

                        self.cells
                            .entry(LogicalPos{ x: wall.pos.x, y: wall.pos.y - 1 })
                            .or_insert_with(MovePossibility::all)
                            .down = false;
                    }
                },
                Side::Down => {
                    if wall.pos.y + 1 != self.side_length.0
                    {
                        self.cells
                            .entry(wall.pos.clone())
                            .or_insert_with(MovePossibility::all)
                            .down = false;

                        self.cells
                            .entry(LogicalPos{ x: wall.pos.x, y: wall.pos.y + 1 })
                            .or_insert_with(MovePossibility::all)
                            .up = false;
                    }
                },
                Side::Left => {
                    if wall.pos.x != 0
                    {
                        self.cells
                            .entry(wall.pos.clone())
                            .or_insert_with(MovePossibility::all)
                            .left = false;

                        self.cells
                            .entry(LogicalPos{ x: wall.pos.x - 1, y: wall.pos.y })
                            .or_insert_with(MovePossibility::all)
                            .right = false;
                    }
                },
                Side::Right => {
                    if wall.pos.x + 1 != self.side_length.0
                    {
                        self.cells
                            .entry(wall.pos.clone())
                            .or_insert_with(MovePossibility::all)
                            .right = false;

                        self.cells
                            .entry(LogicalPos{ x: wall.pos.x + 1, y: wall.pos.y })
                            .or_insert_with(MovePossibility::all)
                            .left = false;
                    }
                },
            }})
    }
}
