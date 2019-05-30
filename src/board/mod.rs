use crate::positionning::{Pos, Way, Hit};


#[derive(Debug)]
pub enum BoardError {
    InvalidDimensions,
    InvalidPosition,
}


#[derive(Debug, Clone)]
pub struct MovePossibility {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}


pub trait GameBoard {
    // Number of rows
    fn row_count(&self) -> isize;
    
    // Number of columns
    fn column_count(&self) -> isize;
    
    // Pair of column count and row count
    fn dim(&self) -> (isize, isize) {
        (self.column_count(), self.row_count())
    }

    // Check if position exists on the board.
    fn pos_exists(&self, pos: &Pos) -> bool {
        pos.x >= 0 
            && pos.y >= 0 
            && pos.x < self.column_count()
            && pos.y < self.row_count()
    }
    
    // Test whether the given position can be used to start a robot on.
    fn is_start_pos(&self, pos: &Pos) -> Result<bool, BoardError>;
    
    // Evaluate what actions can be done at given position.
    fn moves_from(&self, start: &Pos) -> Result<MovePossibility, BoardError>;
    
    // Indicates the position of hit with the board content.
    fn hit_from(&self, start: &Pos, way: Way) -> Result<Hit, BoardError>;

    // Find the hit according only to board dimensions.
    fn side_hit(&self, start: &Pos, way: Way) -> Result<Hit, BoardError> {
        if !self.pos_exists(start) {
            return Err(BoardError::InvalidPosition);
        }

        let pos = match way {
            Way::Up => Pos::new(start.x, 0),
            Way::Down => Pos::new(start.x, self.row_count() - 1),
            Way::Left => Pos::new(0, start.y),
            Way::Right => Pos::new(self.column_count() - 1, start.y),
        };
        
        let distance = start.distance_to(&pos, way);
        Ok(Hit { pos, distance })
    }
}


impl MovePossibility {
    #[allow(dead_code)]
    pub fn can_go(&self, way: &Way) -> bool {
        match way {
            Way::Up => self.up,
            Way::Down => self.down,
            Way::Left => self.left,
            Way::Right => self.right,
        }
    }

    #[allow(dead_code)]
    pub fn all() -> MovePossibility {
        MovePossibility { up: true, down: true, left: true, right: true }
    }

    #[allow(dead_code)]
    pub fn none() -> MovePossibility {
        MovePossibility { up: false, down: false, left: false, right: false }
    }
}

//
mod individual_cells;
mod indirect_table;
pub use self::individual_cells::BoardByHashMap;
pub use self::indirect_table::BoardByIndirectTable;
