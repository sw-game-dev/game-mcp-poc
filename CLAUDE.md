# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**TTTTT - Trash Talkin' Tic Tac Toe** - A game where trash talk is part of the strategy!

Dual interfaces with real-time trash talking:
- **Web UI**: Yew/WASM frontend for human players with trash talk input panel
- **MCP Server**: Model Context Protocol interface for AI agents to play AND taunt

The game assigns X/O randomly, persists state to SQLite (including all taunts!), and allows both humans (via browser) and AI agents (via MCP) to interact with the same game state. Server-Sent Events (SSE) provide real-time updates for moves and trash talk.

## Essential Commands

### Development
```bash
./scripts/dev.sh
```
Starts both servers with hot-reload:
- Backend: http://localhost:3000 (REST API + MCP server)
- Frontend: http://localhost:8080 (Yew/WASM app)

### Build Production
```bash
./scripts/build.sh
```
Builds backend binary and frontend WASM assets. Auto-installs trunk and wasm-bindgen-cli if missing.

### Run Production
```bash
./scripts/serve.sh
```
Runs production backend which serves both API and static frontend files.

### Testing

**All tests:**
```bash
cargo test --all
```

**Specific package:**
```bash
cargo test --package backend
cargo test --package frontend
cargo test --package shared
```

**Single test:**
```bash
cargo test -- test_name
```

**WASM tests** (requires wasm-pack):
```bash
cd frontend
wasm-pack test --headless --firefox
```

### Code Quality (Pre-commit Requirements)

**Format** (must run before commit):
```bash
cargo fmt --all
```

**Lint** (must be clean before commit):
```bash
cargo clippy --all-targets --all-features -- -D warnings
```
**NEVER disable clippy checks.** For WASM test code that appears dead, use proper annotations: `#[cfg(test)]`, `#[wasm_bindgen_test]`.

## Architecture

### Workspace Structure
```
game-mcp-poc/
├── backend/          # Rust REST API + MCP server + static file serving
│   └── src/
│       ├── game/     # Game logic (board.rs, logic.rs, player.rs)
│       ├── db/       # SQLite persistence layer
│       └── main.rs   # Entry point
├── frontend/         # Yew/WASM UI
│   └── src/
│       └── lib.rs    # Yew components
├── shared/           # Common types used by both frontend and backend
│   └── src/
│       └── lib.rs    # Player, Cell, GameState, Move, GameError
└── scripts/          # Build and dev scripts
```

### Key Design Patterns

**Shared Types**: The `shared` crate defines all game types (`Player`, `Cell`, `GameState`, `Move`, `GameError`) used by both backend and frontend. This ensures type safety across the client-server boundary via serde serialization.

**Critical Frontend Issue**: The frontend uses `[lib]` in Cargo.toml, not `[bin]`. The `main()` function in `frontend/src/lib.rs` **requires** `#[wasm_bindgen(start)]` annotation to be called when WASM loads. Without this annotation, the Yew app won't render (only the CSS background gradient appears).

**Database**: SQLite file-based (path configurable). Stores game state, board, turns, move history. Located in `backend/src/db/`.

**MCP Tools** (in backend):
- `view_game_state`: Get board state
- `get_turn`: Determine whose turn
- `make_move`: Submit move (row, col)
- `taunt_player`: Send message to player
- `restart_game`: Start new game
- `get_game_history`: View past moves

### Communication Flow
```
Browser (Yew) <--REST--> Backend (Axum) <---> SQLite
                            ^
                            |
                         MCP Protocol
                            |
                        AI Agent
```

## Development Process (TDD Required - STRICTLY ENFORCED)

This project follows **strict Test-Driven Development**. Every feature MUST have tests BEFORE implementation.

### Red-Green-Refactor (The ONLY Way to Add Code)

1. **Red**: Write a failing test that describes the desired behavior
   - Test must fail for the right reason
   - Run test to verify it fails: `cargo test -- test_name`
2. **Green**: Write minimal code to make the test pass
   - Focus on making it work, not making it perfect
   - Run test to verify it passes
3. **Refactor**: Improve code quality without changing behavior
   - Run tests after each change to ensure nothing breaks

**NEVER write implementation code without a failing test first.** Placeholder tests like `assert_eq!(2 + 2, 4)` are NOT acceptable and violate TDD principles.

### Pre-commit Checklist (MANDATORY - ALL MUST PASS)
- [ ] `cargo fmt --all` (format code)
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` (no warnings allowed)
- [ ] `cargo test --all` (all tests pass - REAL tests, not placeholders)
- [ ] Tests cover actual functionality (not `2 + 2 = 4` placeholders)
- [ ] Validate .gitignore (no secrets, generated files ignored: `target/`, `*.db`, `*.db-shm`, `*.db-wal`, `dist/`, `pkg/`)
- [ ] Update docs if behavior changed
- [ ] Commit with detailed message

**If tests are missing or are placeholders, the code is NOT ready to commit.**

### Commit Message Format
```
<type>: <short summary>

<detailed description>

Tests: [status]
Clippy: [status]
```

Types: `feat`, `fix`, `test`, `refactor`, `docs`, `chore`

Example:
```
feat: Implement win detection for tic-tac-toe

- Add check_winner function for rows, columns, diagonals
- Add GameStatus enum with Won(Player) and Draw variants
- Add comprehensive tests for all win scenarios

Tests: All tests passing (15 total)
Clippy: Clean with no warnings
```

## Logging Requirements

Per README line 40: "Use extensive logging to stdout on the server and to the browser console log, and also to the footer of the UI (in a short scrollable pane)."

- **Backend**: Use `tracing` for structured logging to stdout
- **Frontend**: Use `console_log` with `log` crate for browser console
- **UI**: Include scrollable log pane in footer (already in CSS as `.log-container`)

Initialize frontend logging:
```rust
console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");
```

## Prerequisites

- Rust 2024 edition
- `trunk` (auto-installed by scripts)
- `wasm-bindgen-cli` (auto-installed by scripts)
- Target: `wasm32-unknown-unknown` (auto-added by scripts)

## Documentation

See `docs/` directory:
- `architecture.md`: System design and tech stack
- `prd.md`: Product requirements
- `design.md`: Detailed design specifications
- `plan.md`: Implementation plan
- `process.md`: Full TDD and pre-commit process (reference for all development)
- `status.md`: Current project status

## Common Pitfalls

1. **Frontend not rendering**: Ensure `main()` in `frontend/src/lib.rs` has `#[wasm_bindgen(start)]` annotation
2. **Clippy warnings on WASM tests**: Annotate test code properly with `#[cfg(test)]` and `#[wasm_bindgen_test]`
3. **Missing .js extensions**: Not needed - this is Rust/WASM, not ES modules
4. **Database conflicts**: SQLite uses WAL mode, which creates `.db-shm` and `.db-wal` files - these are normal
5. **Port conflicts**: Backend uses 3000, frontend dev server uses 8080 - ensure these are free
