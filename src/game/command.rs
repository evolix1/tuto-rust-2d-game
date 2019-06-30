use super::error::Result;
use super::Game;


pub trait CommandBase {
    fn redo(&self, game: &mut Game) -> Result<()>;
    fn undo(&self, game: &mut Game) -> Result<()>;
}


pub trait Command : std::fmt::Debug + CommandBase {}
