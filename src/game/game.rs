use crate::robot::RobotId;
use crate::positionning::{LogicalPos, Way};
use crate::world::GameWorld;

use super::error::*;
use super::command::Command;
use super::move_robot_command::MoveRobotCommand;


pub struct Game {
    pub world: GameWorld,
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
}


impl Game {
    pub fn new() -> Game {
        Game {
            world: GameWorld::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn try_move_robot_in_dir(&mut self, robot: RobotId, way: Way)
        -> Result<bool> {
        let source_pos = self.world
            .robot_pos(robot)
            .ok_or_else(|| ErrorKind::RobotHasNoPosition)?;
        let target_pos = self.world.cast_ray(&source_pos, way);

        if target_pos != source_pos {
            self.move_robot(robot, target_pos)?;
            Ok(true)
        }
        else {
            Ok(false)
        }
    }

    pub fn move_robot(&mut self, robot: RobotId, target_pos: LogicalPos) -> Result<()> {
        let source_pos = self.world
            .robot_pos(robot)
            .ok_or_else(|| ErrorKind::RobotHasNoPosition)?;
        let command = MoveRobotCommand::new(
            robot,
            source_pos,
            target_pos,
        );

        self.exec_command(Box::new(command))
    }

    fn exec_command(&mut self, command: Box<dyn Command>) -> Result<()> {
        println!("Exec command: {:?}", command);
        let res = command.redo(self);
        self.undo_stack.push(command);
        self.redo_stack.clear();
        res
    }


    pub fn clear_undo_stack(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }


    pub fn undo(&mut self) -> Result<bool> {
        match self.undo_stack.pop() {
            Some(command) => {
                command.undo(self)?;
                self.redo_stack.push(command);
                Ok(true)
            }
            None => {
                Ok(false)
            }
        }
    }


    pub fn redo(&mut self) -> Result<bool> {
        match self.redo_stack.pop() {
            Some(command) => {
                command.redo(self)?;
                self.undo_stack.push(command);
                Ok(true)
            }
            None => {
                Ok(false)
            }
        }
    }


    pub fn reset_rand_pos(&mut self) {
        self.clear_undo_stack();
        self.world.reset_rand_pos();
    }
}
