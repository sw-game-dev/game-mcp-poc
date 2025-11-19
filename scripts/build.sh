#!/bin/bash
set -e

echo "Building game-mcp-poc..."

# Generate build info
echo "Generating build info..."
./scripts/generate-build-info.sh

# Build backend
echo "Building backend..."
cargo build --package backend --release

# Build frontend (WASM)
echo "Building frontend WASM..."
if ! command -v trunk &> /dev/null; then
    echo "trunk not found. Installing trunk..."
    cargo install trunk
fi

if ! command -v wasm-bindgen &> /dev/null; then
    echo "wasm-bindgen not found. Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli
fi

# Add wasm target if not already added
rustup target add wasm32-unknown-unknown

cd frontend
trunk build --release
cd ..

echo "Build complete!"
echo "Backend binary: ./target/release/backend"
echo "Frontend assets: ./frontend/dist/"
