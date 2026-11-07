# Queue Implementation for Quilt GPU Integration

This file contains the final implementation of the lock‑free SPSC queue used in the Quilt GPU integration demo.

## src/queue.rs

```rust
//! Lock‑free single‑producer/single‑consumer ring buffer that lives in a
//! unified‑memory buffer (accessible by both CPU and GPU).
//!
//! The buffer is split into fixed‑size slots, each slot large enough to hold
//! the largest command we expect (header + payload).  The queue stores only
//! the raw bytes; it is the caller's responsibility to serialize/deserialize
//! the command structures.
//!
//! # Safety
//! The queue assumes a single producer and a single consumer.  If multiple
//! threads try to push or pop concurrently the behavior is undefined.
//!
//! Interior mutability is provided via an `UnsafeCell` for the storage buffer,
//! while the head/tail indices are `AtomicUsize` with Release/Acquire ordering.
//!
//! The type is marked as `Send` and `Sync` because the internal buffer is
//! protected by the atomic head/tail indices: only the producer writes to a
//! slot (after claiming it via tail) and only the consumer reads from a slot
//! (after claiming it via head).  The release‑acquire ordering on the indices
//! ensures proper visibility of the written bytes.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::UnsafeCell;

/// Fixed size of each slot in the queue (in bytes).  Must be at least the
/// size of the largest command you intend to store.
pub const SLOT_SIZE: usize = 4096; // matches QUILT_GPU_MAX_PAYLOAD_BYTES + header

/// A lock‑free SPSC queue backed by a pre‑allocated byte buffer.
pub struct UnifiedQueue {
    /// The raw storage for the queue slots, wrapped in UnsafeCell to allow
    /// interior mutability for the producer.
    buffer: UnsafeCell<Vec<u8>>,
    /// Number of slots in the buffer.
    capacity: usize,
    /// Index of the slot to read from (consumer).
    head: AtomicUsize,
    /// Index of the slot to write to (producer).
    tail: AtomicUsize,
}

impl UnifiedQueue {
    /// Create a new queue that can hold `capacity` commands.
    ///
    /// # Arguments
    /// * `capacity` – maximum number of commands the queue can hold at once.
    ///
    /// The allocated buffer size is `capacity * SLOT_SIZE` bytes.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Queue capacity must be > 0");
        let buf = vec![0u8; capacity * SLOT_SIZE];
        UnifiedQueue {
            buffer: UnsafeCell::new(buf),
            capacity,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    /// Returns the number of slots in the queue.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Try to push a command into the queue.
    ///
    /// # Arguments
    /// * `data` – the serialized command bytes. Must be ≤ `SLOT_SIZE`.
    ///
    /// // Returns
    /// * `Ok(())` if the command was enqueued.
    /// * `Err(QueueFull)` if the queue is full.
    #[inline]
    pub fn push(&self, data: &[u8]) -> Result<(), QueueFull> {
        assert!(data.len() <= SLOT_SIZE, "Command exceeds slot size");
        let mut tail = self.tail.load(Ordering::Relaxed);
        loop {
            let head = self.head.load(Ordering::Acquire);
            let next_tail = if tail + 1 == self.capacity { 0 } else { tail + 1 };
            if next_tail == head {
                // Queue full
                return Err(QueueFull);
            }
            // Try to claim the slot at `tail`.
            if self
                .tail
                .compare_exchange_weak(tail, next_tail, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                // We own the slot; copy the data.
                // Safety: we have exclusive access to this slot because we own the tail index.
                let buf = unsafe { &mut *self.buffer.get() };
                let offset = tail * SLOT_SIZE;
                let dst = &mut buf[offset..offset + SLOT_SIZE];
                // Copy the data.
                dst[..data.len()].copy_from_slice(data);
                // Zero‑out the rest of the slot (optional, helps debugging).
                for byte in &mut dst[data.len()..] {
                    *byte = 0;
                }
                return Ok(());
            }
            // Lost the race to another producer (should not happen in SPSC),
            // retry with the current tail value.
            tail = self.tail.load(Ordering::Relaxed);
        }
    }

    /// Try to pop a command from the queue.
    ///
    /// // Returns
    /// * `Some(vec)` containing the raw bytes if a command was available.
    /// * `None` if the queue is empty.
    #[inline]
    pub fn pop(&self) -> Option<Vec<u8>> {
        let mut head = self.head.load(Ordering::Relaxed);
        loop {
            let tail = self.tail.load(Ordering::Acquire);
            if head == tail {
                // Queue empty
                return None;
            }
            let next_head = if head + 1 == self.capacity { 0 } else { head + 1 };
            // Try to claim the slot at `head`.
            if self
                .head
                .compare_exchange_weak(head, next_head, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                // We own the slot; read the data.
                // Safety: we have exclusive access to this slot because we own the head index.
                let buf = unsafe { &*self.buffer.get() };
                let offset = head * SLOT_SIZE;
                let src = &buf[offset..offset + SLOT_SIZE];
                // Return a copy of the whole slot; the caller can interpret the header
                // and payload length as needed.
                let mut v = Vec::with_capacity(SLOT_SIZE);
                v.extend_from_slice(src);
                return Some(v);
            }
            // Lost the race to the consumer (should not happen in SPSC),
            // retry with the current head.
            head = self.head.load(Ordering::Relaxed);
        }
    }

    /// Returns a slice of the underlying buffer for direct inspection (unsafe).
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { &*self.buffer.get() }
    }
}

/// Error returned when the queue is full.
#[derive(Debug, PartialEq, Eq)]
pub struct QueueFull;

impl std::fmt::Display for QueueFull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "queue is full")
    }
}

impl std::error::Error for QueueFull {}

unsafe impl Send for UnifiedQueue {}
unsafe impl Sync for UnifiedQueue {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_push_pop() {
        let q = UnifiedQueue::new(4);
        let cmd = b"hello";
        assert_eq!(q.push(cmd), Ok(()));
        let popped = q.pop().expect("should have a command");
        assert_eq!(&popped[..cmd.len()], cmd);
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn queue_full() {
        let q = UnifiedQueue::new(2);
        let cmd = [0u8; SLOT_SIZE];
        assert_eq!(q.push(&cmd), Ok(()));
        assert_eq!(q.push(&cmd), Ok(()));
        assert_eq!(q.push(&cmd), Err(QueueFull));
        // Consume one, then we can push again.
        let _ = q.pop().expect("should have a command");
        assert_eq!(q.push(&cmd), Ok(()));
    }

    #[test]
    fn wrap_around() {
        let q = UnifiedQueue::new(2);
        let cmd1 = b"aa";
        let cmd2 = b"bb";
        let cmd3 = b"cc";
        assert_eq!(q.push(&cmd1), Ok(()));
        assert_eq!(q.push(&cmd2), Ok(()));
        let _ = q.pop().expect("first");
        assert_eq!(q.push(&cmd3), Ok(()));
        let popped = q.pop().expect("second");
        assert_eq!(&popped[..cmd2.len()], cmd2);
        let popped = q.pop().expect("third");
        assert_eq!(&popped[..cmd3.len()], cmd3);
        assert_eq!(q.pop(), None);
    }
}
```

