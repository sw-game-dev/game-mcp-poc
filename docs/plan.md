# Implementation Plan

## Phase 1: Project Setup
- [x] Create documentation structure
- [ ] Initialize Rust workspace with 2024 edition
- [ ] Set up Cargo.toml for workspace
- [ ] Create backend, frontend, and shared crates
- [ ] Configure dependencies
- [ ] Set up .gitignore
- [ ] Configure pre-commit hooks

## Phase 2: Core Game Logic (TDD)
- [ ] **Test**: Empty board creation
- [ ] **Impl**: Board struct and initialization
- [ ] **Test**: Valid move placement
- [ ] **Impl**: Move validation and placement
- [ ] **Test**: Invalid moves (out of bounds, occupied)
- [ ] **Impl**: Error handling for invalid moves
- [ ] **Test**: Row win detection
- [ ] **Impl**: Row win checking
- [ ] **Test**: Column win detection
- [ ] **Impl**: Column win checking
- [ ] **Test**: Diagonal win detection
- [ ] **Impl**: Diagonal win checking
- [ ] **Test**: Draw detection
- [ ] **Impl**: Draw checking (full board, no winner)
- [ ] **Test**: Turn switching
- [ ] **Impl**: Turn management
- [ ] **Test**: Player assignment and coin flip
- [ ] **Impl**: Random player assignment

## Phase 3: Database Layer (TDD)
- [ ] **Test**: Database initialization
- [ ] **Impl**: SQLite schema creation
- [ ] **Test**: Save game state
- [ ] **Impl**: Insert/update game state
- [ ] **Test**: Load game state
- [ ] **Impl**: Query game state
- [ ] **Test**: Save move history
- [ ] **Impl**: Insert moves
- [ ] **Test**: Save taunts
- [ ] **Impl**: Insert taunts
- [ ] **Test**: Database transactions
- [ ] **Impl**: Transaction handling

## Phase 4: REST API Backend (TDD)
- [ ] **Test**: GET /api/game/state returns game state
- [ ] **Impl**: Game state endpoint
- [ ] **Test**: POST /api/game/move validates and applies move
- [ ] **Impl**: Move endpoint with validation
- [ ] **Test**: POST /api/game/restart creates new game
- [ ] **Impl**: Restart endpoint
- [ ] **Test**: GET /api/game/history returns moves
- [ ] **Impl**: History endpoint
- [ ] **Test**: GET /api/taunts returns taunts
- [ ] **Impl**: Taunts endpoint
- [ ] **Test**: Static file serving
- [ ] **Impl**: Serve WASM assets

## Phase 5: MCP Server (TDD)
- [ ] **Test**: MCP server initialization
- [ ] **Impl**: MCP server setup
- [ ] **Test**: view_game_state tool
- [ ] **Impl**: view_game_state implementation
- [ ] **Test**: get_turn tool
- [ ] **Impl**: get_turn implementation
- [ ] **Test**: make_move tool with validation
- [ ] **Impl**: make_move implementation
- [ ] **Test**: taunt_player tool
- [ ] **Impl**: taunt_player implementation
- [ ] **Test**: restart_game tool
- [ ] **Impl**: restart_game implementation
- [ ] **Test**: Tool error handling
- [ ] **Impl**: Error responses

## Phase 6: Frontend UI (Yew/WASM)
- [ ] Set up Yew project structure
- [ ] **Test**: Board component renders 3x3 grid
- [ ] **Impl**: Board component
- [ ] **Test**: Cell click triggers API call
- [ ] **Impl**: Click handlers
- [ ] **Test**: Status component shows current state
- [ ] **Impl**: Status component
- [ ] **Test**: Log panel displays events
- [ ] **Impl**: Log panel component
- [ ] **Test**: Restart button works
- [ ] **Impl**: Restart functionality
- [ ] **Test**: API service layer
- [ ] **Impl**: API client with error handling

## Phase 7: Logging Infrastructure
- [ ] Set up tracing for backend
- [ ] Configure stdout logging
- [ ] Add console_log for WASM
- [ ] Implement UI log panel updates
- [ ] Add logging to all major operations
- [ ] Test logging at all levels

## Phase 8: Mock AI
- [ ] Create mock AI module
- [ ] Implement MCP client for testing
- [ ] Add basic move strategy (random valid moves)
- [ ] Test full game playthrough
- [ ] Test error scenarios
- [ ] Add AI taunt generation

## Phase 9: Integration Testing
- [ ] Test full game flow (human vs AI)
- [ ] Test game restart
- [ ] Test persistence across server restarts
- [ ] Test concurrent API calls
- [ ] Test UI with reqwest (HTML validation)
- [ ] Optional: Playwright UI tests

## Phase 10: Polish and Documentation
- [ ] Run rustfmt on all code
- [ ] Run clippy and fix all warnings
- [ ] Ensure all tests pass
- [ ] Update .gitignore
- [ ] Update documentation
- [ ] Add inline code documentation
- [ ] Create README with build/run instructions
- [ ] Test complete MVP

## Pre-commit Checklist (for each commit)
- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test --all`
- [ ] Verify .gitignore is up to date
- [ ] Update docs if needed
- [ ] Commit with detailed message
- [ ] Push to branch

## Dependencies (Estimated)

### Backend
- actix-web or axum (web framework)
- tokio (async runtime)
- serde, serde_json (serialization)
- rusqlite or sqlx (database)
- tracing, tracing-subscriber (logging)
- rand (coin flip)
- uuid (game IDs)

### Frontend
- yew (UI framework)
- wasm-bindgen
- web-sys
- gloo (utilities)
- serde, serde_json
- reqwest (API calls)

### Testing
- reqwest (integration tests)
- wasm-bindgen-test (WASM tests)

## Timeline Estimate
- Phase 1: 1 hour
- Phase 2: 3 hours
- Phase 3: 2 hours
- Phase 4: 3 hours
- Phase 5: 3 hours
- Phase 6: 4 hours
- Phase 7: 1 hour
- Phase 8: 2 hours
- Phase 9: 2 hours
- Phase 10: 2 hours

**Total: ~23 hours** for complete MVP
