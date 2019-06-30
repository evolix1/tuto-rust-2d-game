use crate::positionning::LogicalPos;


/// Robot identity (designated by color, like ghost in pacman)
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RobotId {
    Red,
    Green,
    Blue,
    Yellow,
}


#[derive(Debug)]
pub struct Robot {
    pub id: RobotId,
    pub pos: Option<LogicalPos>
}


impl Robot {
    pub fn new(id: RobotId) -> Robot {
        Robot { id, pos: None }
    }
}
