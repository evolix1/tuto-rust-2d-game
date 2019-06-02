use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Dimensions {
    // rows must be >= 1
    pub rows: usize,
    // colmuns must be >= 1
    pub columns: usize
}
