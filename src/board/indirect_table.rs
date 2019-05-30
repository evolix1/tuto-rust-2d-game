use crate::positionning::{Pos, Way, Hit};
use super::{BoardError, MovePossibility, GameBoard};


#[derive(Debug)]
pub struct BoardByIndirectTable {
    // Walls for X (horizontal movement). Walls on the right.
    walls_to_move_on_x: Vec<Vec<isize>>,
    // Walls for Y (vertical movement). Walls on the bottom.
    walls_to_move_on_y: Vec<Vec<isize>>
}


impl BoardByIndirectTable {
    #[allow(dead_code)]
    pub fn new(rows: isize, columns: isize) -> Result<BoardByIndirectTable, BoardError> {
        if rows >= 2 && columns >= 2 {
            let mut walls_to_move_on_x: Vec<Vec<isize>> = (0..rows).map(|_| Vec::new()).collect();
            let mut walls_to_move_on_y: Vec<Vec<isize>> = (0..columns).map(|_| Vec::new()).collect();

            walls_to_move_on_x[0].extend(&[4, 11]);
            walls_to_move_on_x[1].extend(&[2]);
            walls_to_move_on_y[2].extend(&[1]);
            
            Ok(BoardByIndirectTable { walls_to_move_on_x, walls_to_move_on_y })
        } else {
            Err(BoardError::InvalidDimensions)
        }
    }


    fn column_at(&self, x: isize) -> Result<&Vec<isize>, BoardError> {
        if x >= 0 { 
            self.walls_to_move_on_y
                .get(x as usize)
                .ok_or(BoardError::InvalidPosition)
        }
        else { 
            Err(BoardError::InvalidPosition)
        }
    }


    fn row_at(&self, y: isize) -> Result<&Vec<isize>, BoardError> {
        if y >= 0 { 
            self.walls_to_move_on_x
                .get(y as usize)
                .ok_or(BoardError::InvalidPosition)
        }
        else { 
            Err(BoardError::InvalidPosition)
        }
    }
        
}


impl GameBoard for BoardByIndirectTable {
    fn row_count(&self) -> isize {
        self.walls_to_move_on_x.len() as isize
    }

    fn column_count(&self) -> isize {
        self.walls_to_move_on_y.len() as isize
    }

    fn is_start_pos(&self, _pos: &Pos) -> Result<bool, BoardError> {
        Ok(true)
    }


    fn moves_from(&self, start: &Pos) -> Result<MovePossibility, BoardError> {
        let mut moves = MovePossibility::all();

        {
            let row = self.row_at(start.y)?;
            moves.left = !row.contains(&(start.x - 1));
            moves.right = !row.contains(&start.x);
        }
        
        {
            let column = self.column_at(start.x)?;
            moves.up = !column.contains(&(start.y - 1));
            moves.down = !column.contains(&start.y);
        }
        
        Ok(moves)
    }


    fn hit_from(&self, start: &Pos, way: Way) -> Result<Hit, BoardError> {
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
