use std::vec;

use rand::Rng;

use crate::{
    cell::{Cell, CellKind, CellState, SweeperCell},
    error::Error,
};

pub enum BoardResult {
    Win,
    Lost,
}

pub enum BoardState {
    Uninitialized,
    Playing,
    Finished(BoardResult),
}

pub trait SweeperBoard<T>
where
    Self: Sized,
    T: SweeperCell,
{
    fn new(height: usize, width: usize, mine_count: usize) -> Result<Self, Error>;

    fn open(&mut self, i: usize, j: usize) -> &CellKind;

    fn open_save(&mut self, i: usize, j: usize) -> Result<&CellKind, Error>;

    fn flag(&mut self, i: usize, j: usize) -> &CellState;

    fn flag_save(&mut self, i: usize, j: usize) -> Result<&CellState, Error>;

    fn state(&self) -> &BoardState;

    fn cells(&self) -> &Vec<Vec<T>>;
}

/// Default implementation of the SweeperBoard trait
pub struct Board {
    cells: Vec<Vec<Cell>>,
    mine_count: usize,
    state: BoardState,
    closed_cell_count: usize,
}

macro_rules! count_board_stat {
    ($visibility:vis, $func_name:ident, $expected:pat, $field:ident) => {
        $visibility fn $func_name(&self, i: usize, j: usize) -> usize {
            let mut count = 0;
            for (i_nbr, j_nbr) in self.nbr_indices(i, j) {
                if let $expected = self.cells[i_nbr][j_nbr].$field {
                    count += 1;
                }
            }
            count
        }
    };
}

/// Helper methods to help implement the trait
impl Board {
    /// Initially, the cells are all unitialized. After the first
    /// Cell has been opened, the cell and all of its neighboring
    /// cells are set as free cell.
    fn initialize(&mut self, i: usize, j: usize) {
        self.cells[i][j].kind = CellKind::Free;
        for (i_nbr, j_nbr) in self.nbr_indices(i, j) {
            self.cells[i_nbr][j_nbr].kind = CellKind::Free;
        }
        let mut placed_mine = 0;
        let mut rng = rand::thread_rng();
        while placed_mine < self.mine_count {
            let i = rng.gen_range(0..self.height());
            let j = rng.gen_range(0..self.width());
            if let CellKind::Uninitialized = self.cells[i][j].kind {
                self.cells[i][j].kind = CellKind::Mine;
                placed_mine += 1;
            }
        }
        self.state = BoardState::Playing;
        self.cells.iter_mut().flatten().for_each(|cell| {
            if let CellKind::Uninitialized = cell.kind {
                cell.kind = CellKind::Free;
            }
        })
    }

    /// Returs an array of tupple containing the index of neighboring
    /// cells starting from left to right, top to bottom.
    fn nbr_indices(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut indices: Vec<(usize, usize)> = Vec::new();
        for i_offset in 0..3 {
            for j_offset in 0..3 {
                let i_nbr = i + i_offset;
                let j_nbr = j + j_offset;
                if 0 < i_nbr
                    && i_nbr <= self.height()
                    && 0 < j_nbr
                    && j_nbr <= self.width()
                    && (i_nbr, j_nbr) != (i + 1, j + 1)
                {
                    indices.push((i_nbr - 1, j_nbr - 1));
                }
            }
        }
        indices
    }

    /// A convenient alias from `self.cells.len()`.
    pub fn height(&self) -> usize {
        self.cells.len()
    }

    /// A convenient alias from `self.cells[0].len()`. Guarateed to
    /// return because the height of the board is never less than 9.
    pub fn width(&self) -> usize {
        self.cells[0].len()
    }

    count_board_stat!(pub, count_adjacent_mines, CellKind::Mine, kind);
    count_board_stat!(, count_surrounding_flags, CellState::Flagged, state);
}

macro_rules! save_op {
    ($func_name:ident, $op:ident, $return_type:ident) => {
        fn $func_name(&mut self, i: usize, j: usize) -> Result<&$return_type, Error> {
            if i < self.height() && j < self.width() {
                Ok(self.$op(i, j))
            } else {
                Err(Error::IndexOutOfBoundError)
            }
        }
    };
}

impl SweeperBoard<Cell> for Board {
    /// Create a new minesweeper board. `height` and `width` cannot be under 9,
    /// while `mine_count` cannot exceed `height * width - 9` since the initial
    /// cell and its neighbors must be a free cell. Return error if given invalid
    /// configuration.
    fn new(height: usize, width: usize, mine_count: usize) -> Result<Self, Error> {
        if width < 9 || height < 9 || height * width - 9 < mine_count {
            return Err(Error::InvalidConfigError);
        }
        let cell = Cell {
            kind: CellKind::Uninitialized,
            state: CellState::Closed,
        };
        Ok(Self {
            cells: vec![vec![cell; width]; height],
            mine_count,
            state: BoardState::Uninitialized,
            closed_cell_count: width * height,
        })
    }

