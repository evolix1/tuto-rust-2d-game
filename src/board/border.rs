use crate::positionning::{Pos, Way, RotateAngle};

use super::dim::Dimensions;


#[derive(Debug)]
#[allow(dead_code)]
pub enum Border {
    TopLeft, // NOTE: default angle
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
                println!("[TOP-RIGHT] from {:?} {:?} ({:?})", pos, way, board_dim);
                let pos = Pos::new(
                    board_dim.columns - pos.y - 1,
                    pos.x);
                let way = way.angle(RotateAngle::TurnRight);
                println!("[TOP-RIGHT] to   {:?} {:?} ({:?})", pos, way, board_dim);
                (pos, way)
            },
            Border::BottomLeft => {
                println!("[BOTTOM-LEFT] from {:?} {:?} ({:?})", pos, way, board_dim);
                let pos = Pos::new(
                    board_dim.columns - pos.x - 1,
                    board_dim.rows - pos.y - 1);
                let way = way.angle(RotateAngle::HalfTurn);
                println!("[BOTTOM-LEFT] to   {:?} {:?} ({:?})", pos, way, board_dim);
                (pos, way)
            },
            Border::BottomRight => {
                println!("[BOTTOM-RIGHT] from {:?} {:?} ({:?})", pos, way, board_dim);
                let pos = Pos::new(
                    pos.y,
                    board_dim.rows - pos.x - 1);
                let way = way.angle(RotateAngle::TurnLeft);
                println!("[BOTTOM-RIGHT] to   {:?} {:?} ({:?})", pos, way, board_dim);
                (pos, way)
            },
        }
    }
}
