use crate::board::Board;
use crate::robot::{Robot, RobotId};
use crate::positionning::Pos;


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


    pub fn reset_rand_pos(&mut self) {
        // cancel previous positions
        for robot in self.robots.iter_mut() {
            robot.pos = None;
        }

        for i in 0..self.robots.len() {
            self.robots[i].pos = self.find_start_pos();
        }
    }


    pub fn find_start_pos(&self) -> Option<Pos> {
        (0..1000)
            .map(|_| Pos::rand(self.board.columns, self.board.rows))
            .filter(|pos| !self.board.has_target_symbol(pos))
            .filter(|pos| self.robots.iter().all(|r| match r.pos {
                Some(ref x) if x == pos => false,
                _ => true
            }))
            .next()
    }
}
