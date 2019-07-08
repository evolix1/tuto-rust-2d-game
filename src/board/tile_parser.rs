use crate::positionning::{LogicalPos, SideLength};
use crate::wall::{Wall, Side};

use super::tile_parser_error::*;
use super::tile::Tile;


pub struct TileParser<'a> {
    text: &'a String,
}


impl<'a> TileParser<'a> {
    pub fn new(text: &'a String) -> TileParser<'a> {
        TileParser { text }
    }


    pub fn parse(&self, side_length: &SideLength) -> Result<Tile> {
        let side = side_length.0;

        let mut walls = Vec::new();
        let mut forbidden = Vec::new();
        let mut row = 0;
        let mut column = 0;
                                
        let expected_cells = vec![
            "free cell".into(), 
            "forbidden cell".into()];

        for item in self.text.as_bytes() {
            match *item as char {
                '#' | '.' => {
                    if row >= side {
                        bail!(ErrorKind::TooLargeContent(side, self.text.clone()));
                    }

                    column += 1;
                    if column > side {
                        column = 1;
                        row += 1;
                    }

                    if *item as char == '#' {
                        let pos = LogicalPos{ x: column - 1, y: row };
                        forbidden.push(pos);
                    }
                },
                '|' => {
                    if column <= 0 {
                        bail!(ErrorKind::UnexpectedToken(
                                '|', expected_cells,
                                column, row, self.text.clone()));
                    }

                    let pos = LogicalPos{ x: column - 1, y: row };
                    let side = Side::Right;
                    walls.push(Wall{ pos, side });
                },
                '_' => {
                    if column <= 0 {
                        bail!(ErrorKind::UnexpectedToken(
                                '_', expected_cells,
                                column, row, self.text.clone()));
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
                        unexpected, expected, column, row, self.text.clone()
                    ));
                },
            }
        }

        if row + 1 == side && column == side {
            Ok(Tile::new(walls, forbidden))
        }
        // missing some rows
        else if row < side {
            let missing = side - row;
            bail!(ErrorKind::MissingRows(row, missing, self.text.clone()))
        }
        else {
            unreachable!("too large content error should have been returned earlier");
        }
    }
}
