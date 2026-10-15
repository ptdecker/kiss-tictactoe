# Minimax and the correctness proof

The engine plays O with alpha-beta minimax. It searches the whole game tree from the current position: X is the maximiser, O the minimiser, and each side assumes the other plays its best reply. Alpha-beta pruning skips branches that cannot change the result. On a 3x3 board the tree is tiny, so the answer is instant.

## Depth-adjusted scores

A finished position scores 10 minus the depth for an X win, the depth minus 10 for an O win, and 0 for a draw. Subtracting depth makes the engine prefer a quick win and, when it is already doomed, a slow loss. A plain win/lose score could not tell those apart.

## Reused for move quality

The same search judges your moves for `GoodMove` and `BadMove`: it compares the outcome class of the board before your move (win, draw, loss) with what your move leads to. See the [event system](events.md) chapter.

The search functions are crate-private. You reach them only through [`Game`](../../../target/doc/kiss_tictactoe_engine/struct.Game.html), which replies for the engine after each `play`.

## Why the engine never loses

Two tests in the engine's `ai.rs` back this up, and both run in `cargo test`:

- `engine_never_loses_exhaustive` explores every sequence of X moves. For each empty cell it clones the game, plays that cell through `make_move` (the engine replies automatically), asserts the status is never a win for X, and recurses while the game is in progress. It also asserts that some terminal games were reached and that at least one was a draw, so the check cannot pass vacuously.
- `perfect_play_draws` plays X perfectly, choosing each move by searching X's best outcome against the engine, and asserts the game ends in a draw.

`Game` is `Clone`, so you can re-run the exhaustive proof yourself through the public API:

```rust
# extern crate kiss_tictactoe_engine;
use kiss_tictactoe_engine::{Game, GameStatus, Player, Position};

fn explore(game: &Game, stats: &mut (usize, usize)) {
    if game.status() != GameStatus::InProgress {
        return;
    }
    let board = game.board();
    for row in 0..3u8 {
        for col in 0..3u8 {
            if board[row as usize][col as usize].is_some() {
                continue;
            }
            let mut next = game.clone();
            next.play(Position { row, col }).expect("legal move");
            assert_ne!(next.status(), GameStatus::Win(Player::X));
            match next.status() {
                GameStatus::InProgress => explore(&next, stats),
                GameStatus::Draw => {
                    stats.0 += 1;
                    stats.1 += 1;
                }
                _ => stats.0 += 1,
            }
        }
    }
}

let mut stats = (0, 0);
explore(&Game::new(), &mut stats);
assert!(stats.0 > 0, "no finished games");
assert!(stats.1 > 0, "no draws");
```

The proof covers every move you could make, so the best you can do is draw.
