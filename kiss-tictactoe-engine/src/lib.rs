//! # kiss-tictactoe-engine
//!
//! Platform-agnostic game engine for tic-tac-toe.
//!
//! [`Game`] is the primary entry point. Create a game, call
//! [`Game::make_move`] with a [`Position`], and the engine plays `O`
//! automatically in the same call. [`Game::play`] does the same but returns a
//! [`Turn`] that also carries an optional [`TalkEvent`]; hand that to a
//! [`Commentator`] to get a line of trash talk.
//!
//! ```
//! use kiss_tictactoe_engine::{Game, GameStatus, MoveResult, Player, Position, TalkEvent};
//!
//! let mut game = Game::new();
//! let result = game.make_move(Position { row: 1, col: 1 }).unwrap();
//! assert_eq!(result, MoveResult::Continuing);
//! ```

#![warn(missing_docs)]

mod ai;
mod commentary;
mod detection;
mod events;
mod game;
mod types;

pub use commentary::Commentator;
pub use detection::detect_status;
pub use game::{Game, Turn};
pub use types::{Board, GameStatus, MoveError, MoveResult, Player, Position, TalkEvent};
