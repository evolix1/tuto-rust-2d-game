use crate::positionning::Pos;
use super::dim::Dimensions;

#[derive(Debug)]
pub enum Error {
    DimensionsNotSuitableForBoard,
    OutOfBoardPosition{ pos: Pos, dim: Dimensions },
    InvalidTileStructure(String),
    TileDimensionsDoNotMatchContent(String),
}


pub type Result<T> = std::result::Result<T, Error>;
