use dioxus::prelude::*;
use kiss_tictactoe_engine::{Player, Position};

/// A single cell on the tic-tac-toe board.
#[component]
pub fn Cell(
    pos: Position,
    content: Option<Player>,
    interactive: bool,
    on_click: EventHandler<Position>,
) -> Element {
    rsx! {
        div {
            class: "cell",
            class: if content == Some(Player::X) { "cell--x" },
            class: if content == Some(Player::O) { "cell--o" },
            class: if !interactive { "cell--frozen" },
            onclick: move |_| {
                if interactive {
                    on_click.call(pos);
                }
            },
            {match content {
                Some(Player::X) => "X",
                Some(Player::O) => "O",
                None => "",
            }}
        }
    }
}
