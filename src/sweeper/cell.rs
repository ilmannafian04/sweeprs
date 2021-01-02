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
    pub mine_count: usize,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.is_open {
            f.write_str("O")
        } else {
            if self.mine_count == 0 {
                f.write_str(" ")
            } else {
                f.write_str(&format!("{}", self.mine_count))
            }
        }
    }
}
