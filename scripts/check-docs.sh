#!/usr/bin/env bash
# Local docs rot guard: builds rustdoc and both mdBooks, runs the engine
# manual samples against the real engine crate, and checks links.
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

BOOKS=(docs/user-guide docs/engine)
DEPS_DIR=target/docs-deps

LOG=$(mktemp)
trap 'rm -f "$LOG"' EXIT

echo "==> checking mdbook version"
if ! command -v mdbook >/dev/null 2>&1 || ! mdbook --version | grep -qE '(^|[^0-9.])0\.5\.4($|[^0-9.])'; then
  echo "mdbook 0.5.4 required: cargo install mdbook --version 0.5.4 --locked" >&2
  exit 1
fi

echo "==> markdown lint (rumdl)"
if ! command -v rumdl >/dev/null 2>&1; then
  echo "rumdl required: cargo install rumdl --version 0.2.79 --locked" >&2
  exit 1
fi
rumdl check .

echo "==> checking books exist"
for book in "${BOOKS[@]}"; do
  for f in "$book/book.toml" "$book/src/SUMMARY.md"; do
    if [ ! -f "$f" ]; then
      echo "missing required book file: $f" >&2
      exit 1
    fi
  done
done

echo "==> cargo doc (warnings are errors)"
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps

echo "==> missing_docs gate on the engine"
cargo rustdoc -p kiss-tictactoe-engine -- -D missing_docs

echo "==> building engine into dedicated target dir"
CARGO_TARGET_DIR="$DEPS_DIR" cargo build -p kiss-tictactoe-engine

for book in "${BOOKS[@]}"; do
  echo "==> mdbook build $book"
  mdbook build "$book" >"$LOG" 2>&1 || {
    cat "$LOG"
    echo "mdbook build failed for $book" >&2
    exit 1
  }
  cat "$LOG"
  if grep -E 'WARN|ERROR' "$LOG" >/dev/null; then
    echo "mdbook build reported warnings or errors for $book" >&2
    exit 1
  fi
done

echo "==> mdbook test docs/engine"
mdbook test docs/engine -L "$DEPS_DIR/debug/deps"

echo "==> checking SUMMARY entries and links"
misses=()
for book in "${BOOKS[@]}"; do
  src="$book/src"
  while IFS= read -r target; do
    if [ ! -s "$src/$target" ]; then
      misses+=("missing link: $src/SUMMARY.md -> $target")
    fi
  done < <(grep -oE '\]\([^)]+\.md\)' "$src/SUMMARY.md" | sed -E 's/^\]\((.*)\)$/\1/')

  for file in "$src"/*.md; do
    while IFS= read -r target; do
      case "$target" in
        http://* | https://* | mailto:* | '#'*) continue ;;
      esac
      target="${target%%#*}"
      [ -z "$target" ] && continue
      case "$target" in
        *.md) path="$src/$target" ;;
        *) path="$book/book/$target" ;;
      esac
      if [ ! -e "$path" ]; then
        misses+=("missing link: $file -> $target")
      fi
    done < <(grep -oE '\]\([^)]+\)' "$file" | sed -E 's/^\]\((.*)\)$/\1/' || true)
  done
done

if [ "${#misses[@]}" -gt 0 ]; then
  printf '%s\n' "${misses[@]}" >&2
  exit 1
fi

echo "docs OK"
