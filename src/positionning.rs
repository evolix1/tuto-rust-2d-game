pub struct Pos {
    pub x: u16,
    pub y: u16,
}


impl Pos {
    pub fn new<T>(x: T, y: T) -> Pos where T: Into<u16> {
        Pos { x: x.into(), y: y.into() }
    }
}
