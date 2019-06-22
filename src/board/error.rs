use crate::positionning::{Pos, SideLength};


#[derive(Debug)]
pub enum Error {
    DimensionsNotSuitableForBoard,
    OutOfBoardPosition{ pos: Pos, side_length: SideLength },
    EmptyTileSet,
    InvalidTileStructure(String),
    TileDimensionsDoNotMatchContent(String),
}


pub type Result<T> = std::result::Result<T, Error>;
