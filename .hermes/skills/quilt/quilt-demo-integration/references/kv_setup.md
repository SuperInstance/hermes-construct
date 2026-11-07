# KV Namespace Setup for Quilt Worker

The Quilt demo Worker uses a Workers KV namespace called `USAGE` to enforce per‑IP rate limiting.

## Creating the namespace
```bash
cd /c/Users/casey/quilt_repo/quilt_worker
wrangler kv namespace create USAGE
```
The command prints something like:
```
Created KV Namespace "USAGE" with ID 0123456789abcdef0123456789abcdef
```
Copy the ID.

## Adding the binding to `wrangler.toml`
Edit the file `wrangler.toml` and add the KV binding under the `[kv_namespaces]` section:
```toml
[kv_namespaces]
# Bind the KV namespace to the environment variable USAGE
binding = "USAGE"
id = "0123456789abcdef0123456789abcdef"
```
If you already have a `[kv_namespaces]` block, just add the `binding` and `id` lines.

## Using the KV in the Worker
In `src/index.js` the KV binding is accessed via `env.USAGE`:
```js
const kv = env.USAGE;
const key = `visits:${ip}`;
const countStr = await kv.get(key);
```
The `put` call includes an `expirationTtl: 86400` (24 h) to automatically reset the counter each day.

## Common pitfalls
- **Forgot to add the ID** – the Worker will start but every request will behave as if the KV is missing, leading to a 429 on the first call.
- **Wrong binding name** – the name in `wrangler.toml` must match the variable used in the Worker (`USAGE`).
- **Permissions** – ensure your Cloudflare account has KV enabled for the selected zone.

## Updating or deleting the KV namespace
- To list namespaces: `wrangler kv namespace list`
- To delete: `wrangler kv namespace delete <id>`

Keep this file for future reference when you need to recreate or migrate the KV store.
