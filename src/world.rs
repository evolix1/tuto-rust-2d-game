use crate::robot::{Robot, RobotId};
use crate::positionning::{Pos, Way};

#[allow(unused_imports)]
use crate::board::{
    EditableBoard, 
    BoardByIndividualCells, 
    BoardByIndirectTable,
};


pub struct GameWorld {
    pub board: Box<dyn EditableBoard>,
    pub robots: [Robot; 4],
}


pub enum InvalidCommand {
    /// Robot 
    NotPlayingRobot,
}


impl GameWorld {
    pub fn new() -> GameWorld {
        let board = Box::new(BoardByIndirectTable::new());
        //let board = Box::new(BoardByIndividualCells::new());
        
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
        let dim = self.board.dim();
        (0..1000)
            .map(|_| Pos::rand(dim.columns, dim.rows))
            .filter(|pos| self.board.is_start_pos(pos).unwrap_or(false))
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

        let mut hits = vec![
            self.board.hit_from(&start_pos, way)
                .expect("board can at least hit the wall")
        ];
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
