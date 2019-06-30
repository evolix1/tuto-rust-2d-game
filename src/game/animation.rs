use crate::positionning::PhysicalPos;

use super::robot::RobotId;


pub struct Animation {
    pub robot_id: RobotId,
    pub source_pos: PhysicalPos,
    pub target_pos: PhysicalPos,
    pub time: f32,
    pub duration: f32,
}


impl Animation {
    pub fn new(
            robot_id: RobotId,
            source_pos: PhysicalPos,
            target_pos: PhysicalPos,
            duration: f32) -> Animation {
        Animation {
            robot_id,
            source_pos,
            target_pos,
            time: 0f32,
            duration,
        }
    }
}
