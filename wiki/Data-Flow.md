# Data Flow

This page documents the various data flows and interaction patterns in the tic-tac-toe system, showing how data moves between components in different scenarios.

## Overview

The system supports two primary interaction modes:

1. **Human Player** via web UI (REST API)
2. **AI Agent** via MCP protocol

Both interact with the same underlying game state, enabling hybrid human-AI gameplay.

## System Data Flow

```mermaid
graph TB
    subgraph "Client Layer"
        Browser[Web Browser<br/>Yew/WASM]
        Agent[AI Agent<br/>MCP Client]
    end

    subgraph "Transport Layer"
        REST[HTTP/REST<br/>Port 7397]
        MCP[JSON-RPC/MCP<br/>Port 7397]
    end

    subgraph "Application Layer"
        Router[Axum Router]
        MCPServer[MCP Server]
        Handlers[API Handlers]
        Tools[MCP Tools]
    end

    subgraph "Business Layer"
        Logic[Game Logic<br/>Validation<br/>Win Detection]
    end

    subgraph "Data Layer"
        Repo[Repository]
        DB[(SQLite<br/>games<br/>moves<br/>taunts)]
    end

    Browser <-->|JSON| REST
    Agent <-->|JSON-RPC| MCP
    REST --> Router
    MCP --> MCPServer
    Router --> Handlers
    MCPServer --> Tools
    Handlers --> Logic
    Tools --> Logic
    Logic <--> Repo
    Repo <--> DB

    style Browser fill:#e1bee7
    style Agent fill:#c8e6c9
    style Logic fill:#ffccbc
    style DB fill:#fff9c4
```

## Human Player Flows

### 1. Initial Page Load

```mermaid
sequenceDiagram
    participant Browser
    participant Backend as Backend Server
    participant DB as SQLite DB

    Browser->>Backend: GET /
    Backend->>Browser: index.html + WASM bundle

    Note over Browser: WASM initializes
    Note over Browser: Yew app mounts

    Browser->>Backend: GET /api/game/state
    Backend->>DB: SELECT * FROM games WHERE id='current'
    DB->>Backend: Game state row

    alt Game exists
        Backend->>DB: SELECT * FROM moves WHERE game_id='current'
        DB->>Backend: Move history
        Backend->>Browser: 200 OK (GameState JSON)
    else No game
        Backend->>DB: INSERT INTO games (...)
        DB->>Backend: New game created
        Backend->>Browser: 200 OK (New GameState)
    end

    Note over Browser: Render board
```

### 2. Making a Move

```mermaid
sequenceDiagram
    participant User
    participant UI as Yew Component
    participant API as API Client
    participant Handler as API Handler
    participant Logic as Game Logic
    participant DB as SQLite DB

    User->>UI: Click cell (1, 2)
    UI->>UI: Disable board
    UI->>API: make_move(1, 2)

    API->>Handler: POST /api/game/move {row:1, col:2}
    Handler->>DB: SELECT game state
    DB->>Handler: Current game state

    Handler->>Logic: make_move(&mut state, Player::X, 1, 2)

    Logic->>Logic: Validate bounds
    Logic->>Logic: Check cell empty
    Logic->>Logic: Check correct turn
    Logic->>Logic: Update board
    Logic->>Logic: Check winner
    Logic->>Logic: Switch turn

    alt Move valid
        Logic->>Handler: Ok(())
        Handler->>DB: UPDATE games SET ...
        Handler->>DB: INSERT INTO moves ...
        DB->>Handler: Success
        Handler->>API: 200 OK (Updated GameState)
        API->>UI: StateLoaded(state)
        UI->>UI: Re-render board
        UI->>User: Visual update
    else Move invalid
        Logic->>Handler: Err(GameError)
        Handler->>API: 400 Bad Request (Error message)
        API->>UI: Error(message)
        UI->>UI: Enable board
        UI->>User: Show error
    end
```

### 3. Real-time Game State Updates (SSE)

The UI receives real-time state changes via Server-Sent Events (e.g., when AI makes a move):

