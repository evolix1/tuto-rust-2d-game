use std::collections::HashMap;

use crate::positionning::{Pos, Way, Hit};


#[derive(Debug)]
pub struct Board{
    pub rows: isize,
    pub columns: isize,
    special_cells: HashMap<Pos, MovePossibility>,
}


#[derive(Debug)]
pub enum BoardError {
    InvalidDimension,
    InvalidPosition,
}


#[derive(Debug, Clone)]
pub struct MovePossibility {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}


impl Board {
    #[allow(dead_code)]
    pub fn new_custom(rows: isize, columns: isize) -> Result<Board, BoardError> {
        if rows >= 2 && columns >= 2 {
            Ok(Board { rows, columns, special_cells: HashMap::new() })
        } else {
            Err(BoardError::InvalidDimension)
        }
    }


    #[allow(dead_code)]
    pub fn new_default() -> Board {
        let default = MovePossibility { up: true, down: true, left: true, right: true };
        let special_cells = [
            (Pos::new(4, 0), MovePossibility { right: false, ..default }),
            (Pos::new(5, 0), MovePossibility { left: false, ..default }),
            (Pos::new(11, 0), MovePossibility { right: false, ..default }),
            (Pos::new(12, 0), MovePossibility { left: false, ..default }),
            (Pos::new(2, 1), MovePossibility { down: false, right: false, ..default }),
            (Pos::new(3, 1), MovePossibility { left: false, ..default }),
            (Pos::new(2, 2), MovePossibility { up: false, ..default }),
        ].iter().cloned().collect();
        
        Board { rows: 16, columns: 16, special_cells }
    }


    pub fn is_start_pos(&self, _pos: &Pos) -> bool {
        true
    }


    pub fn possible_moves(&self, pos: &Pos) -> Result<MovePossibility, BoardError> {
        if pos.x < 0 && pos.x >= self.rows && pos.y < 0 && pos.y >= self.columns {
            return Err(BoardError::InvalidPosition);
        }
        
        let default = MovePossibility { up: true, down: true, left: true, right: true };
        Ok(self.special_cells.get(pos).unwrap_or(&default).clone())
    }


    pub fn hit_from(&self, start: &Pos, way: Way) -> Hit {
        let side_pos = self.side_hit(start, way);
        
        start.direct_path_to(&side_pos.pos)
            .unwrap_or(Vec::new())
            .into_iter()
            .filter(|pos| match self.possible_moves(&pos) {
                Err(_) => false,
                Ok(moves) => !moves.can_go(&way),
            })
            .map(|pos| {
                let distance = start.distance_to(&pos, way);
                Hit{ pos, distance }
            })
            .filter(|hit| hit.distance >= 0)
            .min_by_key(|hit| hit.distance)
            .unwrap_or(side_pos)
    }


    fn side_hit(&self, start: &Pos, way: Way) -> Hit {
        let pos = match way {
            Way::Up => Pos::new(start.x, 0),
            Way::Down => Pos::new(start.x, self.rows - 1),
            Way::Left => Pos::new(0, start.y),
            Way::Right => Pos::new(self.columns - 1, start.y),
        };
        
        let distance = start.distance_to(&pos, way);
        Hit { pos, distance }
    }
}


impl MovePossibility {
    pub fn can_go(&self, way: &Way) -> bool {
        match way {
            Way::Up => self.up,
            Way::Down => self.down,
            Way::Left => self.left,
            Way::Right => self.right,
        }
    }
}
