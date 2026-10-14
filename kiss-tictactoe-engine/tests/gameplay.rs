use kiss_tictactoe_engine::{Game, GameStatus, MoveError, MoveResult, Player, Position};

fn first_empty(game: &Game) -> Option<Position> {
    for row in 0..3u8 {
        for col in 0..3u8 {
            if game.board()[row as usize][col as usize].is_none() {
                return Some(Position { row, col });
            }
        }
    }
    None
}

#[test]
fn new_game_starts_empty_in_progress_with_x_to_move() {
    let game = Game::new();
    for row in game.board().iter() {
        for cell in row.iter() {
            assert!(cell.is_none(), "Expected all cells to be empty");
        }
    }
    assert_eq!(game.status(), GameStatus::InProgress);
    assert!(game.history().is_empty());
    assert_eq!(game.current_player(), Player::X);
    assert_eq!(Game::default(), Game::new());
}

#[test]
fn player_move_places_x_and_engine_replies_with_o() {
    let mut game = Game::new();
    let pos = Position { row: 1, col: 1 };
    let result = game.make_move(pos);
    assert_eq!(result, Ok(MoveResult::Continuing));
    assert_eq!(game.board()[1][1], Some(Player::X));
    let o_count = game
        .board()
        .iter()
        .flat_map(|row| row.iter())
        .filter(|cell| **cell == Some(Player::O))
        .count();
    assert_eq!(o_count, 1, "Engine should have placed exactly one O");
    assert_eq!(game.history().len(), 2);
    assert_eq!(game.history()[0], Position { row: 1, col: 1 });
    assert_eq!(game.current_player(), Player::X);
}

#[test]
fn occupied_cell_is_rejected_without_state_change() {
    let mut game = Game::new();
    let pos = Position { row: 0, col: 0 };
    let _ = game.make_move(pos);

    let game_snapshot = game.clone();

    // Try to move on the X cell
    let x_pos = game.history()[0];
    let err = game.make_move(x_pos);
    assert_eq!(err, Err(MoveError::CellOccupied));
    assert_eq!(game, game_snapshot);

    // Try to move on the O cell
    let o_pos = game.history()[1];
    let err = game.make_move(o_pos);
    assert_eq!(err, Err(MoveError::CellOccupied));
    assert_eq!(game, game_snapshot);
}

#[test]
fn out_of_bounds_position_is_rejected() {
    let mut game = Game::new();
    let initial = Game::new();

    let err = game.make_move(Position { row: 3, col: 0 });
    assert_eq!(err, Err(MoveError::InvalidPosition));
    assert_eq!(game, initial);

    let err = game.make_move(Position { row: 0, col: 3 });
    assert_eq!(err, Err(MoveError::InvalidPosition));
    assert_eq!(game, initial);
}

#[test]
fn full_game_ends_and_rejects_further_moves() {
    let mut game = Game::new();

    // Play moves until the game ends
    while game.status() == GameStatus::InProgress {
        if let Some(pos) = first_empty(&game) {
            let _ = game.make_move(pos);
        } else {
            break;
        }
    }

    // Final status should be Win(O) or Draw — never Win(X) since engine plays optimally
    let final_status = game.status();
    assert!(
        matches!(final_status, GameStatus::Win(Player::O) | GameStatus::Draw),
        "Expected Win(O) or Draw, got {:?}",
        final_status
    );

    // Further moves should be rejected
    for row in 0..3u8 {
        for col in 0..3u8 {
            let err = game.make_move(Position { row, col });
            assert_eq!(err, Err(MoveError::GameAlreadyOver));
        }
    }
}
