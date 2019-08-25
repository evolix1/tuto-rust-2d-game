use cgmath::prelude::*;

use crate::positionning::PhysicalPos;

use super::robot::RobotId;
use super::world::World;
use super::game_state::GameState;


pub trait Animation {
    fn get_time(&self) -> f32;
    fn get_duration(&self) -> f32;
    fn render(
        &mut self,
        state: &GameState,
        world: &mut World,
        elapsed: f32
    ) -> bool;
}


pub struct MoveRobotAnimation {
    robot_id: RobotId,
    source_pos: PhysicalPos,
    target_pos: PhysicalPos,
    time: f32,
    duration: f32,
}


impl MoveRobotAnimation {
    pub fn new(
            robot_id: RobotId,
            source_pos: PhysicalPos,
            target_pos: PhysicalPos,
            duration: f32) -> MoveRobotAnimation {
        MoveRobotAnimation {
            robot_id,
            source_pos,
            target_pos,
            time: 0f32,
            duration,
        }
    }
}

impl Animation for MoveRobotAnimation {
    fn get_time(&self) -> f32 {
        self.time
    }

    fn get_duration(&self) -> f32 {
        self.duration
    }

    fn render(
        &mut self,
        state: &GameState,
        world: &mut World,
        elapsed: f32
    ) -> bool {
        let robot_index = match state.robot_index(self.robot_id) {
            Some(index) => { index },
            None => { return false; }
        };
        let robot = &mut world.robots[robot_index];

        self.time += elapsed;
        if self.time < self.duration {
            let t = self.time / self.duration;
            robot.pos = Some(
                self.source_pos.lerp(self.target_pos, t)
            );
            return true;
        }
        else {
            robot.pos = Some(self.target_pos);
            return false;
        }
    }
}


pub struct AnimationSequence {
    animations: Vec<Box<dyn Animation>>,
    time: f32,
    duration: f32,
    current_animation: usize,
}

impl AnimationSequence {
    pub fn new() -> AnimationSequence {
        AnimationSequence {
            animations: Vec::new(),
            time: 0.0f32,
            duration: 0.0f32,
            current_animation: 0,
        }
    }

    pub fn add_animation(&mut self, animation: Box<dyn Animation>) {
        self.duration += animation.get_duration();
        self.animations.push(animation);
    }
}

impl Animation for AnimationSequence {
    fn get_time(&self) -> f32 {
        self.time
    }

    fn get_duration(&self) -> f32 {
        self.duration
    }

    fn render(
        &mut self,
        state: &GameState,
        world: &mut World,
        elapsed: f32
    ) -> bool {
        let mut remaining = elapsed;
        while self.current_animation < self.animations.len()
        && self.animations[self.current_animation].get_time() + remaining >= self.animations[self.current_animation].get_duration() {
            let animation = &self.animations[self.current_animation];
            remaining -= animation.get_duration() - animation.get_time();
            self.current_animation += 1;
        }

        self.time += elapsed;

        if self.current_animation < self.animations.len() {
            let animation = &mut self.animations[self.current_animation];
            animation.render(state, world, remaining);
            self.time < self.duration
        }
        else {
            let last_index = self.animations.len() - 1;
            let animation = &mut self.animations[last_index];
            animation.render(state, world, elapsed);
            false
        }
    }
}
