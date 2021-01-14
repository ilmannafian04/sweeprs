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
    pub fn new(config: BoardConfig) -> Result<Self, Error> {
        if config.height < 9 || config.width < 9 {
            Err(Error::InvalidConfig)
        } else {
            let cell = Cell {
                kind: CellKind::Uninitialized,
                state: CellState::Closed,
                mine_count: 0,
                mine_is_counted: false,
            };
            Ok(Board {
                cells: vec![vec![cell; config.width]; config.height],
                is_initiated: false,
                mine_count: config.mine_count,
            })
        }
    }

    pub fn open(&mut self, i: usize, j: usize) -> Option<CellKind> {
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
                    for (nbr_i, nbr_j) in self.get_neighbor_index(i, j) {
                        self.open(nbr_i, nbr_j);
                    }
                }
                Some(self.cells[i][j].kind.clone())
            }
            _ => None,
        }
    }

    pub fn flag(&mut self, i: usize, j: usize) -> Option<CellState> {
        match self.cells[i][j].state {
            CellState::Closed => {
                self.cells[i][j].state = CellState::Flagged;
                Some(CellState::Flagged)
            }
            CellState::Flagged => {
                self.cells[i][j].state = CellState::Closed;
                Some(CellState::Closed)
            }
            _ => None,
        }
    }

    fn initialize(&mut self, i: usize, j: usize) -> () {
        // mark root and its neighbors as free
        self.cells[i][j].kind = CellKind::Free;
        for (nbr_i, nbr_j) in self.get_neighbor_index(i, j) {
            self.cells[nbr_i][nbr_j].kind = CellKind::Free;
        }
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
            for (nbr_i, nbr_j) in self.get_neighbor_index(i, j) {
                match self.cells[nbr_i][nbr_j].kind {
                    CellKind::Mine => count += 1,
                    _ => (),
                }
            }
            count
        }
    }

    fn get_neighbor_index(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        for i_shift in 0..3 {
            if i + i_shift == 0 || i + i_shift - 1 >= self.get_height() {
                continue;
            }
            for j_shift in 0..3 {
                if j + j_shift == 0
                    || j + j_shift - 1 >= self.get_width()
                    || (i_shift == 1 && j_shift == 1)
                {
                    continue;
                }
                res.push((i + i_shift - 1, j + j_shift - 1));
            }
        }
        res
    }

    fn get_height(&self) -> usize {
        self.cells.len()
    }

    fn get_width(&self) -> usize {
        self.cells[0].len()
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
