# The event system

`Game::play` returns a [`Turn`](../../../target/doc/kiss_tictactoe_engine/struct.Turn.html) 
whose `event` names at most one trash-talk category, a
[`TalkEvent`](../../../target/doc/kiss_tictactoe_engine/enum.TalkEvent.html). The engine decides the
category; the front end turns it into words with a `Commentator`.

## The eight categories

| Event              | When it fires                                                                                                                                                    |
|--------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `GameStart`        | Never returned by `Game::play`. The front end asks for a `GameStart` line itself when it creates a game (the desktop app does so when it opens and on New Game). |
| `EngineWins`       | The final turn, when the engine completes a line.                                                                                                                |
| `UserWins`         | The final turn, when you complete a line. Reserved for the planned training mode; it cannot happen against the perfect engine today.                             |
| `Draw`             | The final turn, when the board fills with no winner.                                                                                                             |
| `EngineAboutToWin` | After the engine's reply it holds two in a row with the third cell open. Only checked if the game continues.                                                     |
| `UserAboutToWin`   | You hold two in a row with the third cell open. Detected after your move and before the engine blocks, so it is reported on the same turn the engine blocks.     |
| `GoodMove`         | Your move was judged fine.                                                                                                                                       |
| `BadMove`          | Your move was judged a mistake.                                                                                                                                  |

Moves are judged by minimax against the board before the move, by outcome class (win, draw, loss).
Within a losing class, a faster loss than the best available is bad.

## Priority

When several categories apply, one wins. The order is: game over, then bad move, then engine about
to win, then user about to win, then good move. One event per turn.

Bad move outranks the threat events because every losing move against the perfect engine hands it a
threat. Without this rule the threat event would always hide the taunt about your mistake.

Every in-progress turn yields exactly one event, so `play` returns `Some(..)` in practice. The field
is an `Option` because the type allows none.

## Scenarios

A sensible opening is a good move:

```rust
extern crate kiss_tictactoe_engine;
fn main() {
    use kiss_tictactoe_engine::{Game, MoveResult, Position, TalkEvent, Turn};

    let mut game = Game::new();
    let turn = game.play(Position { row: 1, col: 1 }).unwrap();
    assert_eq!(
        turn,
        Turn { result: MoveResult::Continuing, event: Some(TalkEvent::GoodMove) }
    );
}
```

Threatening a line is reported on the turn the engine blocks it:

```rust
extern crate kiss_tictactoe_engine;
fn main() {
    use kiss_tictactoe_engine::{Game, Player, Position, TalkEvent};

    let mut game = Game::new();
    game.play(Position { row: 1, col: 1 }).unwrap();
    let turn = game.play(Position { row: 0, col: 1 }).unwrap();
    assert_eq!(turn.event, Some(TalkEvent::UserAboutToWin));
    assert_eq!(game.board()[2][1], Some(Player::O));
}
```

When the engine builds its own threat:

```rust
extern crate kiss_tictactoe_engine;
fn main() {
    use kiss_tictactoe_engine::{Game, Position, TalkEvent};

    let mut game = Game::new();
    game.play(Position { row: 1, col: 1 }).unwrap();
    let turn = game.play(Position { row: 2, col: 2 }).unwrap();
    assert_eq!(turn.event, Some(TalkEvent::EngineAboutToWin));
}
```

A mistake gets a taunt, even though the engine now holds a threat:

```rust
extern crate kiss_tictactoe_engine;
fn main() {
    use kiss_tictactoe_engine::{Game, Position, TalkEvent};

    let mut game = Game::new();
    game.play(Position { row: 0, col: 1 }).unwrap();
    let turn = game.play(Position { row: 0, col: 2 }).unwrap();
    assert_eq!(turn.event, Some(TalkEvent::BadMove));
}
```

And when the engine wins, the game-over event takes over:

```rust
extern crate kiss_tictactoe_engine;
fn main() {
    use kiss_tictactoe_engine::{
        Game, GameStatus, MoveResult, Player, Position, TalkEvent, Turn,
    };

    let mut game = Game::new();
    game.play(Position { row: 1, col: 1 }).unwrap();
    game.play(Position { row: 2, col: 2 }).unwrap();
    let turn = game.play(Position { row: 1, col: 0 }).unwrap();
    assert_eq!(
        turn,
        Turn {
            result: MoveResult::GameOver(GameStatus::Win(Player::O)),
            event: Some(TalkEvent::EngineWins),
        }
    );
}
```

A `Draw` turn works the same way: the last move fills the board and the game is a `Draw`.
