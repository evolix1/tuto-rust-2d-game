mod error;
mod board;
mod moves;
mod dim;
mod tile;

pub use self::error::{Error, Result};
pub use self::board::{Board, EditableBoard};
pub use self::moves::MovePossibility;
pub use self::dim::Dimensions;
pub use self::tile::{Tile, Border};


// Boards implementation
mod individual_cells;
mod indirect_table;

pub use self::individual_cells::BoardByIndividualCells;
pub use self::indirect_table::BoardByIndirectTable;
