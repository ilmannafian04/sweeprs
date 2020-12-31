use std::fmt;

#[derive(Clone)]
enum Cell {
    Mine,
    Flag,
    Empty,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => f.write_str("  "),
            Cell::Flag => f.write_str("! "),
            Cell::Mine => f.write_str("X "),
        }
    }
}

#[derive(Debug)]
pub struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(h: usize, w: usize) -> Self {
        let board = vec![vec![Cell::Empty; w]; h];
        Board { cells: board }
    }
}