    /// Open a cell, propagate if all neighboring cell is a free cell.
    /// Opening an opened cell will propagate if flagged cell count is
    /// equal to surrounding mine count.
    ///
    /// Propagation is stopped when propagation reached a mine cell.
    fn open(&mut self, i: usize, j: usize) -> &CellKind {
        if let BoardState::Uninitialized = self.state {
            self.initialize(i, j);
        }
        match self.cells[i][j].state {
            CellState::Closed => {
                self.cells[i][j].state = CellState::Opened;
                self.closed_cell_count -= 1;
                if let CellKind::Mine = self.cells[i][j].kind {
                    self.state = BoardState::Finished(BoardResult::Lost);
                }
                if let BoardState::Playing = self.state {
                    if self.count_adjacent_mines(i, j) == 0 {
                        for (i_nbr, j_nbr) in self.nbr_indices(i, j) {
                            self.open(i_nbr, j_nbr);
                        }
                    }
                }
            }
            CellState::Opened => {
                let mine_count = self.count_adjacent_mines(i, j);
                if mine_count > 0 && self.count_surrounding_flags(i, j) == mine_count {
                    for (i_nbr, j_nbr) in self.nbr_indices(i, j) {
                        if let CellState::Closed = self.cells[i_nbr][j_nbr].state {
                            self.open(i_nbr, j_nbr);
                        }
                    }
                }
            }
            _ => (),
        }
        if let BoardState::Playing = self.state {
            if self.mine_count == self.closed_cell_count {
                self.state = BoardState::Finished(BoardResult::Win);
            }
        }
        &self.cells[i][j].kind
    }

    save_op!(open_save, open, CellKind);

    /// Flag a cell. Flagged cell cannot be opened until unflagged.
    /// Remove the flag by flagging a flagged cell again. Flagged cell
    /// counts toward opening an opened cell propagation.
    fn flag(&mut self, i: usize, j: usize) -> &CellState {
        self.cells[i][j].flag()
    }

    save_op!(flag_save, flag, CellState);

    fn state(&self) -> &BoardState {
        &self.state
    }

    fn cells(&self) -> &Vec<Vec<Cell>> {
        &self.cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_cell() {
        let mut board = Board::new(9, 9, 10).unwrap();
        board.open(4, 4);

        // check surrounding initial
        for (i_nbr, j_nbr) in board.nbr_indices(4, 4) {
            assert!(matches!(board.cells[i_nbr][j_nbr].kind, CellKind::Free));
        }

        // check opened cell is a free cell
        for cell in board.cells.iter().flatten() {
            if let CellState::Opened = cell.state {
                assert!(matches!(cell.kind, CellKind::Free))
            }
        }
    }

    #[test]
    fn init_board() {
        let mut board = Board::new(9, 9, 10).unwrap();
        let mut mine_count = 0;
        let mut uninitialized_cell = 0;
        for cell in board.cells().iter().flatten() {
            match cell.kind {
                CellKind::Mine => mine_count += 1,
                CellKind::Uninitialized => uninitialized_cell += 1,
                _ => (),
            }
        }
        assert_eq!(mine_count, 0);
        assert_eq!(uninitialized_cell, 81);
        board.initialize(4, 4);
        mine_count = 0;
        uninitialized_cell = 0;
        for cell in board.cells().iter().flatten() {
            match cell.kind {
                CellKind::Mine => mine_count += 1,
                CellKind::Uninitialized => uninitialized_cell += 1,
                _ => (),
            }
        }
        assert_eq!(mine_count, 10);
        assert_eq!(uninitialized_cell, 0);
    }

    #[test]
    fn dimension() {
        let board = Board::new(9, 9, 10).unwrap();
        assert_eq!(board.width(), 9);
        assert_eq!(board.height(), 9);
    }

    #[test]
    fn out_of_bound() {
        let mut board = Board::new(9, 9, 10).unwrap();
        assert!(board.open_save(10, 0).is_err());
        assert!(board.open_save(0, 10).is_err());
        assert!(board.flag_save(10, 0).is_err());
        assert!(board.flag_save(0, 10).is_err());
    }

    #[test]
    fn new_board() {
        let valid = Board::new(9, 9, 10);
        assert!(valid.is_ok());
        let invalid_height = Board::new(8, 9, 10);
        assert!(invalid_height.is_err());
        let invalid_width = Board::new(9, 7, 10);
        assert!(invalid_width.is_err());
        let too_many_mines = Board::new(9, 9, 73);
        assert!(too_many_mines.is_err());
    }

    macro_rules! nbr_indices_test {
        ($func_name:ident, $i:expr, $j:expr, $expected:expr) => {
            #[test]
            fn $func_name() {
                let board = Board::new(9, 9, 5).unwrap();
                let indices = board.nbr_indices($i, $j);
                assert_eq!(indices, $expected)
            }
        };
    }

    nbr_indices_test!(top_left_nbr, 0, 0, [(0, 1), (1, 0), (1, 1)]);
    nbr_indices_test!(top_right_nbr, 0, 8, [(0, 7), (1, 7), (1, 8)]);
    nbr_indices_test!(bot_left_nbr, 8, 0, [(7, 0), (7, 1), (8, 1),]);
    nbr_indices_test!(bot_right_nbr, 8, 8, [(7, 7), (7, 8), (8, 7),]);
    nbr_indices_test!(top_nbr, 0, 4, [(0, 3), (0, 5), (1, 3), (1, 4), (1, 5)]);
    nbr_indices_test!(left_nbr, 4, 0, [(3, 0), (3, 1), (4, 1), (5, 0), (5, 1),]);
    nbr_indices_test!(right_nbr, 4, 8, [(3, 7), (3, 8), (4, 7), (5, 7), (5, 8),]);
    nbr_indices_test!(bot_nbr, 8, 4, [(7, 3), (7, 4), (7, 5), (8, 3), (8, 5),]);
}
