use crate::positionning::{LogicalPos, Way};

use super::robot::RobotId;
use super::game_state::GameState;
use super::world::World;
use super::command::{Command, CommandResult};
use super::move_robot_command::MoveRobotCommand;
use super::animation::Animation;


pub struct Game {
    pub state: GameState,
    pub world: World,
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
    animation: Option<Animation>,
}


impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState::new(),
            world: World::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            animation: None,
        }
    }

    pub fn try_move_robot_in_dir(&mut self, robot: RobotId, way: Way)
        -> CommandResult<bool> {
        let source_pos = self.state
            .robot_pos(robot)
            .ok_or("robot must be placed")?;
        let target_pos = self.state.cast_ray(&source_pos, way);

        if target_pos != source_pos && self.animation.is_none() {
            self.move_robot(robot, target_pos)?;
            Ok(true)
        }
        else {
            Ok(false)
        }
    }

    pub fn move_robot(&mut self, robot: RobotId, target_pos: LogicalPos) -> CommandResult<()> {
        if self.animation.is_some() {
            return Err("Cannot move during animation".into());
        }

        let source_pos = self.state
            .robot_pos(robot)
            .ok_or("robot must be placed")?;
        let command = MoveRobotCommand::new(
            robot,
            source_pos,
            target_pos,
        );

        self.exec_command(Box::new(command))
    }

    fn exec_command(&mut self, command: Box<dyn Command>) -> CommandResult<()> {
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


    pub fn undo(&mut self) -> CommandResult<bool> {
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


    pub fn redo(&mut self) -> CommandResult<bool> {
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
        self.state.reset_rand_pos();
        self.world.reset(&self.state);
    }


    pub fn start_move_animation(
        &mut self,
        robot: RobotId,
        source_pos: &LogicalPos,
        target_pos: &LogicalPos
    ) {
        assert!(self.animation.is_none());
        self.animation = Some(Animation::new(robot, source_pos, target_pos, 1f32));
    }


    pub fn update_animation(&mut self, elapsed: f32) {
        if let Some(mut animation) = self.animation.take() {
            let robot = &mut self.world.robots[0]; // TODO
            animation.time += elapsed;
            if animation.time < animation.duration {
                let t = animation.time / animation.duration;
                robot.pos = Some(
                    (1f32 - t) * animation.source_pos + t * animation.target_pos
                );
                self.animation = Some(animation);
            }
            else {
                robot.pos = Some(animation.target_pos);
            }
        }
    }
}
