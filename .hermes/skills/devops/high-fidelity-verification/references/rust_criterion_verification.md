# Rust/Criterion Benchmark Verification Protocol (2026-08-23)

> **Derived from:** CNS↔DSH Bridge spike validation — `cargo test` (8/8 PASS) + `cargo bench` (4/4 latency targets met)

---

## When to Use

Apply this protocol when verifying Rust crates that use `criterion` for performance benchmarking, especially for:
- Protocol translation layers (CNS↔DSH, USCP↔external)
- Hot-path code with strict latency budgets
- Any crate where performance is a contractual requirement

---

## Verification Protocol

### Phase 1: Build Verification
```bash
cargo build --release --benches
```
- Must compile without errors
- `--release` ensures optimized code for accurate benchmarks
- `--benches` compiles benchmark targets

### Phase 2: Unit Test Verification
```bash
cargo test --release
```
- All tests must pass (target: 100% pass rate)
- Run in `--release` to catch optimization-related bugs
- Tests validate correctness; benchmarks validate performance

### Phase 3: Benchmark Execution
```bash
cargo bench
```
- Runs all `[[bench]]` targets with `harness = false`
- Outputs to `target/criterion/` with HTML reports
- **Key metrics to capture:**
  - Mean latency (ns/μs/ms)
  - Standard deviation
  - Throughput (ops/sec)
  - Comparison against latency budget

### Phase 4: Benchmark Validation

**Acceptance Criteria Template:**
| Benchmark | Target | Measured | Status |
|-----------|--------|----------|--------|
| `ucsp_to_dsh` | <100μs | 4.7μs | ✅ PASS |
| `dsh_to_ucsp` | <100μs | 3.3μs | ✅ PASS |
| `roundtrip_full` | <100μs | 7.9μs | ✅ PASS |
| `roundtrip_minimal` | <100μs | 1.1μs | ✅ PASS |

**Verification Steps:**
1. Parse `target/criterion/*/new/estimates.json` for mean values
2. Compare against declared latency budget
3. Document any outliers or high variance (CV > 10%)
4. Generate summary table for decision makers

### Phase 5: External Validation (CI/CD)
- Benchmarks should run in CI on every PR
- Track regression: fail if mean latency increases >20% from baseline
- Store baseline in `benches/baseline.json` or similar

---

## Cargo.toml Benchmark Configuration

```toml
[[bench]]
name = "translation_bench"
harness = false

[dev-dependencies]
criterion = "0.5"
```

---

## Benchmark Structure Template (`benches/translation_bench.rs`)

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
    
    // Roundtrip benchmarks
    c.bench_function("roundtrip_full", |b| {
        b.iter(|| {
            let dsh = translate_ucsp_to_dsh(black_box(&ucsp_packet)).unwrap();
            translate_dsh_to_ucsp(black_box(&dsh)).unwrap()
        })
    });
    
    c.bench_function("roundtrip_minimal", |b| {
        b.iter(|| {
            let dsh = translate_ucsp_to_dsh(black_box(&minimal_packet)).unwrap();
            translate_dsh_to_ucsp(black_box(&dsh)).unwrap()
        })
    });
}

criterion_group!(benches, bench_translation);
criterion_main!(benches);
```

---

## Windows-Specific Pitfalls

- **MSYS2 vs native paths:** `cargo bench` works in both, but `target/criterion/` HTML reports use native paths
- **Thermal throttling:** Run benchmarks on cooled system; laptop thermals can add 2-5x variance
- **Background processes:** Close browsers, IDEs, heavy apps before benchmarking
- **Power profile:** Use "High Performance" or plugged-in mode for consistent results
- **First-run warmup:** Criterion does warmup automatically, but first `cargo bench` after build may be slower due to filesystem caching

---

## Reporting Format for Decision Makers

When delivering benchmark results for Go/No-Go decisions, include:

1. **Summary Table** (as above)
2. **Raw Data Reference** — path to `target/criterion/` or exported JSON
3. **Environment** — OS, CPU, Rust version, cargo profile
4. **Variance Analysis** — CV%, outlier count, thermal notes
5. **Verdict** — PASS/FAIL with margin (e.g., "PASS: 12-90x under budget")