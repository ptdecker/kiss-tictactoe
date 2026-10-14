//! # kiss-tictactoe-engine
//!
//! Platform-agnostic game engine for tic-tac-toe.
//!
//! [`Game`] is the primary entry point. Create a game, call
//! [`Game::make_move`] with a [`Position`], and the engine plays `O`
//! automatically in the same call.
//!
//! ```
//! use kiss_tictactoe_engine::{Game, GameStatus, MoveResult, Player, Position};
//!
//! let mut game = Game::new();
//! let result = game.make_move(Position { row: 1, col: 1 }).unwrap();
//! assert_eq!(result, MoveResult::Continuing);
//! ```

#![warn(missing_docs)]

mod ai;
mod detection;
mod game;
mod types;

pub use detection::detect_status;
pub use game::Game;
pub use types::{Board, GameStatus, MoveError, MoveResult, Player, Position};