```mermaid
sequenceDiagram
    participant UI as Yew App
    participant Backend
    participant DB
    participant Agent

    Note over UI,Backend: SSE Connection Established on /api/events

    Agent->>Backend: Make move via MCP
    Backend->>DB: UPDATE game state
    DB->>Backend: Success
    Backend->>Backend: Broadcast via SSE channel
    Backend-->>UI: SSE event: Complete GameState
    UI->>UI: Update game_state
    UI->>UI: Re-render components
    Note over UI: Board updates instantly
    Note over UI: Status updates instantly
    Note over UI: Moves logged to Event Log
```

**Implementation**: The frontend establishes a persistent SSE connection on component mount. All game state changes (moves, taunts, restarts) are broadcast immediately to all connected clients. No polling is required.

### 4. Receiving Taunts (via SSE)

```mermaid
sequenceDiagram
    participant UI
    participant Backend
    participant DB
    participant Agent

    Note over UI,Backend: SSE Connection Established

    Agent->>Backend: POST /api/game/taunt {message}
    Backend->>DB: INSERT INTO taunts
    DB->>Backend: Success
    Backend->>Backend: Broadcast via SSE
    Backend-->>UI: SSE event: GameState (includes taunts)
    UI->>UI: Detect new taunt
    UI->>UI: Display in "Trash Talk" panel
    UI->>UI: Log to Event Log
    Note over UI: 💬 MCP: "Nice try, human!"
```

**Note**: Taunts are delivered in real-time via Server-Sent Events (SSE), not polling. The frontend maintains an SSE connection to `/api/events` which receives complete game state updates whenever any change occurs (moves, taunts, game restarts).

### 5. Restarting Game

```mermaid
sequenceDiagram
    participant User
    participant UI
    participant Backend
    participant Logic
    participant DB

    User->>UI: Click "Restart" button
    UI->>Backend: POST /api/game/restart

    Backend->>Logic: create_new_game()
    Logic->>Logic: Random player assignment
    Logic->>Logic: Initialize empty board
    Logic->>Logic: Set first turn

    Backend->>DB: DELETE FROM moves WHERE game_id='current'
    Backend->>DB: DELETE FROM taunts WHERE game_id='current'
    Backend->>DB: UPDATE games SET ...

    DB->>Backend: Success
    Backend->>UI: 200 OK (New GameState)

    UI->>UI: Clear logs
    UI->>UI: Reset board
    UI->>UI: Update status
    UI->>User: Fresh game displayed
```

## AI Agent Flows

### 1. Connecting to MCP Server

```mermaid
sequenceDiagram
    participant Agent as AI Agent
    participant MCP as MCP Server
    participant Auth as Authentication

    Agent->>MCP: HTTP POST /mcp (127.0.0.1:7397)
    MCP->>Agent: Connection Established

    Agent->>MCP: {"method": "initialize", ...}
    MCP->>Auth: Validate credentials (if configured)
    Auth->>MCP: Authorized
    MCP->>Agent: {"result": {"capabilities": ...}}

    Agent->>MCP: {"method": "tools/list"}
    MCP->>Agent: {"result": {"tools": [...]}}
    Note over Agent: Discovers available tools
```

### 2. Making Strategic Move

```mermaid
sequenceDiagram
    participant Agent as AI Agent
    participant MCP as MCP Server
    participant Tools as Tool Handler
    participant Logic as Game Logic
    participant DB

    Agent->>MCP: call_tool("view_game_state")
    MCP->>Tools: view_game_state()
    Tools->>DB: Get current game
    DB->>Tools: GameState
    Tools->>MCP: Formatted board + status
    MCP->>Agent: "Board: X | O | ..."

    Note over Agent: Analyze board
    Note over Agent: Calculate best move

    Agent->>MCP: call_tool("get_turn")
    MCP->>Tools: get_turn()
    Tools->>DB: Get current turn
    DB->>Tools: Current turn = O (AI)
    Tools->>MCP: "Your turn (O)"
    MCP->>Agent: Confirmation

    Agent->>MCP: call_tool("make_move", {row:0, col:2})
    MCP->>Tools: make_move(0, 2)
    Tools->>Logic: make_move(&mut state, O, 0, 2)
    Logic->>Logic: Validate & execute
    Logic->>Tools: Ok(())
    Tools->>DB: UPDATE game state
    Tools->>DB: INSERT move
    DB->>Tools: Success
    Tools->>MCP: "Move successful at (0,2)"
    MCP->>Agent: Success response

    alt Winning move
        Agent->>MCP: call_tool("taunt_player", {message:"I win!"})
        MCP->>Tools: taunt_player("I win!")
        Tools->>DB: INSERT INTO taunts
        Tools->>MCP: "Taunt sent"
        MCP->>Agent: Confirmation
    end
```

