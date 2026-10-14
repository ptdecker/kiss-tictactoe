//! Minimax AI for the tic-tac-toe engine.
//!
//! The engine plays as `O` (minimizer). The human plays as `X` (maximizer).
//! Alpha-beta pruning is used to reduce the search tree.
//! Depth-adjusted scoring ensures the engine prefers faster wins and slower
//! losses: a win at depth 1 scores better than one at depth 3.

use crate::detection::detect_status;
use crate::types::{Board, GameStatus, Player, Position};

/// Return all empty cells in row-major order.
fn empty_cells(board: &Board) -> Vec<Position> {
    let mut cells = Vec::new();
    for row in 0..3u8 {
        for col in 0..3u8 {
            if board[row as usize][col as usize].is_none() {
                cells.push(Position { row, col });
            }
        }
    }
    cells
}

/// Minimax with alpha-beta pruning.
///
/// `X` is the maximizer; `O` is the minimizer.
/// Scores are depth-adjusted so faster terminal states are preferred.
fn minimax(board: &Board, depth: i32, is_maximizing: bool, alpha: i32, beta: i32) -> i32 {
    match detect_status(board) {
        GameStatus::Win(Player::X) => return 10 - depth,
        GameStatus::Win(Player::O) => return depth - 10,
        GameStatus::Draw => return 0,
        GameStatus::InProgress => {}
    }

    if is_maximizing {
        let mut best = i32::MIN;
        let mut alpha = alpha;
        for pos in empty_cells(board) {
            let mut b = *board;
            b[pos.row as usize][pos.col as usize] = Some(Player::X);
            let score = minimax(&b, depth + 1, false, alpha, beta);
            if score > best {
                best = score;
            }
            if best > alpha {
                alpha = best;
            }
            if beta <= alpha {
                break;
            }
        }
        best
    } else {
        let mut best = i32::MAX;
        let mut beta = beta;
        for pos in empty_cells(board) {
            let mut b = *board;
            b[pos.row as usize][pos.col as usize] = Some(Player::O);
            let score = minimax(&b, depth + 1, true, alpha, beta);
            if score < best {
                best = score;
            }
            if best < beta {
                beta = best;
            }
            if beta <= alpha {
                break;
            }
        }
        best
    }
}

/// Find the best move for the engine (O) on the given board.
///
/// Returns `None` when no empty cells remain (i.e., the board is full).
/// Never returns an occupied position.
pub(crate) fn best_engine_move(board: &Board) -> Option<Position> {
    let cells = empty_cells(board);
    if cells.is_empty() {
        return None;
    }

    let mut best_score = i32::MAX;
    let mut best_pos = None;

    for pos in cells {
        let mut b = *board;
        b[pos.row as usize][pos.col as usize] = Some(Player::O);
        let score = minimax(&b, 0, true, i32::MIN, i32::MAX);
        if score < best_score {
            best_score = score;
            best_pos = Some(pos);
        }
    }

    best_pos
}

/// How good the user's move was, judged by the engine's own minimax (D-02).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MoveQuality {
    /// The move preserves the best outcome class X can still force.
    Good,
    /// The move drops to a worse outcome class than the best available.
    Bad,
}

