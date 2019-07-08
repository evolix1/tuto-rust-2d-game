use serde_derive::Deserialize;

use rand::Rng;

use crate::positionning::SideLength;
use crate::board::{EditableBoard, Border};

use super::error::*;
use super::tile_parser::TileParser;
use super::tile::Tile;


#[derive(Debug, Deserialize)]
pub struct TileSet {
    pub side_length: SideLength,
    pub name: String,
    #[serde(rename = "sets")]
    raw_tiles: Vec<String>,
    #[serde(default, skip)]
    pub tiles: Vec<Tile>
}


impl TileSet {
    pub fn parse(&mut self) -> Result<()> {
        self.tiles = TileParser::new(&self.raw_tiles)
            .parse_all(&self.side_length)?;
        println!("tiles {:?}", self.tiles);
        Ok(())
    }


    pub fn build_rand<T>(&self, board: &mut T) -> Result<()>
        where T: AsMut<dyn EditableBoard>
    {
        let mut rng = rand::thread_rng();

        if self.tiles.is_empty() {
            bail!(ErrorKind::EmptyTileSet);
        }

        for border in Border::all() {
            let i = rng.gen_range(0, self.tiles.len());

            println!("Put tile '{}':{} on {} border", self.name, i, border);

            self.tiles[i].apply_on(board, border)?;
        }

        Ok(())
    }
}
