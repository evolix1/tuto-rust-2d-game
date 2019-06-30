use sdl2::rect::Rect;

use crate::game::RobotId;


#[derive(PartialEq, Eq, Hash)]
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
