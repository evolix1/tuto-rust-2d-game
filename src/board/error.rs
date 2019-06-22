use crate::positionning::Pos;
use crate::dim::Dimensions;


#[derive(Debug)]
pub enum Error {
    DimensionsNotSuitableForBoard,
    OutOfBoardPosition{ pos: Pos, dim: Dimensions },
    EmptyTileSet,
    InvalidTileStructure(String),
    TileDimensionsDoNotMatchContent(String),
}


pub type Result<T> = std::result::Result<T, Error>;
