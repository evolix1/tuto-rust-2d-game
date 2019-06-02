use crate::positionning::{Pos, Way, Hit};

use super::error::{Error, Result};
use super::moves::MovePossibility;
use super::dim::Dimensions;
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
            .ok_or_else(|| self.oob_error(Pos::new(x, 0)))
    }


    fn row_at(&self, y: usize) -> Result<&Vec<usize>> {
        self.walls_to_move_on_x
            .get(y)
            .ok_or_else(|| self.oob_error(Pos::new(0, y)))
    }


    fn mut_column_at(&mut self, x: usize) -> Result<&mut Vec<usize>> {
        let err = self.oob_error(Pos::new(x, 0));
        self.walls_to_move_on_y
            .get_mut(x)
            .ok_or(err)
    }


    fn mut_row_at(&mut self, y: usize) -> Result<&mut Vec<usize>> {
        let err = self.oob_error(Pos::new(0, y));
        self.walls_to_move_on_x
            .get_mut(y)
            .ok_or(err)
    }


    fn row_count(&self) -> usize {
        self.walls_to_move_on_x.len()
    }


    fn column_count(&self) -> usize {
        self.walls_to_move_on_y.len()
    }

}


impl Board for BoardByIndirectTable {
    fn dim(&self) -> Dimensions {
        Dimensions {
            rows: self.row_count(),
            columns: self.column_count(),
        }
    }

    fn is_start_pos(&self, _pos: &Pos) -> Result<bool> {
        Ok(true)
    }


    fn moves_from(&self, start: &Pos) -> Result<MovePossibility> {
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


    fn hit_from(&self, start: &Pos, way: Way) -> Result<Hit> {
        let edge = self.side_hit(start, way)?;

        let wall_pos = match way {
            Way::Up => {
                self.column_at(start.x)?
                    .iter()
                    .filter(|&wall| wall < &start.y)
                    .max()
                    .map(|&wall| Pos::new(start.x, wall + 1))
            },
            Way::Down => {
                self.column_at(start.x)?
                    .iter()
                    .filter(|&wall| wall >= &start.y)
                    .min()
                    .map(|&wall| Pos::new(start.x, wall))
            },
            Way::Left => {
                self.row_at(start.y)?
                    .iter()
                    .filter(|&wall| wall < &start.x)
                    .max()
                    .map(|&wall| Pos::new(wall + 1, start.y))
            },
            Way::Right => {
                self.row_at(start.y)?
                    .iter()
                    .filter(|&wall| wall >= &start.x)
                    .min()
                    .map(|&wall| Pos::new(wall, start.y))
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

    fn reset(&mut self, dim: Dimensions) -> Result<()> {
        if dim.rows >= 2 && dim.columns >= 2 {
            self.walls_to_move_on_x = (0..dim.rows).map(|_| Vec::new()).collect();
            self.walls_to_move_on_y = (0..dim.columns).map(|_| Vec::new()).collect();
            Ok(())
        } 
        else {
            Err(Error::DimensionsNotSuitableForBoard)
        }
    }


    fn put_wall(&mut self, pos: &Pos, way: Way) -> Result<()> {
        self.if_exists(pos)
            .and_then(|_| {
                match way {
                    Way::Up => {
                        if pos.x != 0 {
                            self.mut_column_at(pos.x)?.push(pos.y - 1);
                        }
                    },
                    Way::Down => {
                        if pos.x + 1 != self.row_count() {
                            self.mut_column_at(pos.x)?.push(pos.y);
                        } 
                    },
                    Way::Left => {
                        if pos.y != 0 {
                            self.mut_row_at(pos.y)?.push(pos.x - 1);
                        }
                    },
                    Way::Right => {
                        if pos.y + 1 != self.column_count() {
                            self.mut_row_at(pos.y)?.push(pos.x);
                        } 
                    },
                }

                Ok(())
            })
    }
}
