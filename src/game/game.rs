use cgmath::prelude::*;

use crate::positionning::{LogicalPos, Way, physical_from_logical};

use super::error::*;
use super::robot::RobotId;
use super::game_state::GameState;
use super::world::World;
use super::command::Command;
use super::move_robot_command::MoveRobotCommand;
use super::animation::{
    Animation,
    MoveRobotAnimation,
    AnimationSequence,
};


pub struct Game {
    pub state: GameState,
    pub world: World,
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
    animation: Option<AnimationSequence>,
    animation_speed: f32,
}


impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState::new(),
            world: World::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            animation: None,
            animation_speed: 1.0f32,
        }
    }

    pub fn try_move_robot_in_dir(&mut self, robot: RobotId, way: Way)
        -> Result<bool> {
        let source_pos = self.state
            .robot_pos(robot)
            .ok_or(ErrorKind::RobotHasNoPosition)?;
        let target_pos = self.state.cast_ray(&source_pos, way);

        if target_pos != source_pos {
            self.move_robot(robot, target_pos)?;
            Ok(true)
        }
        else {
            Ok(false)
        }
    }

    pub fn move_robot(&mut self, robot: RobotId, target_pos: LogicalPos) -> Result<()> {
        let source_pos = self.state
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
        self.state.reset_rand_pos();
        self.world.reset(&self.state);
    }


    pub fn start_move_animation(
        &mut self,
        robot: RobotId,
        source_pos: &LogicalPos,
        target_pos: &LogicalPos
    ) {
        let mut animation = match self.animation.take() {
            Some(animation) => { animation }
            None => { AnimationSequence::new() }
        };

        let source_pos = physical_from_logical(source_pos);
        let target_pos = physical_from_logical(target_pos);
        let duration = 0.04 * source_pos.distance(target_pos);

        animation.add_animation(
            Box::new(
                MoveRobotAnimation::new(
                    robot, source_pos, target_pos, duration
                )
            )
        );
        self.animation_speed = (animation.get_duration() - animation.get_time()).sqrt();

        self.animation = Some(animation);
    }


    pub fn update_animation(&mut self, elapsed: f32) {
        if let Some(mut animation) = self.animation.take() {
            if animation.render(&self.state, &mut self.world, elapsed * self.animation_speed) {
                self.animation = Some(animation);
            }
        }
    }
}
