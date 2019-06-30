use crate::positionning::{LogicalPos, PosExtra, Way, Hit, SideLength};
use crate::moves::MovePossibility;
use crate::wall::{Wall, Side};

use super::error::*;
use super::board::{Board, EditableBoard};


#[derive(Debug)]
pub struct BoardByIndirectTable {
    // Walls for X (horizontal movement). Walls on the right.
    walls_to_move_on_x: Vec<Vec<usize>>,
    // Walls for Y (vertical movement). Walls on the bottom.
    walls_to_move_on_y: Vec<Vec<usize>>
}


impl BoardByIndirectTable {
    #[allow(dead_code)]
    pub fn new() -> BoardByIndirectTable {
        let walls_to_move_on_x = Vec::new();
        let walls_to_move_on_y = Vec::new();

        BoardByIndirectTable {
            walls_to_move_on_x,
            walls_to_move_on_y
        }
    }


    fn column_at(&self, x: usize) -> Result<&Vec<usize>> {
        self.walls_to_move_on_y
            .get(x)
            .ok_or_else(|| self.oob_error(LogicalPos{ x, y: 0 }).into())
    }


    fn row_at(&self, y: usize) -> Result<&Vec<usize>> {
        self.walls_to_move_on_x
            .get(y)
            .ok_or_else(|| self.oob_error(LogicalPos{ x: 0, y }).into())
    }


    fn mut_column_at(&mut self, x: usize) -> Result<&mut Vec<usize>> {
        let err = self.oob_error(LogicalPos{ x, y: 0 });
        self.walls_to_move_on_y
            .get_mut(x)
            .ok_or(err.into())
    }


    fn mut_row_at(&mut self, y: usize) -> Result<&mut Vec<usize>> {
        let err = self.oob_error(LogicalPos{ x: 0, y });
        self.walls_to_move_on_x
            .get_mut(y)
            .ok_or(err.into())
    }


    fn row_count(&self) -> usize {
        self.walls_to_move_on_x.len()
    }


    fn column_count(&self) -> usize {
        self.walls_to_move_on_y.len()
    }

}


impl Board for BoardByIndirectTable {
    fn side_length(&self) -> SideLength {
        SideLength(self.row_count())
    }

    fn is_start_pos(&self, _pos: &LogicalPos) -> Result<bool> {
        Ok(true)
    }


    fn moves_from(&self, start: &LogicalPos) -> Result<MovePossibility> {
        let mut moves = MovePossibility::none();

        {
            let row = self.row_at(start.y)?;
            moves.left = start.x > 0 && !row.contains(&(start.x - 1));
            moves.right = start.x + 1 < self.column_count() && !row.contains(&start.x);
        }

        {
            let column = self.column_at(start.x)?;
            moves.up = start.y > 0 && !column.contains(&(start.y - 1));
            moves.down = start.y + 1 < self.row_count() && !column.contains(&start.y);
        }

        Ok(moves)
    }


    fn hit_from(&self, start: &LogicalPos, way: Way) -> Result<Hit> {
        let edge = self.side_hit(start, way)?;

        let wall_pos = match way {
            Way::Up => {
                self.column_at(start.x)?
                    .iter()
                    .filter(|&wall| wall < &start.y)
                    .max()
                    .map(|&wall| LogicalPos{ x: start.x, y: wall + 1 })
            },
            Way::Down => {
                self.column_at(start.x)?
                    .iter()
                    .filter(|&wall| wall >= &start.y)
                    .min()
                    .map(|&wall| LogicalPos{ x: start.x, y: wall })
            },
            Way::Left => {
                self.row_at(start.y)?
                    .iter()
                    .filter(|&wall| wall < &start.x)
                    .max()
                    .map(|&wall| LogicalPos{ x: wall + 1, y: start.y })
            },
            Way::Right => {
                self.row_at(start.y)?
                    .iter()
                    .filter(|&wall| wall >= &start.x)
                    .min()
                    .map(|&wall| LogicalPos{ x: wall, y: start.y })
            },
        };

        Ok(wall_pos
           .map(|pos| {
               let distance = start.distance_to(&pos, way);
               Hit{ pos, distance }
           })
           .unwrap_or(edge))
    }
}


impl EditableBoard for BoardByIndirectTable {

    fn reset(&mut self, side_length: &SideLength) -> Result<()> {
        let side = side_length.0;
        if side >= 2 {
            self.walls_to_move_on_x = (0..side).map(|_| Vec::new()).collect();
            self.walls_to_move_on_y = (0..side).map(|_| Vec::new()).collect();
            Ok(())
        }
        else {
            bail!(ErrorKind::InvalidDimensionToBuildBoard)
        }
    }


    fn put_wall(&mut self, wall: &Wall) -> Result<()> {
        self.if_exists(&wall.pos)
            .and_then(|_| {
                match wall.side {
                    Side::Up => {
                        if wall.pos.y != 0 {
                            self.mut_column_at(wall.pos.x)?.push(wall.pos.y - 1);
                        }
                    },
                    Side::Down => {
                        if wall.pos.y + 1 != self.row_count() {
                            self.mut_column_at(wall.pos.x)?.push(wall.pos.y);
                        }
                    },
                    Side::Left => {
                        if wall.pos.x != 0 {
                            self.mut_row_at(wall.pos.y)?.push(wall.pos.x - 1);
                        }
                    },
                    Side::Right => {
                        if wall.pos.x + 1 != self.column_count() {
                            self.mut_row_at(wall.pos.y)?.push(wall.pos.x);
                        }
                    },
                }

                Ok(())
            })
    }
}
