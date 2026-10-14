//! Core types for the kiss-tictactoe engine.

use std::fmt;

/// A position on the 3×3 board using named row and column fields.
///
/// Both `row` and `col` must be in the range `0..=2`. Values outside this
/// range cause [`crate::Game::make_move`] to return
/// [`MoveError::InvalidPosition`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    /// Row index (0 = top, 2 = bottom).
    pub row: u8,
    /// Column index (0 = left, 2 = right).
    pub col: u8,
}

/// Which player occupies a cell, or whose turn it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    /// The human player; always moves first.
    X,
    /// The engine player; always moves second.
    O,
}

/// The 3×3 game board, indexed as `board[row][col]`.
///
/// Each cell contains either `None` (empty) or `Some(Player)`.
pub type Board = [[Option<Player>; 3]; 3];

/// The current state of the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    /// The game is still in progress.
    InProgress,
    /// A player has completed a row, column, or diagonal.
    Win(Player),
    /// All cells are filled and neither player won.
    Draw,
}

/// The outcome of a successful [`crate::Game::make_move`] call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveResult {
    /// The move was accepted and the game continues.
    Continuing,
    /// The move ended the game; contains the final [`GameStatus`].
    GameOver(GameStatus),
}

/// Reasons why a move was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveError {
    /// The target cell is already occupied.
    CellOccupied,
    /// The position is outside the 3×3 grid.
    InvalidPosition,
    /// The game has already ended; no further moves are accepted.
    GameAlreadyOver,
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveError::CellOccupied => write!(f, "cell is already occupied"),
            MoveError::InvalidPosition => write!(f, "position is outside the 3x3 board"),
            MoveError::GameAlreadyOver => write!(f, "the game is already over"),
        }
    }
}

impl std::error::Error for MoveError {}
