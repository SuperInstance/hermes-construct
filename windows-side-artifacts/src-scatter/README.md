# src-scatter (2026-11-04)

Working files that were sitting loose in `C:\Users\casey\src` with no repo
tracking them, rescued before they could be lost. Nothing here is on
`main`; this is an archive of what was on disk, kept as found.

## What each piece is

| path | what it is | where the live version is |
|---|---|---|
| `docs/architecture/EVENT_BUS_HANDOVER.md` | the handover doc for the event bus. Describes the **first** design: a fixed-capacity FIFO `VecDeque` with an `Event` of `id` + `payload`. | superseded by the async bus on `main` |
| `src/bus.rs` | that same first design, with `EventFilter` (channel / agent / time range / payload prefix), `query()` and `latest()`, plus its unit tests. Uses `std::time::SystemTime`. | superseded: `main`'s `src/bus.rs` is the async version (chrono, tokio `RwLock`, uuid, `EventType` enum) |
| `src/lib.rs` | the crate root that declared `pub mod bus` for that draft. | `main`'s `src/lib.rs` |
| `tests/bus_tests.rs` | the integration tests for the first design. | superseded by `main`'s `tests/bus_tests.rs` |
| `tests/chord_tests.rs` | the chord-snapshot test for the async bus. | `main`'s `tests/chord_tests.rs` |
| `voice-bank-cold/` | six cold voice samples (`.wav` + the `.json` that describes each), keyed by hash. | nowhere else — this is the only copy |

## Why it is here

`C:\Users\casey\src` was a working directory that had accumulated files
outside any repository: seat drafts of code that `main` later replaced, a
handover doc that was never committed, and a voice bank with no home. The
Rust drafts are kept for the record even though `main` has newer versions
of the same files, because "superseded" is a judgement about intent and
these were real edits.

The voice bank is the one thing here that exists nowhere else.
