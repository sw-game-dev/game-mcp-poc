# Game MCP POC Wiki

Welcome to the Tic-Tac-Toe MCP Proof-of-Concept documentation wiki!

## Overview

This project demonstrates a **production-ready** dual-interface tic-tac-toe game with trash talk that can be played by both humans (via web UI) and AI agents (via Model Context Protocol). The system showcases modern Rust development practices, WebAssembly frontend technology, real-time updates via Server-Sent Events, and AI-agent integration patterns.

**Status**: ✅ Production Ready - 181 tests passing, full feature set implemented

### Quick Links

**Architecture Documentation:**
- [[Architecture Overview]] - System architecture and component diagrams
- [[Backend Architecture]] - Server-side components and design
- [[Frontend Architecture]] - WASM/Yew UI implementation
- [[MCP Integration]] - Model Context Protocol interface and tools
- [[Data Flow]] - Request flows and sequence diagrams

**Repository Documentation:**
- [Product Requirements (PRD)](https://github.com/sw-game-dev/game-mcp-poc/blob/main/docs/prd.md)
- [Detailed Design Specification](https://github.com/sw-game-dev/game-mcp-poc/blob/main/docs/design.md)
- [Development Process & TDD Guide](https://github.com/sw-game-dev/game-mcp-poc/blob/main/docs/process.md)
- [Project Status](https://github.com/sw-game-dev/game-mcp-poc/blob/main/docs/status.md)
- [AI Agent Examples](https://github.com/sw-game-dev/game-mcp-poc/blob/main/examples/README.md)
- [Online Deployment Plan](https://github.com/sw-game-dev/game-mcp-poc/blob/main/docs/online-plan.md)
- [Claude AI Instructions](https://github.com/sw-game-dev/game-mcp-poc/blob/main/CLAUDE.md)

## System Architecture

```mermaid
%%{init: {'theme':'base', 'themeVariables': {'primaryColor':'#f5f5f5','primaryTextColor':'#000','primaryBorderColor':'#333','lineColor':'#333','secondaryColor':'#f5f5f5','tertiaryColor':'#f5f5f5'}}}%%
graph TB
    subgraph "Client Layer"
        Browser[Web Browser]
        AI[AI Agent via MCP]
    end

    subgraph "Frontend"
        WASM[Yew/WASM UI]
    end

    subgraph "Backend Server"
        REST[REST API<br/>Axum]
        MCP[MCP Server]
        Logic[Game Logic]
        DB[SQLite DB]
    end

    Browser --> WASM
    WASM <--> REST
    AI <--> MCP
    REST --> Logic
    MCP --> Logic
    Logic --> DB

    style WASM fill:#fce4ec,stroke:#333,stroke-width:2px
    style REST fill:#e3f2fd,stroke:#333,stroke-width:2px
    style MCP fill:#e8f5e9,stroke:#333,stroke-width:2px
    style Logic fill:#fbb,stroke:#333,stroke-width:2px
```

## Technology Stack

| Layer | Technology |
|-------|------------|
| Frontend | Yew + WebAssembly (Rust) |
| Backend | Axum + Rust 2024 |
| Database | SQLite (file-based, WAL mode) |
| Protocol | REST API + MCP (HTTP + stdio) |
| Real-time | Server-Sent Events (SSE) |
| Testing | cargo test + wasm-bindgen-test (181 tests) |
| Logging | tracing + console_log |

## Key Features

### For Human Players
- Interactive web-based game board with drag-and-drop
- Real-time game state updates via SSE
- Trash talk input panel
- Live taunt display from AI opponents
- MCP "thinking" indicator
- Scrollable event log
- Restart controls
- GitHub corner link
- Build info footer

### For AI Agents
- **HTTP MCP endpoint** at `POST /mcp` (OpenAI, Gemini, custom agents)
- **Stdio MCP binary** (Claude Desktop native support)
- Protocol discovery (`initialize`, `tools/list`)
- 6 game tools (view_game_state, get_turn, make_move, taunt_player, restart_game, get_game_history)
- Working examples for OpenAI GPT-4, Google Gemini, Claude Desktop
- Complete setup documentation

## Development Workflow

This project follows strict **Test-Driven Development (TDD)**:

1. **Red** - Write failing test
2. **Green** - Implement minimal code to pass
3. **Refactor** - Improve code quality

### Quick Commands

```bash
# Development (hot-reload)
./scripts/dev.sh

# Build production
./scripts/build.sh

# Run tests
cargo test --all

# Format and lint
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

## Getting Started

1. Clone the repository
2. Run `./scripts/dev.sh` to start development servers
3. Open http://localhost:8080 in your browser
4. Configure your AI agent to connect via MCP

## Project Structure

```
game-mcp-poc/
├── backend/          # Rust server (REST + MCP + SSE + game logic)
│   └── src/
│       ├── api/      # HTTP endpoints, SSE, MCP HTTP handler
│       ├── game/     # Game logic and state management
│       ├── db/       # SQLite persistence layer
│       └── mcp/      # MCP protocol and tools
├── frontend/         # Yew/WASM web interface
├── shared/           # Common types (Player, Cell, GameState)
├── examples/         # AI agent integration examples
├── scripts/          # Build and development scripts
├── docs/             # Detailed documentation
└── wiki/             # This wiki content
```

## Documentation Navigation

Use the sidebar to navigate between different architecture topics, or browse the links above to explore specific areas of the system.

## Contributing

This project requires all code to:
- Pass `cargo fmt` and `cargo clippy` with no warnings
- Have comprehensive tests (no placeholder tests)
- Follow TDD red-green-refactor cycle
- Include detailed commit messages

See the [[Development Process]] guide for detailed contribution guidelines.
