use std::fmt;

#[derive(Clone)]
pub enum Cell {
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

