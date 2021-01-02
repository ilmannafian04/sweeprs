use std::fmt;

#[derive(Clone)]
pub enum Cell {
    Mine,
    Flag,
    Free,
    Uninitialized,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Free => f.write_str("  "),
            Cell::Flag => f.write_str("! "),
            Cell::Mine => f.write_str("X "),
            Cell::Uninitialized => f.write_str("/ "),
        }
    }
}
