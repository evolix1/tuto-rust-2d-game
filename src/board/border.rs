use crate::positionning::{Pos, Way, RotateAngle};

use super::dim::Dimensions;


#[derive(Debug)]
pub enum Border {
    TopLeft, // NOTE: default border
    TopRight,
    BottomLeft,
    BottomRight
}


impl Border {
    pub fn angle(
        &self, 
        pos: Pos, 
        way: Way, 
        board_dim: &Dimensions
        ) -> (Pos, Way) 
    {
        match *self {
            Border::TopLeft => (pos, way),
            Border::TopRight => {
                let pos = Pos::new(
                    board_dim.columns - pos.y - 1,
                    pos.x);
                let way = way.angle(RotateAngle::TurnRight);
                (pos, way)
            },
            Border::BottomLeft => {
                let pos = Pos::new(
                    board_dim.columns - pos.x - 1,
                    board_dim.rows - pos.y - 1);
                let way = way.angle(RotateAngle::HalfTurn);
                (pos, way)
            },
            Border::BottomRight => {
                let pos = Pos::new(
                    pos.y,
                    board_dim.rows - pos.x - 1);
                let way = way.angle(RotateAngle::TurnLeft);
                (pos, way)
            },
        }
    }
}
