#!/usr/bin/env bash
# Launch SuperInstance development environment
# 1. Ensure Qdrant is running (docker container "qdrant")
#    If not, start it:
#    docker run -d --name qdrant -p 6333:6333 -p 6334:6334 qdrant/qdrant:latest
# 2. Start the watchdog (vectorization runtime)
python3 watchdog_vectorization.py &
WATCHDOG_PID=$!
# 3. Start the LLM‑as‑Compiler placeholder (if needed)
#    For now we just keep the process alive; replace with real service later.
#    Example: python3 llm_compiler.py &
#    COMPILER_PID=$!
# 4. Open VS Code devcontainer (if using git‑agent‑codespace)
#    code . --folder-uri "vscode-remote://devcontainer+${PWD}"
# 5. Wait for background processes; trap exit to clean up.
function cleanup() {
  echo "Shutting down background services..."
  kill $WATCHDOG_PID 2>/dev/null || true
  # kill $COMPILER_PID 2>/dev/null || true
}
trap cleanup EXIT
wait $WATCHDOG_PID
