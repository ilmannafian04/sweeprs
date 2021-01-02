use std::fmt;

#[derive(Clone)]
pub enum CellKind {
    Mine,
    Flag,
    Free,
    Uninitialized,
}

#[derive(Clone)]
pub struct Cell {
    pub kind: CellKind,
    pub is_open: bool,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            CellKind::Free => f.write_str("  "),
            CellKind::Flag => f.write_str("! "),
            CellKind::Mine => f.write_str("X "),
            CellKind::Uninitialized => f.write_str("/ "),
        }
    }
}
