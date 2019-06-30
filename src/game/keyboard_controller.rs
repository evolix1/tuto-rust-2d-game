use sdl2::keyboard::Scancode;
use sdl2::event::Event;

use crate::robot::RobotId;
use crate::positionning::Way;

use super::error::*;
use super::Game;


#[derive(Debug)]
pub struct KeyboardController {
    move_keys: [Scancode; 4],
    robot_keys: [Scancode; 4],
    current_robot: RobotId,
}


impl KeyboardController {
    pub fn new() -> KeyboardController {
        let move_keys = [
            Scancode::Up,
            Scancode::Down,
            Scancode::Left,
            Scancode::Right,
        ];
        let robot_keys = [
            Scancode::Num1,
            Scancode::Num2,
            Scancode::Num3,
            Scancode::Num4,
        ];

        KeyboardController {
            move_keys,
            robot_keys,
            current_robot: RobotId::Red,
        }
    }

    pub fn process_event(&mut self, game: &mut Game, event: &Event) -> Result<bool> {
        match event {
            Event::KeyDown { scancode: Some(scancode), repeat: false, .. } => {
                self.process_keypress(game, scancode)
            }
            _ => {
                Ok(false)
            }
        }
    }

    pub fn process_keypress(&mut self, game: &mut Game, scancode: &Scancode) -> Result<bool> {
        if let Some(way) = self.way_from_scancode(scancode) {
            game.try_move_robot_in_dir(self.current_robot, way)?;
            Ok(true)
        }
        else if let Some(robot) = self.robot_from_scancode(scancode) {
            self.current_robot = robot;
            println!("Switch to robot {:?}", robot);
            Ok(true)
        }
        else {
            Ok(false)
        }
    }

    fn way_from_scancode(&self, scancode: &Scancode) -> Option<Way> {
        let way_index = self.move_keys.iter()
            .enumerate()
            .filter(|&(_, sc)| sc == scancode)
            .next()?.0;
        Some(WAY_FROM_INDEX[way_index])
    }

    fn robot_from_scancode(&self, scancode: &Scancode) -> Option<RobotId> {
        let way_index = self.robot_keys.iter()
            .enumerate()
            .filter(|&(_, sc)| sc == scancode)
            .next()?.0;
        Some(ROBOT_FROM_INDEX[way_index])
    }
}


const WAY_FROM_INDEX: [Way; 4] = [
    Way::Up,
    Way::Down,
    Way::Left,
    Way::Right,
];

const ROBOT_FROM_INDEX: [RobotId; 4] = [
    RobotId::Red,
    RobotId::Green,
    RobotId::Blue,
    RobotId::Yellow,
];
