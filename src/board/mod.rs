mod error;
mod board;
mod tile;
mod border;
mod tile_parser;

pub use self::error::{Error, Result};
pub use self::board::{Board, EditableBoard};
pub use self::tile::Tile;
pub use self::border::Border;


// Boards implementation
mod individual_cells;
mod indirect_table;

pub use self::individual_cells::BoardByIndividualCells;
pub use self::indirect_table::BoardByIndirectTable;
