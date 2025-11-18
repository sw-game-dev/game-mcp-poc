# MCP Implementation Roadmap

## Status: ⚠️ NOT IMPLEMENTED

**Critical Finding**: The MCP server and tests are completely missing from the codebase.

## What Exists

- ✅ Empty directory: `backend/src/mcp/`
- ✅ Game logic (board, moves, win detection)
- ✅ Database layer (persistence)
- ✅ Shared types (GameState, Player, etc.)

## What's Missing

### 1. MCP Protocol Implementation

**File**: `backend/src/mcp/protocol.rs`

```rust
// Needs to be created
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,  // Must be "2.0"
    pub id: Value,        // Request ID
    pub method: String,   // Tool name
    pub params: Value,    // Tool parameters
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Value,
    pub result: Option<Value>,
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}

// Error codes per JSON-RPC 2.0
pub const PARSE_ERROR: i32 = -32700;
pub const INVALID_REQUEST: i32 = -32600;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;
pub const INTERNAL_ERROR: i32 = -32603;

// Tests needed:
// - test_parse_valid_request
// - test_parse_invalid_json
// - test_serialize_response
// - test_serialize_error
// - test_error_codes
```

**Status**: 0 tests, 0 lines of code ❌

### 2. MCP Server Core

**File**: `backend/src/mcp/server.rs`

```rust
// Needs to be created
use std::io::{self, BufRead, Write};
use super::protocol::*;
use super::tools::*;

pub struct McpServer {
    game_manager: Arc<Mutex<GameManager>>,
}

impl McpServer {
    pub fn new(db_path: &str) -> Result<Self, GameError>;

    pub fn run(&mut self) -> Result<(), GameError> {
        // Read JSON-RPC from stdin
        // Parse request
        // Dispatch to tool handler
        // Write JSON-RPC response to stdout
    }

    fn dispatch(&self, method: &str, params: Value)
        -> Result<Value, JsonRpcError>;
}

// Tests needed:
// - test_server_initialization
// - test_dispatch_to_tool
// - test_unknown_method
// - test_invalid_params
// - test_stdin_stdout_communication
// - test_multiple_requests
```

**Status**: 0 tests, 0 lines of code ❌

### 3. Game State Manager

**File**: `backend/src/game/manager.rs`

```rust
// Needs to be created
use super::board::Board;
use super::logic::*;
use super::player::*;
use crate::db::repository::GameRepository;

pub struct GameManager {
    current_game_id: Option<String>,
    repository: GameRepository,
}

impl GameManager {
    pub fn new(db_path: &str) -> Result<Self, GameError>;

    pub fn get_or_create_game(&mut self) -> Result<&GameState, GameError>;

    pub fn make_move(&mut self, row: u8, col: u8)
        -> Result<GameState, GameError>;

    pub fn restart_game(&mut self) -> Result<GameState, GameError>;

    pub fn add_taunt(&mut self, message: String)
        -> Result<(), GameError>;

    pub fn get_game_state(&self) -> Result<GameState, GameError>;
}

// Tests needed:
// - test_create_new_game
// - test_get_existing_game
// - test_make_valid_move
// - test_make_invalid_move
// - test_restart_game
// - test_add_taunt
// - test_game_state_persistence
// - test_thread_safety (if using Arc<Mutex<>>)
```

**Status**: 0 tests, 0 lines of code ❌

### 4. MCP Tool Handlers

**File**: `backend/src/mcp/tools.rs`

```rust
// Needs to be created
use serde_json::Value;
use super::protocol::JsonRpcError;
use crate::game::manager::GameManager;

pub fn view_game_state(
    manager: &GameManager,
    params: Value,
) -> Result<Value, JsonRpcError>;

pub fn get_turn(
    manager: &GameManager,
    params: Value,
) -> Result<Value, JsonRpcError>;

pub fn make_move(
    manager: &mut GameManager,
    params: Value,
) -> Result<Value, JsonRpcError>;

pub fn taunt_player(
    manager: &mut GameManager,
    params: Value,
) -> Result<Value, JsonRpcError>;

pub fn restart_game(
    manager: &mut GameManager,
    params: Value,
) -> Result<Value, JsonRpcError>;

pub fn get_game_history(
    manager: &GameManager,
    params: Value,
) -> Result<Value, JsonRpcError>;

// Tests needed (6 tools × 3-4 tests each = 18-24 tests):
// For each tool:
// - test_<tool>_success
// - test_<tool>_invalid_params
// - test_<tool>_error_handling
// - test_<tool>_edge_cases
```

**Status**: 0 tests, 0 lines of code ❌

### 5. MCP Binary Entry Point

**File**: `backend/src/bin/game-mcp-server.rs`

```rust
// Needs to be created
use game_mcp_server::mcp::server::McpServer;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Get database path from env
    let db_path = env::var("GAME_DB_PATH")
        .unwrap_or_else(|_| "./game.db".to_string());

    // Create and run server
    let mut server = McpServer::new(&db_path)?;
    server.run()?;

    Ok(())
}
```

**Status**: Not created ❌

### 6. Mock AI Client

**File**: `tests/mock_ai.rs` or `tests/mock-ai/main.rs`

