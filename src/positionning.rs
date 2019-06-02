use rand::{thread_rng, Rng};


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Way {
    Up,
    Down,
    Left,
    Right
}


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


impl Way {
    pub fn angle(&self, angle: RotateAngle) -> Way {
        match (*self, angle) {
            (Way::Up, RotateAngle::NoTurn) |
            (Way::Down, RotateAngle::HalfTurn) |
            (Way::Left, RotateAngle::TurnRight) |
            (Way::Right, RotateAngle::TurnLeft)
                => Way::Up,
            (Way::Up, RotateAngle::HalfTurn) |
            (Way::Down, RotateAngle::NoTurn) |
            (Way::Left, RotateAngle::TurnLeft) |
            (Way::Right, RotateAngle::TurnRight)
                => Way::Down,
            (Way::Up, RotateAngle::TurnLeft) |
            (Way::Down, RotateAngle::TurnRight) |
            (Way::Left, RotateAngle::NoTurn) |
            (Way::Right, RotateAngle::HalfTurn)
                => Way::Left,
            (Way::Up, RotateAngle::TurnRight) |
            (Way::Down, RotateAngle::TurnLeft) |
            (Way::Left, RotateAngle::HalfTurn) |
            (Way::Right, RotateAngle::NoTurn)
                => Way::Right,
        }
     }
}
