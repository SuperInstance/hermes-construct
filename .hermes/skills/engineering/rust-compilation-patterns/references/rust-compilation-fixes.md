# Rust Compilation Fix Patterns (2026-03 session, updated 2026-08-23)

## Schemars + Uuid/DateTime Gaps

When `Uuid`, `DateTime<Utc>`, or `Vec<(Uuid, String)>` are used in a struct deriving `JsonSchema`, add explicit overrides:

- `Uuid` → `#[schemars(with = "String")]`
- `DateTime<Utc>` → `#[schemars(with = "String")]`
- `Vec<(Uuid, String)>` → `#[schemars(with = "Vec<(String, String)>")]`

## Ambiguous Glob Re-exports Warning

When `pub use common::*;` and `pub use claw::*;` both export a shared type name (e.g. `ErrorHandling`), resolve by:

1. Moving the shared type into one module only, or
2. Re-exporting the canonical path explicitly instead of glob, or
3. Living with the warning if dedup is too invasive. (Preferred during active development.)

## Test Fixes for Enum Misuse

If integration tests construct an enum as a struct (e.g. `Capability { name: "..." }`), replace with the correct enum variant (`Capability::MemoryPersistence`). Rust will surface this as a type-error in struct-like syntax.

## Unused Variable in Tests

When `let removed =` (or similar) is unused in an assert-less test, rename to `let _removed =` to suppress the warning without changing semantics.

## Duplicate Type Definitions

If `cargo check` reports ambiguous types between modules, search for duplicate `struct`/`enum` definitions across files. Deduplicate into one canonical module and add `use` statements to consumers. After deduplication, remove orphaned `#[derive(...)]` lines in the file that lost the definition.

## Submodule Pointer Cleanup

If `git status` shows a path in `D` (deleted) or `R` (renamed) with mode `160000`, it is a git submodule pointer without proper `.gitmodules`. Remove via:
```
git rm --cached --quiet <submodule-path>
```
Then recommit. This unblocks future commits in the parent repo.

## Force-Push for Unrelated Histories

When `git pull` fails due to unrelated histories and the branch is a clean-publish integration branch, force-push:
```
git push --force origin <branch>
```
Acceptable for integration branches with no external collaborators.

---

## Edition 2024: `async fn` in Traits (2026-08-23)

**Error:** `async fn` in traits requires Rust 2024 edition or the `async_trait` crate.

**Context:** Building CNS↔DSH bridge with trait-based DSH client interface.

**Fix Options:**
1. **Upgrade to edition 2024** in `Cargo.toml`:
   ```toml
   [package]
   edition = "2024"
   ```
   Then `async fn` in traits works natively.

2. **Use `async_trait` crate** (compatible with edition 2021):
   ```rust
   use async_trait::async_trait;
   
   #[async_trait]
   trait DshClient {
       async fn emit(&self, event: DshEvent) -> Result<()>;
       async fn poll(&self) -> Result<Vec<DshEvent>>;
   }
   ```

**Decision:** For spike/prototype work, upgrading to edition 2024 is simpler and avoids extra dependency. For production code with MSRV constraints, use `async_trait`.

**Pitfall:** Edition 2024 changes `async fn` capture semantics (captures `self` by value in some cases). Test thoroughly after upgrade.

---

## `Default` Derive on Enums (2026-08-23)

**Error:** `the trait `Default` cannot be derived for this enum`

**Context:** `DSHPriority` enum with variants `Low`, `Normal`, `High`, `Critical` — tried `#[derive(Default)]`.

**Root Cause:** Rust's `Default` derive only works on enums where **exactly one variant has no fields** (unit variant). If all variants have fields, or multiple are unit variants, derive fails.

**Fix:** Manual `Default` impl:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DSHPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl Default for DSHPriority {
    fn default() -> Self {
        DSHPriority::Normal
    }
}
```

**Pattern:** For enums representing priority/level/state with a clear "normal" default, always implement `Default` manually rather than deriving.

---

## Criterion Benchmark Configuration (2026-08-23)

**Context:** Adding `criterion` benchmarks to CNS↔DSH bridge crate.

**Required `Cargo.toml`:**
```toml
[[bench]]
name = "translation_bench"
harness = false

[dev-dependencies]
criterion = "0.5"
```

**Benchmark Structure (`benches/translation_bench.rs`):**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cns_dsh_bridge::{translate_ucsp_to_dsh, translate_dsh_to_ucsp};

fn bench_translation(c: &mut Criterion) {
    let ucsp_packet = create_test_packet();
    let dsh_event = create_test_event();
    
    c.bench_function("ucsp_to_dsh", |b| {
        b.iter(|| translate_ucsp_to_dsh(black_box(&ucsp_packet)))
    });
    
    c.bench_function("dsh_to_ucsp", |b| {
        b.iter(|| translate_dsh_to_ucsp(black_box(&dsh_event)))
    });
}

criterion_group!(benches, bench_translation);
criterion_main!(benches);
```

**Run:** `cargo bench` — outputs to `target/criterion/` with HTML reports.

**Latency Targets:** For CNS↔DSH bridge, target <100μs per translation. Achieved: 1.1–7.9μs (12-90x under budget).

---

## Tokio `broadcast::Receiver` Manual `Future` Implementation (2026-08-25)

**Error:** `no method named 'poll_recv' found for mutable reference '&mut tokio::sync::broadcast::Receiver<String>' in the current scope`

**Context:** Implementing a custom `Future` enum `TriggerFuture` that wraps a `broadcast::Receiver` for event-based triggers.

**Root Cause:** The `poll_recv` method requires `Pin<&mut Self>`. Method lookup can be tricky in manual `Future` implementations.

**Fix Options:**
1.  **Avoid Manual Implementation:** Prefer using `tokio::select!` or spawning tasks.
2.  **Use `Pin::new`:** `Pin::new(receiver).poll_recv(cx)`.
3.  **Stub Complex Triggers (Development):** For stubs, avoid `broadcast` in `Future` impls. Stub `Event` and `Cron` to return `Poll::Pending` and focus on `Periodic` (uses `Interval::poll_tick`).

**Pitfall:** `broadcast::Receiver` handles lagged messages. Manual `Future` impl requires careful handling of `Poll::Ready(Err(RecvError::Lagged(_)))`.
