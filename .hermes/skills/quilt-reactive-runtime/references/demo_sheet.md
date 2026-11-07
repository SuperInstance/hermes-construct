# Demo Sheet & Runner

This reference documents the minimal demo sheet and runner script created to sanity‑check the Quilt core after fixing TypeScript build issues.

- **Sheet**: `demo/demo_sheet.json` – exercises every cell kind (value, formula, api, program, router, ai, sensor, listener). It lives in the repository root under `demo/`.
- **Runner**: `demo/run_demo.js` – ES‑module script that loads the compiled core (`packages/core/dist/index.js`), loads the demo sheet, and prints the results of each cell. It includes a fix for `__dirname` using `import.meta.url`.
- **Usage**:
  ```bash
  node demo/run_demo.js
  ```
  Expected output shows static values, computed formulas, successful API call, program execution, router routing, and an error status for the AI cell (no AI engine configured).

The demo is useful for:
- Verifying that the core builds (`npm run build`).
- Ensuring the WASM/JS binding works in a Node environment.
- Providing a quick sanity‑check for future contributors.

**Next steps**: Add a mock AI engine or stub to make the `ai_cell` succeed, and expand the sheet with more complex routing rules.
