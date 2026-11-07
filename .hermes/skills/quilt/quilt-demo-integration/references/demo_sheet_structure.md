# Demo Sheet Structure

The demo sheet (`demo/demo_sheet.json`) defines eight cells that showcase every core Quilt cell kind:

| ID | Kind | Purpose |
|----|------|---------|
| `static_val` | `value` | Constant number (42). |
| `double` | `formula` | Computes `static_val * 2`. |
| `api_call` | `api` | Simple GET to https://httpbin.org/get. |
| `prog` | `program` | Runs a tiny JS snippet that returns the value of `double`. |
| `router` | `router` | Routes unconditionally to `prog`. |
| `ai_cell` | `ai` | Calls Cloudflare AI with a prompt that references `static_val`. |
| `sensor_input` | `sensor` | Placeholder for push‑based data (not used in demo). |
| `listener_alert` | `listener` | Watches `static_val`; logs a message when it exceeds 50. |

The sheet is a plain JSON object with a top‑level `id`, `title`, and a `cells` array. Each cell follows the schema defined in `src/types.ts`.

You can extend the sheet by adding more cells of any kind, referencing existing IDs with the `{{cellId}}` templating syntax.
