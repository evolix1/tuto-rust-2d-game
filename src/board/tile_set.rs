use serde_derive::Deserialize;

use rand::seq::SliceRandom;

use crate::dim::Dimensions;
use crate::board::{EditableBoard, Border};

use super::error::{Result, Error};
use super::tile_parser::TileParser;
use super::tile::Tile;


#[derive(Debug, Deserialize)]
pub struct TileSet {
    #[serde(flatten)]
    pub dim: Dimensions,
    #[serde(rename = "sets")]
    raw_tiles: Vec<String>,
    #[serde(default, skip)]
    pub tiles: Vec<Tile>
}


impl TileSet {
    pub fn parse(&mut self) -> Result<()> {
        self.tiles = TileParser::new(&self.raw_tiles)
            .parse_all(&self.dim)
            .map_err(|e| e.into())?;
        println!("tiles {:?}", self.tiles);
        Ok(())
    }


    pub fn build_rand<T>(&self, board: &mut T) -> Result<()> 
        where T: AsMut<dyn EditableBoard>
    {
        let mut rng = rand::thread_rng();
        
        for border in Border::all() {
            let _ = self.tiles.choose(&mut rng)
                .ok_or(Error::EmptyTileSet)
                .map(|tile| tile.apply_on(board, border))?;
        }
        
        Ok(())
    }
}
