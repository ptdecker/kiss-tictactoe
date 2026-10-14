use kiss_tictactoe_engine::{
    Commentator, Game, GameStatus, MoveError, MoveResult, Player, Position, TalkEvent, Turn,
};

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

#[test]
fn play_matches_make_move() {
    let mut a = Game::new();
    let mut b = Game::new();
    while a.status() == GameStatus::InProgress {
        let pos = first_empty(&a).expect("empty cell while in progress");
        let r = a.make_move(pos).expect("legal move");
        let turn: Turn = b.play(pos).expect("legal move");
        assert_eq!(r, turn.result, "make_move and play must agree");
        assert_eq!(a, b, "game state must match after each turn");
    }
}

#[test]
fn full_game_ends_with_lifecycle_event() {
    let mut game = Game::new();
    loop {
        let pos = first_empty(&game).expect("empty cell while in progress");
        let turn = game.play(pos).expect("legal move");
        if let MoveResult::GameOver(s) = turn.result {
            let expected = match s {
                GameStatus::Win(Player::O) => TalkEvent::EngineWins,
                GameStatus::Win(Player::X) => TalkEvent::UserWins,
                GameStatus::Draw => TalkEvent::Draw,
                GameStatus::InProgress => panic!("GameOver(InProgress)"),
            };
            assert_eq!(turn.event, Some(expected));
            break;
        }
    }
}

#[test]
fn non_final_turns_never_carry_lifecycle_events() {
    let mut game = Game::new();
    while game.status() == GameStatus::InProgress {
        let pos = first_empty(&game).expect("empty cell while in progress");
        let turn = game.play(pos).expect("legal move");
        if turn.result == MoveResult::Continuing {
            assert!(
                !matches!(
                    turn.event,
                    Some(TalkEvent::GameStart)
                        | Some(TalkEvent::EngineWins)
                        | Some(TalkEvent::UserWins)
                        | Some(TalkEvent::Draw)
                ),
                "lifecycle event on a non-final turn: {:?}",
                turn.event
            );
        }
    }
}

#[test]
fn play_after_game_over_is_rejected() {
    let mut game = Game::new();
    while game.status() == GameStatus::InProgress {
        let pos = first_empty(&game).expect("empty cell while in progress");
        game.play(pos).expect("legal move");
    }
    for row in 0..3u8 {
        for col in 0..3u8 {
            assert_eq!(
                game.play(Position { row, col }),
                Err(MoveError::GameAlreadyOver)
            );
        }
    }
}

#[test]
fn commentator_lines_come_from_banks() {
    for ev in TalkEvent::ALL {
        let line = Commentator::new(42).line(ev);
        assert!(!line.trim().is_empty(), "empty line for {ev:?}");
        assert!(
            line.chars().count() <= 56,
            "line too long for {ev:?}: {line}"
        );
    }
}

#[test]
fn commentator_same_seed_same_sequence() {
    let mut a = Commentator::new(7);
    let mut b = Commentator::new(7);
    for i in 0..40 {
        let ev = TalkEvent::ALL[i % TalkEvent::COUNT];
        assert_eq!(a.line(ev), b.line(ev), "sequences diverged at call {i}");
    }
}

fn pos(row: u8, col: u8) -> Position {
    Position { row, col }
}

#[test]
fn user_threat_reported_before_engine_blocks() {
    let mut game = Game::new();
    game.play(pos(1, 1)).expect("legal move");
    assert_eq!(
        game.board()[0][0],
        Some(Player::O),
        "engine should reply in the corner (0,0)"
    );
    let turn = game.play(pos(0, 1)).expect("legal move");
    assert_eq!(
        turn,
        Turn {
            result: MoveResult::Continuing,
            event: Some(TalkEvent::UserAboutToWin)
        }
    );
    assert_eq!(
        game.board()[2][1],
        Some(Player::O),
        "engine must block at (2,1)"
    );
}

#[test]
fn engine_threat_reported_after_reply() {
    let mut game = Game::new();
    game.play(pos(1, 1)).expect("legal move");
    assert_eq!(
        game.board()[0][0],
        Some(Player::O),
        "engine should reply in the corner (0,0)"
    );
    let turn = game.play(pos(2, 2)).expect("legal move");
    assert_eq!(turn.result, MoveResult::Continuing);
    assert_eq!(turn.event, Some(TalkEvent::EngineAboutToWin));
    assert_eq!(
        game.board()[0][2],
        Some(Player::O),
        "engine should reply at (0,2)"
    );
}

#[test]
fn first_move_is_good() {
    let mut game = Game::new();
    let turn = game.play(pos(1, 1)).expect("legal move");
    assert_eq!(turn.event, Some(TalkEvent::GoodMove));
}

#[test]
fn blunder_into_loss_reports_engine_wins() {
    let mut game = Game::new();
    game.play(pos(1, 1)).expect("legal move");
    game.play(pos(2, 2)).expect("legal move");
    assert_eq!(
        game.board()[0][2],
        Some(Player::O),
        "engine should hold (0,0) and (0,2)"
    );
    let turn = game.play(pos(1, 0)).expect("legal move");
    assert_eq!(
        turn.result,
        MoveResult::GameOver(GameStatus::Win(Player::O))
    );
    assert_eq!(turn.event, Some(TalkEvent::EngineWins));
}

#[test]
fn every_turn_has_at_most_one_event_and_matches_make_move() {
    let mut a = Game::new();
    let mut b = Game::new();
    while a.status() == GameStatus::InProgress {
        let p = first_empty(&a).expect("empty cell while in progress");
        let turn = a.play(p).expect("legal move");
        let result = b.make_move(p).expect("legal move");
        assert_eq!(turn.result, result);
        if turn.result == MoveResult::Continuing {
            assert!(matches!(
                turn.event,
                Some(
                    TalkEvent::EngineAboutToWin
                        | TalkEvent::UserAboutToWin
                        | TalkEvent::GoodMove
                        | TalkEvent::BadMove
                )
            ));
        }
    }
}
