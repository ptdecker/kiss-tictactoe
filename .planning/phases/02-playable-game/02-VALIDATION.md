---
phase: 02
slug: playable-game
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-10-14
---

# Phase 02 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust built-in `#[test]` + `proptest` 1.12.0 + `rstest` 0.27.0 |
| **Config file** | none — standard `cargo test` discovery |
| **Quick run command** | `cargo test -p kiss-tictactoe-engine` |
| **Full suite command** | `cargo test --workspace` |
| **Estimated runtime** | ~5 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p kiss-tictactoe-engine`
- **After every plan wave:** Run `cargo test --workspace`
- **Before `/gsd:verify-work`:** Full suite must be green + purity check + manual smoke test
- **Max feedback latency:** ~5 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 02-??-01 | TBD | 1 | ENGI-01 | — | N/A | unit | `cargo test -p kiss-tictactoe-engine game::tests` | ❌ W0 | ⬜ pending |
| 02-??-02 | TBD | 1 | ENGI-02 | — | N/A | unit (rstest) | `cargo test -p kiss-tictactoe-engine detection::tests` | ❌ W0 | ⬜ pending |
| 02-??-03 | TBD | 1 | ENGI-04 | — | N/A | property (proptest) | `cargo test -p kiss-tictactoe-engine ai::tests` | ❌ W0 | ⬜ pending |
| 02-??-04 | TBD | 1 | ENGI-03 | — | `cargo tree -p kiss-tictactoe-engine \| grep dioxus` returns empty | smoke | `cargo tree -p kiss-tictactoe-engine \| grep dioxus` | ❌ W0 | ⬜ pending |
| 02-??-05 | TBD | 2 | UI-01 | T-02-01 | Occupied cells non-interactive (`pointer-events: none`) | manual smoke | `dx serve --platform desktop` + click test | — manual | ⬜ pending |
| 02-??-06 | TBD | 2 | UI-06 | — | Layout intact at 320px viewport, no horizontal scroll | manual smoke | Resize webview window to 320px | — manual | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `kiss-tictactoe-engine/src/types.rs` — `Position`, `Player`, `MoveResult`, `MoveError` type definitions
- [ ] `kiss-tictactoe-engine/src/detection.rs` — win detection logic + inline `#[cfg(test)]` module with rstest parameterized cases
- [ ] `kiss-tictactoe-engine/src/ai.rs` — minimax algorithm + inline `#[cfg(test)]` module with proptest suite (engine never loses)
- [ ] `kiss-tictactoe-engine/src/game.rs` — `Game` struct + inline `#[cfg(test)]` module covering state transitions

---

## Threat Model

**Surface:** Local desktop game — no network, no persistence, no authentication, no sensitive data.

| Threat | STRIDE | Mitigation | Plan Coverage |
|--------|--------|------------|---------------|
| Invalid board position (row/col out of bounds) | Tampering | `MoveError::InvalidPosition` from engine; UI enforces `interactive: bool` | ENGI-01/03 |
| Move on occupied cell | Tampering | `MoveError::CellOccupied` returned; cell has `pointer-events: none` via CSS | ENGI-01, UI-01 |
| Move after game over | Tampering | `MoveError::GameAlreadyOver`; overlay covers board | ENGI-01, UI-01 |

**Risk level:** Minimal — zero external attack surface.
