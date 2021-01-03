use std::fmt;

use rand::Rng;

use cell::{Cell, CellKind};

mod cell;

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
        Board {
            cells: vec![
                vec![
                    Cell {
                        is_open: false,
                        kind: CellKind::Uninitialized,
                        mine_count: 0
                    };
                    config.width
                ];
                config.height
            ],
            is_initiated: false,
            mine_count: config.mine_count,
        }
    }

    fn initialize(&mut self, x: usize, y: usize) -> () {
        // mark root and its neighbors as free
        Board::traverse_neighbors(x, y, |i: usize, j: usize| {
            if self.cell_is_within_range(x + i - 1, y + j - 1) {
                self.cells[x + i - 1][y + j - 1].kind = CellKind::Free
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

    pub fn open(&mut self, x: usize, y: usize) -> () {
        if self.cell_is_within_range(x, y) {
            if !self.is_initiated {
                self.initialize(x, y);
            }
            if !self.cells[x][y].is_open {
                self.cells[x][y].is_open = true;
                let neighbor_mines = self.count_neighbors_for_mine(x, y);
                self.cells[x][y].mine_count = neighbor_mines;
                if neighbor_mines == 0 {
                    Board::traverse_neighbors(x, y, |i: usize, j: usize| {
                        if i <= self.get_height() - 1 && j <= self.get_width() - 1 {
                            self.open(x + i - 1, y + j - 1);
                        }
                    })
                }
            }
        }
    }

    fn count_neighbors_for_mine(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        Board::traverse_neighbors(x, y, |i: usize, j: usize| {
            if self.cell_is_within_range(x + i - 1, y + j - 1) {
                match self.cells[x + i - 1][y + j - 1].kind {
                    CellKind::Mine => {
                        count += 1;
                    }
                    _ => (),
                }
            }
        });
        count
    }

    fn cell_is_within_range(&self, x: usize, y: usize) -> bool {
        if x >= self.get_height() - 1 {
            return false;
        };
        if y >= self.get_width() - 1 {
            return false;
        };
        true
    }

    fn get_height(&self) -> usize {
        self.cells.len() + 1
    }

    fn get_width(&self) -> usize {
        self.cells[0].len() + 1
    }

    fn traverse_neighbors<F: FnMut(usize, usize)>(x: usize, y: usize, mut f: F) {
        for i in 0..3 {
            if x + i == 0 {
                continue;
            }
            for j in 0..3 {
                if y + j == 0 {
                    continue;
                }
                f(i, j);
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
