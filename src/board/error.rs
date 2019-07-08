use error_chain::error_chain;
pub use error_chain::bail; // Re-export

use crate::positionning::{LogicalPos, SideLength};

use super::tile_parser_error;


error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {
        TileParseError(tile_parser_error::Error, tile_parser_error::ErrorKind);
    }

    errors {
        InvalidDimensionToBuildBoard {
            description("robot has no position"),
            display("robot has no position"),
        }

        InvalidTileFormat(tile_set_name: String, tile_index: usize) {
            description("invalid tile format"),
            display("invalid tile #{} of tile set '{}'", tile_index, tile_set_name),
        }

        OutOfBoardPosition(pos: LogicalPos, side_length: SideLength) {
            description("out-of-board position"),
            display("out-of-board position {0:?} (board={1}x{1})", pos, side_length),
        }

        EmptyTileSet {
            description("empty tile set"),
            display("empty tile set"),
        }
    }
}
