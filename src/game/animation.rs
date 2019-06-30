use crate::positionning::{LogicalPos, PhysicalPos, physical_from_logical};

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
            source_pos: &LogicalPos,
            target_pos: &LogicalPos,
            duration: f32) -> Animation {
        Animation {
            robot_id,
            source_pos: physical_from_logical(source_pos),
            target_pos: physical_from_logical(target_pos),
            time: 0f32,
            duration,
        }
    }
}
