#!/bin/sh
set -e

echo "=== shcut startup ==="
echo "Database path: ${DATABASE_URL:-/app/data/shcut.db}"

# Ensure data directory exists and is writable
DATA_DIR=$(dirname "${DATABASE_URL:-/app/data/shcut.db}")
mkdir -p "$DATA_DIR"

# Test write access
if touch "$DATA_DIR/.shcut_write_test" 2>/dev/null; then
    rm -f "$DATA_DIR/.shcut_write_test"
    echo "Data directory: OK (writable)"
else
    echo "ERROR: Data directory is not writable: $DATA_DIR"
    echo "Listing directory permissions:"
    ls -la "$(dirname "$DATA_DIR")" 2>/dev/null || true
    exit 1
fi

# Show what's in the data directory
echo "Data directory contents:"
ls -la "$DATA_DIR" 2>/dev/null || echo "(empty)"

exec "$@"
