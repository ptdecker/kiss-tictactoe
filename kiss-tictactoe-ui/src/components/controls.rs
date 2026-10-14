use dioxus::prelude::*;

/// Session controls: New Game (keeps scores) and a quiet Reset Scores (D-15, D-16).
#[component]
pub fn Controls(on_new_game: EventHandler<()>, on_reset_scores: EventHandler<()>) -> Element {
    rsx! {
        div { class: "controls",
            button {
                class: "btn btn--primary",
                r#type: "button",
                onclick: move |_| on_new_game.call(()),
                "New Game"
            }
            button {
                class: "btn btn--secondary",
                r#type: "button",
                onclick: move |_| on_reset_scores.call(()),
                "Reset Scores"
            }
        }
    }
}
