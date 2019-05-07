use crate::positionning::Pos;


/// Robot identity (designated by color, like ghost in pacman)
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum RobotId {
    Red,
    Green,
    Blue,
    Yellow,
}


pub struct Robot {
    pub id: RobotId,
    pub pos: Option<Pos>
}


impl Robot {
    pub fn new(id: RobotId) -> Robot {
        Robot { id, pos: None }
    }
}
