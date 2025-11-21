# GEMINI.md - Project Overview

## Project Overview

This project is a "Trash Talkin' Tic Tac Toe" game implemented in Rust. It features a dual-interface: a web UI for human players and a Model Context Protocol (MCP) server for AI agents. The project is structured as a Rust workspace with three main crates: `backend`, `frontend`, and `shared`.

- **Backend**: A Rust server built with `tokio` and `axum`. It serves a REST API for the frontend, a static file server for the WASM assets, and an MCP server for AI agents. It uses SQLite for persistence.
- **Frontend**: A single-page application built with the Yew framework, compiled to WebAssembly (WASM). It provides an interactive tic-tac-toe board, game status display, and a chat interface for "trash talk".
- **Shared**: A crate containing shared data structures and types used by both the `backend` and `frontend`, such as game state, player information, and board cells.

The project follows a Test-Driven Development (TDD) approach and emphasizes code quality with `rustfmt` and `clippy`.

## Building and Running

### Development

To run the project in development mode with hot-reloading:

```bash
./scripts/dev.sh
```

This script will:
1.  Start the backend server at `http://localhost:7397`.
2.  Start the frontend development server with `trunk` at `http://localhost:8080` and open it in your browser.

### Production

To build the project for production:

```bash
./scripts/build.sh
```

To run the production server:

```bash
./scripts/serve.sh
```

### Testing

To run all tests:

```bash
cargo test --all
```

To run tests for a specific package:

```bash
cargo test --package backend
```

To run the frontend WASM tests:

```bash
cd frontend
wasm-pack test --headless --firefox
```

## Development Conventions

- **Language**: Rust 2024 Edition
- **Formatting**: `cargo fmt --all`
- **Linting**: `cargo clippy --all-targets --all-features -- -D warnings`
- **Development Process**: Test-Driven Development (TDD) is strictly followed.
- **Committing**: Before committing, ensure all tests pass, the code is formatted, and there are no clippy warnings.

## MCP Tools

The MCP server provides the following tools for AI agents:

- `view_game_state`: Get the current game state.
- `get_turn`: Determine whose turn it is.
- `make_move`: Make a move on the board.
- `taunt_player`: Send a taunt to the human player.
- `restart_game`: Start a new game.
- `get_game_history`: Get the history of all moves played.

You can manually test the MCP server using the following script:

```bash
./test-mcp-manual.sh
```
