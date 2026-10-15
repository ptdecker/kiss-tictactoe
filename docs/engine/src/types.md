# Data types

This chapter says what each public type is for. Exact signatures live in rustdoc; each section links to it.

## Player

[`Player`](../../../target/doc/kiss_tictactoe_engine/enum.Player.html) is `X` or `O`. The human is always `X` and moves first; the engine is `O`.

## Position

[`Position`](../../../target/doc/kiss_tictactoe_engine/struct.Position.html) names a cell with a `row` and a `col`, each from 0 to 2. Row 0 is the top and column 0 is the left. Values above 2 are rejected with an error when you play them.

## Board

[`Board`](../../../target/doc/kiss_tictactoe_engine/type.Board.html) is a type alias, not a struct: a 3 by 3 array of `Option<Player>`, indexed `board[row][col]`. `None` is an empty cell.

## GameStatus

[`GameStatus`](../../../target/doc/kiss_tictactoe_engine/enum.GameStatus.html) is `InProgress`, `Win(Player)` or `Draw`. The free function [`detect_status`](../../../target/doc/kiss_tictactoe_engine/fn.detect_status.html) computes it from any board.

## MoveResult

[`MoveResult`](../../../target/doc/kiss_tictactoe_engine/enum.MoveResult.html) is what a successful move reports: `Continuing`, or `GameOver(GameStatus)` carrying the final status.

## MoveError

[`MoveError`](../../../target/doc/kiss_tictactoe_engine/enum.MoveError.html) is why a move was rejected: `CellOccupied`, `InvalidPosition` or `GameAlreadyOver`. It implements `Display` and `std::error::Error`. See [The Game API](game-api.md).

## Turn

[`Turn`](../../../target/doc/kiss_tictactoe_engine/struct.Turn.html) is what `Game::play` returns: a `result` (a `MoveResult`) and an `event` (an optional `TalkEvent`).

## TalkEvent

[`TalkEvent`](../../../target/doc/kiss_tictactoe_engine/enum.TalkEvent.html) names one of eight trash-talk categories. `TalkEvent::ALL` lists them in declaration order and `TalkEvent::COUNT` is 8. Details are in [The event system](events.md).

## Samples

A move places `X` at the centre and the engine answers in the corner. The board and history reflect both:

```rust
# extern crate kiss_tictactoe_engine;
use kiss_tictactoe_engine::{Game, Player, Position};

let mut game = Game::new();
game.play(Position { row: 1, col: 1 }).unwrap();
assert_eq!(game.board()[1][1], Some(Player::X));
assert_eq!(game.board()[0][0], Some(Player::O));
assert_eq!(
    game.history(),
    &[Position { row: 1, col: 1 }, Position { row: 0, col: 0 }]
);
```

`detect_status` works on a hand-built board:

```rust
# extern crate kiss_tictactoe_engine;
use kiss_tictactoe_engine::{detect_status, Board, GameStatus, Player};

let empty: Board = [[None; 3]; 3];
assert_eq!(detect_status(&empty), GameStatus::InProgress);

let mut board: Board = [[None; 3]; 3];
board[0] = [Some(Player::X); 3];
assert_eq!(detect_status(&board), GameStatus::Win(Player::X));
```

The event list has a fixed size:

```rust
# extern crate kiss_tictactoe_engine;
use kiss_tictactoe_engine::TalkEvent;

assert_eq!(TalkEvent::COUNT, 8);
assert_eq!(TalkEvent::ALL.len(), TalkEvent::COUNT);
```
