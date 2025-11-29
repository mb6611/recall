# CLAUDE.md

## Purpose

Search and resume past conversations from Claude Code and Codex CLI.

## Principles

- **Delightful** - Made with love, feels good to use
- **Simple, uncluttered, focused** - Two panes, keyboard-driven, no chrome
- **Responsive** - Instant startup via background indexing, sub-100ms search
- **Match-recency matters** - Rank by most recent message containing the match (human memory anchors to recent context)
- **Seamless resume** - Enter execs directly into the CLI, no intermediate steps

## Development

```bash
cargo check          # Fast compile check (no binary)
cargo run            # Build debug + run
cargo test           # Run tests
cargo clippy         # Lint
```

To test the TUI end-to-end, use tmux:
```bash
cargo build && tmux new-session -d -s test './target/debug/recall'
tmux send-keys -t test 'search query'
tmux capture-pane -t test -p          # See output
tmux kill-session -t test             # Cleanup
```

## Install

```bash
cargo install --path .
```

## Architecture

Rust TUI for searching Claude Code and Codex CLI conversation history.

- `src/main.rs` - Entry point, event loop, exec into CLI on resume
- `src/app.rs` - Application state, search logic, background indexing thread
- `src/ui.rs` - Two-pane ratatui rendering, match highlighting
- `src/tui.rs` - Terminal setup/teardown
- `src/theme.rs` - Light/dark theme with auto-detection
- `src/session.rs` - Core types: Session, Message, SearchResult
- `src/parser/` - JSONL parsers for Claude (`~/.claude/projects/`) and Codex (`~/.codex/sessions/`)
- `src/index/` - Tantivy full-text search index, stored in `~/.cache/recall/`

## Key Patterns

- Background indexing: spawns thread on startup, indexes most recent files first, sends progress via mpsc channel
- Unicode-safe string handling: use char indices not byte indices when slicing (see `highlight_matches`, `create_snippet`)
- Search ranking: combines BM25 relevance with recency boost (exponential decay, 7-day half-life)
- Theme detection: queries terminal bg color via crossterm, falls back to COLORFGBG env var
- Event handling: drains all pending events each frame to prevent mouse event flooding
- Contextual status bar: hints adapt to state (e.g., scroll hint only when preview is scrollable)

## Releasing

1. Bump version in `Cargo.toml`
2. Commit and tag:
   ```bash
   git add -A && git commit -m "Release message"
   git tag -a v0.X.Y -m "v0.X.Y: Short description"
   git push && git push --tags
   ```
3. GitHub Actions automatically builds binaries for all platforms and creates a release
4. Monitor with `gh run watch` or check https://github.com/zippoxer/recall/actions
5. Update homebrew tap (https://github.com/zippoxer/homebrew-tap):
   ```bash
   # Download release assets and compute SHA256
   gh release download v0.X.Y -R zippoxer/recall --pattern "*.tar.gz" -D /tmp/release
   cd /tmp/release && shasum -a 256 *.tar.gz

   # Update Formula/recall.rb with new version and SHA256 hashes
   # Commit and push to homebrew-tap
   ```
6. Verify:
   ```bash
   brew update && brew upgrade zippoxer/tap/recall

   # Test with tmux
   tmux new-session -d -s test -x 120 -y 40
   tmux send-keys -t test 'recall test query' Enter
   sleep 2 && tmux capture-pane -t test -p
   tmux kill-session -t test
   ```
