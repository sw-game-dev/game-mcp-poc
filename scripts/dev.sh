#!/bin/bash
set -e

echo "Starting development server..."

# Check for required tools
if ! command -v trunk &> /dev/null; then
    echo "trunk not found. Installing trunk..."
    cargo install trunk
fi

# Add wasm target if not already added
rustup target add wasm32-unknown-unknown 2>/dev/null || true

# Start backend in background
echo "Starting backend server..."
cargo run --package backend &
BACKEND_PID=$!

# Give backend time to start
sleep 2

# Start frontend dev server
echo "Starting frontend dev server..."
cd frontend
trunk serve --open &
FRONTEND_PID=$!
cd ..

# Cleanup function
cleanup() {
    echo "Shutting down servers..."
    kill $BACKEND_PID 2>/dev/null || true
    kill $FRONTEND_PID 2>/dev/null || true
    exit 0
}

# Trap CTRL+C and cleanup
trap cleanup INT TERM

echo ""
echo "Development servers running:"
echo "  Backend:  http://localhost:7397"
echo "  Frontend: http://localhost:8080"
echo ""
echo "Press CTRL+C to stop all servers"

# Wait for processes
wait
