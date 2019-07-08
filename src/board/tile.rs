use crate::wall::{Wall, Side};

use crate::positionning::{LogicalPos, RotateAngle, SideLength};

use super::error::*;
use super::board::EditableBoard;
use super::border::Border;


#[derive(Debug, Default)]
pub struct Tile{
    walls: Vec<Wall>, 
    forbidden: Vec<LogicalPos>
}


impl Tile {
    // NOTE: By default, tile are considered to be `Border::TopLeft`.

    pub fn new(walls: Vec<Wall>, forbidden: Vec<LogicalPos>) -> Tile {
        Tile { walls, forbidden }
    }

    pub fn apply_on<T>(&self, board: &mut T, border: &Border) -> Result<()>
        where T: AsMut<dyn EditableBoard>
    {
        let board = board.as_mut();

        for wall in self.walls.iter() {
            let wall = Self::situate_on_board(
                wall, 
                border, 
                &board.side_length());
            
            board.put_wall(&wall)
                .expect("board can put a wall at given position");
        }

        for pos in self.forbidden.iter() {
            let correct_pos = Self::situate_on_board(
                &Wall { pos: pos.clone(), side: Side::Left }, 
                border, 
                &board.side_length()).pos;
            
            board.forbid_cell(&correct_pos)
                .expect("board can forbid cell at given position");
        }

        Ok(())
    }

    fn situate_on_board(
        wall: &Wall,
        border: &Border,
        board_side: &SideLength) -> Wall {
        match *border {
            Border::TopLeft => wall.clone(),
            Border::TopRight => {
                let pos = LogicalPos{
                    x: board_side.0 - wall.pos.y - 1,
                    y: wall.pos.x
                };
                let side = wall.side.rotate(RotateAngle::TurnRight);
                Wall{ pos, side }
            },
            Border::BottomLeft => {
                let pos = LogicalPos{
                    x: board_side.0 - wall.pos.x - 1,
                    y: board_side.0 - wall.pos.y - 1
                };
                let side = wall.side.rotate(RotateAngle::HalfTurn);
                Wall{ pos, side }
            },
            Border::BottomRight => {
                let pos = LogicalPos{
                    x: wall.pos.y,
                    y: board_side.0 - wall.pos.x - 1
                };
                let side = wall.side.rotate(RotateAngle::TurnLeft);
                Wall{ pos, side }
            },
        }
    }
}
