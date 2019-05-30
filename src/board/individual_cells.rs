use std::collections::HashMap;

use crate::positionning::{Pos, Way, Hit};
use super::{BoardError, MovePossibility, GameBoard};


#[derive(Debug)]
pub struct BoardByHashMap {
    pub rows: isize,
    pub columns: isize,
    special_cells: HashMap<Pos, MovePossibility>,
}


impl BoardByHashMap {
    #[allow(dead_code)]
    pub fn new_custom(rows: isize, columns: isize) -> Result<BoardByHashMap, BoardError> {
        if rows >= 2 && columns >= 2 {
            Ok(BoardByHashMap { rows, columns, special_cells: HashMap::new() })
        } else {
            Err(BoardError::InvalidDimensions)
        }
    }


    #[allow(dead_code)]
    pub fn new_default() -> BoardByHashMap {
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
        
        BoardByHashMap { rows: 16, columns: 16, special_cells }
    }
}


impl GameBoard for BoardByHashMap {
    fn row_count(&self) -> isize {
        self.rows
    }

    fn column_count(&self) -> isize {
        self.columns
    }

    fn is_start_pos(&self, _pos: &Pos) -> Result<bool, BoardError> {
        Ok(true)
    }


    fn moves_from(&self, start: &Pos) -> Result<MovePossibility, BoardError> {
        if start.x < 0 && start.x >= self.rows && start.y < 0 && start.y >= self.columns {
            return Err(BoardError::InvalidPosition);
        }

        let default = MovePossibility { up: true, down: true, left: true, right: true };
        Ok(self.special_cells.get(start).unwrap_or(&default).clone())
    }


    fn hit_from(&self, start: &Pos, way: Way) -> Result<Hit, BoardError> {
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
