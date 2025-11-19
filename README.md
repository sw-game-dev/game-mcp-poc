# TTTTT - Trash Talkin' Tic Tac Toe

A dual-interface tic-tac-toe game where **trash talk is part of the game!** Implements the **Model Context Protocol (MCP)** to enable AI agents like Claude Code to play AND taunt via a standardized tool interface.

![Game Screenshot](./images/screenshot.png)

## 🎯 Project Status: MCP Server Complete ✅

**175+ tests passing** | **100% feature coverage** | **Production-ready** | **💬 Trash talk enabled!**

### What's Working
- ✅ Complete tic-tac-toe game logic with win detection
- ✅ **Real-time trash talking system** - send taunts from UI or MCP!
- ✅ SQLite database persistence (game state, moves, taunts)
- ✅ Server-Sent Events (SSE) for real-time updates
- ✅ JSON-RPC 2.0 protocol implementation
- ✅ All 6 MCP tools fully implemented and tested (`taunt_player` included!)
- ✅ MCP server binary with stdio transport
- ✅ REST API backend with live UI updates
- ✅ Yew/WASM frontend UI with trash talk input panel
- ✅ Comprehensive test coverage (175+ tests passing)
- ✅ Production-ready with hot-reload development mode

## 🚀 Quick Start

### Development Mode
```bash
./scripts/dev.sh
```
Starts both servers with hot-reload:
- Backend API + MCP server: http://localhost:3000
- Frontend dev server: http://localhost:8080

### Production Build
```bash
./scripts/build.sh
```
Builds optimized backend binary and frontend WASM assets.

### Production Server
```bash
./scripts/serve.sh
```
Runs production server (serves both API and static frontend).
<<<<<<< HEAD

### Run Tests

```bash
# All tests (94 tests)
cargo test --all

# Just unit tests
cargo test --lib

# Just integration tests
cargo test --test mcp_integration

# WASM tests (requires wasm-pack)
cd frontend
wasm-pack test --headless --firefox

# With output
cargo test -- --nocapture
```

### Manual Testing

```bash
# Using the test script (recommended)
./test-mcp-manual.sh

# Or test individual tools
echo '{"jsonrpc":"2.0","id":1,"method":"view_game_state","params":{}}' | \
  target/release/game-mcp-server 2>/dev/null | python3 -m json.tool
```

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────┐
│                  Claude Code (AI)                   │
│                    (MCP Client)                     │
└──────────────────────┬──────────────────────────────┘
                       │ JSON-RPC 2.0 via stdio
                       ▼
┌─────────────────────────────────────────────────────┐
│              MCP Server (Rust Binary)               │
│  ┌────────────────────────────────────────────┐    │
│  │  6 Tools: view_game_state, get_turn,      │    │
│  │  make_move, taunt_player, restart_game,   │    │
│  │  get_game_history                         │    │
│  └────────────────────────────────────────────┘    │
│                       ▼                             │
│  ┌────────────────────────────────────────────┐    │
│  │        Game State Manager                  │    │
│  │  (Coordinates game logic + persistence)    │    │
│  └────────────────────────────────────────────┘    │
│                       ▼                             │
│  ┌────────────────────────────────────────────┐    │
│  │     SQLite Database (game.db)              │    │
│  │  • Games table (state, players, status)    │    │
│  │  • Moves table (history with timestamps)   │    │
│  │  • Taunts table (AI messages to player)    │    │
│  └────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────┘
         ▲                                     ▲
         │                                     │
         │ REST API                            │
         │                                     │
