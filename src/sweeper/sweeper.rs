use std::fmt;

use rand::Rng;

use crate::sweeper::cell::{Cell, CellKind, CellState};
use crate::sweeper::error::Error;

pub use crate::sweeper::constant::*;

pub struct BoardConfig {
    pub height: usize,
    pub width: usize,
    pub mine_count: usize,
}

#[derive(Clone, Debug)]
pub enum SweeperState {
    Uninitialized,
    Playing,
    Lost,
    Win,
}

pub struct Sweeper {
    board: Vec<Vec<Cell>>,
    state: SweeperState,
    closed_cell: usize,
    mine_count: usize,
}

impl Sweeper {
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
            Ok(Sweeper {
                board: vec![vec![cell; config.width]; config.height],
                state: SweeperState::Uninitialized,
                closed_cell: config.width * config.height,
                mine_count: config.mine_count,
            })
        }
    }

    pub fn open(&mut self, i: usize, j: usize) -> Option<CellKind> {
        match self.board[i][j].state {
            CellState::Closed => {
                match self.state {
                    SweeperState::Uninitialized => self.initialize(i, j),
                    _ => (),
                }
                match self.board[i][j].kind {
                    CellKind::Free => {
                        let count = self.count_mine_in_nbrs(i, j);
                        let cell = &mut self.board[i][j];
                        cell.state = CellState::Open;
                        cell.mine_count = count;
                        cell.mine_is_counted = true;
                        if count == 0 {
                            for (nbr_i, nbr_j) in self.get_nbr_indices(i, j) {
                                self.open(nbr_i, nbr_j);
                            }
                        }
                        self.closed_cell -= 1;
                        if self.closed_cell <= self.mine_count {
                            self.state = SweeperState::Win;
                        }
                    }
                    CellKind::Mine => {
                        self.state = SweeperState::Lost;
                        self.open_all_cell();
                    }
                    _ => (),
                }
                Some(self.board[i][j].kind.clone())
            }
            _ => None,
        }
    }

    pub fn flag(&mut self, i: usize, j: usize) -> Option<CellState> {
        match self.board[i][j].state {
            CellState::Closed => {
                self.board[i][j].state = CellState::Flagged;
                Some(CellState::Flagged)
            }
            CellState::Flagged => {
                self.board[i][j].state = CellState::Closed;
                Some(CellState::Closed)
            }
            _ => None,
        }
    }

    pub fn game_state(&self) -> SweeperState {
        self.state.clone()
    }

    fn open_all_cell(&mut self) {
        for i in 0..self.get_height() {
            for j in 0..self.get_width() {
                match self.board[i][j].state {
                    CellState::Closed => {
                        self.board[i][j].state = CellState::Open;
                        if !self.board[i][j].mine_is_counted {
                            let count = self.count_mine_in_nbrs(i, j);
                            self.board[i][j].mine_count = count;
                            self.board[i][j].mine_is_counted = true;
                        }
                        self.closed_cell -= 1;
                    }
                    _ => (),
                }
            }
        }
    }

    fn initialize(&mut self, i: usize, j: usize) -> () {
        // mark root and its neighbors as free
        self.board[i][j].kind = CellKind::Free;
        for (nbr_i, nbr_j) in self.get_nbr_indices(i, j) {
            self.board[nbr_i][nbr_j].kind = CellKind::Free;
        }
        // randomize mine placement
        let mut placed_mine = 0;
        let mut rng = rand::thread_rng();
        while placed_mine < self.mine_count {
            let i = rng.gen_range(0..self.get_height());
            let j = rng.gen_range(0..self.get_width());
            match self.board[i][j].kind {
                CellKind::Uninitialized => {
                    self.board[i][j].kind = CellKind::Mine;
                    placed_mine += 1;
                }
                _ => (),
            }
        }
        // fill the rest of uninitialized cells with free cell
        for i in 0..self.get_height() {
            for j in 0..self.get_width() {
                match self.board[i][j].kind {
                    CellKind::Uninitialized => {
                        self.board[i][j].kind = CellKind::Free;
                    }
                    _ => (),
                }
            }
        }
        self.state = SweeperState::Playing;
    }

    fn count_mine_in_nbrs(&self, i: usize, j: usize) -> usize {
        if self.board[i][j].mine_is_counted {
            self.board[i][j].mine_count
        } else {
            let mut count = 0;
            for (nbr_i, nbr_j) in self.get_nbr_indices(i, j) {
                match self.board[nbr_i][nbr_j].kind {
                    CellKind::Mine => count += 1,
                    _ => (),
                }
            }
            count
        }
    }

    fn get_nbr_indices(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
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
        self.board.len()
    }

    fn get_width(&self) -> usize {
        self.board[0].len()
    }
}

impl fmt::Debug for Sweeper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();
        for row in &self.board {
            for cell in row {
                debug_str.push_str(&format!("| {:?} ", cell));
            }
            debug_str.push_str("|\n");
        }
        f.write_str(&debug_str)
    }
}
