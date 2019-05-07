use crate::board::Board;
use crate::robot::{Robot, RobotId};


pub struct GameWorld {
    pub board: Board,
    pub robots: [Robot; 4],
}


impl GameWorld {
    pub fn new() -> GameWorld {
        let board = Board::new_custom(8, 16);
        let robots = [
            Robot::new(RobotId::Red),
            Robot::new(RobotId::Green),
            Robot::new(RobotId::Blue),
            Robot::new(RobotId::Yellow),
        ];
        
        GameWorld {
            board,
            robots
        }
    }
}
