//! The `Game` struct: the primary entry point for the engine API.

use crate::ai::{best_engine_move, move_quality};
use crate::detection::detect_status;
use crate::events::{classify, has_threat};
use crate::types::{Board, GameStatus, MoveError, MoveResult, Player, Position, TalkEvent};

/// One full turn: the human's move plus the engine's reply.
///
/// `event` holds at most one trash-talk event, chosen by fixed priority
/// (D-03).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Turn {
    /// What happened to the game as a result of this turn.
    pub result: MoveResult,
    /// The trash-talk event for this turn, if any.
    pub event: Option<TalkEvent>,
}

/// A tic-tac-toe game tracking board state, current player, game status,
/// and move history.
///
/// The human always plays as `X` (moves first); the engine always plays as
/// `O`. After each valid human move the engine responds automatically inside
/// the same [`Game::play`] (or [`Game::make_move`]) call.
///
/// # Example
///
/// ```
/// use kiss_tictactoe_engine::{Game, GameStatus, MoveResult, Player, Position};
///
/// let mut game = Game::new();
/// assert_eq!(game.status(), GameStatus::InProgress);
/// assert_eq!(game.current_player(), Player::X);
///
/// let result = game.make_move(Position { row: 1, col: 1 }).unwrap();
/// assert_eq!(result, MoveResult::Continuing);
/// // The engine has already replied — there are exactly 2 history entries.
/// assert_eq!(game.history().len(), 2);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    board: Board,
    current_player: Player,
    status: GameStatus,
    history: Vec<Position>,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    /// Create a new game with an empty board, `X` to move first.
    pub fn new() -> Self {
        Self {
            board: [[None; 3]; 3],
            current_player: Player::X,
            status: GameStatus::InProgress,
            history: Vec::new(),
        }
    }

    /// Attempt to place the human's mark (`X`) at `pos`, then have the engine
    /// respond with `O` if the game continues.
    ///
    /// # Errors
    ///
    /// Returns an error (without mutating state) when:
    /// - [`MoveError::GameAlreadyOver`]: the game has already ended
    /// - [`MoveError::InvalidPosition`]: `pos.row > 2` or `pos.col > 2`
    /// - [`MoveError::CellOccupied`]: the target cell is not empty
    pub fn make_move(&mut self, pos: Position) -> Result<MoveResult, MoveError> {
        self.play(pos).map(|t| t.result)
    }

    /// Play one full turn (human move plus engine reply) and report the
    /// result together with at most one trash-talk event.
    ///
    /// Event priority (D-03, amended): game over, bad move, engine threat,
    /// user threat, good move.
    ///
    /// # Errors
    ///
    /// Returns an error (without mutating state) when:
    /// - [`MoveError::GameAlreadyOver`]: the game has already ended
    /// - [`MoveError::InvalidPosition`]: `pos.row > 2` or `pos.col > 2`
    /// - [`MoveError::CellOccupied`]: the target cell is not empty
    ///
    /// # Example
    ///
    /// ```
    /// use kiss_tictactoe_engine::{Game, MoveResult, Position};
    ///
    /// let mut game = Game::new();
    /// let turn = game.play(Position { row: 1, col: 1 }).unwrap();
    /// assert_eq!(turn.result, MoveResult::Continuing);
    /// assert_eq!(game.history().len(), 2);
    /// ```
    pub fn play(&mut self, pos: Position) -> Result<Turn, MoveError> {
        // Guard: game must be in progress
        if self.status != GameStatus::InProgress {
            return Err(MoveError::GameAlreadyOver);
        }

        // Guard: position must be within bounds
        if pos.row > 2 || pos.col > 2 {
            return Err(MoveError::InvalidPosition);
        }

        // Guard: cell must be empty
        if self.board[pos.row as usize][pos.col as usize].is_some() {
            return Err(MoveError::CellOccupied);
        }

        // Judge the move against the board BEFORE it is placed (D-02)
        let quality = move_quality(&self.board, pos);

        // Place the human's X
        self.board[pos.row as usize][pos.col as usize] = Some(Player::X);
        self.history.push(pos);
        self.status = detect_status(&self.board);

        // Human's move ended the game
        if self.status != GameStatus::InProgress {
            let result = MoveResult::GameOver(self.status);
            return Ok(Turn { result, event: classify(self.status, false, false, quality) });
        }

        // User threat is seen before the engine blocks it (D-04).
        // INVARIANT: the engine is perfect, so it always blocks (or wins
        // over) an open X threat. If the engine ever becomes imperfect
        // (TRAIN-01), re-check that the threat was actually blocked after
        // the reply before reporting UserAboutToWin.
        let user_threat = has_threat(&self.board, Player::X);

        // Engine responds as O
        self.current_player = Player::O;

        match best_engine_move(&self.board) {
            Some(engine_pos) => {
                self.board[engine_pos.row as usize][engine_pos.col as usize] = Some(Player::O);
                self.history.push(engine_pos);
                self.status = detect_status(&self.board);
            }
            None => {
                // Unreachable in well-formed play: detect_status would have
                // returned Draw before best_engine_move finds no moves.
                // Handled defensively without panicking (ENGI-04).
                self.status = detect_status(&self.board);
            }
        }

        // Restore turn to X regardless of outcome
        self.current_player = Player::X;

        let result = if self.status != GameStatus::InProgress {
            MoveResult::GameOver(self.status)
        } else {
            MoveResult::Continuing
        };
        let engine_threat =
            self.status == GameStatus::InProgress && has_threat(&self.board, Player::O);
        Ok(Turn { result, event: classify(self.status, engine_threat, user_threat, quality) })
    }

    /// The current game status.
    pub fn status(&self) -> GameStatus {
        self.status
    }

    /// A reference to the current board state.
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// All positions played so far, in order (human and engine moves
    /// interleaved: index 0 is the human's first move, index 1 is the
    /// engine's first reply, etc.).
    pub fn history(&self) -> &[Position] {
        &self.history
    }

    /// The player whose turn it is (always [`Player::X`] between human
    /// moves; briefly [`Player::O`] while the engine is computing inside
    /// [`Game::make_move`]).
    pub fn current_player(&self) -> Player {
        self.current_player
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_is_empty_and_x_to_move() {
        let game = Game::new();
        assert_eq!(game.status(), GameStatus::InProgress);
        assert_eq!(game.current_player(), Player::X);
        assert!(game.history().is_empty());
        for row in game.board().iter() {
            for cell in row.iter() {
                assert!(cell.is_none());
            }
        }
    }

    #[test]
    fn default_equals_new() {
        assert_eq!(Game::default(), Game::new());
    }

    /// After two make_move calls, history has 4 entries; entries 0 and 2 are
    /// the human's chosen positions (indices 1 and 3 are the engine's replies).
    #[test]
    fn history_records_human_then_engine_moves() {
        let mut game = Game::new();
        let pos1 = Position { row: 0, col: 0 };
        let _ = game.make_move(pos1);
        // After one make_move: X placed at pos1, engine replied → 2 entries
        assert_eq!(game.history().len(), 2);
        assert_eq!(game.history()[0], pos1);

        let pos2 = Position { row: 2, col: 2 };
        let _ = game.make_move(pos2);
        // After two make_move calls: 4 entries total
        assert_eq!(game.history().len(), 4);
        assert_eq!(game.history()[2], pos2); // index 2 is the second human move
    }

    /// current_player() is always X between human turns.
    #[test]
    fn current_player_is_x_between_turns() {
        let mut game = Game::new();
        assert_eq!(game.current_player(), Player::X);
        let _ = game.make_move(Position { row: 1, col: 1 });
        // Engine has already responded; current player is restored to X
        assert_eq!(game.current_player(), Player::X);
    }

    #[test]
    fn invalid_position_rejected() {
        let mut game = Game::new();
        assert_eq!(game.make_move(Position { row: 3, col: 0 }), Err(MoveError::InvalidPosition));
        assert_eq!(game.make_move(Position { row: 0, col: 3 }), Err(MoveError::InvalidPosition));
        assert_eq!(game, Game::new());
    }

    #[test]
    fn occupied_cell_rejected() {
        let mut game = Game::new();
        let _ = game.make_move(Position { row: 0, col: 0 });
        let snapshot = game.clone();
        let x_pos = game.history()[0];
        assert_eq!(game.make_move(x_pos), Err(MoveError::CellOccupied));
        assert_eq!(game, snapshot);
    }

    #[test]
    fn game_already_over_rejected() {
        let mut game = Game::new();
        // Play until done
        while game.status() == GameStatus::InProgress {
            for row in 0..3u8 {
                for col in 0..3u8 {
                    if game.board()[row as usize][col as usize].is_none() {
                        let _ = game.make_move(Position { row, col });
                        break;
                    }
                }
                if game.status() != GameStatus::InProgress {
                    break;
                }
            }
        }
        assert_ne!(game.status(), GameStatus::InProgress);
        let err = game.make_move(Position { row: 0, col: 0 });
        assert_eq!(err, Err(MoveError::GameAlreadyOver));
    }

    #[test]
    fn play_error_leaves_state_unchanged() {
        let mut game = Game::new();
        game.play(Position { row: 0, col: 0 }).expect("legal move");
        let snapshot = game.clone();
        let occupied = game.history()[0];
        assert_eq!(game.play(occupied), Err(MoveError::CellOccupied));
        assert_eq!(game, snapshot);
        assert_eq!(game.play(Position { row: 3, col: 0 }), Err(MoveError::InvalidPosition));
        assert_eq!(game, snapshot);
    }
}
