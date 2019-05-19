use rand::{thread_rng, Rng};


#[derive(Debug, PartialEq)]
pub struct Pos {
    pub x: u16,
    pub y: u16,
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
}
