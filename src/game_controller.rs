use crate::config::AppConfig;
use crate::robot::RobotId;
use crate::positionning::{Pos, Way};
use crate::world::GameWorld;


pub type CommandResult<T> = Result<T, String>;


pub trait CommandBase {
    fn redo(&self, game: &mut GameController) -> CommandResult<()>;
    fn undo(&self, game: &mut GameController) -> CommandResult<()>;
}


pub trait Command : std::fmt::Debug + CommandBase {
}


#[derive(Debug, Clone)]
struct MoveRobotCommand {
    robot: RobotId,
    source_pos: Pos,
    target_pos: Pos,
}


impl CommandBase for MoveRobotCommand {
    fn redo(&self, game: &mut GameController) -> CommandResult<()> {
        game.world.place_robot(self.robot, self.target_pos.clone());
        // game.start_move_animation(self.robot, self.source_pos, self.target_pos);
        Ok(())
    }

    fn undo(&self, game: &mut GameController) -> CommandResult<()> {
        game.world.place_robot(self.robot, self.source_pos.clone());
        // game.start_move_animation(self.robot, self.target_pos, self.source_pos);
        Ok(())
    }
}


impl Command for MoveRobotCommand {}


pub struct GameController {
    pub world: GameWorld,
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
}


impl GameController {
    pub fn new(config: &AppConfig) -> GameController {
        GameController {
            world: GameWorld::new(config),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn try_move_robot_in_dir(&mut self, robot: RobotId, way: Way)
        -> CommandResult<bool> {
        let source_pos = self.world
            .robot_pos(robot)
            .ok_or("robot must be placed")?;
        let target_pos = self.world.cast_ray(&source_pos, way);

        if target_pos != source_pos {
            self.move_robot(robot, target_pos)?;
            Ok(true)
        }
        else {
            Ok(false)
        }
    }

    pub fn move_robot(&mut self, robot: RobotId, target_pos: Pos) -> CommandResult<()> {
        let source_pos = self.world
            .robot_pos(robot)
            .ok_or("robot must be placed")?;
        let command = MoveRobotCommand {
            robot,
            source_pos,
            target_pos,
        };

        self.exec_command(Box::new(command))
    }

    fn exec_command(&mut self, command: Box<dyn Command>) -> CommandResult<()> {
        println!("Exec command: {:?}", command);
        let res = command.redo(self);
        self.undo_stack.push(command);
        self.redo_stack.clear();
        res
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
}
