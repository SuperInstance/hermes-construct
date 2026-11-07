use std::collections::VecDeque;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

/// Represents an event in the system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub id: u64,
    pub channel: String,
    pub agent: String,
    pub payload: String,
    pub timestamp: SystemTime,
}

/// Criteria for filtering events.
#[derive(Debug, Clone, Default)]
pub struct EventFilter {
    pub channel: Option<String>,
    pub agent: Option<String>,
    pub start_time: Option<SystemTime>,
    pub end_time: Option<SystemTime>,
    pub payload_prefix: Option<String>,
}

impl EventFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_channel(mut self, channel: impl Into<String>) -> Self {
        self.channel = Some(channel.into());
        self
    }

    pub fn with_agent(mut self, agent: impl Into<String>) -> Self {
        self.agent = Some(agent.into());
        self
    }

    pub fn with_time_range(mut self, start: SystemTime, end: SystemTime) -> Self {
        self.start_time = Some(start);
        self.end_time = Some(end);
        self
    }

    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.payload_prefix = Some(prefix.into());
        self
    }

    /// Checks if an event matches the filter criteria.
    pub fn matches(&self, event: &Event) -> bool {
        if let Some(ref channel) = self.channel {
            if &event.channel != channel {
                return false;
            }
        }
        if let Some(ref agent) = self.agent {
            if &event.agent != agent {
                return false;
            }
        }
        if let Some(start) = self.start_time {
            if event.timestamp < start {
                return false;
            }
        }
        if let Some(end) = self.end_time {
            if event.timestamp > end {
                return false;
            }
        }
        if let Some(ref prefix) = self.payload_prefix {
            if !event.payload.starts_with(prefix) {
                return false;
            }
        }
        true
    }
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

    /// Queries the bus with a filter.
    pub fn query(&self, filter: &EventFilter) -> Vec<Event> {
        self.queue
            .iter()
            .filter(|e| filter.matches(e))
            .cloned()
            .collect()
    }

    /// Returns the latest event that matches the filter.
    pub fn latest(&self, filter: &EventFilter) -> Option<Event> {
        self.queue
            .iter()
            .filter(|e| filter.matches(e))
            .rev()
            .cloned()
            .next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_push_and_len() {
        let mut bus = EventBus::new(10);
        assert_eq!(bus.len(), 0);
        bus.push(Event {
            id: 1,
            channel: "sys".to_string(),
            agent: "host".to_string(),
            payload: "a".to_string(),
            timestamp: SystemTime::now(),
        });
        assert_eq!(bus.len(), 1);
    }

    #[test]
    fn test_eviction_fifo() {
        let mut bus = EventBus::new(2);
        let now = SystemTime::now();
        bus.push(Event {
            id: 1,
            channel: "sys".to_string(),
            agent: "host".to_string(),
            payload: "first".to_string(),
            timestamp: now,
        });
        bus.push(Event {
            id: 2,
            channel: "sys".to_string(),
            agent: "host".to_string(),
            payload: "second".to_string(),
            timestamp: now,
        });
        assert_eq!(bus.len(), 2);

        bus.push(Event {
            id: 3,
            channel: "sys".to_string(),
            agent: "host".to_string(),
            payload: "third".to_string(),
            timestamp: now,
        });
        assert_eq!(bus.len(), 2);

        let events: Vec<u64> = bus.events().map(|e| e.id).collect();
        assert_eq!(events, vec![2, 3]);
    }

    #[test]
    fn test_query_filtering() {
        let mut bus = EventBus::new(10);
        let now = SystemTime::now();

        bus.push(Event {
            id: 1,
            channel: "chan1".to_string(),
            agent: "agent1".to_string(),
            payload: "hello".to_string(),
            timestamp: now,
        });
        bus.push(Event {
            id: 2,
            channel: "chan2".to_string(),
            agent: "agent1".to_string(),
            payload: "world".to_string(),
            timestamp: now,
        });
        bus.push(Event {
            id: 3,
            channel: "chan1".to_string(),
            agent: "agent2".to_string(),
            payload: "prefix_test".to_string(),
            timestamp: now,
        });

        // Filter by channel
        let filter = EventFilter::new().with_channel("chan1");
        let results = bus.query(&filter);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|e| e.channel == "chan1"));

        // Filter by agent
        let filter = EventFilter::new().with_agent("agent1");
        let results = bus.query(&filter);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|e| e.agent == "agent1"));

        // Filter by prefix
        let filter = EventFilter::new().with_prefix("prefix");
        let results = bus.query(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, 3);

        // Complex filter
        let filter = EventFilter::new()
            .with_channel("chan1")
            .with_agent("agent2");
        let results = bus.query(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, 3);
    }

    #[test]
    fn test_latest() {
        let mut bus = EventBus::new(10);
        let now = SystemTime::now();

        bus.push(Event {
            id: 1,
            channel: "chan1".to_string(),
            agent: "agent1".to_string(),
            payload: "old".to_string(),
            timestamp: now,
        });
        bus.push(Event {
            id: 2,
            channel: "chan1".to_string(),
            agent: "agent1".to_string(),
            payload: "new".to_string(),
            timestamp: now + Duration::from_secs(1),
        });

        let filter = EventFilter::new().with_channel("chan1");
        let latest = bus.latest(&filter).unwrap();
        assert_eq!(latest.id, 2);

        let no_match = bus.latest(&EventFilter::new().with_channel("none"));
        assert!(no_match.is_none());
    }

    #[test]
    fn test_time_filtering() {
        let mut bus = EventBus::new(10);
        let start = SystemTime::now();
        let mid = start + Duration::from_secs(10);
        let end = start + Duration::from_secs(20);

        bus.push(Event {
            id: 1,
            channel: "c".to_string(),
            agent: "a".to_string(),
            payload: "early".to_string(),
            timestamp: start,
        });
        bus.push(Event {
            id: 2,
            channel: "c".to_string(),
            agent: "a".to_string(),
            payload: "mid".to_string(),
            timestamp: mid,
        });
        bus.push(Event {
            id: 3,
            channel: "c".to_string(),
            agent: "a".to_string(),
            payload: "late".to_string(),
            timestamp: end,
        });

        let filter = EventFilter::new().with_time_range(start + Duration::from_secs(5), end - Duration::from_secs(5));
        let results = bus.query(&filter);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, 2);
    }
}
