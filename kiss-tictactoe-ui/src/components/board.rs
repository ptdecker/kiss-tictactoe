use dioxus::prelude::*;
use kiss_tictactoe_engine::{Board as EngineBoard, Game, GameStatus, Player, Position, Turn};

use super::cell::Cell;
use super::game_over_overlay::GameOverOverlay;

/// Returns the 9 board cells in row-major order, each as
/// `(position, content, interactive)`.
///
/// A cell is interactive when it is empty **and** the game is still in
/// progress.
pub(crate) fn board_cells(
    board: &EngineBoard,
    status: GameStatus,
) -> Vec<(Position, Option<Player>, bool)> {
    let mut cells = Vec::with_capacity(9);
    for row in 0..3u8 {
        for col in 0..3u8 {
            let content = board[row as usize][col as usize];
            let interactive = content.is_none() && status == GameStatus::InProgress;
            cells.push((Position { row, col }, content, interactive));
        }
    }
    cells
}

/// The 3×3 game board component.
///
/// Reads board state from `game`, renders nine [`Cell`] components, and
/// forwards clicks to `game.write().play(pos)`. Each resulting [`Turn`] is
/// passed to `on_turn`; the overlay's Play again calls `on_new_game`.
#[component]
pub fn Board(
    game: Signal<Game>,
    on_turn: EventHandler<Turn>,
    on_new_game: EventHandler<()>,
) -> Element {
    // Clone values out before any write to avoid borrow conflicts (Pitfall 1)
    let board = *game.read().board();
    let status = game.read().status();
    let cells = board_cells(&board, status);

    rsx! {
        div { class: "board-wrapper",
            div { class: "board",
                for (pos, content, interactive) in cells {
                    Cell {
                        key: "{pos.row}-{pos.col}",
                        pos,
                        content,
                        interactive,
                        on_click: move |p: Position| {
                            let outcome = game.write().play(p);
                            debug_assert!(outcome.is_ok(), "unexpected move error: {outcome:?}");
                            if let Ok(turn) = outcome {
                                on_turn.call(turn);
                            }
                        },
                    }
                }
            }
            if status != GameStatus::InProgress {
                GameOverOverlay {
                    status,
                    on_play_again: move |_| on_new_game.call(()),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kiss_tictactoe_engine::Player::{O, X};

    #[test]
    fn board_cells_new_game_all_interactive() {
        let game = Game::new();
        let cells = board_cells(game.board(), game.status());
        assert_eq!(cells.len(), 9, "Should return 9 cells");
        for (i, (pos, content, interactive)) in cells.iter().enumerate() {
            let expected_row = (i / 3) as u8;
            let expected_col = (i % 3) as u8;
            assert_eq!(pos.row, expected_row, "Row mismatch at index {i}");
            assert_eq!(pos.col, expected_col, "Col mismatch at index {i}");
            assert!(content.is_none(), "Expected empty cell at {i}");
            assert!(*interactive, "Expected interactive cell at {i}");
        }
    }

    #[test]
    fn board_cells_after_move_freezes_occupied() {
        let mut game = Game::new();
        let _ = game.make_move(Position { row: 0, col: 0 });

        let cells = board_cells(game.board(), game.status());
        assert_eq!(cells.len(), 9);

        let non_interactive: Vec<_> = cells
            .iter()
            .filter(|(_, _, interactive)| !interactive)
            .collect();
        assert_eq!(non_interactive.len(), 2, "X and O cells should be frozen");

        let interactive_count = cells.iter().filter(|(_, _, i)| *i).count();
        assert_eq!(interactive_count, 7, "7 cells should remain interactive");

        // Verify occupied cells are non-interactive
        for (pos, content, interactive) in &cells {
            if content.is_some() {
                assert!(
                    !interactive,
                    "Occupied cell at ({},{}) should not be interactive",
                    pos.row,
                    pos.col
                );
            }
        }

        // Verify X cell is marked
        let x_cells: Vec<_> = cells
            .iter()
            .filter(|(_, c, _)| *c == Some(X))
            .collect();
        assert_eq!(x_cells.len(), 1, "Exactly one X cell");

        // Verify O cell is marked
        let o_cells: Vec<_> = cells
            .iter()
            .filter(|(_, c, _)| *c == Some(O))
            .collect();
        assert_eq!(o_cells.len(), 1, "Exactly one O cell");
    }

    #[test]
    fn board_cells_game_over_freezes_all() {
        let mut game = Game::new();

        // Play first-available-cell until game ends
        while game.status() == GameStatus::InProgress {
            for row in 0..3u8 {
                let mut moved = false;
                for col in 0..3u8 {
                    if game.board()[row as usize][col as usize].is_none() {
                        let _ = game.make_move(Position { row, col });
                        moved = true;
                        break;
                    }
                }
                if moved || game.status() != GameStatus::InProgress {
                    break;
                }
            }
        }

        assert_ne!(game.status(), GameStatus::InProgress);

        let cells = board_cells(game.board(), game.status());
        assert_eq!(cells.len(), 9);
        for (pos, _content, interactive) in &cells {
            assert!(
                !interactive,
                "All cells should be frozen after game over, but ({},{}) is interactive",
                pos.row,
                pos.col
            );
        }
    }
}
