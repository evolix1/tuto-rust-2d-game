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
    pub x: u16,
    pub y: u16,
}


#[derive(Debug, PartialEq, Clone)]
pub struct Hit {
    pub pos: Pos,
    pub distance: i16
}


impl Pos {
    #[allow(dead_code)]
    pub fn new<T>(x: T, y: T) -> Pos where T: Into<u16> {
        Pos { x: x.into(), y: y.into() }
    }


    #[allow(dead_code)]
    pub fn rand<T>(columns: T, rows: T) -> Pos where T: Into<u16> {
        let mut rng = thread_rng();
        let x = rng.gen_range(0, columns.into());
        let y = rng.gen_range(0, rows.into());
        Pos { x, y }
    }


    pub fn hit_along_to(&self, other: &Pos, way: Way) -> Option<Hit> {
        match way {
            Way::Up if self.x != other.x || self.y <= other.y => None,
            Way::Up => Some(Hit {
                pos: Pos{ x: self.x, y: other.y + 1 }, 
                distance: self.y as i16 - other.y as i16 - 1
            }),
            Way::Down if self.x != other.x || self.y >= other.y => None,
            Way::Down => Some(Hit {
                pos: Pos{ x: self.x, y: other.y - 1 },
                distance: other.y as i16 - self.y as i16 - 1
            }),
            Way::Left if self.y != other.y || self.x <= other.x => None,
            Way::Left => Some(Hit {
                pos: Pos{ x: other.x + 1, y: self.y },
                distance: self.x as i16 - other.x as i16 - 1
            }),
            Way::Right if self.y != other.y || self.x >= other.x => None,
            Way::Right => Some(Hit {
                pos: Pos{ x: other.x - 1, y: self.y },
                distance: other.x as i16 - self.x as i16 - 1
            }),
        }
    }
}
