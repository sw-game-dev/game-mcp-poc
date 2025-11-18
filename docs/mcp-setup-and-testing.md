# MCP Server Setup and Testing Guide

## Overview

This document describes how to configure Claude Code to use the tic-tac-toe MCP server and how to test it end-to-end.

## Current Status

⚠️ **WARNING**: The MCP server is **NOT YET IMPLEMENTED**. This document describes how it will work once implemented.

## MCP Server Architecture

### Transport Protocol
The MCP server will use **stdio** (standard input/output) for communication with Claude Code, following the JSON-RPC 2.0 protocol.

### Server Binary
- **Location**: `backend/target/release/game-mcp-server` (or `game-mcp-server.exe` on Windows)
- **Purpose**: Standalone MCP server that manages game state and provides tools

### Communication Flow
```
Claude Code (Client)
        ↓ stdin (JSON-RPC requests)
        ↓
   MCP Server (Rust)
        ↓
   Game State Manager
        ↓
   SQLite Database
```

## MCP Tools Specification

### 1. view_game_state
**Description**: View the current tic-tac-toe game state

**Input Schema**:
```json
{
  "type": "object",
  "properties": {},
  "required": []
}
```

**Output**:
```json
{
  "id": "game-uuid",
  "board": [
    ["X", null, "O"],
    [null, "X", null],
    ["O", null, null]
  ],
  "currentTurn": "X",
  "humanPlayer": "X",
  "aiPlayer": "O",
  "status": "InProgress",
  "moveHistory": [...],
  "taunts": ["Nice try!", "Is that the best you've got?"]
}
```

### 2. get_turn
**Description**: Get whose turn it is (X or O, human or AI)

**Input Schema**:
```json
{
  "type": "object",
  "properties": {},
  "required": []
}
```

**Output**:
```json
{
  "currentTurn": "X",
  "isHumanTurn": true,
  "isAiTurn": false
}
```

### 3. make_move
**Description**: Make a move on the board

**Input Schema**:
```json
{
  "type": "object",
  "properties": {
    "row": {
      "type": "number",
      "description": "Row index (0-2)",
      "minimum": 0,
      "maximum": 2
    },
    "col": {
      "type": "number",
      "description": "Column index (0-2)",
      "minimum": 0,
      "maximum": 2
    }
  },
  "required": ["row", "col"]
}
```

**Output**:
```json
{
  "success": true,
  "gameState": { /* full game state */ },
  "message": "Move made successfully"
}
```

**Error Responses**:
- Cell already occupied
- Out of bounds
- Not AI's turn
- Game already over

### 4. taunt_player
**Description**: Send a taunt message to the human player

**Input Schema**:
```json
{
  "type": "object",
  "properties": {
    "message": {
      "type": "string",
      "description": "The taunt message"
    }
  },
  "required": ["message"]
}
```

**Output**:
```json
{
  "success": true,
  "message": "Taunt sent successfully"
}
```

### 5. restart_game
**Description**: Restart the game with a new board

**Input Schema**:
```json
{
  "type": "object",
  "properties": {},
  "required": []
}
```

**Output**:
```json
{
  "success": true,
  "gameState": { /* new game state */ },
  "message": "Game restarted"
}
```

### 6. get_game_history
**Description**: View the history of moves

**Input Schema**:
```json
{
  "type": "object",
  "properties": {},
  "required": []
}
```

**Output**:
```json
{
  "moves": [
    {"player": "X", "row": 0, "col": 0, "timestamp": 1234567890},
    {"player": "O", "row": 1, "col": 1, "timestamp": 1234567891}
  ]
}
```

## Claude Code Configuration

### MCP Server Registration

Claude Code looks for MCP server configurations in specific locations:

**macOS/Linux**: `~/.config/claude-code/mcp.json`
**Windows**: `%APPDATA%\claude-code\mcp.json`

### Configuration File Format

Create or edit the MCP configuration file:

```json
{
  "mcpServers": {
    "tic-tac-toe": {
      "command": "/path/to/game-mcp-poc/backend/target/release/game-mcp-server",
      "args": [],
      "env": {
        "GAME_DB_PATH": "/path/to/game-mcp-poc/game.db",
        "RUST_LOG": "info"
      }
    }
  }
}
```

### Configuration Options

- **command**: Absolute path to the MCP server binary
- **args**: Command-line arguments (none needed for this server)
- **env**: Environment variables
  - `GAME_DB_PATH`: Where to store the SQLite database
  - `RUST_LOG`: Logging level (trace, debug, info, warn, error)

### Development Configuration

For development, use the debug binary:

