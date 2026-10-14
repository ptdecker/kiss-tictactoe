mod components;
mod state;

use components::board::Board;
use components::controls::Controls;
use components::scoreboard::Scoreboard;
use components::speech_bubble::SpeechBubble;
use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use dioxus::prelude::*;
use kiss_tictactoe_engine::{Game, MoveResult, TalkEvent, Turn};
use state::{Tally, opening};

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    // The default window is too short to show the scoreboard and controls below
    // the board, so size it to fit the whole column on first launch.
    let window = WindowBuilder::new()
        .with_title("kiss-tictactoe")
        .with_inner_size(LogicalSize::new(520.0, 840.0))
        .with_min_inner_size(LogicalSize::new(320.0, 560.0));
    LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(window))
        .launch(App);
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
    let mut tally = use_signal(Tally::default);

    let start_new_game = move |_: ()| {
        game.set(Game::new());
        let line = commentator.write().line(TalkEvent::GameStart);
        let next = bubble().greeting(line);
        bubble.set(next);
    };

    let handle_turn = move |turn: Turn| {
        // Single increment point: the engine cannot report GameOver twice.
        if let MoveResult::GameOver(status) = turn.result {
            tally.write().record(status);
        }
        let line = turn.event.map(|ev| commentator.write().line(ev));
        let next = bubble().after_turn(turn.result, line);
        bubble.set(next);
    };

    let b = bubble();
    let t = tally();

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
            Scoreboard { wins: t.wins, draws: t.draws, losses: t.losses }
            Controls {
                on_new_game: start_new_game,
                on_reset_scores: move |_| tally.set(Tally::default()),
            }
        }
    }
}
