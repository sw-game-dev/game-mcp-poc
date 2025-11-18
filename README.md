# game-mcp-poc
A Proof of Concept game that provides an MCP server for AI Agent to play the game

agent instructions: create a Yew/Rust/WASM simple game, that has a CRUD/REST/static-serving back-end that also provides an MCP server interface, so that the user can interact with the game state on the server via a WASM UI and an AI agent can play the game via MCP tool calls.

Use Rust 2024 edition; follow TDD process (Red/Green). use pre-commit process: formatted code, clean clippy (never disabled checks) [note: special case wasm-test code looks dead to clippy unless annotated correctly, q.v. wasm testing best practices, online), tests pass, .gitignore validated, docs updated as needed.  After successful pre-commit process, commit w/details and push.

The initial game could be tic-tac-toe with persistence (sqlite file db).  The MCP tools allow an AI agent to view game state, determine whose turn it is, make a move, and taunt the player.

The game logic should assign X and O to player and agent and flip a coin to see who goes first.  The player can quit or restart a game.

Use MCP/Playwright, if available, to test the UI.  Otherwise use Rust reqwest in a curl-like way to validate html and csss has expected elements.  Use extensive logging to stdout on the server and to the browser console log, and also to the footer of the UI (in a short scrollable pane).  

start by creating a ./docs directory with markdown files for: architecture, prd, design, plan, process, and status.

Create a mock AI to test the MCP interfaces

Continue working until there is a testable MVP.
