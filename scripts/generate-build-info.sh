#!/bin/bash
# Generate build info file for the frontend

# Get git commit SHA (short form)
GIT_SHA=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")

# Get ISO timestamp
BUILD_TIME=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

# Create build info as a Rust source file
cat > shared/src/build_info.rs << EOF
// Auto-generated build info - DO NOT EDIT
pub const GIT_SHA: &str = "${GIT_SHA}";
pub const BUILD_TIME: &str = "${BUILD_TIME}";
EOF

echo "Build info generated: ${GIT_SHA} @ ${BUILD_TIME}"
