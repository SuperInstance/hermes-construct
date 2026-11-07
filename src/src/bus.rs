use std::collections::VecDeque;

/// Represents an event in the system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: u64,
    pub payload: String,
}

/// An event bus that handles ingesting events with a fixed capacity.
/// When the capacity is reached, the oldest event is evicted (FIFO).
pub struct EventBus {
    capacity: usize,
    queue: VecDeque<Event>,
}

impl EventBus {
    /// Creates a new EventBus with the specified capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            queue: VecDeque::with_capacity(capacity),
        }
    }

    /// Pushes an event into the bus.
    /// If the bus is at capacity, the oldest event is removed to make room.
    pub fn push(&mut self, event: Event) {
        if self.queue.len() >= self.capacity {
            self.queue.pop_front();
        }
        self.queue.push_back(event);
    }

    /// Returns the current number of events in the bus.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Returns the capacity of the bus.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Clears all events from the bus.
    pub fn clear(&mut self) {
        self.queue.clear();
    }

    /// Returns a reference to the events in the bus in order (oldest to newest).
    pub fn events(&self) -> impl Iterator<Item = &Event> {
        self.queue.iter()
    }
}
