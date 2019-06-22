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


    pub fn robot(&self, robot_id: RobotId) -> Option<&Robot> {
        self.robots.iter()
            .find(|ref robot| robot.id == robot_id)
    }


    pub fn robot_mut(&mut self, robot_id: RobotId) -> Option<&mut Robot> {
        self.robots.iter_mut()
            .find(|ref robot| robot.id == robot_id)
    }


    pub fn robot_pos(&self, robot_id: RobotId) -> Option<Pos> {
        self.robot(robot_id)?
            .pos
            .clone()
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


    pub fn cast_ray(&self, source_pos: &Pos, way: Way) -> Pos {
        let mut hits = vec![
            self.board.hit_from(&source_pos, way)
                .expect("board can at least hit the wall")
        ];

        hits.extend(
            self.robots.iter()
            .filter(|robot| robot.pos.as_ref() != Some(source_pos))
            .filter_map(|robot| robot.pos.as_ref())
            .filter_map(|pos| source_pos.find_hit_to(&pos, way))
        );

        hits.into_iter()
            .min_by_key(|hit| hit.distance)
            .map(|hit| hit.pos)
            .expect("at least one hit from the board should exist")
    }


    pub fn place_robot(&mut self, robot: RobotId, pos: Pos) {
        self.robot_mut(robot)
            .expect("robot exists")
            .pos = Some(pos);
    }
}


impl From<InvalidCommand> for String {
    fn from(err: InvalidCommand) -> Self {
        match err {
            InvalidCommand::NotPlayingRobot => "robot is not playing".into(),
        }
    }
}
