use sdl2::rect::Rect;

use crate::robot::RobotId;


#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum SpriteId {
    // Board management
    CellBackground,
    CurrentBoard,
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
