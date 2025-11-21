# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**TTTTT - Trash Talkin' Tic Tac Toe** - A tic-tac-toe game with integrated trash talking.

**Dual interfaces:**
- **Web UI**: Yew/WASM frontend for human players with trash talk input panel
- **MCP Server**: Model Context Protocol interface for AI agents to play AND taunt

The game randomly assigns X/O, persists state to SQLite (including taunts), and allows both humans (via browser) and AI agents (via MCP) to interact with the same game state. Server-Sent Events (SSE) provide real-time updates.

## Essential Commands

### Development
```bash
./scripts/dev.sh
```
Starts the backend server with hot-reload on http://localhost:7397. The server provides:
- REST API endpoints (`/api/*`)
- MCP HTTP endpoint (`/mcp`)
- SSE endpoint (`/api/events`)
- Static files for Yew/WASM frontend

During development, Trunk may run a dev server on port 8080 for faster WASM rebuilds, but this is temporary and only for development convenience.

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
├── backend/          # Rust backend server
│   └── src/
│       ├── api/      # REST API + SSE (routes.rs, server.rs, mcp_handler.rs)
│       ├── game/     # Game logic (board.rs, logic.rs, player.rs, manager.rs)
│       ├── db/       # SQLite persistence (schema.rs, repository.rs)
│       ├── mcp/      # MCP protocol (protocol.rs, tools.rs, server.rs)
│       ├── main.rs   # HTTP server entry point (Axum)
│       └── bin/      # Binary entry points
│           └── game-mcp-server.rs  # Stdio MCP server
├── frontend/         # Yew/WASM UI
│   └── src/
│       └── lib.rs    # Yew components
├── shared/           # Common types used by both frontend and backend
│   └── src/
│       └── lib.rs    # Player, Cell, GameState, Move, GameError, etc.
├── examples/         # AI agent integration examples
│   ├── README.md     # Setup guide for OpenAI, Gemini, Claude Desktop
│   ├── openai_agent.py
│   ├── gemini_agent.py
│   └── claude-desktop-config.json
└── scripts/          # Build and dev scripts
```

### Critical Architecture Details

**Single Server Architecture:**
- **HTTP Server** (`backend/src/main.rs`): Single Axum server on port 7397
  - Serves REST API endpoints (`/api/*`) for UI interactions
  - Serves MCP HTTP endpoint (`/mcp`) for AI agent tool calls
  - Serves SSE endpoint (`/api/events`) for real-time browser updates
  - Serves static frontend files from `frontend/dist`
- **Stdio MCP Binary** (`backend/src/bin/game-mcp-server.rs`): Separate binary for Claude Desktop
  - Reads JSON-RPC from stdin, writes to stdout
  - Used by Claude Desktop via stdio transport
  - Shares same game state via SQLite database

**Shared Game State:**
- `GameManager` coordinates all game operations
- `GameRepository` handles SQLite persistence
- Database stores **current game ID** shared across processes (HTTP + stdio can access same game)
- `get_or_create_game()` checks database for current game ID first, enabling cross-process state sharing

**MCP Activity Tracking:**
- `last_mcp_activity` timestamp in `GameState` tracks MCP tool usage
- Updated in HTTP handler (`backend/src/api/mcp_handler.rs`), NOT in individual tools
- SSE broadcasts "mcp-activity-start" and "mcp-activity-end" events
- Frontend displays "AI is thinking..." indicator based on SSE events

**Shared Types Pattern:**
The `shared` crate defines ALL game types used by both backend and frontend:
- `Player`, `Cell`, `GameState`, `GameStatus`, `Move`, `GameError`, `Taunt`
- Ensures type safety across client-server boundary via serde serialization
- Changes to game types require rebuilding both frontend and backend

**Critical Frontend Issue:**
The frontend uses `[lib]` in Cargo.toml, not `[bin]`. The `main()` function in `frontend/src/lib.rs` **requires** `#[wasm_bindgen(start)]` annotation to be called when WASM loads. Without this annotation, the Yew app won't render (only CSS background gradient appears).

**Database:**
- SQLite file-based (path: `GAME_DB_PATH` env var, default: `game.db`)
- WAL mode creates `.db-shm` and `.db-wal` files (normal behavior, add to .gitignore)
- Tables: `games`, `moves`, `taunts`, `current_game`
- Located in `backend/src/db/`

**MCP Tools** (in `backend/src/mcp/tools.rs`):
- `initialize`: MCP protocol handshake (returns protocol version)
- `tools/list`: MCP tool discovery (returns all 6 tools with JSON schemas)
- `view_game_state`: Get board state, history, taunts
- `get_turn`: Determine whose turn (X/O, human/AI)
- `make_move`: Submit move (row, col)
- `taunt_player`: Send trash talk message
- `restart_game`: Start new game
- `get_game_history`: View past moves

**SSE (Server-Sent Events):**
- Endpoint: `/api/events`
- Events: `game-update`, `taunt`, `mcp-activity-start`, `mcp-activity-end`
- Frontend subscribes on mount, updates UI in real-time
- Broadcast channel capacity: 100 events

### Communication Flow
```
┌─────────────────┐
│ Browser (Yew)   │◄──REST/SSE──►┐
└─────────────────┘               │
                                  │
┌─────────────────┐               ▼
│ AI Agent (HTTP) │◄──MCP HTTP───┤
└─────────────────┘               │     ┌────────────────────┐
                                  ├────►│ Single HTTP Server │◄───►│ SQLite │
┌─────────────────┐               │     │   (Port 7397)      │     │ game.db│
│ Claude Desktop  │◄──MCP stdio───┤     │  - REST API        │     └────────┘
│ (stdio binary)  │               │     │  - MCP endpoints   │
└─────────────────┘               │     │  - SSE             │
                                  │     │  - Static files    │
                                  │     └────────────────────┘
                            GameManager
                          (coordinates all)
```

## Development Process (TDD Required)

This project follows **strict Test-Driven Development**.

### Red-Green-Refactor (Required for All Code)

1. **Red**: Write a failing test that describes desired behavior
   - Run test to verify it fails: `cargo test -- test_name`
2. **Green**: Write minimal code to make the test pass
   - Run test to verify it passes
3. **Refactor**: Improve code quality without changing behavior
   - Run tests after each change

**NEVER write implementation code without a failing test first.** Placeholder tests like `assert_eq!(2 + 2, 4)` are NOT acceptable.

### Pre-commit Checklist (MANDATORY)
- [ ] `cargo fmt --all`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` (zero warnings required)
- [ ] `cargo test --all` (all tests pass)
- [ ] Tests cover actual functionality (no placeholders)
- [ ] Update docs if behavior changed
- [ ] Commit with detailed message

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
feat: Add MCP protocol discovery endpoints

- Add `initialize` endpoint returning protocol version 2024-11-05
- Add `tools/list` endpoint with complete JSON schemas for all 6 tools
- Enable standard tool discovery for AI platforms

Tests: All 181 tests passing
Clippy: Clean with no warnings
```

## Logging Requirements

- **Backend**: Use `tracing` for structured logging to stdout
- **Frontend**: Use `console_log` with `log` crate for browser console
- **UI**: Scrollable log pane in footer (CSS class: `.log-container`)

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
- `process.md`: Full TDD workflow
- `status.md`: Current project status
- `online-plan.md`: Deployment plan for hosting online

## AI Agent Integration

### Examples Directory
The `examples/` directory contains working integrations for:
- **Claude Desktop**: stdio transport (native MCP support)
- **OpenAI GPT-4**: HTTP transport with function calling
- **Google Gemini**: HTTP transport with function declarations

See `examples/README.md` for complete setup instructions.

### Testing MCP Endpoints
```bash
# Initialize
curl -X POST http://localhost:7397/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"initialize","params":{},"id":1}' | jq

# List tools
curl -X POST http://localhost:7397/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"tools/list","params":{},"id":2}' | jq

# View game state
curl -X POST http://localhost:7397/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"view_game_state","params":{},"id":3}' | jq
```

## Common Pitfalls

1. **Frontend not rendering**: Ensure `main()` in `frontend/src/lib.rs` has `#[wasm_bindgen(start)]` annotation
2. **Clippy warnings on WASM tests**: Annotate test code with `#[cfg(test)]` and `#[wasm_bindgen_test]`
3. **Database conflicts**: SQLite WAL mode creates `.db-shm` and `.db-wal` files - this is normal
4. **Port conflicts**: Backend server uses port 7397 - ensure it's available
5. **MCP thinking indicator not showing**: Activity tracking happens in HTTP handler, not individual tools
6. **Cross-process game state**: Database stores current game ID; both HTTP and stdio servers share state via DB
7. **SSE not updating**: Check browser console for connection errors at `/api/events` endpoint
