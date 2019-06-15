use sdl2::rect::Rect;

use crate::robot::RobotId;


#[derive(PartialEq, Eq, Hash)]
pub enum SpriteId {
    // Board management
    CellBackground,
    SizedBoard { width: u32, height: u32 },
    DefaultBoard,
    // Corner overlay
    CornerWall,
    SideWall,
    //
    Robot(RobotId),
}


#[derive(Clone)]
pub struct Sprite {
    pub texture_index: usize,
    pub geom: Rect
}
