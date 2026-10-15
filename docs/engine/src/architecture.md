# Architecture

kiss-tictactoe is a two-crate Cargo workspace:

- `kiss-tictactoe-engine`: a library holding all game rules, the perfect-play AI, trash-talk event selection and the commentary lines.
- `kiss-tictactoe-ui`: a Dioxus desktop app that renders the board and drives the engine.

The dependency runs one way. The UI depends on the engine; the engine never depends on the UI.

## Why the engine stays pure

The engine has zero dependencies. It reads no clock, touches no file system and makes no platform calls, so it compiles unchanged for desktop, web and mobile targets, and any front end (a CLI, a web page, a phone app) can drive it.

You can check the boundary yourself. This prints nothing when the engine is free of Dioxus:

```sh
cargo tree -p kiss-tictactoe-engine | grep dioxus
```

## Module map

```text
kiss-tictactoe-engine/src
  lib.rs         crate docs and the public re-exports
  types.rs       Player, Position, Board, GameStatus, MoveResult, MoveError, TalkEvent
  game.rs        Game and Turn: the state machine and the play/make_move API
  detection.rs   detect_status and the eight winning lines
  ai.rs          minimax engine reply and move-quality judging
  events.rs      picks the single TalkEvent for a turn
  commentary.rs  Commentator: turns an event into a line of text
```

## What lives in the UI instead

Anything that depends on the outside world is the front end's job:

- rendering the board and the speech bubble
- the random seed handed to the commentator
- the session score
- speech-bubble timing and fading

Start with [Data types](types.md), then [The Game API](game-api.md), then [The event system](events.md).
