use crate::state::Tally;
use dioxus::prelude::*;

/// Compact Wins / Draws / Losses row, player perspective (D-17).
#[component]
pub fn Scoreboard(wins: u32, draws: u32, losses: u32) -> Element {
    let tally = Tally {
        wins,
        draws,
        losses,
    };
    rsx! {
        div { class: "scoreboard",
            for (label, count) in tally.score_items() {
                div { key: "{label}", class: "score-item",
                    span { class: "score-count", "{count}" }
                    span { class: "score-label", "{label}" }
                }
            }
        }
    }
}