### 3. Game History Analysis

```mermaid
sequenceDiagram
    participant Agent
    participant MCP
    participant Tools
    participant DB

    Agent->>MCP: call_tool("get_game_history")
    MCP->>Tools: get_game_history()

    Tools->>DB: SELECT * FROM moves WHERE game_id='current' ORDER BY timestamp
    DB->>Tools: List of moves

    Tools->>Tools: Format move list
    Tools->>MCP: "1. X at (0,0)\n2. O at (1,1)\n..."
    MCP->>Agent: Formatted history

    Note over Agent: Analyze patterns
    Note over Agent: Identify human strategy
    Note over Agent: Plan counter-strategy
```

## Hybrid Interaction Flow

Human and AI playing together:

```mermaid
sequenceDiagram
    participant Human
    participant UI
    participant Backend
    participant DB
    participant MCP
    participant AI

    Note over Human,AI: Game starts (Human=X, AI=O)

    Human->>UI: Click cell (0, 0)
    UI->>Backend: POST /api/game/move {row:0, col:0}
    Backend->>DB: Update: X at (0,0), turn=O
    DB->>Backend: Success
    Backend->>UI: Updated state
    UI->>Human: "AI's turn"

    AI->>MCP: call_tool("view_game_state")
    MCP->>DB: Get current state
    DB->>MCP: State (turn=O, X at 0,0)
    MCP->>AI: Board state

    AI->>MCP: call_tool("make_move", {row:1, col:1})
    MCP->>DB: Update: O at (1,1), turn=X
    DB->>MCP: Success
    MCP->>AI: "Move successful"

    AI->>MCP: call_tool("taunt_player", {message:"Center is mine!"})
    MCP->>DB: INSERT INTO taunts
    DB->>MCP: Success

    Note over UI: Polling detects changes
    UI->>Backend: GET /api/game/state
    Backend->>DB: Get current state
    DB->>Backend: State (turn=X, O at 1,1)
    Backend->>UI: Updated state
    UI->>Human: Board updated + taunt displayed

    Human->>UI: Click cell (0, 1)
    Note over Human,AI: Game continues...
```

## State Synchronization

```mermaid
graph TB
    A[Game State Change] --> B{Change Source}

    B -->|Human Move| C[REST API Handler]
    B -->|AI Move| D[MCP Tool Handler]

    C --> E[Update Game Logic]
    D --> E

    E --> F[Persist to SQLite]

    F --> G[Database State Updated]

    G --> H[UI Polls State]
    G --> I[AI Queries State]

    H --> J[UI Re-renders]
    I --> K[AI Makes Decision]

    style A fill:#ffe082
    style F fill:#ffccbc
    style G fill:#fff9c4
    style J fill:#c8e6c9
    style K fill:#bbdefb
```

## Data Models

### GameState Transfer Object

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub id: String,
    pub board: Board,
    pub current_turn: Player,
    pub human_player: Player,
    pub ai_player: Player,
    pub status: GameStatus,
    pub move_history: Vec<Move>,
    pub taunts: Vec<String>,
}
```

**JSON Representation:**
```json
{
  "id": "current",
  "board": [
    ["X", "O", null],
    ["X", "O", null],
    [null, null, null]
  ],
  "currentTurn": "X",
  "humanPlayer": "X",
  "aiPlayer": "O",
  "status": "InProgress",
  "moveHistory": [
    {"player": "X", "row": 0, "col": 0, "timestamp": 1234567890},
    {"player": "O", "row": 0, "col": 1, "timestamp": 1234567895}
  ],
  "taunts": ["Center is mine!"]
}
```

### Move Object

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub player: Player,
    pub row: u8,
    pub col: u8,
    pub timestamp: i64,
}
```

### Database Schema Flow

