---
phase: 01-foundation
reviewed: 2026-10-14T00:00:00Z
depth: standard
files_reviewed: 14
files_reviewed_list:
  - .github/ISSUE_TEMPLATE/bug_report.md
  - .github/ISSUE_TEMPLATE/feature_request.md
  - .github/PULL_REQUEST_TEMPLATE.md
  - .gitignore
  - Cargo.toml
  - CODE_OF_CONDUCT.md
  - CONTRIBUTING.md
  - kiss-tictactoe-engine/Cargo.toml
  - kiss-tictactoe-engine/src/lib.rs
  - kiss-tictactoe-ui/assets/main.css
  - kiss-tictactoe-ui/Cargo.toml
  - kiss-tictactoe-ui/Dioxus.toml
  - kiss-tictactoe-ui/src/main.rs
  - LICENSE
findings:
  critical: 0
  warning: 5
  info: 4
  total: 9
status: issues_found
---

# Phase 01: Code Review Report

**Reviewed:** 2026-10-14T00:00:00Z
**Depth:** standard
**Files Reviewed:** 14
**Status:** issues_found

## Summary

This is the Phase 1 foundation review covering workspace scaffolding, open-source artifacts (license, contributing guide, code of conduct, issue templates), and placeholder crate stubs. No application logic exists yet — `kiss-tictactoe-engine/src/lib.rs` is a single comment and `kiss-tictactoe-ui/src/main.rs` is a skeleton launcher.

No critical/blocker issues were found. Five warnings are present: the most impactful are that the `dist/` build output directory is absent from `.gitignore` (a concrete risk of committing generated artifacts), that `main.css` is never loaded into the running application (the `asset!()` macro called for by CLAUDE.md is absent), and that `manganis` is declared as a dependency but never imported. These three are coupled — fixing the CSS loading issue resolves the unused-dependency issue simultaneously.

## Warnings

### WR-01: `dist/` build output not excluded from version control

**File:** `.gitignore:1-5`
**Issue:** `Dioxus.toml` declares `out_dir = "dist"`. Running `dx build` or `dx serve` writes compiled assets into `dist/`. That directory is not listed in `.gitignore`, so the entire build output will be staged and committed the first time a contributor runs the dev toolchain. This will pollute the repository with generated binaries and will conflict across platforms.
**Fix:**
```
/target/
dist/
.idea/
*.iml
**/.DS_Store
*.swp
```

---

### WR-02: `main.css` is never loaded — CSS file is dead weight

**File:** `kiss-tictactoe-ui/src/main.rs:1-12`
**Issue:** `assets/main.css` exists and is referenced by `Dioxus.toml`'s `asset_dir = "assets"`, but no `asset!()` call appears in `main.rs`. The stylesheet is never bundled or injected into the running application. The `manganis` crate (added for exactly this purpose) is imported in `Cargo.toml` but never used in code. CLAUDE.md explicitly requires the `asset!()` macro for CSS bundling.
**Fix:** Add the stylesheet reference inside `App` using the `manganis` `asset!()` macro:
```rust
use dioxus::prelude::*;
use manganis::asset;

const MAIN_CSS: manganis::ImageAsset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div { "kiss-tictactoe — coming soon" }
    }
}
```
(Exact Dioxus 0.7 stylesheet injection API should be verified against 0.7.10 docs; `document::Stylesheet` is the current pattern for injecting assets in Dioxus 0.7.)

---

### WR-03: `manganis` declared as dependency but never used

**File:** `kiss-tictactoe-ui/Cargo.toml:7`
**Issue:** `manganis = "0.7.10"` is present but `main.rs` neither `use`s it nor calls `asset!()`. This causes unnecessary compile-time overhead and will cause `cargo udeps` or similar auditing tools to flag it as dead weight. The fix is the same as WR-02 — actually using the crate resolves both findings simultaneously.
**Fix:** Resolve by implementing WR-02. If CSS loading is intentionally deferred to a later phase, remove the `manganis` dependency until it is actually needed.

