use std::fmt;

use crate::positionning::{LogicalPos, SideLength};
use crate::wall::{Wall, Side};

use super::tile::Tile;
use super::error::Error;


pub enum ParseError {
    UnexpectedToken{
        unexpected: char,
        expected: Vec<String>,
        column: usize,
        row: usize
    },
    MissingRows{ last_row: usize, missing: usize },
    TooLargeContent{ max_rows: usize },
}


pub struct TileParser<'a> {
    texts: &'a Vec<String>,
}


impl<'a> TileParser<'a> {
    pub fn new(texts: &'a Vec<String>) -> TileParser<'a> {
        TileParser { texts }
    }


    // NOTE: make it an actual iterator
    pub fn parse_all(&mut self, side_length: &SideLength) -> std::result::Result<Vec<Tile>, ParseError> {
        self.texts.iter()
            .map(|text| Self::parse(side_length, text))
            .collect::<std::result::Result<_, _>>()
    }

    pub fn parse(side_length: &SideLength, text: &String) -> std::result::Result<Tile, ParseError> {
        let side = side_length.0;

        let mut walls = Vec::new();
        let mut row = 0;
        let mut column = 0;

        for item in text.as_bytes() {
            match *item as char {
                '.' => {
                    if row >= side {
                        return Err(ParseError::TooLargeContent{ max_rows: side });
                    }

                    column += 1;
                    if column == side {
                        column = 0;
                        row += 1;
                    }
                },
                '|' => {
                    let pos = LogicalPos{ x: column - 1, y: row };
                    let side = Side::Right;
                    walls.push(Wall{ pos, side });
                },
                '_' => {
                    let pos = LogicalPos{ x: column - 1, y: row };
                    let side = Side::Down;
                    walls.push(Wall{ pos, side });
                }
                ' ' | '\t' | '\n' => {},
                unexpected => {
                    let expected = vec![
                        "cell".into(),
                        "vertical wall".into(),
                        "horizontal wall".into()];

                    return Err(ParseError::UnexpectedToken{
                        expected,
                        unexpected,
                        column,
                        row
                    });
                },
            }
        }

        if row == side && column == 0 {
            Ok(Tile::new(walls))
        }
        // missing some rows
        else if row < side {
            let missing = side - row;
            Err(ParseError::MissingRows{ last_row: row, missing })
        }
        else {
            unreachable!("too large content error should have been returned earlier");
        }
    }
}


impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        match &self {
            &ParseError::UnexpectedToken{ expected, unexpected, row, column } =>
                write!(f,
                        "at {}:{} unexpected {} (wanted: {})",
                        row,
                        column,
                        unexpected,
                        expected.join("', '")),
            &ParseError::MissingRows{ last_row, missing } =>
                write!(f,
                       "missing {} row{} from the {}-th row in the tile to be complete",
                       missing,
                       if *missing == 1 { "" } else { "s" },
                       last_row),
            &ParseError::TooLargeContent{ max_rows } =>
                write!(f,
                       "cannot have an {}-th row when tile only have {} rows",
                       max_rows + 1,
                       max_rows),
        }
    }
}


impl Into<Error> for ParseError {
    fn into(self) -> Error {
        match &self {
            &ParseError::UnexpectedToken{..} =>
                Error::InvalidTileStructure(format!("{:?}", self)),
            &ParseError::MissingRows{..}
            | &ParseError::TooLargeContent{..} =>
                Error::TileDimensionsDoNotMatchContent(format!("{:?}", self)),
        }
    }
}
