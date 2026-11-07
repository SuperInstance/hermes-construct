# Event-Driven CNS Watcher Template

```rust
//! CNS↔DSH Bridge — CNS File Watcher (Notify-based, Event-driven)
//! Uses notify crate for real-time filesystem events instead of polling

use crate::types::USCPv3Packet;
use anyhow::Result;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{interval, Duration};
use tracing::{debug, error, info, warn};

/// CNS Watcher Events
#[derive(Debug)]
pub enum CNSWatcherEvent {
    NewPacket(PathBuf),
    Stopped,
}

/// CNS Watcher Builder
pub struct CNSWatcherBuilder {
    inbox_path: PathBuf,
    outbox_path: PathBuf,
    debounce_ms: u64,
}

impl CNSWatcherBuilder {
    pub fn new() -> Self {
        Self {
            inbox_path: PathBuf::new(),
            outbox_path: PathBuf::new(),
            debounce_ms: 100,
        }
    }

    pub fn inbox_path(mut self, path: PathBuf) -> Self {
        self.inbox_path = path;
        self
    }

    pub fn outbox_path(mut self, path: PathBuf) -> Self {
        self.outbox_path = path;
        self
    }

    pub fn debounce_ms(mut self, ms: u64) -> Self {
        self.debounce_ms = ms;
        self
    }

    pub async fn build(self) -> Result<(CNSWatcher, mpsc::Receiver<CNSWatcherEvent>)> {
        let (event_tx, event_rx) = mpsc::channel(1000);
        
        let watcher = CNSWatcher {
            inbox_path: self.inbox_path,
            outbox_path: self.outbox_path,
            event_tx,
            watcher: None,
            seen_packets: Arc::new(Mutex::new(HashSet::new())),
            debounce_ms: self.debounce_ms,
        };
        
        Ok((watcher, event_rx))
    }
}

/// CNS File Watcher
pub struct CNSWatcher {
    inbox_path: PathBuf,
    outbox_path: PathBuf,
    event_tx: mpsc::Sender<CNSWatcherEvent>,
    watcher: Option<RecommendedWatcher>,
    seen_packets: Arc<Mutex<HashSet<PathBuf>>>,
    debounce_ms: u64,
}

impl CNSWatcher {
    pub async fn start(&mut self) -> Result<()> {
        let (tx, mut rx) = mpsc::channel(1000);
        let event_tx = self.event_tx.clone();
        let inbox_path = self.inbox_path.clone();
        let debounce_ms = self.debounce_ms;
        let seen_packets = self.seen_packets.clone();
        
        let watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    if matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_)) {
                        for path in event.paths {
                            if path.extension().map(|e| e == "json").unwrap_or(false) {
                                let _ = tx.try_send(path);
                            }
                        }
                    }
                }
            },
            notify::Config::default(),
        )?;
        
        watcher.watch(&inbox_path, RecursiveMode::NonRecursive)?;
        self.watcher = Some(watcher);
        
        // Process existing files on startup
        self.process_existing_files().await?;
        
        // Debounced event processor
        let event_tx_clone = event_tx.clone();
        tokio::spawn(async move {
            let mut debounce_buffer = Vec::new();
            let mut debounce_interval = interval(Duration::from_millis(debounce_ms));
            
            loop {
                tokio::select! {
                    Some(path) = rx.recv() => {
                        debounce_buffer.push(path);
                    }
                    _ = debounce_interval.tick() => {
                        if !debounce_buffer.is_empty() {
                            // Deduplicate
                            let mut seen = seen_packets.lock().await;
                            for path in debounce_buffer.drain(..) {
                                if seen.insert(path.clone()) {
                                    let _ = event_tx_clone.try_send(CNSWatcherEvent::NewPacket(path));
                                }
                            }
                        }
                    }
                }
            }
        });
        
        info!("CNS Watcher started: inbox={}", self.inbox_path.display());
        Ok(())
    }
    
    async fn process_existing_files(&self) -> Result<()> {
        let mut seen = self.seen_packets.lock().await;
        let mut entries: Vec<_> = std::fs::read_dir(&self.inbox_path)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|ext| ext == "json").unwrap_or(false))
            .collect();
        
        entries.sort_by_key(|e| e.path());
        
        for entry in entries {
            let path = entry.path();
            if seen.insert(path.clone()) {
                let _ = self.event_tx.try_send(CNSWatcherEvent::NewPacket(path));
            }
        }
        
        debug!("Processed {} existing files", seen.len());
        Ok(())
    }
    
    pub async fn write_outbox(&self, packet: &USCPv3Packet) -> Result<()> {
        let json = serde_json::to_string_pretty(packet)?;
        let filename = format!("{}.json", packet.header.packet_id);
        let path = self.outbox_path.join(filename);
        tokio::fs::write(path, json).await?;
        Ok(())
    }
    
    pub async fn stop(&mut self) {
        self.watcher = None;
        let _ = self.event_tx.send(CNSWatcherEvent::Stopped).await;
        info!("CNS Watcher stopped");
    }
}
```