use dioxus::prelude::*;
use kiss_tictactoe_engine::GameStatus;

/// Returns the outcome text to display for a given game status.
///
/// Returns `None` when the game is still in progress so the caller can
/// render nothing rather than panic on an unexpected variant.
pub fn outcome_text(status: GameStatus) -> Option<&'static str> {
    use kiss_tictactoe_engine::Player;
    match status {
        GameStatus::Win(Player::X) => Some("You win!"),
        GameStatus::Win(Player::O) => Some("Engine wins!"),
        GameStatus::Draw => Some("Draw!"),
        GameStatus::InProgress => None,
    }
}

/// Overlay component shown when the game ends.
///
/// Renders an empty element when `status` is [`GameStatus::InProgress`].
/// Otherwise renders a semi-transparent overlay on top of the frozen board
/// with the outcome text and a "Play again" button.
#[component]
pub fn GameOverOverlay(status: GameStatus, on_play_again: EventHandler<()>) -> Element {
    match outcome_text(status) {
        Some(text) => rsx! {
            div { class: "game-over-overlay",
                span { class: "outcome-text", {text} }
                button {
                    class: "play-again-btn",
                    onclick: move |_| on_play_again.call(()),
                    "Play again"
                }
            }
        },
        None => rsx! {},
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kiss_tictactoe_engine::{GameStatus, Player};

    #[test]
    fn outcome_text_player_win() {
        assert_eq!(outcome_text(GameStatus::Win(Player::X)), Some("You win!"));
    }

    #[test]
    fn outcome_text_engine_win() {
        assert_eq!(
            outcome_text(GameStatus::Win(Player::O)),
            Some("Engine wins!")
        );
    }

    #[test]
    fn outcome_text_draw() {
        assert_eq!(outcome_text(GameStatus::Draw), Some("Draw!"));
    }

    #[test]
    fn outcome_text_in_progress_is_none() {
        assert_eq!(outcome_text(GameStatus::InProgress), None);
    }
}
