#!/bin/bash
# verify_rust_embedder.sh - Quick sanity check for superinstance-embedder CLI and Python bindings

set -euo pipefail

EMBEDDER_DIR="/c/Users/casey/polln/superinstance-embedder"
BINARY="${EMBEDDER_DIR}/target/release/si-embedder.exe"

echo "=== Rust Embedder Verification ==="

# Check binary exists
if [[ ! -f "$BINARY" ]]; then
    echo "ERROR: Binary not found at $BINARY"
    echo "Run: cd $EMBEDDER_DIR && cargo build --release"
    exit 1
fi
echo "✓ Binary found: $BINARY"

# Test help
echo ""
echo "--- Testing CLI help ---"
"$BINARY" --help

# Test domain command (no Qdrant needed)
echo ""
echo "--- Testing domain vector ---"
"$BINARY" domain "ternary math agent music"

# Test embed command (requires model download on first run)
echo ""
echo "--- Testing semantic embed ---"
"$BINARY" embed "test query"

# Test config generation
echo ""
echo "--- Testing config generation ---"
"$BINARY" config --output /tmp/embedder_config.json
cat /tmp/embedder_config.json | head -20

# Test Python bindings (if installed)
echo ""
echo "--- Testing Python bindings ---"
if python -c "import superinstance_embedder" 2>/dev/null; then
    python -c "
import superinstance_embedder as sie
e = sie.PyEmbedder()
v = e.embed('test query')
print(f'Embed result: {len(v)} dims')
print(f'First 5: {v[:5]}')
"
else
    echo "Python bindings not installed. Install with:"
    echo "  cd $EMBEDDER_DIR && maturin develop --release"
fi

echo ""
echo "=== Verification Complete ==="