#[derive(Debug)]
pub enum Error {
    DimensionsNotSuitableForBoard,
    OutOfBoardPosition,
}


pub type Result<T> = std::result::Result<T, Error>;
