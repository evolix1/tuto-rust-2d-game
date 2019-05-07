use crate::board::Board;

pub struct GameWorld {
    pub board: Board,
}


impl GameWorld {
    pub fn new() -> GameWorld {
        let board = Board::new_custom(8, 16);
        
        GameWorld {
            board
        }
    }
}
