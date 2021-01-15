use crate::sweeper::BoardConfig;

pub const EASY_CONFIG: BoardConfig = BoardConfig {
    height: 9,
    width: 9,
    mine_count: 10,
};
pub const MED_CONFIG: BoardConfig = BoardConfig {
    height: 16,
    width: 16,
    mine_count: 40,
};
pub const HARD_CONFIG: BoardConfig = BoardConfig {
    height: 24,
    width: 24,
    mine_count: 99,
};
