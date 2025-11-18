# Architecture

## Overview
This project implements a tic-tac-toe game with dual interfaces: a web UI for human players and an MCP (Model Context Protocol) server for AI agents.

## System Components

### 1. Frontend (Yew/WASM)
- **Framework**: Yew (Rust-based reactive framework)
- **Compilation**: WebAssembly (WASM)
- **Features**:
  - Interactive tic-tac-toe board
  - Game status display
  - Action buttons (restart, quit)
  - Scrollable log footer
  - Browser console logging

### 2. Backend (Rust)
- **Edition**: Rust 2024
- **Components**:
  - REST API server
  - Static file serving (WASM assets)
  - MCP server interface
  - Game logic engine
  - SQLite persistence layer

### 3. Database
- **Type**: SQLite file-based database
- **Schema**:
  - Game state (board, current turn)
  - Player assignments (X/O)
  - Game history
  - Move logs

### 4. MCP Server
- **Protocol**: Model Context Protocol
- **Tools**:
  - `view_game_state`: Get current board state
  - `get_turn`: Determine whose turn it is
  - `make_move`: Submit a move (row, col)
  - `taunt_player`: Send a message to the player
  - `restart_game`: Start a new game
  - `get_game_history`: View past moves

## Communication Flow

```
┌─────────────┐         ┌──────────────┐         ┌──────────┐
│ Yew/WASM UI │ ◄─────► │ REST API     │ ◄─────► │ SQLite   │
│  (Browser)  │         │  (Rust)      │         │ Database │
└─────────────┘         └──────────────┘         └──────────┘
                               ▲
                               │ MCP Protocol
                               │
                        ┌──────▼──────┐
                        │  AI Agent   │
                        │ (via MCP)   │
                        └─────────────┘
```

## Technology Stack

- **Language**: Rust 2024 edition
- **Frontend**: Yew + WASM
- **Backend**: Actix-web or Axum
- **Database**: SQLite + rusqlite or sqlx
- **MCP**: Custom MCP server implementation
- **Testing**:
  - Unit tests (cargo test)
  - Integration tests (reqwest)
  - WASM tests (wasm-bindgen-test)
  - Optional: Playwright for UI testing
- **Logging**: tracing + console_log

## Design Principles

1. **TDD (Test-Driven Development)**: Red/Green/Refactor cycle
2. **Clean Code**: Formatted with rustfmt, linted with clippy
3. **Type Safety**: Leverage Rust's type system
4. **Separation of Concerns**: Clear layer boundaries
5. **Testability**: Mock-friendly interfaces
