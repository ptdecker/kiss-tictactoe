//! Classifies what happened in a turn into at most one [`TalkEvent`].
//!
//! Selection of the event is deterministic and pure (D-03); only the line
//! chosen within a category is random, handled by the `Commentator`.

use crate::ai::MoveQuality;
use crate::detection::WINNING_LINES;
use crate::types::{Board, GameStatus, Player, TalkEvent};

/// True iff some winning line holds exactly two marks of `player` and one
/// empty cell (a win is one move away).
pub(crate) fn has_threat(board: &Board, player: Player) -> bool {
    WINNING_LINES.iter().any(|line| {
        let cells = line.map(|(r, c)| board[r][c]);
        cells.iter().filter(|c| **c == Some(player)).count() == 2
            && cells.iter().filter(|c| c.is_none()).count() == 1
    })
}

/// Choose the single event for a turn by fixed priority (D-03, amended):
/// game over, then bad move, then engine threat, then user threat, then
/// good move. A bad move outranks the threat events because every losing
/// move against the perfect engine leaves it holding a threat, which would
/// otherwise hide the bad-move taunt entirely.
pub(crate) fn classify(
    status: GameStatus,
    engine_threat: bool,
    user_threat: bool,
    quality: MoveQuality,
) -> Option<TalkEvent> {
    if let Some(event) = lifecycle_event(status) {
        return Some(event);
    }
    if quality == MoveQuality::Bad {
        return Some(TalkEvent::BadMove);
    }
    if engine_threat {
        return Some(TalkEvent::EngineAboutToWin);
    }
    if user_threat {
        return Some(TalkEvent::UserAboutToWin);
    }
    Some(TalkEvent::GoodMove)
}

/// Map a final game status to its lifecycle event, if any.
pub(crate) fn lifecycle_event(status: GameStatus) -> Option<TalkEvent> {
    match status {
        GameStatus::Win(Player::O) => Some(TalkEvent::EngineWins),
        // Unreachable against perfect minimax until TRAIN-01 (D-01).
        GameStatus::Win(Player::X) => Some(TalkEvent::UserWins),
        GameStatus::Draw => Some(TalkEvent::Draw),
        GameStatus::InProgress => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(GameStatus::Win(Player::O), Some(TalkEvent::EngineWins))]
    #[case(GameStatus::Win(Player::X), Some(TalkEvent::UserWins))]
    #[case(GameStatus::Draw, Some(TalkEvent::Draw))]
    #[case(GameStatus::InProgress, None)]
    fn maps_status_to_event(#[case] status: GameStatus, #[case] expected: Option<TalkEvent>) {
        assert_eq!(lifecycle_event(status), expected);
    }

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

    #[rstest]
    fn threat_on_every_line_for_both_players(
        #[values(0, 1, 2, 3, 4, 5, 6, 7)] line: usize,
        #[values(Player::X, Player::O)] player: Player,
    ) {
        let mut board: Board = [[None; 3]; 3];
        let cells = WINNING_LINES[line];
        board[cells[0].0][cells[0].1] = Some(player);
        board[cells[1].0][cells[1].1] = Some(player);
        assert!(has_threat(&board, player));
    }

    #[test]
    fn no_threat_cases() {
        assert!(!has_threat(&board_from(&[], &[]), Player::X));
        // Third cell blocked by the opponent.
        assert!(!has_threat(
            &board_from(&[(0, 0), (0, 1)], &[(0, 2)]),
            Player::X
        ));
        // Two-mark line belongs to the other player.
        assert!(!has_threat(&board_from(&[(0, 0), (0, 1)], &[]), Player::O));
        // Completed line with no open third cell.
        assert!(!has_threat(
            &board_from(&[(0, 0), (0, 1), (0, 2)], &[]),
            Player::X
        ));
    }

    #[rstest]
    #[case(
        GameStatus::Win(Player::O),
        true,
        true,
        MoveQuality::Bad,
        Some(TalkEvent::EngineWins)
    )]
    #[case(
        GameStatus::Win(Player::X),
        true,
        true,
        MoveQuality::Bad,
        Some(TalkEvent::UserWins)
    )]
    #[case(GameStatus::Draw, true, true, MoveQuality::Bad, Some(TalkEvent::Draw))]
    #[case(
        GameStatus::InProgress,
        true,
        true,
        MoveQuality::Bad,
        Some(TalkEvent::BadMove)
    )]
    #[case(
        GameStatus::InProgress,
        false,
        true,
        MoveQuality::Bad,
        Some(TalkEvent::BadMove)
    )]
    #[case(
        GameStatus::InProgress,
        true,
        true,
        MoveQuality::Good,
        Some(TalkEvent::EngineAboutToWin)
    )]
    #[case(
        GameStatus::InProgress,
        false,
        true,
        MoveQuality::Good,
        Some(TalkEvent::UserAboutToWin)
    )]
    #[case(
        GameStatus::InProgress,
        false,
        false,
        MoveQuality::Bad,
        Some(TalkEvent::BadMove)
    )]
    #[case(
        GameStatus::InProgress,
        false,
        false,
        MoveQuality::Good,
        Some(TalkEvent::GoodMove)
    )]
    fn classify_priority(
        #[case] status: GameStatus,
        #[case] engine_threat: bool,
        #[case] user_threat: bool,
        #[case] quality: MoveQuality,
        #[case] expected: Option<TalkEvent>,
    ) {
        assert_eq!(
            classify(status, engine_threat, user_threat, quality),
            expected
        );
    }
}
