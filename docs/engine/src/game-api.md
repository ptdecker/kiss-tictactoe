# The Game API

[`Game`](../../../target/doc/kiss_tictactoe_engine/struct.Game.html) is the single entry point.

## Lifecycle

`Game::new()` starts a fresh game with `X` (the human) to move. Each successful call places your `X`
and then the engine's `O` reply, both inside the same call, until a move ends the game with
`MoveResult::GameOver`.

There are two ways to move:

- `make_move(Position)` returns `Result<MoveResult, MoveError>`.
- `play(Position)` does the same work but returns `Result<Turn, MoveError>`, whose `event` says
  which trash-talk category applies. Front ends that want commentary call `play`.

After a move you can inspect the game with `status()`, `board()`, `history()` (every move so far,
alternating `X` then `O`) and `current_player()`.

## Errors

Moves are checked in a fixed order: `GameAlreadyOver` first, then `InvalidPosition`, then
`CellOccupied`. A rejected move changes nothing.

Playing the same cell twice is rejected, and the history is untouched:

```rust
extern crate kiss_tictactoe_engine;
fn main() {
    use kiss_tictactoe_engine::{Game, MoveError, Position};

    let mut game = Game::new();
    let center = Position { row: 1, col: 1 };
    game.play(center).unwrap();
    assert_eq!(game.play(center), Err(MoveError::CellOccupied));
    assert_eq!(game.history().len(), 2);
}
```

A position off the grid is rejected:

```rust
extern crate kiss_tictactoe_engine;
fn main() {
    use kiss_tictactoe_engine::{Game, MoveError, Position};

    let mut game = Game::new();
    assert_eq!(
        game.play(Position { row: 3, col: 0 }).unwrap_err(),
        MoveError::InvalidPosition
    );
}
```

Once the game is over, every further move is rejected. This sequence loses to the engine:

```rust
extern crate kiss_tictactoe_engine;
fn main() {
    use kiss_tictactoe_engine::{Game, GameStatus, MoveError, Player, Position};

    let mut game = Game::new();
    for (row, col) in [(1, 1), (2, 2), (1, 0)] {
        game.play(Position { row, col }).unwrap();
    }
    assert_eq!(game.status(), GameStatus::Win(Player::O));
    assert_eq!(
        game.play(Position { row: 0, col: 1 }),
        Err(MoveError::GameAlreadyOver)
    );
}
```

`MoveError` implements `Display`, so it can go straight into a message:

```rust
extern crate kiss_tictactoe_engine;
fn main() {
    use kiss_tictactoe_engine::MoveError;

    assert!(!MoveError::CellOccupied.to_string().is_empty());
}
```
