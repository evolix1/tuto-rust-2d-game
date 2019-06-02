use serde_derive::Deserialize;

use crate::dim::Dimensions;
use crate::wall::Wall;
use crate::positionning::{Pos, RotateAngle};

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
    walls: Option<Vec<Wall>>,
}


// NOTE: By default, tile are considered to be `Border::TopLeft`.

impl Tile {
    pub fn apply_on<T>(&self, board: &mut T, border: Border) -> Result<()> 
        where T: AsMut<dyn EditableBoard>
    {
        let no_walls = Vec::new();
        let board = board.as_mut();

        for wall in self.walls.as_ref().unwrap_or(&no_walls).iter() {
            let wall = self.situate_on_board(wall, &border, &board.dim());
            board.put_wall(&wall)
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


    fn situate_on_board(
        &self, 
        wall: &Wall,
        border: &Border,
        board_dim: &Dimensions) -> Wall {
        match *border {
            Border::TopLeft => wall.clone(),
            Border::TopRight => {
                let pos = Pos::new(
                    board_dim.columns - wall.pos.y - 1,
                    wall.pos.x);
                let side = wall.side.rotate(RotateAngle::TurnRight);
                Wall{ pos, side }
            },
            Border::BottomLeft => {
                let pos = Pos::new(
                    board_dim.columns - wall.pos.x - 1,
                    board_dim.rows - wall.pos.y - 1);
                let side = wall.side.rotate(RotateAngle::HalfTurn);
                Wall{ pos, side }
            },
            Border::BottomRight => {
                let pos = Pos::new(
                    wall.pos.y,
                    board_dim.rows - wall.pos.x - 1);
                let side = wall.side.rotate(RotateAngle::TurnLeft);
                Wall{ pos, side }
            },
        }
    }
}