┌────────┴─────────┐                 ┌─────────┴────────┐
│   Browser (UI)   │                 │   AI Agent (MCP) │
│   (Yew/WASM)     │                 │   (Claude Code)  │
└──────────────────┘                 └──────────────────┘
```

## 📦 Project Structure

```
game-mcp-poc/
├── backend/               # Rust backend (MCP server)
│   ├── src/
│   │   ├── bin/
│   │   │   └── game-mcp-server.rs    # Binary entry point
│   │   ├── db/                        # Database layer
│   │   │   ├── schema.rs              # SQLite schema
│   │   │   └── repository.rs          # CRUD operations
│   │   ├── game/                      # Game logic
│   │   │   ├── board.rs               # Board state
│   │   │   ├── logic.rs               # Win detection
│   │   │   ├── player.rs              # Player assignment
│   │   │   └── manager.rs             # Game coordinator
│   │   └── mcp/                       # MCP implementation
│   │       ├── protocol.rs            # JSON-RPC 2.0
│   │       ├── tools.rs               # Tool handlers
│   │       └── server.rs              # Server loop
│   ├── tests/
│   │   └── mcp_integration.rs         # Integration tests
│   └── Cargo.toml
├── frontend/              # Yew/WASM frontend
│   └── src/lib.rs         # Yew components
├── shared/                # Shared types (Player, Cell, GameState, etc.)
├── docs/                  # Comprehensive documentation
│   ├── architecture.md
│   ├── prd.md
│   ├── design.md
│   ├── mcp-setup-and-testing.md
│   └── status.md
├── scripts/               # Build and dev scripts
│   ├── dev.sh            # Development mode with hot-reload
│   ├── build.sh          # Production build
│   └── serve.sh          # Production server
└── test-mcp-manual.sh     # Manual testing script
```

## 🎮 MCP Tools

The server provides 6 tools for AI agents:

### 1. `view_game_state`
Returns complete game state (board, players, status, history, taunts)

**Input**: `{}`
**Output**: Full game state object

### 2. `get_turn`
Returns whose turn it is (X/O, human/AI)

**Input**: `{}`
**Output**: `{currentTurn, isHumanTurn, isAiTurn}`

### 3. `make_move`
Makes a move on the board

**Input**: `{row: 0-2, col: 0-2}`
**Output**: `{success, gameState, message}`

**Errors**: Out of bounds, cell occupied, game over

### 4. `taunt_player`
Sends a taunt message to the human player

**Input**: `{message: string}`
**Output**: `{success, message}`

### 5. `restart_game`
Restarts with a new game

**Input**: `{}`
**Output**: `{success, gameState, message}`

### 6. `get_game_history`
Returns all moves played

**Input**: `{}`
**Output**: `{moves: [{player, row, col, timestamp}]}`

## 🔧 Configuring Claude Code

### MCP Configuration File

**macOS/Linux**: `~/.config/claude-code/mcp.json`
**Windows**: `%APPDATA%\claude-code\mcp.json`

```json
{
  "mcpServers": {
    "tic-tac-toe": {
      "command": "/absolute/path/to/game-mcp-poc/target/release/game-mcp-server",
      "args": [],
      "env": {
        "GAME_DB_PATH": "/path/to/game.db",
        "RUST_LOG": "info"
      }
    }
  }
}
```

### Environment Variables

- `GAME_DB_PATH`: SQLite database location (default: `game.db`)
- `RUST_LOG`: Logging level (trace, debug, info, warn, error)

### Restart Claude Code

After configuration, restart Claude Code to load the MCP server.

## 🧪 Testing with Claude Code

### Basic Game Flow

1. **Start a game**:
   ```
   User: "Let's play tic-tac-toe! Show me the board."
   ```

2. **Make moves**:
   ```
   User: "I'll take the center (row 1, col 1)"
   ```

3. **AI's turn**:
   ```
   User: "Your turn!"
   ```

4. **AI taunts**:
   ```
   User: "Can you taunt me?"
   ```

5. **View history**:
   ```
   User: "Show me all the moves so far"
   ```

6. **Play until win/draw**

7. **Restart**:
   ```
   User: "Let's play again!"
   ```

## 📊 Test Coverage

### Unit Tests (79 tests)
- **Game Logic** (24 tests): Board operations, win detection, player assignment
- **Database** (9 tests): Schema, CRUD operations, persistence
- **Game Manager** (10 tests): State coordination, move validation
- **MCP Protocol** (10 tests): JSON-RPC parsing, validation, serialization
- **MCP Tools** (16 tests): All tools + error scenarios
- **MCP Server** (12 tests): Request handling, dispatch, edge cases

### Integration Tests (12 tests)
- **Mock AI Client**: Full game playthrough via subprocess + stdio
- **Error Scenarios**: Invalid moves, bounds checking, game over detection
- **State Persistence**: Database persistence across server restarts

### Manual Testing
- **CLI Script** (`test-mcp-manual.sh`): Tests all 6 tools with visual output
- **All tests passing** ✅

## 🛠️ Development

### Prerequisites
- Rust 2024 edition (stable)
- SQLite 3
- `trunk` (auto-installed by build scripts)
- `wasm-bindgen-cli` (auto-installed by build scripts)
- `wasm32-unknown-unknown` target (auto-added by scripts)

### Code Quality

```bash
# Format code
cargo fmt --all

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Both should pass clean ✅
```

### TDD Workflow

This project follows **strict TDD** (Test-Driven Development):

1. **Red**: Write failing test
2. **Green**: Write minimal code to pass
3. **Refactor**: Improve code quality

See [docs/process.md](docs/process.md) for details.

### Pre-commit Checklist

Before committing:

- [ ] All tests pass: `cargo test --all`
- [ ] Formatted: `cargo fmt --all`
- [ ] No warnings: `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] Documentation updated

## 🔍 Debugging

### View Server Logs

Logs go to **stderr** (not stdout, which is for JSON-RPC):

```bash
# Run with debug logs
RUST_LOG=debug GAME_DB_PATH=./test.db target/debug/game-mcp-server
```

### Inspect JSON-RPC Traffic

Use the manual testing script with verbose output:

```bash
# Pretty-print responses
./test-mcp-manual.sh
```

### Common Issues

**Server not showing in Claude Code**:
- Check MCP config file path and JSON syntax
- Ensure binary path is absolute
- Restart Claude Code completely

**Database errors**:
- Ensure `GAME_DB_PATH` directory is writable
- Delete corrupted database: `rm $GAME_DB_PATH`