```rust
// Needs to be created
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Write};
use serde_json::json;

struct MockAiClient {
    server_process: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl MockAiClient {
    pub fn new() -> Result<Self, Error>;

    pub fn call_tool(&mut self, method: &str, params: Value)
        -> Result<Value, Error>;

    pub fn play_game(&mut self) -> Result<(), Error> {
        // Simulate a full game:
        // 1. View initial state
        // 2. Make moves
        // 3. Send taunts
        // 4. Play until game over
        // 5. Restart
    }
}

// Tests needed:
// - test_client_connects
// - test_view_initial_state
// - test_make_valid_moves
// - test_handle_errors
// - test_full_game_playthrough
// - test_restart_game
// - test_multiple_games
```

**Status**: 0 tests, 0 lines of code ❌

## Test Coverage Goals

### Current Coverage
- **Backend**: 32 tests (game logic + database)
- **MCP**: 0 tests ❌
- **Integration**: 0 tests ❌

### Target Coverage for MVP
- **MCP Protocol**: 5 tests minimum
- **MCP Server**: 6 tests minimum
- **Game Manager**: 8 tests minimum
- **MCP Tools**: 18 tests minimum (3 per tool)
- **Mock AI**: 7 tests minimum
- **Integration**: 5 tests minimum

**Total MCP-related tests needed**: ~50 tests

## Implementation Priority

### Phase 1: Foundation (High Priority)
1. **Game Manager** - Core state management
   - Estimated: 1-2 hours
   - Tests: 8
   - Blockers: None

2. **MCP Protocol** - JSON-RPC handling
   - Estimated: 1 hour
   - Tests: 5
   - Blockers: None

### Phase 2: Tools (High Priority)
3. **MCP Tools** - All 6 tool handlers
   - Estimated: 2-3 hours
   - Tests: 18-24
   - Blockers: Game Manager, Protocol

### Phase 3: Server (High Priority)
4. **MCP Server** - Main server loop
   - Estimated: 1-2 hours
   - Tests: 6
   - Blockers: Protocol, Tools, Game Manager

5. **Binary Entry Point** - Executable
   - Estimated: 30 minutes
   - Tests: Manual
   - Blockers: MCP Server

### Phase 4: Testing (Critical)
6. **Mock AI Client** - Integration testing
   - Estimated: 2 hours
   - Tests: 7
   - Blockers: MCP Server running

7. **End-to-End Tests** - Full system validation
   - Estimated: 1-2 hours
   - Tests: 5
   - Blockers: Mock AI, MCP Server

**Total Estimated Time**: 8-12 hours

## Dependencies to Add

Update `backend/Cargo.toml`:

```toml
[dependencies]
# ... existing dependencies ...
serde_json = "1.0"

# For binary
[[bin]]
name = "game-mcp-server"
path = "src/bin/game-mcp-server.rs"
```

## Validation Checklist

Before considering MCP implementation complete:

- [ ] All MCP protocol tests pass
- [ ] All MCP server tests pass
- [ ] All game manager tests pass
- [ ] All MCP tool tests pass
- [ ] Mock AI can play a full game
- [ ] Integration tests pass
- [ ] Binary builds successfully
- [ ] Manual testing with echo/pipe works
- [ ] Claude Code can connect and use tools
- [ ] State persists across server restarts
- [ ] Error handling is robust
- [ ] Logging provides useful debug info

## Current Risk Assessment

### Critical Risks
1. **No MCP code exists** - Complete implementation needed
2. **No integration tests** - Can't verify end-to-end functionality
3. **Untested with Claude Code** - Unknown if it will work

### Mitigation Strategy
1. **Implement incrementally** - Test each layer before moving on
2. **Start with Game Manager** - Foundation for everything else
3. **Write tests first** - TDD approach reduces bugs
4. **Manual testing early** - Verify JSON-RPC with echo/jq
5. **Use MCP Inspector** - Debug protocol issues
6. **Test with Claude Code ASAP** - Validate real-world usage

## Success Criteria

MCP implementation is complete when:

✅ All 50+ MCP tests pass
✅ Mock AI can play multiple games successfully
✅ Manual testing with CLI tools works
✅ Claude Code can connect and use all tools
✅ Game state persists correctly
✅ Error messages are helpful
✅ Logging helps debug issues
✅ Code passes rustfmt and clippy
✅ Documentation is updated

## Next Immediate Steps

1. **Create Game Manager** (`backend/src/game/manager.rs`)
   - Write 8 unit tests first (TDD)
   - Implement game state management
   - Test with existing database layer

2. **Create MCP Protocol** (`backend/src/mcp/protocol.rs`)
   - Define JSON-RPC structs
   - Write 5 unit tests
   - Test parsing and serialization

3. **Create MCP Tools** (`backend/src/mcp/tools.rs`)
   - Implement 6 tool handlers
   - Write 3-4 tests per tool (18-24 tests)
   - Use Game Manager

4. **Create MCP Server** (`backend/src/mcp/server.rs`)
   - Implement stdio transport
   - Write 6 unit tests
   - Connect protocol, tools, and manager

5. **Create Binary** (`backend/src/bin/game-mcp-server.rs`)
   - Main entry point
   - Environment variable handling
   - Logging setup

6. **Test Manually**
   - Use echo and jq to test tools
   - Verify JSON-RPC compliance
   - Check error handling

7. **Create Mock AI**
   - Build test client
   - Simulate full game
   - Write 7 integration tests

8. **Test with Claude Code**
   - Configure MCP settings
   - Run end-to-end scenarios
   - Document any issues

## Time Estimate Summary

- **Development**: 8-12 hours
- **Testing**: 2-3 hours
- **Documentation**: 1 hour
- **Debugging**: 2-3 hours (buffer)

**Total**: 13-19 hours to fully working MCP server with comprehensive tests
