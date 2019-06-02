use std::fmt;

use serde_derive::Deserialize;
use pest::Parser;
use pest_derive::Parser;

use crate::positionning::{Pos, Way};

use super::error::{Result, Error};
use super::board::EditableBoard;
use super::dim::Dimensions;
use super::border::Border;


#[derive(Debug, Deserialize)]
pub struct Tile {
    #[serde(flatten)]
    pub dim: Dimensions,
    #[serde(rename = "tile")]
    pub as_string: Option<String>,
}


impl Tile {
    pub fn apply_on<T>(&self, board: &mut T, border: Border) -> Result<()> 
        where T: AsMut<dyn EditableBoard>
    {
        let board = board.as_mut();
        
        for (i, (pos, way)) in self.parse().all()
                .map_err(|e| e.into())?
                .into_iter()
                .enumerate() {
            
            let (pos, way) = border.angle(pos, way, &board.dim());
            
            println!("wall {:?}: {:?} {:?}", i, pos, way);
            
            board.put_wall(&pos, way)
                .expect("board can put a wall at given position");
        }
        
        Ok(())
    }


    pub fn parse(&self) -> TileParser {
        TileParser::new(self.dim.clone(),
                        self.as_string.clone().unwrap_or_else(String::new))
    }
}


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
    pub fn all(&mut self) -> std::result::Result<Vec<(Pos, Way)>, ParseError> {
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
                // NOTE: `x` is 1-offseted, because it is incremented before 
                // we hit a wall.
                Rule::v_wall => res.push((Pos::new(x - 1, y), Way::Right)),
                Rule::h_wall => res.push((Pos::new(x - 1, y), Way::Down)),
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
