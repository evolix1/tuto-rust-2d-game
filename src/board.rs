pub struct Board{
    pub rows: u16,
    pub columns: u16
}


impl Board {
    pub fn new() -> Board {
        Board {
            rows: 8,
            columns: 16,
        }
    }
}
