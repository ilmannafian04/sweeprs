use std::fmt;

#[derive(Clone, Debug)]
pub enum CellKind {
    Mine,
    Free,
    Uninitialized,
}

#[derive(Clone)]
pub enum CellState {
    Closed,
    Flagged,
    Open,
}

#[derive(Clone)]
pub struct Cell {
    pub kind: CellKind,
    pub state: CellState,
    pub mine_count: usize,
    pub mine_is_counted: bool,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.state {
            CellState::Closed => f.write_str("O"),
            CellState::Flagged => f.write_str("F"),
            CellState::Open => match self.kind {
                CellKind::Mine => f.write_str("X"),
                CellKind::Free => {
                    if self.mine_is_counted && self.mine_count > 0 {
                        f.write_str(&format!("{}", self.mine_count))
                    } else {
                        f.write_str(" ")
                    }
                }
                CellKind::Uninitialized => f.write_str("/"),
            },
        }
    }
}
