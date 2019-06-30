use crate::positionning::LogicalPos;

use super::robot::RobotId;
use super::Game;
use super::command::{Command, CommandBase, CommandResult};


#[derive(Debug, Clone)]
pub struct MoveRobotCommand {
    robot: RobotId,
    source_pos: LogicalPos,
    target_pos: LogicalPos,
}


impl MoveRobotCommand {
    pub fn new(robot: RobotId, source_pos: LogicalPos, target_pos: LogicalPos) -> MoveRobotCommand {
        MoveRobotCommand {
            robot,
            source_pos,
            target_pos,
        }
    }
}


impl CommandBase for MoveRobotCommand {
    fn redo(&self, game: &mut Game) -> CommandResult<()> {
        game.state.place_robot(self.robot, self.target_pos.clone());
        game.start_move_animation(self.robot, &self.source_pos, &self.target_pos);
        Ok(())
    }

    fn undo(&self, game: &mut Game) -> CommandResult<()> {
        game.state.place_robot(self.robot, self.source_pos.clone());
        game.start_move_animation(self.robot, &self.target_pos, &self.source_pos);
        Ok(())
    }
}


impl Command for MoveRobotCommand {}
