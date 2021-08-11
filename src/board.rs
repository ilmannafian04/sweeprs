use std::vec;

use rand::Rng;

use crate::{
    cell::{BoardCell, Cell, CellKind, CellState},
    error::Error,
};

pub enum BoardState {
    Uninitialized,
    Playing,
    Finished,
}

pub trait SweeperBoard<T>
where
    Self: Sized,
{
    fn new(height: usize, width: usize, mine_count: usize) -> Result<Self, Error>;

    fn open(&mut self, i: usize, j: usize) -> &CellKind;

    fn flag(&mut self, i: usize, j: usize) -> &CellState;

    fn state(&self) -> &BoardState;

    fn height(&self) -> usize;

    fn width(&self) -> usize;
}

pub struct Board {
    cells: Vec<Vec<Cell>>,
    mine_count: usize,
    state: BoardState,
}

macro_rules! count_board_stat {
    ($func_name:ident, $expected:pat, $field:ident) => {
        fn $func_name(&self, i: usize, j: usize) -> usize {
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

impl Board {
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
        self.cells.iter_mut().flatten().for_each(|cell| {
            if let CellKind::Uninitialized = cell.kind {
                cell.kind = CellKind::Free;
            }
        })
    }

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

    count_board_stat!(count_surrounding_mines, CellKind::Mine, kind);
    count_board_stat!(count_surrounding_flags, CellState::Flagged, state);
}

impl SweeperBoard<Cell> for Board {
    fn new(height: usize, width: usize, mine_count: usize) -> Result<Self, Error> {
        if width < 9 || height < 9 || height * width - 9 < mine_count {
            return Err(Error::InvalidConfig);
        }
        let cell = Cell {
            kind: CellKind::Uninitialized,
            state: CellState::Closed,
            mine_count: 0,
            mine_is_counted: false,
        };
        Ok(Self {
            cells: vec![vec![cell; width]; height],
            mine_count,
            state: BoardState::Uninitialized,
        })
    }

    fn open(&mut self, i: usize, j: usize) -> &CellKind {
        if let BoardState::Uninitialized = self.state {
            self.initialize(i, j);
        }
        match self.cells[i][j].state {
            CellState::Closed => {
                self.cells[i][j].state = CellState::Opened;
                if let CellKind::Mine = self.cells[i][j].kind {
                    self.state = BoardState::Finished;
                }
                if let BoardState::Playing = self.state {
                    if self.count_surrounding_mines(i, j) == 0 {
                        for (i_nbr, j_nbr) in self.nbr_indices(i, j) {
                            self.open(i_nbr, j_nbr);
                        }
                    }
                }
            }
            CellState::Opened => {
                let mine_count = self.count_surrounding_mines(i, j);
                if mine_count > 0 && self.count_surrounding_flags(i, j) == mine_count {
                    for (i_nbr, j_nbr) in self.nbr_indices(i, j) {
                        self.open(i_nbr, j_nbr);
                    }
                }
            }
            _ => (),
        }
        &self.cells[i][j].kind
    }

    fn flag(&mut self, i: usize, j: usize) -> &CellState {
        self.cells[i][j].flag()
    }

    fn state(&self) -> &BoardState {
        &self.state
    }

    fn height(&self) -> usize {
        self.cells.len()
    }

    fn width(&self) -> usize {
        self.cells[0].len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
