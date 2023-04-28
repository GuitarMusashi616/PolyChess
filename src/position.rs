#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Position {
            row,
            col,
        }
    }
}