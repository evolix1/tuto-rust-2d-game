use error_chain::error_chain;
pub use error_chain::bail; // Re-export


error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors {
        UnexpectedToken(
            unexpected: char, 
            expected: Vec<String>, 
            column: usize, 
            row: usize,
            tile: String) {
            description("unexpected token"),
            display("at {}:{} unexpected {} (wanted: {})",
                    row,
                    column,
                    unexpected,
                    expected.join("', '")),
        }

        MissingRows(last_row: usize, missing: usize, tile: String) {
            description("missing rows from tile definition"),
            display("missing {} row{} from the {}-th row in the tile to be complete",
                    missing,
                    if *missing == 1 { "" } else { "s" },
                    last_row),
        }

        TooLargeContent(max_rows: usize, tile: String) {
            description("too large content in tile definition"),
            display("cannot have an {}-th row when tile only have {} rows",
                    max_rows + 1,
                    max_rows),
        }
    }
}
