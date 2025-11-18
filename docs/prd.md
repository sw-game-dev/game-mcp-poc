# Product Requirements Document (PRD)

## Project Overview
A proof-of-concept tic-tac-toe game demonstrating dual-interface gameplay: web UI for humans and MCP server for AI agents.

## Goals
- Demonstrate MCP server capabilities for game interaction
- Provide a working example of WASM/Rust web applications
- Enable AI agents to play games via structured tool calls
- Showcase clean architecture and TDD practices

## User Stories

### Human Player
- As a player, I want to see the game board in my browser
- As a player, I want to click cells to make my move
- As a player, I want to see whose turn it is
- As a player, I want to restart or quit the game
- As a player, I want to see game logs in the UI
- As a player, I want to receive taunts from the AI opponent

### AI Agent
- As an AI agent, I want to view the current game state via MCP
- As an AI agent, I want to determine whose turn it is
- As an AI agent, I want to make moves by specifying coordinates
- As an AI agent, I want to taunt the human player
- As an AI agent, I want to restart games

## Functional Requirements

### Game Logic
1. **Board**: 3x3 grid
2. **Players**: Human (assigned X or O) and AI agent (assigned opposite)
3. **Turn Order**: Random coin flip determines who goes first
4. **Win Conditions**: Three in a row (horizontal, vertical, diagonal)
5. **Draw Condition**: All cells filled with no winner
6. **Persistence**: Game state saved to SQLite database

### UI Requirements
1. **Display**:
   - 3x3 clickable grid
   - Current turn indicator
   - Game status (playing, won, draw)
   - Player assignments (X/O)
2. **Actions**:
   - Click to make move
   - Restart button
   - Quit button
3. **Logging**:
   - Scrollable log footer (last 10-20 entries)
   - Browser console logging
   - Server stdout logging

### MCP Tools
1. **view_game_state**: Returns board state, turn, status
2. **get_turn**: Returns whose turn it is
3. **make_move**: Accepts row, col; validates and executes move
4. **taunt_player**: Sends message to player (displayed in UI)
5. **restart_game**: Resets game to initial state
6. **get_game_history**: Returns list of past moves

### REST API Endpoints
1. `GET /api/game/state` - Get current game state
2. `POST /api/game/move` - Make a move
3. `POST /api/game/restart` - Restart game
4. `GET /api/game/history` - Get move history
5. `GET /api/taunts` - Get recent taunts
6. `GET /` - Serve static WASM UI

## Non-Functional Requirements

### Quality
- **Code Quality**: Pass rustfmt and clippy with no disabled checks
- **Test Coverage**: All game logic tested (TDD approach)
- **Documentation**: Clear inline docs and markdown files

### Performance
- **Response Time**: < 100ms for API calls
- **WASM Size**: Optimize for reasonable bundle size
- **Database**: Efficient queries with proper indexing

### Development Process
1. **TDD**: Write failing test, make it pass, refactor
2. **Pre-commit Hooks**:
   - Run rustfmt
   - Run clippy (no warnings)
   - Run tests (all pass)
   - Validate .gitignore
   - Update docs as needed
3. **Git Workflow**: Commit after successful pre-commit, then push

## Success Criteria
- [ ] Human can play tic-tac-toe via web UI
- [ ] AI agent can play via MCP tools
- [ ] Game state persists across sessions
- [ ] All tests pass
- [ ] Code is clean (rustfmt + clippy)
- [ ] Logging works on all channels
- [ ] Mock AI successfully tests MCP interface

## Out of Scope (v1)
- Multiple simultaneous games
- User authentication
- Multiplayer (human vs human)
- Advanced AI strategy
- Mobile optimization
- Game replays/history UI
