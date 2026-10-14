---
phase: 03
slug: personality
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-10-14
---

# Phase 03 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | built-in `#[test]` + `rstest` 0.27.0 + `proptest` 1.12.0 (engine); `#[test]` (UI crate) |
| **Config file** | none (Cargo defaults) |
| **Quick run command** | `cargo test -p kiss-tictactoe-engine` |
| **Full suite command** | `cargo test --workspace` |
| **Estimated runtime** | ~30 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p kiss-tictactoe-engine` (or `-p kiss-tictactoe-ui` for UI tasks)
- **After every plan wave:** Run `cargo test --workspace`
- **Before `/gsd:verify-work`:** Full suite green + engine purity check + manual visual check
- **Max feedback latency:** 60 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| TBD (filled by planner) | — | — | ENGI-05 | — | N/A | unit (rstest) | `cargo test -p kiss-tictactoe-engine events` | ❌ W0 | ⬜ pending |
| TBD | — | — | ENGI-06 | — | N/A | unit (rstest) | `cargo test -p kiss-tictactoe-engine events` | ❌ W0 | ⬜ pending |
| TBD | — | — | ENGI-07 | — | N/A | unit + proptest | `cargo test -p kiss-tictactoe-engine commentary` | ❌ W0 | ⬜ pending |
| TBD | — | — | ENGI-06 (D-04) | — | N/A | integration | `cargo test -p kiss-tictactoe-engine --test gameplay` | extend | ⬜ pending |
| TBD | — | — | UI-02/03/04/05 | — | N/A | unit | `cargo test -p kiss-tictactoe-ui state` | ❌ W0 | ⬜ pending |
| TBD | — | — | Engine purity | — | No dioxus in engine | smoke | `! cargo tree -p kiss-tictactoe-engine \| grep -q dioxus` | exists | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `kiss-tictactoe-engine/src/events.rs` — tests for ENGI-05/06
- [ ] `kiss-tictactoe-engine/src/commentary.rs` — tests for ENGI-07
- [ ] `kiss-tictactoe-ui/src/state.rs` — tests for pure `Tally` / bubble state (UI-02..05)

No new framework install required.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Bubble fade, 320px layout, tail, focus ring | UI-02 | Visual/webview behavior | `dx serve` in kiss-tictactoe-ui (use `RUSTC_WRAPPER=""` if non-interactive); play a game and inspect |

---

## Validation Sign-Off

- [ ] All tasks have automated verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 60s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
