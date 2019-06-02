#[derive(Debug)]
pub enum Error {
    DimensionsNotSuitableForBoard,
    OutOfBoardPosition,
    InvalidTileStructure(String),
    TileDimensionsDoNotMatchContent(String),
}


pub type Result<T> = std::result::Result<T, Error>;
