use cudaclaw::bus::{Event, EventBus};

#[test]
fn test_bus_fifo_eviction() {
    let mut bus = EventBus::new(3);
    
    bus.push(Event { id: 1, payload: "P1".to_string() });
    bus.push(Event { id: 2, payload: "P2".to_string() });
    bus.push(Event { id: 3, payload: "P3".to_string() });
    
    assert_eq!(bus.len(), 3);
    
    // Evict 1
    bus.push(Event { id: 4, payload: "P4".to_string() });
    assert_eq!(bus.len(), 3);
    
    let ids: Vec<u64> = bus.events().map(|e| e.id).collect();
    assert_eq!(ids, vec![2, 3, 4]);
}

#[test]
fn test_bus_capacity_limit() {
    let mut bus = EventBus::new(1);
    bus.push(Event { id: 1, payload: "A".to_string() });
    bus.push(Event { id: 2, payload: "B".to_string() });
    
    assert_eq!(bus.len(), 1);
    let ids: Vec<u64> = bus.events().map(|e| e.id).collect();
    assert_eq!(ids, vec![2]);
}

#[test]
fn test_bus_clear() {
    let mut bus = EventBus::new(5);
    bus.push(Event { id: 1, payload: "A".to_string() });
    bus.clear();
    assert_eq!(bus.len(), 0);
}
