use std::fmt;

use pest::Parser;
use pest_derive::Parser;

use crate::positionning::Pos;
use crate::dim::Dimensions;
use crate::wall::{Wall, Side};

use super::error::Error;


pub enum ParseError {
    BadSyntax(String), // NOTE: usage mainly to forward error from `pest`
    UnexpectedToken{ tokens: Vec<String>, pos: usize },
    MissingRows{ last_row: usize, missing: usize },
    TooLargeContent{ max_rows: usize },
}


#[derive(Parser)]
#[grammar = "board/tile.pest"]
pub struct TileParser {
    dim: Dimensions,
    text: String,
}


impl TileParser {
    pub fn new(dim: Dimensions, text: String) -> TileParser {
        TileParser { dim, text }
    }
    
    
    // NOTE: make it an actual iterator
    pub fn all(&mut self) -> std::result::Result<Vec<Wall>, ParseError> {
        let mut res = Vec::new();
        let pairs = 
            Self::parse(Rule::content, &self.text)
            .map_err(|e| ParseError::BadSyntax(format!("{:?}", e)))?;

        // NOTE: In case content is partially valid, `pest` will parse until
        // it fails to recognise the content. So, this manually checks that
        // we indeed parsed the whole content. Otherwise, we suppose it has 
        // encountered an unexpected token.
        {
            let parsed_len = pairs.as_str().len();
            if parsed_len != self.text.len() {
                assert!(parsed_len < self.text.len());
                let msg = format!(
                    "parsing error, unexpected '{}' at pos {}",
                    &self.text[parsed_len..=parsed_len],
                    parsed_len);
                return Err(ParseError::BadSyntax(msg));
            }
        }
        
        // NOTE: As of now, the input content is a string WITHOUT any line
        // breaker (CR/LF). So, we cannot use them to count lines. Instead,
        // it we simply count how many state we found, end break line when
        // we reached out given dimensions for the tile.
        let mut y = 0;
        let mut x = 0;
        
        for item in pairs {
            match item.as_rule() {
                Rule::cell_state => {
                    if y >= self.dim.rows {
                        let max_rows = self.dim.rows;
                        return Err(ParseError::TooLargeContent{ max_rows });
                    }
                    
                    x += 1;
                    if x == self.dim.columns {
                        x = 0;
                        y += 1;
                    }
                },
                // NOTE: `x` represent current column, but it starts with 0 
                // (and not -1). So it is 1-greater than it should be when we 
                // encounter a wall.
                Rule::v_wall => {
                    let pos = Pos::new(x - 1, y);
                    let side = Side::Right;
                    res.push(Wall{ pos, side });
                },
                Rule::h_wall => {
                    let pos = Pos::new(x - 1, y);
                    let side = Side::Down;
                    res.push(Wall{ pos, side });
                },
                _ => {
                    let tokens = vec![
                        "cell".into(), 
                        "vertical wall".into(), 
                        "horizontal wall".into()];
                    let pos = item.as_span().start();
                    return Err(ParseError::UnexpectedToken{ tokens, pos });
                },
            }
        };

        if y == self.dim.rows && x == 0 {
            Ok(res)
        } 
        // missing some rows
        else if y < self.dim.rows {
            let missing = self.dim.rows - y;
            Err(ParseError::MissingRows{ last_row: y, missing })
        }
        else {
            unreachable!("too large content error should have been returned earlier");
        }
    }
}


impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        match &self {
            &ParseError::BadSyntax(text) => 
                write!(f, "{}", text),
            &ParseError::UnexpectedToken{ tokens, pos } => 
                write!(f, 
                        "expected token(s) '{}' at pos {}", 
                        tokens.join("', '"),
                        pos),
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
            &ParseError::BadSyntax(..)
            | &ParseError::UnexpectedToken{..} => 
                Error::InvalidTileStructure(format!("{:?}", self)),
            &ParseError::MissingRows{..}
            | &ParseError::TooLargeContent{..} =>
                Error::TileDimensionsDoNotMatchContent(format!("{:?}", self)),
        }
    }
}
