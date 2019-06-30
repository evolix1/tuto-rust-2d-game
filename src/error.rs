use error_chain::error_chain;
pub use error_chain::bail; // Re-export

use crate::graphics;
use crate::game;
use crate::board;


error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Io(std::io::Error);
        Json5(json5::Error);
    }

    links {
        GraphicsError(graphics::error::Error, graphics::error::ErrorKind);
        GameError(game::error::Error, game::error::ErrorKind);
        BoardError(board::error::Error, board::error::ErrorKind);
    }
}
