pub mod cell;
pub mod error;
mod board;

pub use board::{Board, SweeperConfig, SweeperState};

pub const EASY_CONFIG: SweeperConfig = SweeperConfig {
    height: 9,
    width: 9,
    mine_count: 10,
};
pub const MED_CONFIG: SweeperConfig = SweeperConfig {
    height: 16,
    width: 16,
    mine_count: 40,
};
pub const HARD_CONFIG: SweeperConfig = SweeperConfig {
    height: 24,
    width: 24,
    mine_count: 99,
};
