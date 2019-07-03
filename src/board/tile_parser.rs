use crate::positionning::{LogicalPos, SideLength};
use crate::wall::{Wall, Side};

use super::tile_parser_error::*;
use super::tile::Tile;


pub struct TileParser<'a> {
    texts: &'a Vec<String>,
}


impl<'a> TileParser<'a> {
    pub fn new(texts: &'a Vec<String>) -> TileParser<'a> {
        TileParser { texts }
    }


    // NOTE: make it an actual iterator
    pub fn parse_all(&mut self, side_length: &SideLength) -> Result<Vec<Tile>> {
        self.texts.iter()
            .map(|text| Self::parse(side_length, text))
            .collect::<Result<_>>()
    }

    pub fn parse(side_length: &SideLength, text: &String) -> Result<Tile> {
        let side = side_length.0;

        let mut walls = Vec::new();
        let mut row = 0;
        let mut column = 0;

        for item in text.as_bytes() {
            match *item as char {
                '.' => {
                    if row >= side {
                        bail!(ErrorKind::TooLargeContent(side, text.clone()));
                    }

                    column += 1;
                    if column > side {
                        column = 1;
                        row += 1;
                    }
                },
                '|' => {
                    if column <= 0 {
                        bail!(ErrorKind::UnexpectedToken(
                                '|', 
                                vec!["cell".into()], 
                                column, 
                                row, 
                                text.clone()));
                    }

                    let pos = LogicalPos{ x: column - 1, y: row };
                    let side = Side::Right;
                    walls.push(Wall{ pos, side });
                },
                '_' => {
                    if column <= 0 {
                        bail!(ErrorKind::UnexpectedToken(
                                '|', 
                                vec!["cell".into()], 
                                column, 
                                row, 
                                text.clone()));
                    }

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

                    bail!(ErrorKind::UnexpectedToken(
                        unexpected, expected, column, row, text.clone()
                    ));
                },
            }
        }

        if row + 1 == side && column == side {
            Ok(Tile::new(walls))
        }
        // missing some rows
        else if row < side {
            let missing = side - row;
            bail!(ErrorKind::MissingRows(row, missing, text.clone()))
        }
        else {
            unreachable!("too large content error should have been returned earlier");
        }
    }
}
