mod error;
mod board;
mod tile;
mod tile_set;
mod border;
mod tile_parser;
mod builder;

pub use self::error::{Error, Result};
pub use self::board::{Board, EditableBoard};
pub use self::tile::Tile;
pub use self::tile_set::TileSet;
pub use self::border::Border;
pub use self::builder::Builder;


// Boards implementation
mod individual_cells;
mod indirect_table;

pub use self::individual_cells::BoardByIndividualCells;
pub use self::indirect_table::BoardByIndirectTable;
