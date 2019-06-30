use error_chain::error_chain;
pub use error_chain::bail; // Re-export

use crate::graphics;
use crate::game;


error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {
        GraphicsError(graphics::error::Error, graphics::error::ErrorKind);
        GameError(game::error::Error, game::error::ErrorKind);
    }
}
