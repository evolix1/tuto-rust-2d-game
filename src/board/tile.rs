use serde_derive::Deserialize;

use crate::dim::Dimensions;
use crate::positionning::{Pos, Way};

use super::error::Result;
use super::board::EditableBoard;
use super::border::Border;
use super::tile_parser::TileParser;


#[derive(Debug, Deserialize)]
pub struct Tile {
    #[serde(flatten)]
    pub dim: Dimensions,
    #[serde(rename = "tile")]
    pub as_string: Option<String>,
    #[serde(skip)]
    walls: Option<Vec<(Pos, Way)>>,
}


// NOTE: By default, tile are considered to be `Border::TopLeft`.

impl Tile {
    pub fn apply_on<T>(&self, board: &mut T, border: Border) -> Result<()> 
        where T: AsMut<dyn EditableBoard>
    {
        let no_walls = Vec::new();
        let board = board.as_mut();
        
        for (pos, way) in self.walls.as_ref().unwrap_or(&no_walls).iter() {
            let (pos, way) = border.angle(pos, way, &board.dim());
            board.put_wall(&pos, way)
                .expect("board can put a wall at given position");
        }
        
        Ok(())
    }


    pub fn parse(&mut self) -> Result<()> {
        self.walls = Some(self.parser().all().map_err(|e| e.into())?);
        println!("walls {:?}", self.walls.as_ref().unwrap());
        Ok(())
    }


    fn parser(&self) -> TileParser {
        TileParser::new(self.dim.clone(),
                        self.as_string.clone().unwrap_or_else(String::new))
    }
}
