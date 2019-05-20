use crate::board::Board;
use crate::robot::{Robot, RobotId};
use crate::positionning::{Pos, Way};


pub struct GameWorld {
    pub board: Board,
    pub robots: [Robot; 4],
}


pub enum InvalidCommand {
    /// Robot 
    NotPlayingRobot,
}


impl GameWorld {
    pub fn new() -> GameWorld {
        //let board = Board::new_custom(8, 16).expect("valid dimension");
        let board = Board::new_default();
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
            .filter(|pos| self.board.is_start_pos(pos))
            .filter(|pos| self.robots.iter().all(|r| match r.pos {
                Some(ref p) if p == pos => false,
                _ => true
            }))
            .next()
    }


    pub fn move_robot(&mut self, id: RobotId, way: Way) -> Result<(), InvalidCommand> {
        let start_pos = self.robots
            .iter_mut()
            .find(|ref robot| robot.id == id)
            .ok_or(InvalidCommand::NotPlayingRobot)? 
            .pos
            .take()
            .ok_or(InvalidCommand::NotPlayingRobot)?;

        let mut hits = vec![self.board.hit_from(&start_pos, way)];
        hits.extend(
            self.robots.iter()
            .filter(|robot| robot.id != id)
            .filter_map(|robot| robot.pos.as_ref())
            .filter_map(|pos| start_pos.find_hit_to(&pos, way))
        );

        let end_pos = hits
            .into_iter()
            .min_by_key(|hit| hit.distance)
            .map(|hit| hit.pos)
            .expect("at least one hit from the board should exist");

        println!("Robot {:?} move from {:?} to {:?}", id, start_pos, end_pos);

        self.robots.iter_mut()
            .find(|ref robot| robot.id == id)
            .expect("robot exists")
            .pos = Some(end_pos);

        Ok(())
    }
}


impl From<InvalidCommand> for String {
    fn from(err: InvalidCommand) -> Self {
        match err {
            InvalidCommand::NotPlayingRobot => "robot is not playing".into(),
        }
    }
}
