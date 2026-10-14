# Requirements: kiss-tictactoe

**Defined:** 2026-10-14
**Core Value:** A fully playable, good-looking tic-tac-toe game where the engine trash talks you — demonstrating idiomatic Rust architecture to a friend.

## v1 Requirements

### Engine

- [ ] **ENGI-01**: Game state tracks board contents, current player, and move history
- [ ] **ENGI-02**: Engine detects win (any row/column/diagonal) and draw (board full, no winner) after every move
- [ ] **ENGI-03**: Engine exposes a clean public API usable from any UI or interface with zero Dioxus dependency
- [ ] **ENGI-04**: Engine AI uses minimax and plays optimally (always draws against perfect play)
- [x] **ENGI-05**: Engine emits game-lifecycle trash talk events: game start, engine wins, user wins, draw result
- [ ] **ENGI-06**: Engine emits tactical trash talk events: engine about to win, user about to win (two in a row), engine assesses a good move, engine assesses a bad move
- [x] **ENGI-07**: Message bank contains 4–8 strings per trash talk event category (~32–64 strings total), selected randomly per trigger

### UI

- [ ] **UI-01**: User can click any empty cell to place their mark; occupied and invalid cells are non-interactive
- [x] **UI-02**: Engine speech bubble displays context-appropriate trash talk after each game event
- [ ] **UI-03**: Session scoreboard shows cumulative wins, losses, and draws for the current session
- [ ] **UI-04**: User can start a new game (board resets; score persists)
- [ ] **UI-05**: User can reset the session scoreboard independently of starting a new game
- [ ] **UI-06**: UI layout is responsive from phone width upward using plain CSS (no Tailwind)

### Infrastructure

- [ ] **INFRA-01**: Cargo workspace with two crates: `kiss-tictactoe-engine` (lib, no platform deps) and `kiss-tictactoe-ui` (Dioxus 0.7.10 desktop binary)
- [ ] **INFRA-02**: Engine crate purity enforced: `cargo tree -p kiss-tictactoe-engine | grep dioxus` returns nothing
- [ ] **INFRA-03**: `.gitignore` covers Rust build artifacts (`target/`) and JetBrains RustRover IDE files (`.idea/`, `*.iml`)
- [ ] **INFRA-04**: GitHub open-source artifacts: `LICENSE` (MIT), `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, issue templates (bug report, feature request), PR template
- [ ] **INFRA-05**: Public GitHub repository synced from local with all planning and source files
- [ ] **INFRA-06**: `docs/user-guide/` — mdBook user guide covering how to play, modes, and interface
- [ ] **INFRA-07**: `docs/engine/` — mdBook technical manual covering engine API, data types, event system, and extension points

## v2 Requirements

### Game Setup

- **SETUP-01**: User can choose who goes first (play as X or O) before each game
- **SETUP-02**: User can select engine difficulty level

### Training Mode

- **TRAIN-01**: Training mode engine plays suboptimally using epsilon-greedy strategy (allows user to win)
- **TRAIN-02**: On-demand hint button shows the user a recommended move in training mode
- **TRAIN-03**: Engine emits training-specific trash talk (handicap acknowledgements, hint-used reactions)
- **TRAIN-04**: UI provides a mode toggle between Normal and Training

### Polish

- **POLSH-01**: Winning three cells are visually highlighted after a win
- **POLSH-02**: Keyboard navigation for board (arrow keys) and controls (Tab); ARIA labels on all interactive elements
- **POLSH-03**: CI pipeline (GitHub Actions) runs `cargo test`, `cargo doc`, and `mdbook build` on push

## Out of Scope

| Feature | Reason |
|---------|--------|
| Multiplayer / second human player | Always user vs engine — adding P2P requires networking, not in scope |
| Networking of any kind | Local game only; adds complexity with no demo value |
| Persistent cross-session stats | Scoreboard resets on app restart; no database warranted |
| Mobile builds (v1) | Architecture is mobile-ready but dioxus-mobile 0.7 is still alpha |
| Multiple board sizes or variants | 3×3 classic only; KISS principle |
| Settings menu | No configurable options in v1 |
| Save / resume game | No persistence needed for a demonstration project |
| Animated engine "thinking" delay | Minimax on 3×3 is instant; fake delays add no value |
| Always-on hints | On-demand only (v2); always-on hints undermine player agency |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| INFRA-01 | Phase 1 | Pending |
| INFRA-02 | Phase 1 | Pending |
| INFRA-03 | Phase 1 | Pending |
| INFRA-04 | Phase 1 | Pending |
| INFRA-05 | Phase 1 | Pending |
| ENGI-01 | Phase 2 | Pending |
| ENGI-02 | Phase 2 | Pending |
| ENGI-03 | Phase 2 | Pending |
| ENGI-04 | Phase 2 | Pending |
| UI-01 | Phase 2 | Pending |
| UI-06 | Phase 2 | Pending |
| ENGI-05 | Phase 3 | Complete |
| ENGI-06 | Phase 3 | Pending |
| ENGI-07 | Phase 3 | Complete |
| UI-02 | Phase 3 | Complete |
| UI-03 | Phase 3 | Pending |
| UI-04 | Phase 3 | Pending |
| UI-05 | Phase 3 | Pending |
| INFRA-06 | Phase 4 | Pending |
| INFRA-07 | Phase 4 | Pending |

**Coverage:**
- v1 requirements: 20 total
- Mapped to phases: 20
- Unmapped: 0 ✓

---
*Requirements defined: 2026-10-14*
*Last updated: 2026-10-14 after roadmap creation*
