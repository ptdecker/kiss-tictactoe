//! Win and draw detection for the 3×3 board.

use crate::types::{Board, GameStatus};

/// All eight winning lines (3 rows, 3 columns, 2 diagonals).
const WINNING_LINES: [[(usize, usize); 3]; 8] = [
    // Rows
    [(0, 0), (0, 1), (0, 2)],
    [(1, 0), (1, 1), (1, 2)],
    [(2, 0), (2, 1), (2, 2)],
    // Columns
    [(0, 0), (1, 0), (2, 0)],
    [(0, 1), (1, 1), (2, 1)],
    [(0, 2), (1, 2), (2, 2)],
    // Diagonals
    [(0, 0), (1, 1), (2, 2)],
    [(0, 2), (1, 1), (2, 0)],
];

/// Determine the current game status by inspecting all eight winning lines.
///
/// Returns [`GameStatus::Win`] if any line contains three identical marks.
/// Returns [`GameStatus::Draw`] if all cells are filled and no line is won.
/// Returns [`GameStatus::InProgress`] otherwise.
///
/// Win is checked before draw: a fully-filled board that contains a completed
/// line is reported as a win, not a draw.
pub fn detect_status(board: &Board) -> GameStatus {
    for line in &WINNING_LINES {
        let [a, b, c] = *line;
        match (board[a.0][a.1], board[b.0][b.1], board[c.0][c.1]) {
            (Some(p1), Some(p2), Some(p3)) if p1 == p2 && p2 == p3 => {
                return GameStatus::Win(p1);
            }
            _ => {}
        }
    }

    let full = board.iter().all(|row| row.iter().all(|cell| cell.is_some()));
    if full {
        GameStatus::Draw
    } else {
        GameStatus::InProgress
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Player;
    use crate::types::Player::{O, X};
    use rstest::rstest;

    /// Fill only the three cells of `WINNING_LINES[line_index]` with player `p`.
    ///
    /// All other cells remain empty.  The module-private `WINNING_LINES` const
    /// is visible here because this is a child module of `detection`.
    fn board_with_line(line_index: usize, p: Player) -> Board {
        let mut board = [[None; 3]; 3];
        for (r, c) in WINNING_LINES[line_index] {
            board[r][c] = Some(p);
        }
        board
    }

    /// All 8 winning lines × 2 players = 16 parameterised cases.
    #[rstest]
    fn each_line_wins_for_each_player(
        #[values(0, 1, 2, 3, 4, 5, 6, 7)] line: usize,
        #[values(Player::X, Player::O)] p: Player,
    ) {
        assert_eq!(detect_status(&board_with_line(line, p)), GameStatus::Win(p));
    }

    /// Named cases covering draw, win-on-full-board, in-progress variants.
    #[rstest]
    // Empty board -> InProgress
    #[case(
        [[None; 3]; 3],
        GameStatus::InProgress,
    )]
    // Full board, no winning line -> Draw  (X O X / X X O / O X O)
    #[case(
        [
            [Some(X), Some(O), Some(X)],
            [Some(X), Some(X), Some(O)],
            [Some(O), Some(X), Some(O)],
        ],
        GameStatus::Draw,
    )]
    // Full board that also contains an X diagonal -> Win(X) checked before Draw
    #[case(
        [
            [Some(X), Some(O), Some(X)],
            [Some(X), Some(X), Some(O)],
            [Some(O), Some(O), Some(X)],
        ],
        GameStatus::Win(X),
    )]
    // Partial board, no winning line -> InProgress
    #[case(
        [
            [None, Some(X), None],
            [None, Some(O), None],
            [None, None, None],
        ],
        GameStatus::InProgress,
    )]
    // Two marks plus a gap in a line (X X _) -> InProgress
    #[case(
        [
            [Some(X), Some(X), None],
            [None, None, None],
            [None, None, None],
        ],
        GameStatus::InProgress,
    )]
    fn detection_cases(#[case] board: Board, #[case] expected: GameStatus) {
        assert_eq!(detect_status(&board), expected);
    }
}
