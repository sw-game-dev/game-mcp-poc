# Design Document

## Module Structure

```
game-mcp-poc/
├── Cargo.toml (workspace)
├── backend/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── game/
│       │   ├── mod.rs
│       │   ├── board.rs
│       │   ├── player.rs
│       │   └── logic.rs
│       ├── db/
│       │   ├── mod.rs
│       │   ├── schema.rs
│       │   └── repository.rs
│       ├── api/
│       │   ├── mod.rs
│       │   ├── routes.rs
│       │   └── handlers.rs
│       ├── mcp/
│       │   ├── mod.rs
│       │   ├── server.rs
│       │   └── tools.rs
│       └── logging/
│           └── mod.rs
├── frontend/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── components/
│       │   ├── mod.rs
│       │   ├── board.rs
│       │   ├── status.rs
│       │   └── log_panel.rs
│       └── services/
│           ├── mod.rs
│           └── api.rs
├── shared/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── types.rs (shared types between frontend/backend)
└── tests/
    ├── integration_tests.rs
    └── mock_ai.rs
```

## Core Data Types

```rust
// Player representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Player {
    X,
    O,
}

// Cell state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Cell {
    Empty,
    Occupied(Player),
}

// Board state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    cells: [[Cell; 3]; 3],
}

// Game state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    id: String,
    board: Board,
    current_turn: Player,
    human_player: Player,
    ai_player: Player,
    status: GameStatus,
    move_history: Vec<Move>,
    taunts: Vec<String>,
}

// Game status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameStatus {
    InProgress,
    Won(Player),
    Draw,
}

// Move
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    player: Player,
    row: u8,
    col: u8,
    timestamp: i64,
}
```

## Database Schema

```sql
CREATE TABLE games (
    id TEXT PRIMARY KEY,
    human_player TEXT NOT NULL,  -- 'X' or 'O'
    ai_player TEXT NOT NULL,     -- 'X' or 'O'
    current_turn TEXT NOT NULL,  -- 'X' or 'O'
    status TEXT NOT NULL,        -- 'InProgress', 'Won_X', 'Won_O', 'Draw'
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE TABLE moves (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    game_id TEXT NOT NULL,
    player TEXT NOT NULL,
    row INTEGER NOT NULL,
    col INTEGER NOT NULL,
    timestamp INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES games(id)
);

CREATE TABLE taunts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    game_id TEXT NOT NULL,
    message TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES games(id)
);
```

## Game Logic Algorithm

### Move Validation
```rust
fn is_valid_move(board: &Board, row: u8, col: u8) -> Result<(), MoveError> {
    if row >= 3 || col >= 3 {
        return Err(MoveError::OutOfBounds);
    }
    if board.cells[row as usize][col as usize] != Cell::Empty {
        return Err(MoveError::CellOccupied);
    }
    Ok(())
}
```

### Win Detection
```rust
fn check_winner(board: &Board) -> Option<Player> {
    // Check rows
    for row in 0..3 {
        if let Some(winner) = check_line(board, [(row, 0), (row, 1), (row, 2)]) {
            return Some(winner);
        }
    }
    // Check columns
    for col in 0..3 {
        if let Some(winner) = check_line(board, [(0, col), (1, col), (2, col)]) {
            return Some(winner);
        }
    }
    // Check diagonals
    if let Some(winner) = check_line(board, [(0, 0), (1, 1), (2, 2)]) {
        return Some(winner);
    }
    if let Some(winner) = check_line(board, [(0, 2), (1, 1), (2, 0)]) {
        return Some(winner);
    }
    None
}
```

## MCP Tool Definitions

```json
{
  "tools": [
    {
      "name": "view_game_state",
      "description": "View the current tic-tac-toe game state",
      "inputSchema": {
        "type": "object",
        "properties": {},
        "required": []
      }
    },
    {
      "name": "get_turn",
      "description": "Get whose turn it is (X or O, human or ai)",
      "inputSchema": {
        "type": "object",
        "properties": {},
        "required": []
      }
    },
    {
      "name": "make_move",
      "description": "Make a move on the board",
      "inputSchema": {
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
    },
    {
      "name": "taunt_player",
      "description": "Send a taunt message to the human player",
      "inputSchema": {
        "type": "object",
        "properties": {
          "message": {
            "type": "string",
            "description": "The taunt message"
          }
        },
        "required": ["message"]
      }
    },
    {
      "name": "restart_game",
      "description": "Restart the game with a new board",
      "inputSchema": {
        "type": "object",
        "properties": {},
        "required": []
      }
    }
  ]
}
```

## REST API Specification

### GET /api/game/state
**Response**:
```json
{
  "id": "game-uuid",
  "board": [[null, "X", null], ["O", "X", null], [null, "O", null]],
  "currentTurn": "X",
  "humanPlayer": "X",
  "aiPlayer": "O",
  "status": "InProgress",
  "moveHistory": [...],
  "taunts": ["You call that a move?"]
}
```

### POST /api/game/move
**Request**:
```json
{
  "row": 0,
  "col": 2
}
```
**Response**: GameState (same as above)

### POST /api/game/restart
**Response**: New GameState

## UI Component Design

### Board Component
- 3x3 grid of clickable cells
- Disabled if not player's turn or game over
- Visual feedback on hover
- Display X or O in cells

### Status Component
- Current turn indicator
- Game result (win/draw)
- Player assignments

### Log Panel Component
- Scrollable container (max height)
- Shows recent moves and taunts
- Auto-scrolls to latest entry

## Logging Strategy

1. **Server stdout**: All requests, game events, errors
2. **Browser console**: API calls, state changes, errors
3. **UI footer**: User-friendly game events (moves, taunts, results)

Format: `[timestamp] [level] [component] message`
