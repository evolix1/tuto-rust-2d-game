use crate::positionning::{Pos, Way, Hit};


#[derive(Debug)]
pub struct Board{
    pub rows: isize,
    pub columns: isize
}


#[derive(Debug)]
pub enum BoardError {
    InvalidDimension
}


impl Board {
    pub fn new_custom(rows: isize, columns: isize) -> Result<Board, BoardError> {
        if rows >= 2 && columns >= 2 {
            Ok(Board { rows, columns })
        } else {
            Err(BoardError::InvalidDimension)
        }
    }


    pub fn is_start_pos(&self, _pos: &Pos) -> bool {
        true
    }


    pub fn hit_from(&self, start: &Pos, way: Way) -> Hit {
        self.side_hit(start, way)
    }


    fn side_hit(&self, start: &Pos, way: Way) -> Hit {
        let outside_pos = match way {
            Way::Up => Pos::new(start.x, -1),
            Way::Down => Pos::new(start.x, self.rows),
            Way::Left => Pos::new(-1, start.y),
            Way::Right => Pos::new(self.columns, start.y),
        };
        
        start.find_hit_to(&outside_pos, way).expect("position must hit")
    }
}