```json
{
  "mcpServers": {
    "tic-tac-toe-dev": {
      "command": "/path/to/game-mcp-poc/backend/target/debug/game-mcp-server",
      "args": [],
      "env": {
        "GAME_DB_PATH": "/tmp/tictactoe-dev.db",
        "RUST_LOG": "debug"
      }
    }
  }
}
```

## Building the MCP Server

### Development Build
```bash
cd backend
cargo build
```
Binary location: `backend/target/debug/game-mcp-server`

### Release Build
```bash
cd backend
cargo build --release
```
Binary location: `backend/target/release/game-mcp-server`

### Running Standalone (for testing)
```bash
# The server reads JSON-RPC from stdin and writes to stdout
echo '{"jsonrpc":"2.0","id":1,"method":"view_game_state","params":{}}' | \
  GAME_DB_PATH=./test.db ./backend/target/debug/game-mcp-server
```

## Testing the MCP Server

### Unit Tests

Run all MCP server tests:
```bash
cd backend
cargo test mcp::
```

Run specific test:
```bash
cargo test mcp::tests::test_view_game_state
```

### Integration Tests with Mock AI

The mock AI client simulates Claude Code making tool calls:

```bash
cd tests
cargo run --bin mock-ai
```

This will:
1. Start a new game
2. View the game state
3. Make moves as the AI
4. Send taunts
5. Play until game over
6. Restart and play again

### Manual Testing with `jq`

Test individual tools using `jq` for pretty output:

#### View Game State
```bash
echo '{"jsonrpc":"2.0","id":1,"method":"view_game_state","params":{}}' | \
  GAME_DB_PATH=./test.db ./backend/target/debug/game-mcp-server | jq
```

#### Make a Move
```bash
echo '{"jsonrpc":"2.0","id":2,"method":"make_move","params":{"row":0,"col":0}}' | \
  GAME_DB_PATH=./test.db ./backend/target/debug/game-mcp-server | jq
```

#### Send a Taunt
```bash
echo '{"jsonrpc":"2.0","id":3,"method":"taunt_player","params":{"message":"You call that a move?"}}' | \
  GAME_DB_PATH=./test.db ./backend/target/debug/game-mcp-server | jq
```

#### Restart Game
```bash
echo '{"jsonrpc":"2.0","id":4,"method":"restart_game","params":{}}' | \
  GAME_DB_PATH=./test.db ./backend/target/debug/game-mcp-server | jq
```

### Testing with MCP Inspector

The MCP Inspector is a tool for testing MCP servers interactively:

```bash
# Install MCP Inspector (if not already installed)
npm install -g @modelcontextprotocol/inspector

# Run the inspector
mcp-inspector ./backend/target/debug/game-mcp-server
```

This opens a web UI where you can:
- View available tools
- Test tool calls interactively
- Inspect JSON-RPC messages
- Debug server responses

## End-to-End Testing with Claude Code

### Prerequisites
1. Build the MCP server (release build recommended)
2. Configure Claude Code MCP settings (see above)
3. Restart Claude Code to load the new MCP server

### Test Scenarios

#### Scenario 1: Basic Game Flow

1. **Start Claude Code** and open a new conversation

2. **Check if MCP server is loaded**:
   - Claude Code should show "tic-tac-toe" in available tools
   - You may need to check the MCP settings/tools panel

3. **Ask Claude to start a game**:
   ```
   User: "Let's play tic-tac-toe! Start a new game and show me the board."
   ```

   Expected: Claude should use `view_game_state` or `restart_game` and show the board

4. **Make a move**:
   ```
   User: "I'll take the center (row 1, col 1)"
   ```

   Expected: Claude should use `make_move` with params `{"row": 1, "col": 1}`

5. **AI makes a move**:
   ```
   User: "Your turn!"
   ```

   Expected: Claude should use `get_turn` to verify it's the AI's turn, then `make_move`

6. **AI sends taunt**:
   ```
   User: "Can you taunt me?"
   ```

   Expected: Claude should use `taunt_player` with a creative message

7. **View game history**:
   ```
   User: "Show me all the moves so far"
   ```

   Expected: Claude should use `get_game_history`

8. **Complete the game**:
   Continue playing until someone wins or it's a draw

   Expected: Claude should detect game over and announce the result

9. **Restart**:
   ```
   User: "Let's play again!"
   ```

   Expected: Claude should use `restart_game`

#### Scenario 2: Error Handling

1. **Invalid move**:
   ```
   User: "Make a move at row 5, col 5"
   ```

   Expected: Claude should receive an error and explain it's out of bounds

2. **Occupied cell**:
   Make a move, then try the same cell again

   Expected: Claude should receive error and choose a different cell

