//! Pure, Dioxus-free UI state.
//!
//! Kept free of Dioxus types so the transitions can be unit-tested without a
//! runtime.

use kiss_tictactoe_engine::{Commentator, MoveResult, TalkEvent};

/// What the speech bubble currently shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BubbleState {
    /// The current engine line, if any.
    pub message: Option<&'static str>,
    /// Bumps on every new message; drives the animation restart key.
    pub serial: u32,
    /// A game-over message that must not fade (D-11).
    pub is_final: bool,
}

impl BubbleState {
    /// Shows a game-start greeting (never final).
    pub fn greeting(self, line: &'static str) -> Self {
        Self {
            message: Some(line),
            serial: self.serial.wrapping_add(1),
            is_final: false,
        }
    }

    /// Applies the outcome of a turn. With no line the state is unchanged.
    pub fn after_turn(self, result: MoveResult, line: Option<&'static str>) -> Self {
        match line {
            None => self,
            Some(line) => Self {
                message: Some(line),
                serial: self.serial.wrapping_add(1),
                is_final: matches!(result, MoveResult::GameOver(_)),
            },
        }
    }
}

/// Creates the commentator and the initial greeting bubble for app launch.
pub fn opening(seed: u64) -> (Commentator, BubbleState) {
    let mut commentator = Commentator::new(seed);
    let line = commentator.line(TalkEvent::GameStart);
    (commentator, BubbleState::default().greeting(line))
}

#[cfg(test)]
mod tests {
    use super::*;
    use kiss_tictactoe_engine::GameStatus;

    #[test]
    fn default_is_empty() {
        let s = BubbleState::default();
        assert_eq!(s.message, None);
        assert_eq!(s.serial, 0);
        assert!(!s.is_final);
    }

    #[test]
    fn greeting_sets_message_and_clears_final() {
        let s = BubbleState::default().greeting("hi");
        assert_eq!(s.message, Some("hi"));
        assert_eq!(s.serial, 1);
        assert!(!s.is_final);
        let fin = s.after_turn(MoveResult::GameOver(GameStatus::Draw), Some("x"));
        assert!(fin.is_final);
        let again = fin.greeting("yo");
        assert!(!again.is_final);
        assert_eq!(again.serial, 3);
    }

    #[test]
    fn after_turn_with_line_continuing() {
        let s = BubbleState::default().after_turn(MoveResult::Continuing, Some("x"));
        assert_eq!(s.message, Some("x"));
        assert_eq!(s.serial, 1);
        assert!(!s.is_final);
    }

    #[test]
    fn after_turn_with_line_game_over() {
        let s = BubbleState::default()
            .after_turn(MoveResult::GameOver(GameStatus::Draw), Some("x"));
        assert!(s.is_final);
        assert_eq!(s.serial, 1);
    }

    #[test]
    fn after_turn_without_line_is_unchanged() {
        let s = BubbleState::default().greeting("hi");
        assert_eq!(s.after_turn(MoveResult::Continuing, None), s);
    }

    #[test]
    fn opening_has_greeting_and_is_deterministic() {
        let (_, a) = opening(42);
        let (_, b) = opening(42);
        assert!(a.message.is_some_and(|m| !m.is_empty()));
        assert_eq!(a.serial, 1);
        assert!(!a.is_final);
        assert_eq!(a, b);
    }

    #[test]
    fn serial_wraps_at_max() {
        let s = BubbleState {
            message: None,
            serial: u32::MAX,
            is_final: false,
        };
        assert_eq!(s.greeting("x").serial, 0);
    }
}
