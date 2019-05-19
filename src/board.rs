use crate::positionning::Pos;


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
}
