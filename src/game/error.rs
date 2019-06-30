use error_chain::error_chain;
pub use error_chain::bail; // Re-export


error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors {
        RobotHasNoPosition {
            description("robot has no position"),
            display("robot has no position"),
        }
    }
}
