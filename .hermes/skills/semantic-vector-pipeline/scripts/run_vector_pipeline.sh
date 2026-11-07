#!/bin/bash
# run_vector_pipeline.sh - Convenience wrapper for the semantic vector pipeline
# Starts Qdrant (if needed) and launches the watchdog with the selected embedder provider.

set -euo pipefail

QDRANT_CONTAINER="qdrant"
QDRANT_REST_PORT=6333
QDRANT_GRPC_PORT=6334
EMBEDDER_PROVIDER="${EMBEDDER_PROVIDER:-local}"

echo "=== Semantic Vector Pipeline Launcher ==="
echo "Embedder provider: $EMBEDDER_PROVIDER"

# Check if Qdrant is running
if ! docker ps --format '{{.Names}}' | grep -q "^${QDRANT_CONTAINER}$"; then
    echo "Starting Qdrant container..."
    docker run -d \
        --name "${QDRANT_CONTAINER}" \
        -p ${QDRANT_REST_PORT}:6333 \
        -p ${QDRANT_GRPC_PORT}:6334 \
        -v qdrant_storage:/qdrant/storage \
        qdrant/qdrant:latest
    echo "Waiting for Qdrant to be ready..."
    sleep 3
else
    echo "Qdrant already running."
fi

# Export provider for watchdog
export EMBEDDER_PROVIDER

# Start watchdog
echo "Starting watchdog with EMBEDDER_PROVIDER=$EMBEDDER_PROVIDER..."
python3 watchdog_vectorization.py &

WATCHDOG_PID=$!
echo "Watchdog started (PID: $WATCHDOG_PID)"
echo "To stop: kill $WATCHDOG_PID"
echo "To search: EMBEDDER_PROVIDER=$EMBEDDER_PROVIDER python3 query_qdrant.py \"your query\""

# Keep script alive to show logs
wait $WATCHDOG_PID