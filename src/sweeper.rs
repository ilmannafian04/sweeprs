use std::fmt;

pub const EASY_CONFIG: BoardConfig = BoardConfig {height: 9, width: 9, mine_count: 10};
pub const MED_CONFIG: BoardConfig = BoardConfig {height: 16, width: 16, mine_count: 40};
pub const HARD_CONFIG: BoardConfig = BoardConfig {height: 24, width: 24, mine_count: 99};

#[derive(Clone)]
enum Cell {
    Mine,
    Flag,
    Empty,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => f.write_str("  "),
            Cell::Flag => f.write_str("! "),
            Cell::Mine => f.write_str("X "),
        }
    }
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

pub struct BoardConfig {
    height: usize,
    width: usize,
    mine_count: usize,
}

impl Board {
    pub fn new(config: BoardConfig) -> Self {
        let board = vec![vec![Cell::Empty; config.width]; config.height];
        Board { config, cells: board }
    }
}
