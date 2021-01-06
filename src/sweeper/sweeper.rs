use std::fmt;

use rand::Rng;

use super::cell::{Cell, CellKind};
use super::error::Error;

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
            is_open: false,
            kind: CellKind::Uninitialized,
            mine_count: 0,
            mine_is_counted: false,
        };
        Board {
            cells: vec![vec![cell; config.width]; config.height],
            is_initiated: false,
            mine_count: config.mine_count,
        }
    }

    pub fn open(&mut self, x: usize, y: usize) -> Result<CellKind, Error> {
        if self.cell_is_within_range(x, y) && !self.cells[x][y].is_open {
            if !self.is_initiated {
                self.initialize(x, y);
            }
            let count = self.count_mine_in_neighbors(x, y);
            let cell = &mut self.cells[x][y];
            if !cell.is_open {
                cell.is_open = true;
                cell.mine_count = count;
                cell.mine_is_counted = true;
                if count == 0 {
                    Board::traverse_neighbors(x, y, |x_shift, y_shift| {
                        self.open(x + x_shift - 1, y + y_shift - 1).ok();
                    })
                }
            }
            Ok(self.cells[x][y].kind.clone())
        } else {
            Err(Error::CellOutOfBound)
        }
    }

    pub fn flag(&mut self, x: usize, y: usize) -> Result<bool, Error> {
        if self.cell_is_within_range(x, y) {
            Ok(true)
        } else {
            Err(Error::CellOutOfBound)
        }
    }

    fn initialize(&mut self, x: usize, y: usize) -> () {
        // mark root and its neighbors as free
        self.cells[x][y].kind = CellKind::Free;
        Board::traverse_neighbors(x, y, |x_shift, y_shift: usize| {
            match self.get_mut_cell(x + x_shift - 1, y + y_shift - 1) {
                Some(cell) => cell.kind = CellKind::Free,
                _ => (),
            }
        });
        // randomize mine placement
        let mut placed_mine = 0;
        let mut rng = rand::thread_rng();
        while placed_mine < self.mine_count {
            let i = rng.gen_range(0..self.get_height() - 1);
            let j = rng.gen_range(0..self.get_width() - 1);
            match self.cells[i][j].kind {
                CellKind::Uninitialized => {
                    self.cells[i][j].kind = CellKind::Mine;
                    placed_mine += 1;
                }
                _ => (),
            }
        }
        // fill the rest of uninitialized cells with free cell
        for i in 0..self.get_height() - 1 {
            for j in 0..self.get_width() - 1 {
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

    fn count_mine_in_neighbors(&self, x: usize, y: usize) -> usize {
        if self.cells[x][y].mine_is_counted {
            self.cells[x][y].mine_count
        } else {
            let mut count = 0;
            Board::traverse_neighbors(x, y, |x_shift, y_shift| {
                match self.get_cell(x + x_shift - 1, y + y_shift - 1) {
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

    fn cell_is_within_range(&self, x: usize, y: usize) -> bool {
        if x >= self.get_width() - 1 {
            return false;
        };
        if y >= self.get_height() - 1 {
            return false;
        };
        true
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(y).and_then(|row| row.get(x))
    }

    fn get_mut_cell(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        self.cells.get_mut(y).and_then(|row| row.get_mut(x))
    }

    fn get_height(&self) -> usize {
        self.cells.len() + 1
    }

    fn get_width(&self) -> usize {
        self.cells[0].len() + 1
    }

    fn traverse_neighbors<F: FnMut(usize, usize)>(x: usize, y: usize, mut f: F) {
        for y_shift in 0..3 {
            if y + y_shift == 0 {
                continue;
            }
            for x_shift in 0..3 {
                if x + x_shift == 0 || (x_shift == 1 && y_shift == 1) {
                    continue;
                }
                f(x_shift, y_shift);
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
