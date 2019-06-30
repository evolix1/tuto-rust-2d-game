use super::RobotId;
use super::GameState;
use super::robot::Robot;

use crate::positionning::{PhysicalPos, physical_from_logical};

#[derive(Debug, Clone)]
pub struct RobotState {
    pub id: RobotId,
    pub pos: Option<PhysicalPos>,
}


#[derive(Debug)]
pub struct World {
    pub robots: Vec<RobotState>,
}


impl From<&Robot> for RobotState {
    fn from(robot: &Robot) -> Self {
        let pos = robot.pos.as_ref().map(physical_from_logical);

        Self {
            id: robot.id,
            pos,
        }
    }
}


impl World {
    pub fn new(game_state: &GameState) -> World {
        let robots = game_state.robots.iter()
            .map(|robot| robot.into())
            .collect();

        World {
            robots,
        }
    }
}
