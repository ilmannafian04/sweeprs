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
    Opened,
}

#[derive(Clone)]
pub struct Cell {
    pub kind: CellKind,
    pub state: CellState,
    pub mine_count: usize,
    pub mine_is_counted: bool,
}

pub trait BoardCell {
    fn open(&mut self) -> &CellKind;

    fn flag(&mut self) -> &CellState;
}

impl BoardCell for Cell {
    fn open(&mut self) -> &CellKind {
        if let CellState::Closed = self.state {
            self.state = CellState::Opened
        }
        &self.kind
    }

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
            CellState::Opened => match self.kind {
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

    macro_rules! test_board_cell_trait {
        ($func_name:ident, $state:expr, $func:ident, $expected:pat) => {
            #[test]
            fn $func_name() {
                let mut cell = Cell {
                    kind: CellKind::Free,
                    state: $state,
                    mine_count: 0,
                    mine_is_counted: true,
                };
                cell.$func();
                assert!(matches!(cell.state, $expected));
            }
        };
    }

    test_board_cell_trait!(open_closed, CellState::Closed, open, CellState::Opened);
    test_board_cell_trait!(open_flagged, CellState::Flagged, open, CellState::Flagged);
    test_board_cell_trait!(open_opened, CellState::Opened, open, CellState::Opened);
    test_board_cell_trait!(flag_closed, CellState::Closed, flag, CellState::Flagged);
    test_board_cell_trait!(flag_flagged, CellState::Flagged, flag, CellState::Closed);
    test_board_cell_trait!(flag_opened, CellState::Opened, flag, CellState::Opened);
}
