use crate::wall::Wall;

use crate::positionning::{Pos, RotateAngle};
use crate::dim::Dimensions;

use super::error::Result;
use super::board::EditableBoard;
use super::border::Border;


#[derive(Debug, Default)]
pub struct Tile(Vec<Wall>);


impl Tile {
    // NOTE: By default, tile are considered to be `Border::TopLeft`.
   
    pub fn new(wall: Vec<Wall>) -> Tile {
        Tile(wall)
    }

    pub fn apply_on<T>(&self, board: &mut T, border: &Border) -> Result<()> 
        where T: AsMut<dyn EditableBoard>
    {
        let board = board.as_mut();

        for wall in self.0.iter() {
            let wall = Self::situate_on_board(wall, border, &board.dim());
            // TODO: better handling
            board.put_wall(&wall)
                .expect("board can put a wall at given position");
        }

        Ok(())
    }

    fn situate_on_board(
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
