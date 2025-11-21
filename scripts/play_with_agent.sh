#!/bin/bash
# Script to run the AI agent connected to the MCP server
#
# This script starts the MCP server and connects the AI agent to it via stdin/stdout.
# The agent will automatically play moves when it's their turn.
#
# Usage:
#   ./scripts/play_with_agent.sh [options]
#
# Options:
#   --verbose, -v       Enable verbose logging from the agent
#   --poll-interval N   Set polling interval in seconds (default: 1.0)
#   --max-turns N       Set maximum number of turns (default: 100)
#   --db-path PATH      Database path for the MCP server (default: :memory:)
#
# Examples:
#   # Run with default settings
#   ./scripts/play_with_agent.sh
#
#   # Run with verbose logging
#   ./scripts/play_with_agent.sh --verbose
#
#   # Run with faster polling
#   ./scripts/play_with_agent.sh --poll-interval 0.5

set -e

# Default values
DB_PATH=":memory:"
AGENT_ARGS=""

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose|-v)
            AGENT_ARGS="$AGENT_ARGS --verbose"
            shift
            ;;
        --poll-interval|-p)
            AGENT_ARGS="$AGENT_ARGS --poll-interval $2"
            shift 2
            ;;
        --max-turns|-m)
            AGENT_ARGS="$AGENT_ARGS --max-turns $2"
            shift 2
            ;;
        --db-path)
            DB_PATH="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 [--verbose] [--poll-interval N] [--max-turns N] [--db-path PATH]"
            exit 1
            ;;
    esac
done

# Check if MCP server binary exists
if [ ! -f "./target/release/game-mcp-server" ]; then
    echo "Error: MCP server binary not found."
    echo "Please build the project first:"
    echo "  ./scripts/build.sh"
    exit 1
fi

# Check if Python script exists
if [ ! -f "./scripts/ai_agent.py" ]; then
    echo "Error: AI agent script not found at ./scripts/ai_agent.py"
    exit 1
fi

echo "Starting AI agent connected to MCP server..."
echo "Database: $DB_PATH"
echo "Agent args: $AGENT_ARGS"
echo ""
echo "The agent will play automatically when it's their turn."
echo "You can interact with the game via the web UI at http://localhost:7397"
echo ""
echo "Press Ctrl+C to stop."
echo ""

# Run the MCP server and pipe the agent's output to it
GAME_DB_PATH="$DB_PATH" ./target/release/game-mcp-server < <(python3 ./scripts/ai_agent.py $AGENT_ARGS)
