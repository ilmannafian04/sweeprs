/// Indicate what the mine contain
#[derive(Clone)]
pub enum CellKind {
    Mine,
    Free,
    Uninitialized,
}

/// Hold the state of the cell
#[derive(Clone)]
pub enum CellState {
    Closed,
    Flagged,
    Opened,
}

/// Default cell struct
#[derive(Clone)]
pub struct Cell {
    pub kind: CellKind,
    pub state: CellState,
}

/// Cell trait
pub trait BoardCell {
    fn open(&mut self) -> &CellKind;

    fn flag(&mut self) -> &CellState;
}

/// Default implementation of the cell trait
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
            _ => (),
        }
        &self.state
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
