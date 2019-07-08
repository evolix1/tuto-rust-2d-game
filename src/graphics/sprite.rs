use sdl2::rect::Rect;

use crate::game::RobotId;


#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum SpriteId {
    // 
    CellBackground,
    ForbiddenCell,
    CornerWall,
    SideWall,
    //
    CurrentBoard,
    //
    Robot(RobotId),
}


#[derive(Clone)]
pub struct Sprite {
    pub texture_index: usize,
    pub geom: Rect
}
