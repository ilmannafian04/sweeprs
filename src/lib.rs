//! sweeprs is a minesweeper engine written in rust.
//!
//! Example:
//!
//! ```
//! use sweeprs::{Board, BoardState, BoardResult};
//!
//! let mut board = Board::new(9, 9, 10);
//! board.open(4, 4);
//! board.flag(0, 0);
//! match board.state() {
//!     BoardState::Playing => println!("Keep going!"),
//!     BoardState::Finished(BoardResult::Win) => println!("You win!").
//!     _ => (),
//! }
//! ```

mod board;
mod cell;
mod error;

pub use board::*;
pub use cell::*;
pub use error::*;