---

### WR-04: `box-sizing: border-box` applied only to `body`, not inherited by children

**File:** `kiss-tictactoe-ui/assets/main.css:3`
**Issue:** `box-sizing: border-box` is set on `body` only. `box-sizing` is not an inherited CSS property; child elements default to `content-box`. Any future element that relies on `border-box` sizing (e.g., grid cells with padding) will compute dimensions incorrectly unless this is fixed now at the reset level.
**Fix:**
```css
*, *::before, *::after {
    box-sizing: border-box;
}

body {
    margin: 0;
}
```

---

### WR-05: Crate `Cargo.toml` files missing `license` field

**File:** `kiss-tictactoe-engine/Cargo.toml:1-7`, `kiss-tictactoe-ui/Cargo.toml:1-8`
**Issue:** The repository ships an MIT `LICENSE` file, but neither crate's `Cargo.toml` declares `license = "MIT"`. When these crates are built or inspected via `cargo metadata`, the license field is absent. If the engine crate is ever published to crates.io, crates.io will reject it without a `license` or `license-file` field. The PR checklist does not catch this.
**Fix:** Add to both `Cargo.toml` files:
```toml
license = "MIT"
```
Or, if workspace-level metadata is introduced (see IN-01), set it once in `[workspace.package]` and inherit with `license.workspace = true`.

---

## Info

### IN-01: Workspace `Cargo.toml` missing `[workspace.package]` for shared metadata

**File:** `Cargo.toml:1-7`
**Issue:** There is no `[workspace.package]` table. Fields like `version`, `edition`, `license`, `authors`, and `repository` are currently duplicated (or absent) in each member crate's manifest. Adding a workspace-level table allows members to use `field.workspace = true` to inherit shared values, reducing drift.
**Fix:**
```toml
[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT"
authors = ["P. Todd Decker <ptdecker@mac.com>"]
repository = "https://github.com/ptdecker/kiss-tictactoe"
```

---

### IN-02: `Dioxus.toml` bundle metadata is entirely empty

**File:** `kiss-tictactoe-ui/Dioxus.toml:22-29`
**Issue:** `copyright`, `short_description`, `long_description`, and `icon` are all empty strings or empty lists. These feed directly into the platform-native installer/bundle that `dx bundle` produces. An empty `icon` means the app ships with the default Dioxus icon; empty descriptions produce broken App Store / package manager entries.
**Fix:** Populate these fields before any bundle is produced:
```toml
copyright = "Copyright (c) 2026 P. Todd Decker"
short_description = "A tic-tac-toe game powered by a Rust engine"
long_description = "kiss-tictactoe is a demonstration of idiomatic Rust architecture …"
icon = ["assets/icons/icon.png"]
```

---

### IN-03: PR checklist does not include `cargo doc` verification

**File:** `.github/PULL_REQUEST_TEMPLATE.md:19-23`
**Issue:** The checklist enforces `fmt`, `clippy`, and `test` but omits `cargo doc --workspace --no-deps`. Doc-comment errors (`missing_docs` lint, broken intra-doc links) are not caught by clippy unless explicitly configured, meaning broken documentation can merge undetected.
**Fix:** Add to checklist:
```markdown
- [ ] `cargo doc --workspace --no-deps` builds without warnings
```

---

### IN-04: Engine `lib.rs` is a single placeholder comment

**File:** `kiss-tictactoe-engine/src/lib.rs:1`
**Issue:** The entire file is `// Phase 2 defines the engine API.` — no public items, no `#![deny(missing_docs)]`, no module structure. This is expected for Phase 1 but is noted because Phase 2 work should start from a clean slate that includes at minimum a crate-level doc comment and the `#![deny(missing_docs)]` attribute to enforce documentation discipline from the first public API addition.
**Fix:** When Phase 2 begins, open with:
```rust
//! kiss-tictactoe game engine — platform-agnostic logic for tic-tac-toe.
#![deny(missing_docs)]
```

---

_Reviewed: 2026-10-14T00:00:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
