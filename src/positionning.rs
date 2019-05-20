use rand::{thread_rng, Rng};


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Way {
    Up,
    Down,
    Left,
    Right
}


#[derive(Debug, PartialEq, Clone)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}


#[derive(Debug, PartialEq, Clone)]
pub struct Hit {
    pub pos: Pos,
    pub distance: isize
}


impl Pos {
    pub fn new(x: isize, y: isize) -> Pos {
        Pos { x, y }
    }


    pub fn rand(columns: isize, rows: isize) -> Pos {
        let mut rng = thread_rng();
        let x = rng.gen_range(0, columns);
        let y = rng.gen_range(0, rows);
        Pos { x, y }
    }


    pub fn find_hit_to(&self, other: &Pos, way: Way) -> Option<Hit> {
        match way {
            Way::Up if self.x != other.x || self.y <= other.y => None,
            Way::Up => Some(Hit {
                pos: Pos{ x: self.x, y: other.y + 1 }, 
                distance: self.y - other.y - 1
            }),
            Way::Down if self.x != other.x || self.y >= other.y => None,
            Way::Down => Some(Hit {
                pos: Pos{ x: self.x, y: other.y - 1 },
                distance: other.y - self.y - 1
            }),
            Way::Left if self.y != other.y || self.x <= other.x => None,
            Way::Left => Some(Hit {
                pos: Pos{ x: other.x + 1, y: self.y },
                distance: self.x - other.x - 1
            }),
            Way::Right if self.y != other.y || self.x >= other.x => None,
            Way::Right => Some(Hit {
                pos: Pos{ x: other.x - 1, y: self.y },
                distance: other.x - self.x - 1
            }),
        }
    }
}
