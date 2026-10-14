mod components;

use components::board::Board;
use dioxus::prelude::*;
use kiss_tictactoe_engine::Game;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let game = use_signal(Game::new);
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div { class: "game-container",
            h1 { class: "app-title", "Tic-Tac-Toe" }
            Board { game }
        }
    }
}
