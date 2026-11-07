---
name: quilt-gpu-integration
title: Quilt GPU Integration
description: Implement lock‑free SPSC queue for GPU‑CPU communication in Rust for Quilt reactive engine acceleration.
category: quilt
---

# Quilt GPU Integration Skill

## Overview
This skill covers the implementation of a lock‑free single‑producer/single‑consumer (SPSC) ring buffer in Rust that lives in a unified memory buffer accessible by both CPU and GPU. It is designed for the Quilt reactive engine to batch dirty cell commands and feed them to a persistent GPU kernel without per‑invocation launch overhead.

## When to Use
- You need to transfer large batches of commands (e.g., formula evaluations, cell updates) from a TypeScript/JavaScript Quilt core to a Rust/CUDA GPU kernel.
- You want to avoid per‑call CUDA launch latency by keeping a persistent kernel running on the GPU.
- You are working within the Quilt repository (`quilt_repo/gpu_integration/` or similar) and have a Cargo project set up.

## Key Concepts
- **Slot‑based ring buffer**: Fixed‑size slots (default 4096 bytes) hold serialized command structures.
- **Atomic head/tail indices**: `AtomicUsize` with Release/Acquire ordering guarantee safe publication of written data.
- **Interior mutability via `UnsafeCell`**: The storage buffer is wrapped in `UnsafeCell<Vec<u8>>` to allow the producer to write to a slot after claiming it via the tail index.
- **`Send` + `Sync`**: The type is marked unsafe `Send` and `Sync` because only the owning producer/consumer thread accesses each slot, and the atomic indices provide the necessary synchronization.
- **Command format**: The queue stores raw bytes; the caller must serialize/deserialize command structs (header + payload). A simple header could include a command ID and payload length.

## Implementation Steps

1. **Create the Cargo project**
   ```bash
   cargo new quilt_gpu --bin
   cd quilt_gpu
   ```

2. **Add dependencies (if any)** – none required for the core queue; you may add `cust` for CUDA bindings later.

3. **Define the queue (`src/queue.rs`)**
   - Constant `SLOT_SIZE` (e.g., 4096).
   - Struct `UnifiedQueue` with `buffer: UnsafeCell<Vec<u8>>`, `capacity`, `head: AtomicUsize`, `tail: AtomicUsize`.
   - `new(capacity)` allocates `capacity * SLOT_SIZE` bytes.
   - `push(&self, data: &[u8]) -> Result<(), QueueFull>`:
     - Verify `data.len() <= SLOT_SIZE`.
     - Load tail, compute next_tail (wrap‑around).
     - If next_tail == head → queue full.
     - CAS tail to claim slot.
     - Copy data into `buffer[offset..offset+SLOT_SIZE]`.
     - Zero‑out remainder of slot (optional).
   - `pop(&self) -> Option<Vec<u8>>`:
     - Load head, compute next_head.
     - If head == tail → empty.
     - CAS head to claim slot.
     - Copy entire slot into a `Vec<u8>` and return.
   - `as_slice(&self) -> &[u8]` for inspection.
   - Define `QueueFull` error type.
   - Add `unsafe impl Send for UnifiedQueue {}` and `unsafe impl Sync for UnifiedQueue {}`.
   - Write unit tests for basic push/pop, full queue, wrap‑around.

4. **Implement a demo or kernel loop (`src/main.rs`)**
   - Use `Arc<UnifiedQueue>` shared between producer (CPU) and consumer (GPU‑simulated thread).
   - Producer loop: serialize commands, call `queue.push()`, handle `QueueFull` by brief sleep and retry.
   - Consumer loop: while `running` flag, `queue.pop()` and process (e.g., count, or forward to real CUDA kernel).
   - Measure latency/throughput with `std::time::Instant`.

5. **Integrate with Quilt core**
   - In the Quilt engine (TypeScript), when cells become dirty, serialize them into command bytes (e.g., using flatbuffers or custom struct) and push onto the queue via a WASM‑bound Rust function or via a native node‑addon.
   - On the GPU side, a persistent kernel reads commands, dispatches to appropriate CUDA kernels (formula evaluation, etc.), writes results back into another queue or shared memory.
   - Use a second queue for results if needed.

6. **Testing & Validation**
   - Run `cargo test` to verify queue correctness.
   - Run the demo binary to observe push latency and throughput (should be >1M commands/sec on modern CPUs).
   - If integrating real CUDA, verify correctness by comparing GPU results with a CPU baseline.

## Pitfalls & Troubleshooting
- **Data races**: Ensure only one producer and one consumer; multiple producers/consumers without additional synchronization will cause undefined behavior.
- **Slot size too small**: If a command exceeds `SLOT_SIZE`, `push` will panic (or you can return an error). Choose a size large enough for your largest command (including header).
- **Memory ordering**: Use `Release` when publishing a slot (updating tail/index) and `Acquire` when consuming (loading the opposite index). This guarantees visibility of the written bytes.
- **Spinning vs sleeping**: In the demo, a short `sleep(Duration::from_micros(100))` when the queue is empty prevents CPU hogging. In a real GPU kernel you might prefer a true wait (e.g., CUDA event or futex) but that adds complexity.
- **CUDA integration**: When moving to real CUDA kernels, remember that the host and device must share the same memory (e.g., via `cudaHostAlloc` or unified memory). The queue buffer must reside in memory accessible by both.

## Reference
See `references/queue_implementation.md` for the annotated source code and a deeper walkthrough of the synchronization reasoning.

## Version
Initial version: 2026-08-24

---
*This skill is part of the Quilt family of skills. It pairs well with `quilt-demo-integration` (Cloudflare Worker AI backend) and `quilt-pattern-mapping` (cellular/GPU pattern mapping).*
