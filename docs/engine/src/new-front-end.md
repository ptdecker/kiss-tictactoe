# Writing a new front end

The engine is a plain library, so any interface can drive it. This chapter lists who owns what and walks through a complete command-line front end.

## The contract

The engine owns the rules, the opponent, the game status, and which event a turn produced.

A front end owns:

- input and rendering;
- the random seed for the `Commentator`;
- asking for a `GameStart` line when a game begins (`Game::play` never returns that event);
- turning events into words with a `Commentator`;
- presentation timing;
- any score keeping.

In the desktop app those responsibilities live in:

```text
input and rendering   Board component (kiss-tictactoe-ui)
speech text           SpeechBubble component
score keeping         Tally in state.rs
random seed           clock_seed in main.rs
```

## A worked CLI example

The array of moves stands in for user input so the sample can run in tests. It plays the sequence (1,1), (2,2), (1,0). The later moves are mistakes, so the engine wins. A move the engine rejects prints its error and the loop carries on.

```rust
# extern crate kiss_tictactoe_engine;
use kiss_tictactoe_engine::{
    Commentator, Game, GameStatus, MoveResult, Player, Position, TalkEvent,
};

fn render(game: &Game) {
    for row in game.board() {
        let line: String = row
            .iter()
            .map(|cell| match cell {
                Some(Player::X) => 'X',
                Some(Player::O) => 'O',
                None => '.',
            })
            .collect();
        println!("{line}");
    }
}

let mut game = Game::new();
let mut commentator = Commentator::new(7);
println!("{}", commentator.line(TalkEvent::GameStart));

let moves = [(1, 1), (2, 2), (1, 0)];
for (row, col) in moves {
    match game.play(Position { row, col }) {
        Err(error) => {
            println!("{error}");
            continue;
        }
        Ok(turn) => {
            render(&game);
            if let Some(event) = turn.event {
                println!("{}", commentator.line(event));
            }
            if let MoveResult::GameOver(status) = turn.result {
                println!("Game over: {status:?}");
                break;
            }
        }
    }
}
assert_eq!(game.status(), GameStatus::Win(Player::O));
```

## Reading from stdin

The same loop reading real input. It is marked `no_run` because it blocks waiting for stdin; `mdbook test` still compiles it, so it cannot drift from the API.

```rust,no_run
# extern crate kiss_tictactoe_engine;
use kiss_tictactoe_engine::{Commentator, Game, MoveResult, Position, TalkEvent};

let mut game = Game::new();
let mut commentator = Commentator::new(7);
println!("{}", commentator.line(TalkEvent::GameStart));

for input in std::io::stdin().lines() {
    let Ok(input) = input else { break };
    let parts: Vec<&str> = input.split_whitespace().collect();
    let [row, col] = parts[..] else {
        println!("Enter a move as: row col");
        continue;
    };
    let (Ok(row), Ok(col)) = (row.parse::<u8>(), col.parse::<u8>()) else {
        println!("Row and column must be numbers from 0 to 2");
        continue;
    };
    match game.play(Position { row, col }) {
        Err(error) => println!("{error}"),
        Ok(turn) => {
            if let Some(event) = turn.event {
                println!("{}", commentator.line(event));
            }
            if let MoveResult::GameOver(status) = turn.result {
                println!("Game over: {status:?}");
                break;
            }
        }
    }
}
```

Bad input never panics: parse failures are reported, and a `MoveError` (occupied cell, off the board, game over) is shown through its `Display` message.

## Other front ends

- A web or mobile Dioxus front end reuses the engine unchanged.
- A test harness can use a fixed seed for deterministic lines.
- `Turn` derives `PartialEq`, so a whole turn is easy to assert on.

See [The Game API](game-api.md) for the calls used here and [The event system](events.md) for what each event means.
