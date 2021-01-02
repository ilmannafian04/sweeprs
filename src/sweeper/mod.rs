use std::fmt;

use rand::Rng;

use cell::Cell;

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

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_str = String::new();
        for row in &self.cells {
            debug_str.push_str(&format!("{:?}\n", row));
        }
        f.write_str(&debug_str)
    }
}

impl Board {
    pub fn new(config: BoardConfig) -> Self {
        Board {
            cells: vec![vec![Cell::Uninitialized; config.width]; config.height],
            is_initiated: false,
            mine_count: config.mine_count,
        }
    }

    fn initialize(&mut self, x: usize, y: usize) -> () {
        // mark root and its neighbors as free
        for i in 0..3 {
            if x == 0 && i == 0 {
                continue;
            };
            for j in 0..3 {
                if y == 0 && j == 0 {
                    continue;
                };
                match self.cells.get_mut(x + i - 1) {
                    Some(row) => match row.get_mut(y + j - 1) {
                        Some(cell) => {
                            *cell = Cell::Free;
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
        println!("{:?}", self);
        // randomize mine placement
        let mut placed_mine = 0;
        let mut rng = rand::thread_rng();
        while placed_mine < self.mine_count {
            let i = rng.gen_range(0..self.get_height());
            let j = rng.gen_range(0..self.get_width());
            match self.cells[i][j] {
                Cell::Uninitialized => {
                    self.cells[i][j] = Cell::Mine;
                    placed_mine += 1;
                }
                _ => {}
            }
        }
        println!("{:?}", self);
        // fill the rest of uninitialized cells with free cell
        for i in 0..self.get_height() {
            for j in 0..self.get_width() {
                match self.cells[i][j] {
                    Cell::Uninitialized => {
                        self.cells[i][j] = Cell::Free;
                    }
                    _ => (),
                }
            }
        }
        println!("{:?}", self);
        self.is_initiated = true;
    }

    pub fn open(&mut self, x: usize, y: usize) -> () {
        if !self.is_initiated {
            self.initialize(x, y);
        }
    }

    fn get_height(&self) -> usize {
        self.cells.len()
    }

    fn get_width(&self) -> usize {
        self.cells[0].len()
    }
}