/// Judge the user's move `pos` (as X) on the board `before` the move.
///
/// `before` must be the board BEFORE the user's move is placed (Pitfall 3).
/// Reuses the engine's [`minimax`] rather than a second evaluator (D-02).
/// Outcomes are compared by `signum()` (win/draw/loss class), never by raw
/// depth-adjusted scores (Pitfall 4), except within the losing class, where
/// a faster loss than the best available is judged Bad.
pub(crate) fn move_quality(before: &Board, pos: Position) -> MoveQuality {
    let score_of = |p: Position| {
        let mut b = *before;
        b[p.row as usize][p.col as usize] = Some(Player::X);
        minimax(&b, 0, false, i32::MIN, i32::MAX)
    };
    let best = empty_cells(before)
        .into_iter()
        .map(score_of)
        .max()
        .unwrap_or(0);
    let chosen = score_of(pos);
    // When every move loses, a move that loses sooner than the best
    // available (depth-adjusted score) is still Bad.
    if chosen.signum() < best.signum() || (best < 0 && chosen < best) {
        MoveQuality::Bad
    } else {
        MoveQuality::Good
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;
    use crate::types::MoveError;
    use crate::types::Player::{O, X};
    use proptest::prelude::*;
    use rstest::rstest;

    // ── Helpers ───────────────────────────────────────────────────────────

    /// Build a board from slices of (row, col) pairs for X and O.
    fn board_from(xs: &[(u8, u8)], os: &[(u8, u8)]) -> Board {
        let mut board = [[None; 3]; 3];
        for &(r, c) in xs {
            board[r as usize][c as usize] = Some(Player::X);
        }
        for &(r, c) in os {
            board[r as usize][c as usize] = Some(Player::O);
        }
        board
    }

    // ── Move quality (D-02) ───────────────────────────────────────────────

    #[rstest]
    #[case(board_from(&[], &[]), Position { row: 1, col: 1 }, MoveQuality::Good)]
    #[case(board_from(&[], &[]), Position { row: 0, col: 0 }, MoveQuality::Good)]
    #[case(board_from(&[(0, 0), (2, 2)], &[(1, 0), (1, 1)]), Position { row: 1, col: 2 }, MoveQuality::Good)]
    #[case(board_from(&[(0, 0), (2, 2)], &[(1, 0), (1, 1)]), Position { row: 0, col: 2 }, MoveQuality::Bad)]
    #[case(board_from(&[(0, 0), (0, 1)], &[(1, 0), (1, 1)]), Position { row: 0, col: 2 }, MoveQuality::Good)]
    // Doomed position: every move loses, but only (2,2) delays the loss.
    #[case(board_from(&[(0, 0), (0, 1)], &[(0, 2), (1, 2)]), Position { row: 2, col: 2 }, MoveQuality::Good)]
    #[case(board_from(&[(0, 0), (0, 1)], &[(0, 2), (1, 2)]), Position { row: 2, col: 0 }, MoveQuality::Bad)]
    fn move_quality_cases(
        #[case] before: Board,
        #[case] pos: Position,
        #[case] expected: MoveQuality,
    ) {
        assert_eq!(move_quality(&before, pos), expected);
    }

    // ── Exhaustive never-loses proof ──────────────────────────────────────

    /// Recursively explore every possible sequence of X moves from `game`.
    ///
    /// For each empty cell, clone the game, play that cell as X (engine responds
    /// automatically), assert Ok and never Win(X), then recurse if InProgress.
    /// `stats`: (terminal_games, draws).
    fn explore(game: &Game, stats: &mut (usize, usize)) {
        let board = game.board();
        for row in 0..3u8 {
            for col in 0..3u8 {
                if board[row as usize][col as usize].is_none() {
                    let mut g = game.clone();
                    let result = g.make_move(Position { row, col });
                    assert!(result.is_ok(), "make_move returned Err at ({row},{col})");
                    let status = g.status();
                    assert!(
                        status != GameStatus::Win(Player::X),
                        "Engine lost! board: {:?}",
                        g.board()
                    );
                    match status {
                        GameStatus::InProgress => explore(&g, stats),
                        GameStatus::Draw => {
                            stats.0 += 1;
                            stats.1 += 1;
                        }
                        _ => {
                            stats.0 += 1;
                        }
                    }
                }
            }
        }
    }

    /// Exhaustive proof: for every X move sequence the engine never loses.
    ///
    /// Enumerates up to 9×7×5×3 = 945 human move sequences (X plays first on
    /// each branch; engine responds optimally after every X move via make_move).
    /// Satisfies ROADMAP success criterion 2.
    #[test]
    fn engine_never_loses_exhaustive() {
        let game = Game::new();
        let mut stats = (0usize, 0usize);
        explore(&game, &mut stats);
        assert!(stats.0 > 0, "No terminal games reached — explore is broken");
        assert!(
            stats.1 > 0,
            "No draws reached — engine wins every game? stats={:?}",
            stats
        );
    }

    // ── Perfect-play draw (ENGI-04) ───────────────────────────────────────

    /// Minimax outcome for X: +1 Win(X), 0 Draw, -1 Win(O).
    ///
    /// Searches the game tree where make_move places X and engine responds.
    fn x_best_outcome(game: &Game) -> i32 {
        match game.status() {
            GameStatus::Win(Player::X) => return 1,
            GameStatus::Win(Player::O) => return -1,
            GameStatus::Draw => return 0,
            GameStatus::InProgress => {}
        }
        let board = game.board();
        let mut best = i32::MIN;
        for row in 0..3u8 {
            for col in 0..3u8 {
                if board[row as usize][col as usize].is_none() {
                    let mut g = game.clone();
                    g.make_move(Position { row, col }).expect("legal move");
                    let score = x_best_outcome(&g);
                    if score > best {
                        best = score;
                    }
                }
            }
        }
        best
    }

    /// Perfect X play (maximises minimax score) vs. optimal engine always draws.
    #[test]
    fn perfect_play_draws() {
        let mut game = Game::new();
        while game.status() == GameStatus::InProgress {
            let board = game.board();
            let mut best_score = i32::MIN;
            let mut best_move = None;
            for row in 0..3u8 {
                for col in 0..3u8 {
                    if board[row as usize][col as usize].is_none() {
                        let mut trial = game.clone();
                        trial.make_move(Position { row, col }).expect("legal move");
                        let score = x_best_outcome(&trial);
                        if score > best_score {
                            best_score = score;
                            best_move = Some(Position { row, col });
                        }
                    }
                }
            }
            match best_move {
                Some(pos) => {
                    game.make_move(pos).expect("legal move");
                }
                None => break,
            }
        }
        assert_eq!(
            game.status(),
            GameStatus::Draw,
            "Perfect X vs. optimal engine should draw"
        );
    }

    // ── Tactical rstest cases ─────────────────────────────────────────────

    /// Engine takes an immediate winning move (O completes row 0 at (0,2)).
    #[test]
    fn engine_takes_immediate_win() {
        // Board: X at (1,0),(1,1),(2,2), O at (0,0),(0,1)
        // O can win at (0,2) by completing the top row O O O.
        let board = board_from(&[(1, 0), (1, 1), (2, 2)], &[(0, 0), (0, 1)]);
        let result = best_engine_move(&board);
        assert!(result.is_some(), "Engine should have a winning move");
        assert_eq!(result, Some(Position { row: 0, col: 2 }));
    }

    /// Engine blocks X's immediate winning threat at (0,2).
    #[test]
    fn engine_blocks_immediate_threat() {
        // Board: X at (0,0),(0,1), O at (1,1)
        // X threatens row 0 at (0,2); engine must block.
        let board = board_from(&[(0, 0), (0, 1)], &[(1, 1)]);
        let result = best_engine_move(&board);
        assert!(result.is_some(), "Engine should block the threat");
        assert_eq!(result, Some(Position { row: 0, col: 2 }));
    }

    /// Engine answers any single corner with the center.
    #[rstest]
    #[case(Position { row: 0, col: 0 })]
    #[case(Position { row: 0, col: 2 })]
    #[case(Position { row: 2, col: 0 })]
    #[case(Position { row: 2, col: 2 })]
    fn engine_answers_corner_with_center(#[case] corner: Position) {
        let board = board_from(&[(corner.row, corner.col)], &[]);
        let result = best_engine_move(&board);
        assert!(result.is_some(), "Engine should have a move");
        assert_eq!(
            result,
            Some(Position { row: 1, col: 1 }),
            "Engine should answer corner {:?} with center",
            corner
        );
    }

    /// Engine answers a center play with one of the four corners.
    #[test]
    fn engine_answers_center_with_corner() {
        let board = board_from(&[(1, 1)], &[]);
        let result = best_engine_move(&board);
        assert!(result.is_some(), "Engine should have a move");
        let corners = [
            Position { row: 0, col: 0 },
            Position { row: 0, col: 2 },
            Position { row: 2, col: 0 },
            Position { row: 2, col: 2 },
        ];
        let is_corner = result.map(|p| corners.contains(&p)).unwrap_or(false);
        assert!(
            is_corner,
            "Engine should answer center with a corner, got {:?}",
            result
        );
    }

    /// Full board → no move available.
    #[test]
    fn best_engine_move_on_full_board_is_none() {
        // X O X / X X O / O X O (Draw board, 9 filled cells)
        let board = board_from(
            &[(0, 0), (0, 2), (1, 0), (1, 1), (2, 1)],
            &[(0, 1), (1, 2), (2, 0), (2, 2)],
        );
        assert!(best_engine_move(&board).is_none());
    }

    // ── Proptest invariants ───────────────────────────────────────────────

    proptest! {
        /// Random X play (choosing by index into empty cells) never violates
        /// game-state invariants and the engine never loses.
        #[test]
        fn random_play_preserves_invariants(
            choices in prop::collection::vec(0usize..9, 1..=9),
        ) {
            let mut game = Game::new();
            for choice in choices {
                if game.status() != GameStatus::InProgress {
                    break;
                }
                let board = game.board();
                let empty: Vec<Position> = (0..3u8)
                    .flat_map(|r| (0..3u8).map(move |c| Position { row: r, col: c }))
                    .filter(|p| board[p.row as usize][p.col as usize].is_none())
                    .collect();
                if empty.is_empty() {
                    break;
                }
                let pos = empty[choice % empty.len()];
                let result = game.make_move(pos);
                prop_assert!(result.is_ok(), "make_move failed: {:?}", result);

                let board = game.board();
                let x_count = board.iter().flat_map(|r| r.iter())
                    .filter(|c| **c == Some(Player::X)).count();
                let o_count = board.iter().flat_map(|r| r.iter())
                    .filter(|c| **c == Some(Player::O)).count();
                let some_count = x_count + o_count;

                prop_assert_eq!(
                    game.history().len(), some_count,
                    "history len != mark count"
                );
                prop_assert!(
                    x_count == o_count || x_count == o_count + 1,
                    "x={x_count} o={o_count}: diff should be 0 or 1"
                );
                prop_assert_eq!(
                    game.status(),
                    detect_status(board),
                    "status() disagrees with detect_status()"
                );
                prop_assert!(
                    game.status() != GameStatus::Win(Player::X),
                    "Engine lost! status={:?}", game.status()
                );
            }
        }

        /// Any out-of-range position is rejected and leaves game state unchanged.
        #[test]
        fn out_of_range_positions_rejected(
            row in any::<u8>(),
            col in any::<u8>(),
        ) {
            prop_assume!(row > 2 || col > 2);
            let initial = Game::new();
            let mut game = initial.clone();
            let result = game.make_move(Position { row, col });
            prop_assert_eq!(result, Err(MoveError::InvalidPosition));
            prop_assert_eq!(game, initial);
        }
    }

    // ── Retained basic tests ──────────────────────────────────────────────

    #[test]
    fn best_move_returns_none_on_full_board() {
        let board = [
            [Some(X), Some(O), Some(X)],
            [Some(X), Some(X), Some(O)],
            [Some(O), Some(X), Some(O)],
        ];
        assert!(best_engine_move(&board).is_none());
    }

    #[test]
    fn best_move_never_returns_occupied_position() {
        let board = [
            [Some(X), None, None],
            [None, Some(O), None],
            [None, None, None],
        ];
        if let Some(pos) = best_engine_move(&board) {
            assert!(
                board[pos.row as usize][pos.col as usize].is_none(),
                "Engine chose an occupied cell"
            );
        }
    }

    #[test]
    fn engine_takes_winning_move() {
        // O can win by placing at (0,2) — completing the top row: O O _
        let board = [
            [Some(O), Some(O), None],
            [Some(X), Some(X), None],
            [None, None, None],
        ];
        let result = best_engine_move(&board);
        assert!(result.is_some(), "Engine should have a move available");
        assert_eq!(result, Some(Position { row: 0, col: 2 }));
    }

    #[test]
    fn engine_blocks_x_winning_move() {
        // X threatens to win at (0,2); O must block there
        let board = [
            [Some(X), Some(X), None],
            [Some(O), None, None],
            [None, None, None],
        ];
        let result = best_engine_move(&board);
        assert!(result.is_some(), "Engine should have a move available");
        assert_eq!(result, Some(Position { row: 0, col: 2 }));
    }
}
