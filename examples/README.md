# AI Agent Examples

This directory contains example scripts showing how different AI platforms can interact with the TTTTT MCP server.

## Quick Start

1. **Start the server:**
   ```bash
   cd ..
   ./scripts/serve.sh
   ```

2. **Choose your AI platform** and follow the instructions below.

---

## Claude Desktop (Recommended for MCP)

Claude Desktop has native MCP support via stdio transport.

### Setup

1. Build the MCP server binary:
   ```bash
   cargo build --release --bin game-mcp-server
   ```

2. Get the full path to your binary:
   ```bash
   realpath target/release/game-mcp-server
   ```

3. Edit your Claude Desktop config file:
   - **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
   - **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
   - **Linux**: `~/.config/Claude/claude_desktop_config.json`

4. Add this configuration (replace paths with your actual paths):
   ```json
   {
     "mcpServers": {
       "tictactoe": {
         "command": "/full/path/to/game-mcp-poc/target/release/game-mcp-server",
         "env": {
           "GAME_DB_PATH": "/full/path/to/game-mcp-poc/game.db"
         }
       }
     }
   }
   ```

5. Restart Claude Desktop

### Usage

In Claude Desktop, you can now ask:
- "Let's play tic-tac-toe!"
- "Make a move and trash talk me"
- "Check the game state"
- "Restart the game"

The tools will appear in Claude's interface when the MCP server is connected.

---

## OpenAI GPT-4

Uses OpenAI's function calling API with the HTTP MCP endpoint.

### Setup

1. Install dependencies:
   ```bash
   pip install openai requests
   ```

2. Get your OpenAI API key from https://platform.openai.com/api-keys

3. Set your API key:
   ```bash
   export OPENAI_API_KEY="sk-..."
   ```

### Usage

```bash
python3 examples/openai_agent.py
```

The agent will:
- Check the game state
- Make strategic moves
- Send trash talk messages
- Play until the game ends

### Example Output

```
🤖 Starting OpenAI agent...
============================================================

--- Turn 1 ---

🎮 Calling view_game_state with args: {}
✅ Result: {
  "board": [...],
  "currentTurn": "X",
  ...
}

--- Turn 2 ---

🎮 Calling make_move with args: {"row": 1, "col": 1}
✅ Result: {
  "success": true,
  "message": "Move made successfully"
}

--- Turn 3 ---

🎮 Calling taunt_player with args: {"message": "Center square is mine! Your move!"}
✅ Result: {
  "success": true
}
```

---

## Google Gemini

Uses Gemini's function calling API with the HTTP MCP endpoint.

### Setup

1. Install dependencies:
   ```bash
   pip install google-generativeai requests
   ```

2. Get your Google AI API key from https://makersuite.google.com/app/apikey

3. Set your API key:
   ```bash
   export GOOGLE_API_KEY="AIza..."
   ```

### Usage

```bash
python3 examples/gemini_agent.py
```

The agent will:
- View the game board
- Make moves strategically
- Trash talk the opponent
- Continue until game over

### Example Output

```
🤖 Starting Gemini agent...
============================================================

💬 User: Let's play tic-tac-toe! ...

--- Turn 1 ---

🎮 Calling view_game_state with args: {}
✅ Result: {...}

--- Turn 2 ---

🎮 Calling make_move with args: {'row': 0, 'col': 0}
✅ Result: {...}

💬 Gemini says: I've taken the top-left corner! Your move, if you dare!
```

---

## Testing MCP Endpoints

You can test the MCP server directly with curl:

### Initialize

```bash
curl -X POST http://localhost:3000/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"initialize","params":{},"id":1}' \
  | jq
```

### List Available Tools

```bash
curl -X POST http://localhost:3000/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"tools/list","params":{},"id":2}' \
  | jq
```

### View Game State

```bash
curl -X POST http://localhost:3000/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"view_game_state","params":{},"id":3}' \
  | jq
```

### Make a Move

```bash
curl -X POST http://localhost:3000/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"make_move","params":{"row":1,"col":1},"id":4}' \
  | jq
```

### Send a Taunt

```bash
curl -X POST http://localhost:3000/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"taunt_player","params":{"message":"Nice try!"},"id":5}' \
  | jq
```

---

## Available MCP Tools

All platforms have access to these tools:

| Tool | Description | Parameters |
|------|-------------|------------|
| `view_game_state` | Get current board, turn, status, and history | None |
| `get_turn` | Check whose turn it is (X or O) | None |
| `make_move` | Place your mark on the board | `row` (0-2), `col` (0-2) |
| `taunt_player` | Send trash talk to opponent | `message` (string) |
| `restart_game` | Start a fresh game | None |
| `get_game_history` | View all moves made | None |

---

## Troubleshooting

### Server Not Running
```
❌ Error: Connection refused
```
**Solution:** Start the server with `./scripts/serve.sh`

### API Key Issues
```
❌ Error: Invalid API key
```
**Solution:** Check your API key is set correctly:
```bash
echo $OPENAI_API_KEY  # or GOOGLE_API_KEY
```

### Claude Desktop Not Showing Tools
**Solution:**
1. Check the config file path is correct
2. Verify the binary path is absolute
3. Restart Claude Desktop
4. Check Claude Desktop logs for errors

### MCP Protocol Errors
```
❌ MCP Error: Method 'xyz' not found
```
**Solution:** Use `tools/list` to see available methods

---

## Architecture

```
┌─────────────────────┐
│   AI Agent          │
│  (Claude/GPT/Gemini)│
└──────────┬──────────┘
           │
           │ HTTP or stdio
           │
┌──────────▼──────────┐
│   MCP Server        │
│  (JSON-RPC 2.0)     │
└──────────┬──────────┘
           │
           │
┌──────────▼──────────┐
│   Game Logic        │
│  + SQLite DB        │
└─────────────────────┘
```

The MCP server supports both:
- **stdio** transport for Claude Desktop
- **HTTP** transport for custom integrations

All game state is shared via SQLite, so multiple clients can interact with the same game!

---

## Next Steps

- Try playing a game with each platform
- Compare the trash talk quality 😈
- Build your own integration
- Add more sophisticated strategy logic

For more info, see the main README.md
