use std::fmt;


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


impl fmt::Display for Border {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Border::TopLeft => write!(f, "top-left"),
            &Border::TopRight => write!(f, "top-right"),
            &Border::BottomLeft => write!(f, "bottom-left"),
            &Border::BottomRight => write!(f, "bottom-right"),
        }
    }
}