```mermaid
graph LR
    A[GameState Struct] -->|Serialize| B[JSON]
    B -->|HTTP Response| C[Client]

    D[Database Row] -->|Query Result| E[Deserialize]
    E -->|Into| A

    F[Move Struct] -->|INSERT| G[(moves table)]
    H[Taunt String] -->|INSERT| I[(taunts table)]

    A -->|UPDATE| J[(games table)]

    style A fill:#e1bee7
    style B fill:#bbdefb
    style D fill:#fff9c4
```

## Error Propagation

```mermaid
graph TD
    A[Error Occurs] --> B{Error Layer}

    B -->|Game Logic| C[GameError enum]
    B -->|Database| D[SQLite Error]
    B -->|API| E[HTTP Error]
    B -->|MCP| F[JSON-RPC Error]

    C --> G{Handler Layer}
    D --> G
    E --> G
    F --> G

    G -->|REST| H[HTTP Status Code]
    G -->|MCP| I[isError: true]

    H --> J[Client Error Handling]
    I --> K[Agent Error Handling]

    J --> L[Display to User]
    K --> M[Retry or Fallback Strategy]

    style A fill:#ffcdd2
    style C fill:#ffccbc
    style D fill:#ffccbc
    style E fill:#ffccbc
    style F fill:#ffccbc
    style L fill:#ffe082
    style M fill:#ffe082
```

**Error Mapping Table:**

| GameError | HTTP Status | MCP Response | User Message |
|-----------|-------------|--------------|--------------|
| `OutOfBounds` | 400 Bad Request | `isError: true` | "Invalid position" |
| `CellOccupied` | 400 Bad Request | `isError: true` | "Cell already taken" |
| `NotYourTurn` | 400 Bad Request | `isError: true` | "Not your turn" |
| `GameNotFound` | 404 Not Found | `isError: true` | "No active game" |
| `DatabaseError` | 500 Internal Error | `isError: true` | "Server error" |

## Performance Considerations

### Caching Strategy

```mermaid
graph TB
    A[Request] --> B{Cache Layer}

    B -->|Cache Hit| C[Return Cached State]
    B -->|Cache Miss| D[Query Database]

    D --> E[Game State]
    E --> F[Update Cache]
    F --> G[Return State]

    H[State Mutation] --> I[Invalidate Cache]
    I --> D

    style C fill:#c8e6c9
    style D fill:#fff9c4
    style I fill:#ffccbc
```

**Current Implementation:** No caching (simplicity)
**Future Optimization:** In-memory cache with TTL

### Database Connection Pooling

```mermaid
graph LR
    A[Request 1] --> B[Connection Pool]
    C[Request 2] --> B
    D[Request 3] --> B

    B --> E[Conn 1]
    B --> F[Conn 2]
    B --> G[Conn 3]

    E --> H[(SQLite WAL)]
    F --> H
    G --> H

    H --> I[Concurrent Reads]
    H --> J[Serialized Writes]

    style B fill:#bbdefb
    style H fill:#fff9c4
```

**SQLite WAL Mode Benefits:**
- Multiple concurrent readers
- Non-blocking reads during writes
- Better concurrency for our use case

## Logging Flow

```mermaid
graph TB
    A[Event Occurs] --> B{Event Type}

    B -->|Frontend| C[console.log]
    B -->|Backend| D[tracing]

    C --> E[Browser Console]
    D --> F[stdout]

    B --> G[User Action]
    G --> H[UI Log Panel]

    E --> I[Developer Tools]
    F --> J[Server Logs]
    H --> K[Player Visibility]

    style C fill:#e1bee7
    style D fill:#bbdefb
    style H fill:#c8e6c9
```

**Log Levels:**
- **ERROR**: Critical failures
- **WARN**: Recoverable issues
- **INFO**: Important events (moves, wins)
- **DEBUG**: Detailed flow information
- **TRACE**: Verbose debugging

## Related Pages

- [[Architecture Overview]] - System architecture
- [[Backend Architecture]] - Server components
- [[Frontend Architecture]] - Client components
- [[MCP Integration]] - AI agent protocol
- [[Home]] - Return to wiki home

## Further Reading

- [Detailed Design Document](https://github.com/sw-game-dev/game-mcp-poc/blob/main/docs/design.md)
- [API Handlers Source](https://github.com/sw-game-dev/game-mcp-poc/tree/main/backend/src/api)
- [MCP Tools Source](https://github.com/sw-game-dev/game-mcp-poc/tree/main/backend/src/mcp)
