use rand::{thread_rng, Rng};

use serde_derive::Deserialize;


#[derive(Debug, Clone, Deserialize)]
pub struct SideLength(pub usize);


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}


#[derive(Debug, PartialEq, Clone)]
pub struct Hit {
    pub pos: Pos,
    pub distance: isize
}
        

#[derive(Debug)]
#[allow(dead_code)]
pub enum FlipAxis {
    NoFlip,
    FlipHorizontal,
    FlipVertical,
    FlipBoth
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum RotateAngle {
    NoTurn,
    TurnLeft,
    TurnRight,
    HalfTurn
}


impl Pos {
    pub fn new(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }


    pub fn rand(columns: usize, rows: usize) -> Pos {
        let mut rng = thread_rng();
        let x = rng.gen_range(0, columns);
        let y = rng.gen_range(0, rows);
        Pos { x, y }
    }
    
    
    pub fn distance_to(&self, other: &Pos, way: Way) -> isize {
        match way { 
            Way::Up => self.y as isize - other.y as isize,
            Way::Down => other.y as isize - self.y as isize,
            Way::Left => self.x as isize - other.x as isize,
            Way::Right => other.x as isize - self.x as isize,
        }
    }


    pub fn find_hit_to(&self, other: &Pos, way: Way) -> Option<Hit> {
        match way {
            Way::Up if self.x != other.x || self.y <= other.y => None,
            Way::Up => Some(Hit {
                pos: Pos{ x: self.x, y: other.y + 1 }, 
                distance: self.y as isize - other.y as isize - 1
            }),
            Way::Down if self.x != other.x || self.y >= other.y => None,
            Way::Down => Some(Hit {
                pos: Pos{ x: self.x, y: other.y - 1 },
                distance: other.y as isize - self.y as isize - 1
            }),
            Way::Left if self.y != other.y || self.x <= other.x => None,
            Way::Left => Some(Hit {
                pos: Pos{ x: other.x + 1, y: self.y },
                distance: self.x as isize - other.x as isize - 1
            }),
            Way::Right if self.y != other.y || self.x >= other.x => None,
            Way::Right => Some(Hit {
                pos: Pos{ x: other.x - 1, y: self.y },
                distance: other.x as isize - self.x as isize - 1
            }),
        }
    }


    pub fn direct_path_to(&self, dest: &Pos) -> Option<Vec<Pos>> {
        if self.x == dest.x && self.y <= dest.y {
            Some((0..(dest.y - self.y)).map(|dy| Pos::new(self.x, self.y + dy)).collect())
        } 
        else if self.x == dest.x && self.y > dest.y {
            Some((0..(self.y - dest.y)).map(|dy| Pos::new(self.x, self.y - dy)).collect())
        } 
        else if self.y == dest.y && self.x <= dest.x {
            Some((0..(dest.x - self.x)).map(|dx| Pos::new(self.x + dx, self.y)).collect())
        } 
        else if self.y == dest.y && self.x > dest.x {
            Some((0..(self.x - dest.x)).map(|dx| Pos::new(self.x - dx, self.y)).collect())
        } 
        else {
            return None;
        }
    }
}


#[macro_export]
macro_rules! impl_way {
    ($name:ident) => {
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub enum $name {
            Up,
            Down,
            Left,
            Right
        }


        impl $name {
            #[allow(dead_code)]
            pub fn rotate(&self, angle: RotateAngle) -> $name {
                match (*self, angle) {
                    ($name::Up, RotateAngle::NoTurn) |
                    ($name::Down, RotateAngle::HalfTurn) |
                    ($name::Left, RotateAngle::TurnRight) |
                    ($name::Right, RotateAngle::TurnLeft)
                        => $name::Up,
                    ($name::Up, RotateAngle::HalfTurn) |
                    ($name::Down, RotateAngle::NoTurn) |
                    ($name::Left, RotateAngle::TurnLeft) |
                    ($name::Right, RotateAngle::TurnRight)
                        => $name::Down,
                    ($name::Up, RotateAngle::TurnLeft) |
                    ($name::Down, RotateAngle::TurnRight) |
                    ($name::Left, RotateAngle::NoTurn) |
                    ($name::Right, RotateAngle::HalfTurn)
                        => $name::Left,
                    ($name::Up, RotateAngle::TurnRight) |
                    ($name::Down, RotateAngle::TurnLeft) |
                    ($name::Left, RotateAngle::HalfTurn) |
                    ($name::Right, RotateAngle::NoTurn)
                        => $name::Right,
                }
             }
        }
    }
}

impl_way!(Way);