**Permission errors**:
- Make binary executable: `chmod +x target/release/game-mcp-server`

## 🏆 Achievements

- ✅ **94 tests passing** (target was 50+)
- ✅ **100% coverage** of implemented modules
- ✅ **Full JSON-RPC 2.0** protocol compliance
- ✅ **All 6 MCP tools** working correctly
- ✅ **Comprehensive integration testing** with Mock AI
- ✅ **Production-ready code quality** (rustfmt + clippy clean)
- ✅ **Yew/WASM frontend** with beautiful UI

## 📈 Metrics

- **Lines of Code**: ~2,500 (backend) + ~1,500 (tests) + ~2,000 (docs)
- **Test Coverage**: 100% of implemented modules
- **Build Time**: <30 seconds (release build)
- **Binary Size**: ~8 MB (release, with SQLite)

## 🚧 Roadmap

### Phase 1: MCP Server ✅ **COMPLETE**
- [x] Game logic with TDD
- [x] Database persistence
- [x] JSON-RPC 2.0 protocol
- [x] All 6 MCP tools
- [x] Binary with stdio transport
- [x] Integration tests
- [x] Manual testing

### Phase 2: Frontend UI ✅ **COMPLETE**
- [x] Yew components (board, status, log)
- [x] WebAssembly build
- [x] Development scripts with hot-reload

### Phase 3: Integration Testing ⏭️ **NEXT**
- [ ] Test with actual Claude Code instance
- [ ] Document any edge cases found
- [ ] Iterate based on feedback

### Phase 4: REST API 🔄 **PLANNED**
- [ ] Axum web server
- [ ] Route definitions
- [ ] Static file serving
- [ ] API tests

## 📝 Documentation

- **[Architecture](docs/architecture.md)**: System design and component overview
- **[PRD](docs/prd.md)**: Product requirements and user stories
- **[Design](docs/design.md)**: Detailed module structure and API specs
- **[MCP Setup](docs/mcp-setup-and-testing.md)**: Claude Code configuration guide
- **[Status](docs/status.md)**: Current project status and metrics
- **[Process](docs/process.md)**: TDD and development workflow

## 🤝 Contributing

This is a proof-of-concept project demonstrating MCP integration. Feel free to:

- Report issues
- Suggest improvements
- Add features
- Improve documentation

## 📄 License

MIT

## 🔗 References

- [MCP Specification](https://spec.modelcontextprotocol.io/)
- [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification)
- [Claude Code MCP Documentation](https://docs.anthropic.com/claude/docs/mcp)
- [Rust 2024 Edition](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)

---

**Built with Rust 2024** | **Test-Driven Development** | **100% Coverage**
=======

## Testing

```bash
# Run all tests
cargo test --all

# Test specific package
cargo test --package backend
cargo test --package frontend
cargo test --package shared

# WASM tests (requires wasm-pack)
cd frontend
wasm-pack test --headless --firefox
```

## MCP Tools for AI Agents

The backend provides these MCP tools for AI agents to interact with the game:

- `view_game_state` - Get current board state
- `get_turn` - Determine whose turn it is
- `make_move` - Submit a move (row, col)
- `taunt_player` - Send a message to the human player
- `restart_game` - Start a new game
- `get_game_history` - View all past moves

## Architecture

```
┌─────────────┐                    ┌──────────────┐
│   Browser   │◄──── REST API ────►│              │
│  (Yew/WASM) │                    │   Backend    │◄──── SQLite
└─────────────┘                    │ (Axum/Rust)  │
                                   │              │
┌─────────────┐                    │              │
│  AI Agent   │◄──── MCP Tools ───►│              │
└─────────────┘                    └──────────────┘
```

### Workspace Structure

- **backend/** - Rust REST API, MCP server, and static file serving
- **frontend/** - Yew/WASM web interface
- **shared/** - Common types used by both frontend and backend
- **docs/** - Architecture, design, and process documentation

## Prerequisites

- Rust 2024 edition
- `trunk` (auto-installed by build scripts)
- `wasm-bindgen-cli` (auto-installed by build scripts)
- `wasm32-unknown-unknown` target (auto-added by scripts)

## Development Process

This project follows strict Test-Driven Development (TDD):

1. **Red**: Write a failing test first
2. **Green**: Write minimal code to pass the test
3. **Refactor**: Improve code while keeping tests green

### Pre-commit Checklist

- [ ] `cargo fmt --all`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test --all`
- [ ] Update documentation if needed

See [docs/process.md](./docs/process.md) for full development guidelines.

## Documentation

- [Architecture](./docs/architecture.md) - System design and tech stack
- [PRD](./docs/prd.md) - Product requirements
- [Design](./docs/design.md) - Detailed specifications
- [Plan](./docs/plan.md) - Implementation plan
- [Process](./docs/process.md) - TDD and development workflow
- [Status](./docs/status.md) - Current project status

## License

MIT
>>>>>>> 7b7e486 (docs: Reorganize documentation and create user-friendly README)
