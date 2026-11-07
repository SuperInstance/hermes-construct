---
name: typescript-integration-testing
description: Governs the creation, execution, and debugging of TypeScript integration tests within monorepo environments.
---

# TypeScript & Node.js Integration Testing

Governs the creation, execution, and debugging of TypeScript integration tests within monorepo environments.

## Triggers
- When a new module, adapter, or interface is created and requires functional verification.
- When a "silent failure" (empty stdout) occurs during test execution.
- When module resolution errors (e.g., `MODULE_NOT_FOUND`) occur in a complex directory structure.
- When a backend package (`agent-core`, `engine-core`, etc.) accumulates dead imports or frontend framework leakage (React, Vue) from monitoring/performance modules during modularization.
- When `npx tsc --noEmit` fails in a package that should be pure backend logic.

## Standard Workflow

1. **Write the Test via High-Level Tools**: Always prefer `write_file` or `patch` over shell redirection (`printf`, `echo`, etc.) to prevent encoding corruption and unterminated string literals.
2. **Localize Execution**: Execute tests from the relevant package root (e.g., `cd packages/agent-core && ...`) to ensure `node_modules` and `tsconfig.json` are correctly resolved.
3. **Verify Existence**: If a test fails with `MODULE_NOT_FOUND`, immediately run `ls -F` or `find` to confirm the file was actually written to the intended path.
4. **Handle Resolution Failies**: If `ts-node` fails with complex syntax or resolution errors despite file existence, check:
    - The relative import paths in the test (e.g., `import { X } from '../path/to/X'`).
    - The `tsconfig.json` of the package being tested.
    - The `node_modules` availability in the current working directory.
5. **Backend Hygiene Scan**: Before declaring a backend package "clean," grep for `import React` and frontend-specific hooks. Remove them and replace with stubs or TODO comments indicating migration to a UI-bound package (`agent-ui`).

## Pitfalls & Debugging

### Backend Framework Leakage (React in agent-core)
- **Symptom**: `npx tsc --noEmit` fails with `Module not found: 'react'` or `JSX element implicitly has type 'any'` in a package that is supposed to be framework-agnostic backend.
- **Cause**: Monitoring or performance modules imported `React` or React hooks, violating the backend-only boundary.
- **Fix**: 
    1. `grep -r "import React" src/` to find all leaks.
    2. Remove React imports and replace hook-based logic with plain utility functions.
    3. Add `// TODO: migrate to agent-ui` comments on UI-bound symbols.
    4. Re-run `npx tsc --noEmit` to confirm clean.

### The "Silent Failure" (Empty Stdout)
- **Symptom**: The tool returns successfully (exit code 0) but provides no output.
- **Cause**: Often indicates the script finished immediately without executing the main logic (e.g., missing `runTest()` call) or the shell/environment suppressed the output.
- **Fix**: Verify the script contains an immediate function call and attempt execution with `npx ts-node` using an absolute path.

### The Shell Redirection Trap
- **Symptom**: `ts-node` reports `Unteriminated string literal` or `Invalid character`.
- **Cause**: Using `printf` or `echo` in `terminal` to write multi-line TypeScript files. Shells often interpret backslashes, quotes, or newlines incorrectly, corrupting the source.
- **Fix**: **NEVER** use shell redirection for `.ts` or `.js` files. Always use the `write_file` tool.

### Module Resolution Loops
- **Symptom**: `Error: Cannot find module './filename.ts'` even when `ls` confirms it exists.
- **Cause**: `ts-node` or the underlying `Node.js` resolver is being confused by a mismatch between the `cwd` and the relative import paths, or the `tsconfig.json` is misconfigured for the target sub-package.
- **Fix**: 
    - Ensure the test file uses correct relative paths based on its position in the `src/` tree.
    - Run the command from the package root.
    - Verify the `package.json` in that directory is valid.

## Verification Checklist
- [ ] Does the test import from the correct relative path?
- [ ] Is the test file in the same package as the code being tested?
- [ ] Does `ls` confirm the file's presence before execution?
- [ ] Is the execution command run from the package root?
