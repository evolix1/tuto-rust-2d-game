use crate::positionning::Way;


#[derive(Debug, Clone)]
pub struct MovePossibility {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub forbidden: bool,
}


impl MovePossibility {
    pub fn can_go(&self, way: &Way) -> bool {
        if self.forbidden {
            return false;
        }
        
        match way {
            Way::Up => self.up,
            Way::Down => self.down,
            Way::Left => self.left,
            Way::Right => self.right,
        }
    }

    pub fn all() -> MovePossibility {
        MovePossibility { up: true, down: true, left: true, right: true, forbidden: false }
    }

    pub fn none() -> MovePossibility {
        MovePossibility { up: false, down: false, left: false, right: false, forbidden: true }
    }
}
