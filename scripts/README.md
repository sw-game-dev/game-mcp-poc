# Scripts Directory

This directory contains build, development, and testing scripts for the Tic-Tac-Toe MCP Game.

## Build and Development Scripts

### `build.sh`
Builds the entire project (backend + frontend) for production.

```bash
./scripts/build.sh
```

- Installs required tools (`trunk`, `wasm-bindgen-cli`) if missing
- Adds WASM target if not present
- Builds backend in release mode
- Builds frontend WASM bundle

### `dev.sh`
Starts both backend and frontend development servers with hot-reload.

```bash
./scripts/dev.sh
```

- Backend: http://localhost:7397 (REST API + static files)
- Frontend: http://localhost:8080 (Trunk dev server with hot-reload)
- Uses in-memory database for development

### `serve.sh`
Runs the production build (backend serving static frontend).

```bash
./scripts/serve.sh
```

Serves the complete application at http://localhost:7397.

## AI Agent Scripts

These scripts demonstrate the MCP (Model Context Protocol) interface by simulating an AI agent playing tic-tac-toe.

### `ai_agent_simple.py` ⭐ Recommended
Generates a complete sequence of JSON-RPC commands for an AI agent to play the game.

```bash
# Generate game commands (pipe to MCP server)
python3 scripts/ai_agent_simple.py | GAME_DB_PATH=":memory:" ./target/release/game-mcp-server

# With verbose logging
python3 scripts/ai_agent_simple.py --verbose | GAME_DB_PATH=":memory:" ./target/release/game-mcp-server
```

**How it works:**
- Generates JSON-RPC requests for all MCP tools
- Makes random moves until game ends
- Sends taunts occasionally
- Demonstrates: `view_game_state`, `get_turn`, `make_move`, `taunt_player`, `get_game_history`

### `run_ai_agent.sh` ⭐ Easiest Way
Wrapper script that runs the AI agent with pretty output.

```bash
# Run with formatted output
./scripts/run_ai_agent.sh

# Run with verbose agent logging
./scripts/run_ai_agent.sh --verbose

# Save raw JSON output to file
./scripts/run_ai_agent.sh --output game_output.json
```

**Output:**
- Shows each tool call with status (✓ success / ✗ error)
- Displays game status updates
- Summarizes what the agent demonstrated

### `test_mcp_agent.sh`
Simple test script that sends a few JSON-RPC commands to verify the MCP server.

```bash
./scripts/test_mcp_agent.sh
```

Sends basic commands (view_game_state, get_turn, make_move) and shows raw JSON responses.

### `ai_agent.py` (Experimental)
Interactive AI agent that attempts bidirectional communication with MCP server. Currently not fully functional due to stdin/stdout pipe limitations. Kept for reference.

### `play_with_agent.sh` (Experimental)
Attempts to run the interactive agent. Not fully functional yet.

## MCP Tools Available

The AI agent can call these MCP tools via JSON-RPC 2.0:

| Tool | Parameters | Description |
|------|-----------|-------------|
| `view_game_state` | - | Get complete game state including board, status, moves, taunts |
| `get_turn` | - | Check whose turn it is (human/AI) |
| `make_move` | `{row, col}` | Make a move at the specified position |
| `taunt_player` | `{message}` | Send a taunt message to the opponent |
| `restart_game` | - | Start a new game |
| `get_game_history` | - | Get all moves made in the current game |

## Examples

### Quick AI Agent Demo
```bash
# Build the project
./scripts/build.sh

# Run AI agent demonstration
./scripts/run_ai_agent.sh
```

### Manual MCP Testing
```bash
# Start MCP server (reads from stdin, writes to stdout)
GAME_DB_PATH=":memory:" ./target/release/game-mcp-server

# In another terminal, send JSON-RPC commands:
echo '{"jsonrpc":"2.0","id":1,"method":"view_game_state","params":{}}' | \
  GAME_DB_PATH=":memory:" ./target/release/game-mcp-server
```

### Development Workflow
```bash
# Start dev servers
./scripts/dev.sh

# In another terminal, run AI agent
./scripts/run_ai_agent.sh

# The AI will play against the game state that's visible in the browser
```

## JSON-RPC 2.0 Format

All MCP tool calls use JSON-RPC 2.0 format:

**Request:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "make_move",
  "params": {"row": 0, "col": 0}
}
```

**Success Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "success": true,
    "message": "Move made successfully",
    "gameState": { ... }
  }
}
```

**Error Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32602,
    "message": "Cell already occupied"
  }
}
```

## Requirements

- **Python 3**: For AI agent scripts
- **jq** (optional): For pretty-printing JSON output in `run_ai_agent.sh`
- **Rust toolchain**: For building the MCP server
- **trunk**: For building the frontend (auto-installed by build scripts)

## Troubleshooting

### "MCP server binary not found"
Run `./scripts/build.sh` to build the project first.

### "Python: command not found"
Install Python 3, or use `python3` explicitly.

### AI agent doesn't make moves
Check that you're using the right script:
- Use `ai_agent_simple.py` (works) instead of `ai_agent.py` (experimental)

### Seeing JSON parse errors
This usually means stderr and stdout are mixed. Use the wrapper script (`run_ai_agent.sh`) which handles this correctly.
