use super::Game;

pub type CommandResult<T> = Result<T, String>;


pub trait CommandBase {
    fn redo(&self, game: &mut Game) -> CommandResult<()>;
    fn undo(&self, game: &mut Game) -> CommandResult<()>;
}


pub trait Command : std::fmt::Debug + CommandBase {}