## src/main.rs (demo)

```rust
#![allow(dead_code)]

mod queue; // <-- declare the local queue module

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use queue::{UnifiedQueue, QueueFull};

/// Simulated command processing – in a real GPU kernel this would be the
/// actual work done on the device. For this demo we just count the commands.
fn process_command(_cmd: &[u8]) -> u64 {
    // Pretend we did some work and return a dummy result.
    1
}

/// The persistent kernel loop (runs on a CPU thread for this demo).
///
/// # Arguments
/// * `queue` – reference to the shared queue.
// * `running` – atomic flag that tells the kernel when to stop.
fn kernel_loop(queue: &UnifiedQueue, running: &AtomicBool) {
    let mut processed = 0u64;
    while running.load(Ordering::Acquire) {
        if let Some(cmd) = queue.pop() {
            // In a real kernel we would interpret the command header and
            // dispatch to the appropriate handler. Here we just count.
            processed += process_command(&cmd);
            // Optional: simulate some work duration.
            // std::thread::sleep(Duration::from_nanos(100));
        } else {
            // Queue empty – spin briefly or yield to reduce CPU usage.
            // Using a short sleep keeps the demo from hogging the CPU.
            thread::sleep(Duration::from_micros(100));
        }
    }
    // When the loop exits we could post a final status, but for demo we just
    // print the total.
    println!("[GPU Kernel] Processed {} commands", processed);
}

/// Demo program that measures the round‑trip latency of pushing N commands
/// through the lock‑free queue and having the kernel thread consume them.
fn main() {
    let capacity = 1024; // number of slots
    let queue = Arc::new(UnifiedQueue::new(capacity));
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);
    let queue_clone = Arc::clone(&queue);

    // Spawn the kernel thread.
    let handle = thread::spawn(move || {
        kernel_loop(&queue_clone, &running_clone);
    });

    // Number of commands to push in this demo.
    let num_cmds = 100_000;
    // A simple dummy command: just a byte pattern.
    let dummy_cmd = [0u8; 64]; // fits comfortably in SLOT_SIZE (4096)

    let start = Instant::now();
    let mut pushed = 0;
    for i in 0..num_cmds {
        // In a real scenario we would serialize a proper command struct.
        // Here we just push raw bytes.
        match queue.push(&dummy_cmd) {
            Ok(()) => pushed += 1,
            Err(QueueFull) => {
                // If the queue fills up we wait a bit and retry.
                thread::sleep(Duration::from_micros(10));
                // Retry the same command.
                let mut j = i;
                while j < num_cmds {
                    match queue.push(&dummy_cmd) {
                        Ok(()) => {
                            pushed += 1;
                            j += 1;
                        }
                        Err(QueueFull) => {
                            thread::sleep(Duration::from_micros(10));
                        }
                    }
                }
                break;
            }
        }
    }
    let push_duration = start.elapsed();
    println!("[CPU] Pushed {} commands in {:?}", pushed, push_duration);

    // Signal the kernel to stop.
    running.store(false, Ordering::Release);
    // Wait for the kernel thread to finish.
    let _ = handle.join().expect("Kernel thread panicked");

    let total = start.elapsed();
    println!("[CPU] Total elapsed time: {:?}", total);
    println!(
        "[CPU] Throughput: {:.2} commands/sec",
        pushed as f64 / total.as_secs_f64()
    );
}
```

This reference file can be copied into the `quilt_repo/gpu_integration/src/` directory to rebuild the demo.