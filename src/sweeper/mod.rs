use std::fmt;

use rand::Rng;

use cell::Cell;

mod cell;

pub const EASY_CONFIG: BoardConfig = BoardConfig {height: 9, width: 9, mine_count: 10};
// pub const MED_CONFIG: BoardConfig = BoardConfig {height: 16, width: 16, mine_count: 40};
// pub const HARD_CONFIG: BoardConfig = BoardConfig {height: 24, width: 24, mine_count: 99};

pub struct BoardConfig {
    height: usize,
    width: usize,
    mine_count: usize,
}

pub struct Board {
    config: BoardConfig,
    cells: Vec<Vec<Cell>>,
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
        let mut board = vec![vec![Cell::Empty; config.width]; config.height];
        let mut placed_mine = 0;
        let mut rng = rand::thread_rng();
        while placed_mine < config.mine_count {
            let i = rng.gen_range(0..config.height - 1);
            let j = rng.gen_range(0..config.width - 1);
            match board[i][j] {
                Cell::Empty => {
                    board[i][j] = Cell::Mine;
                    placed_mine += 1;
                }
                _ => {}
            }
        }
        Board { config, cells: board }
    }
}
