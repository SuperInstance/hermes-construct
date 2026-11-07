# Qdrant Setup & Troubleshooting

## Starting the container
```bash
# If the container does not exist, create it:
docker run -d --name qdrant -p 6333:6333 -p 6334:6334 qdrant/qdrant:latest
# If it already exists but is stopped:

docker start qdrant
```

## Verifying the collection
```bash
curl http://localhost:6333/collections
```
You should see `superinstance_vectors` in the JSON response.

## Common issues
- **Port conflict**: Ensure no other service is bound to 6333/6334.
- **Container name clash**: If a container named `qdrant` already exists, either remove it (`docker rm -f qdrant`) or rename the new one.
- **Collection missing**: The watchdog creates the collection on first run. If you delete it manually, simply restart the watchdog (`./launch.sh`).

## Health check
```bash
curl -s http://localhost:6333/health | jq .
```
Should return `{ "ok": true }`.