3. **Wrong turn**:
   Try to make a move when it's not the AI's turn

   Expected: Claude should receive error and wait for human's turn

#### Scenario 3: State Persistence

1. **Start a game and make some moves**
2. **Exit Claude Code** (close the application)
3. **Restart Claude Code**
4. **Ask Claude**: "What's the current game state?"

   Expected: Game state should be restored from database

### Debugging End-to-End Tests

#### Check MCP Server Logs

If using `RUST_LOG=debug`, logs will be written to stderr:

```bash
# On macOS/Linux, Claude Code may log to:
~/Library/Logs/claude-code/mcp-tic-tac-toe.log

# Or check system logs:
journalctl -f | grep game-mcp-server
```

#### Common Issues

**Issue**: MCP server not showing in Claude Code
- **Fix**: Check MCP config file path and JSON syntax
- **Fix**: Ensure binary path is absolute and executable
- **Fix**: Restart Claude Code completely

**Issue**: "Command not found" error
- **Fix**: Verify the binary exists at the specified path
- **Fix**: Check file permissions: `chmod +x /path/to/game-mcp-server`

**Issue**: Database errors
- **Fix**: Ensure `GAME_DB_PATH` directory is writable
- **Fix**: Delete corrupted database: `rm $GAME_DB_PATH`

**Issue**: Tool calls failing
- **Fix**: Check tool schema matches implementation
- **Fix**: Verify JSON-RPC 2.0 protocol compliance
- **Fix**: Review server logs for error details

#### Viewing MCP Protocol Traffic

To see the actual JSON-RPC messages:

1. **Enable debug logging** in MCP config:
   ```json
   "env": {
     "RUST_LOG": "debug"
   }
   ```

2. **Use a protocol sniffer**:
   ```bash
   # Create a wrapper script that logs traffic
   #!/bin/bash
   tee /tmp/mcp-input.log | \
     /path/to/game-mcp-server | \
     tee /tmp/mcp-output.log
   ```

3. **Point Claude Code** to the wrapper script instead

## Automated Testing Script

Create a test script for CI/CD:

```bash
#!/bin/bash
# test-mcp-e2e.sh

set -e

echo "Building MCP server..."
cd backend
cargo build --release

echo "Running unit tests..."
cargo test

echo "Testing MCP protocol..."
cd ..

# Test view_game_state
echo "Testing view_game_state..."
RESPONSE=$(echo '{"jsonrpc":"2.0","id":1,"method":"view_game_state","params":{}}' | \
  GAME_DB_PATH=/tmp/test-$(date +%s).db \
  ./backend/target/release/game-mcp-server)

echo "$RESPONSE" | jq -e '.result' > /dev/null || {
  echo "ERROR: view_game_state failed"
  echo "$RESPONSE"
  exit 1
}

# Test make_move
echo "Testing make_move..."
RESPONSE=$(echo '{"jsonrpc":"2.0","id":2,"method":"make_move","params":{"row":0,"col":0}}' | \
  GAME_DB_PATH=/tmp/test-$(date +%s).db \
  ./backend/target/release/game-mcp-server)

echo "$RESPONSE" | jq -e '.result' > /dev/null || {
  echo "ERROR: make_move failed"
  echo "$RESPONSE"
  exit 1
}

echo "All MCP tests passed!"
```

Make it executable:
```bash
chmod +x test-mcp-e2e.sh
./test-mcp-e2e.sh
```

## Performance Testing

Test server performance under load:

```bash
# Send 100 requests rapidly
for i in {1..100}; do
  echo '{"jsonrpc":"2.0","id":'$i',"method":"view_game_state","params":{}}' | \
    GAME_DB_PATH=/tmp/perf-test.db ./backend/target/release/game-mcp-server
done
```

## Security Considerations

1. **Database Path**: Always validate `GAME_DB_PATH` environment variable
2. **Input Validation**: All JSON-RPC parameters must be validated
3. **Resource Limits**: Prevent infinite loops or resource exhaustion
4. **Error Messages**: Don't leak sensitive information in errors
5. **File Permissions**: Database should not be world-readable

## Next Steps

Once MCP server is implemented:

1. ✅ Build the server
2. ✅ Run unit tests
3. ✅ Run mock AI integration test
4. ✅ Configure Claude Code MCP settings
5. ✅ Test with Claude Code end-to-end
6. ✅ Document any issues found
7. ✅ Iterate and improve

## Reference Links

- [MCP Specification](https://spec.modelcontextprotocol.io/)
- [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification)
- [Claude Code MCP Documentation](https://docs.anthropic.com/claude/docs/mcp)
- [MCP Inspector Tool](https://github.com/modelcontextprotocol/inspector)
