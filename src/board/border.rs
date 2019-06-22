#[derive(Debug)]
pub enum Border {
    TopLeft, // NOTE: default border
    TopRight,
    BottomLeft,
    BottomRight
}


const ALL_BORDERS: [Border; 4] = [
    Border::TopLeft,
    Border::TopRight,
    Border::BottomLeft,
    Border::BottomRight,
];


impl Border {
    pub fn all() -> &'static [Border] {
        &ALL_BORDERS
    }
}
