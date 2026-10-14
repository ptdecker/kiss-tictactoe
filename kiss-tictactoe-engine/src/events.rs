//! Classifies what happened in a turn into at most one [`TalkEvent`].
//!
//! Selection of the event is deterministic and pure (D-03); only the line
//! chosen within a category is random, handled by the `Commentator`.

use crate::types::{GameStatus, Player, TalkEvent};

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
}
