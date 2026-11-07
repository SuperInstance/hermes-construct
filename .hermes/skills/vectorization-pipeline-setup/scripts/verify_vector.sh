#!/usr/bin/env bash
# verify_vector.sh – simple sanity check that the latest upserted point exists in Qdrant
# Usage: ./verify_vector.sh <search_term>

set -e
TERM="${1:-test}"

# Use the local embedder to get a vector (fallback)
VECTOR=$(python3 - <<PY
import os, json, hashlib
from embedder import embed_text
print(json.dumps(embed_text("$TERM")))
PY
)

# Query Qdrant
curl -s -X POST -H "Content-Type: application/json" -d "{\"vector\": $VECTOR, \"top\": 5, \"with_payload\": true}" http://localhost:6333/collections/superinstance_vectors/points/search | jq '.'
