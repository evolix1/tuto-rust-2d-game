use crate::positionning::{Pos, Way, Hit};


#[derive(Debug)]
pub struct Board{
    pub rows: u16,
    pub columns: u16
}


impl Board {
    pub fn new_custom(rows: u16, columns: u16) -> Board {
        Board { rows, columns }
    }


    pub fn has_target_symbol(&self, _pos: &Pos) -> bool {
        false
    }


    pub fn hits_along(&self, start: &Pos, way: Way) -> Vec<Hit> {
        let pos = match way {
            Way::Up => Pos::new(start.x, 0),
            Way::Down => Pos::new(start.x, self.rows - 1),
            Way::Left => Pos::new(0, start.y),
            Way::Right => Pos::new(self.columns - 1, start.y),
        };
        
        let distance = start
            .hit_along_to(&pos, way)
            .expect("positions are aligned")
            .distance + 1;

        vec![Hit { pos, distance }]
    }
}
