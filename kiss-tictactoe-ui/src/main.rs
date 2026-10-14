mod components;
mod state;

use components::board::Board;
use components::speech_bubble::SpeechBubble;
use dioxus::prelude::*;
use kiss_tictactoe_engine::{Game, TalkEvent, Turn};
use state::opening;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

/// Seeds the commentator from the system clock.
///
/// The engine has no platform dependencies, so the UI supplies the seed (D-07;
/// a zero seed is handled inside `Commentator::new`).
fn clock_seed() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0x5EED)
}

#[component]
fn App() -> Element {
    let mut game = use_signal(Game::new);
    // Greeting is built once so it is present on first paint without writing
    // a signal during render.
    let init = use_hook(|| opening(clock_seed()));
    let mut commentator = use_signal(|| init.0.clone());
    let mut bubble = use_signal(|| init.1);

    let start_new_game = move |_: ()| {
        game.set(Game::new());
        let line = commentator.write().line(TalkEvent::GameStart);
        let next = bubble().greeting(line);
        bubble.set(next);
    };

    let handle_turn = move |turn: Turn| {
        let line = turn.event.map(|ev| commentator.write().line(ev));
        let next = bubble().after_turn(turn.result, line);
        bubble.set(next);
    };

    let b = bubble();

    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div { class: "game-container",
            h1 { class: "app-title", "Tic-Tac-Toe" }
            SpeechBubble {
                message: b.message.map(str::to_owned),
                serial: b.serial,
                is_final: b.is_final,
            }
            Board { game, on_turn: handle_turn, on_new_game: start_new_game }
        }
    }
}
