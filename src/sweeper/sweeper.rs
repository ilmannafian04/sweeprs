use std::fmt;

use rand::Rng;

use crate::sweeper::cell::{Cell, CellKind, CellState};
use crate::sweeper::error::Error;

pub const EASY_CONFIG: BoardConfig = BoardConfig {
    height: 9,
    width: 9,
    mine_count: 10,
};
// pub const MED_CONFIG: BoardConfig = BoardConfig {height: 16, width: 16, mine_count: 40};
// pub const HARD_CONFIG: BoardConfig = BoardConfig {height: 24, width: 24, mine_count: 99};

pub struct BoardConfig {
    height: usize,
    width: usize,
    mine_count: usize,
}

pub struct Board {
    cells: Vec<Vec<Cell>>,
    is_initiated: bool,
    mine_count: usize,
}

impl Board {
    pub fn new(config: BoardConfig) -> Self {
        let cell = Cell {
            kind: CellKind::Uninitialized,
            state: CellState::Closed,
            mine_count: 0,
            mine_is_counted: false,
        };
        Board {
            cells: vec![vec![cell; config.width]; config.height],
            is_initiated: false,
            mine_count: config.mine_count,
        }
    }

    pub fn open(&mut self, i: usize, j: usize) -> Option<Result<CellKind, Error>> {
        if self.cell_is_within_range(i, j) {
            match self.cells[i][j].state {
                CellState::Closed => {
                    if !self.is_initiated {
                        self.initialize(i, j);
                    }
                    let count = self.count_mine_in_neighbors(i, j);
                    let cell = &mut self.cells[i][j];
                    cell.state = CellState::Open;
                    cell.mine_count = count;
                    cell.mine_is_counted = true;
                    if count == 0 {
                        Board::traverse_neighbors(i, j, |i_shift, j_shift| {
                            self.open(i + i_shift - 1, j + j_shift - 1);
                        })
                    }
                    Some(Ok(self.cells[i][j].kind.clone()))
                }
                _ => None,
            }
        } else {
            Some(Err(Error::CellOutOfBound))
        }
    }

    pub fn flag(&mut self, i: usize, j: usize) -> Result<bool, Error> {
        if self.cell_is_within_range(i, j) {
            Ok(true)
        } else {
            Err(Error::CellOutOfBound)
        }
    }

    fn initialize(&mut self, i: usize, j: usize) -> () {
        // mark root and its neighbors as free
        self.cells[i][j].kind = CellKind::Free;
        Board::traverse_neighbors(i, j, |i_shift, j_shift: usize| {
            match self.get_mut_cell(i + i_shift - 1, j + j_shift - 1) {
                Some(cell) => cell.kind = CellKind::Free,
                _ => (),
            }
        });
        // randomize mine placement
        let mut placed_mine = 0;
        let mut rng = rand::thread_rng();
        while placed_mine < self.mine_count {
            let i = rng.gen_range(0..self.get_height());
            let j = rng.gen_range(0..self.get_width());
            match self.cells[i][j].kind {
                CellKind::Uninitialized => {
                    self.cells[i][j].kind = CellKind::Mine;
                    placed_mine += 1;
                }
                _ => (),
            }
        }
        // fill the rest of uninitialized cells with free cell
        for i in 0..self.get_height() {
            for j in 0..self.get_width() {
                match self.cells[i][j].kind {
                    CellKind::Uninitialized => {
                        self.cells[i][j].kind = CellKind::Free;
                    }
                    _ => (),
                }
            }
        }
        self.is_initiated = true;
    }

    fn count_mine_in_neighbors(&self, i: usize, j: usize) -> usize {
        if self.cells[i][j].mine_is_counted {
            self.cells[i][j].mine_count
        } else {
            let mut count = 0;
            Board::traverse_neighbors(i, j, |i_shift, j_shift| {
                match self.get_cell((i + i_shift) - 1, (j + j_shift) - 1) {
                    Some(cell) => match cell.kind {
                        CellKind::Mine => count += 1,
                        _ => (),
                    },
                    _ => (),
                }
            });
            count
        }
    }

    fn cell_is_within_range(&self, i: usize, j: usize) -> bool {
        if i >= self.get_height() {
            return false;
        };
        if j >= self.get_width() {
            return false;
        };
        true
    }

    fn get_cell(&self, i: usize, j: usize) -> Option<&Cell> {
        self.cells.get(i).and_then(|row| row.get(j))
    }

    fn get_mut_cell(&mut self, i: usize, j: usize) -> Option<&mut Cell> {
        self.cells.get_mut(i).and_then(|row| row.get_mut(j))
    }

    fn get_height(&self) -> usize {
        self.cells.len()
    }

    fn get_width(&self) -> usize {
        self.cells[0].len()
    }

    fn traverse_neighbors<F: FnMut(usize, usize)>(i: usize, j: usize, mut f: F) {
        for i_shift in 0..3 {
            if i + i_shift == 0 {
                continue;
            }
            for j_shift in 0..3 {
                if j + j_shift == 0 || (i_shift == 1 && j_shift == 1) {
                    continue;
                }
                f(i_shift, j_shift);
            }
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();
        for row in &self.cells {
            for cell in row {
                debug_str.push_str(&format!("| {:?} ", cell));
            }
            debug_str.push_str("|\n");
        }
        f.write_str(&debug_str)
    }
}
