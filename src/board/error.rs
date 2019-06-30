use crate::positionning::{LogicalPos, SideLength};


#[derive(Debug)]
pub enum Error {
    DimensionsNotSuitableForBoard,
    OutOfBoardPosition{ pos: LogicalPos, side_length: SideLength },
    EmptyTileSet,
    InvalidTileStructure(String),
    TileDimensionsDoNotMatchContent(String),
}


pub type Result<T> = std::result::Result<T, Error>;
