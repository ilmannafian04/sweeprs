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

pub trait Open {
    fn open(&mut self) -> &CellKind;
}

impl Open for Cell {
    fn open(&mut self) -> &CellKind {
        if let CellState::Closed = self.state {
            self.state = CellState::Open
        }
        &self.kind
    }
}

pub trait Flag {
    fn flag(&mut self) -> &CellState;
}

impl Flag for Cell {
    fn flag(&mut self) -> &CellState {
        match self.state {
            CellState::Closed => self.state = CellState::Flagged,
            CellState::Flagged => self.state = CellState::Closed,
            _ => ()
        }
        &self.state
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_closed() {
        let mut cell = Cell {
            kind: CellKind::Free,
            state: CellState::Closed,
            mine_count: 0,
            mine_is_counted: true,
        };
        cell.open();
        assert!(matches!(cell.state, CellState::Open));
    }

    #[test]
    fn open_flagged() {
        let mut cell = Cell {
            kind: CellKind::Free,
            state: CellState::Flagged,
            mine_count: 0,
            mine_is_counted: true,
        };
        cell.open();
        assert!(matches!(cell.state, CellState::Flagged));
    }

    #[test]
    fn flag_closed() {
        let mut cell = Cell {
            kind: CellKind::Free,
            state: CellState::Closed,
            mine_count: 0,
            mine_is_counted: true,
        };
        cell.flag();
        assert!(matches!(cell.state, CellState::Flagged));
    }

    #[test]
    fn flag_flagged() {
        let mut cell = Cell {
            kind: CellKind::Free,
            state: CellState::Flagged,
            mine_count: 0,
            mine_is_counted: true,
        };
        cell.flag();
        assert!(matches!(cell.state, CellState::Closed));
    }

    #[test]
    fn flag_opened() {
        let mut cell = Cell {
            kind: CellKind::Free,
            state: CellState::Open,
            mine_count: 0,
            mine_is_counted: true,
        };
        cell.flag();
        assert!(matches!(cell.state, CellState::Open));
    }
}
