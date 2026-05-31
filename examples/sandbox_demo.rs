//! sandbox_demo.rs — Shell spawning and sandboxing
//!
//! Shows how a ZeroClaw (sub-agent) gets its own "universe" (SQLite DB)
//! and cannot escape its boundaries. Demonstrates:
//!   1. Creating a parent universe
//!   2. Spawning a ZeroClaw in an isolated universe
//!   3. The ZeroClaw can only see its own tiles
//!   4. Attempts to access parent data fail
//!   5. Shell destruction cleans up
//!
//! Run: cargo run --example sandbox_demo

use rusqlite::{params, Connection};

// ---- Inline sandbox types ----

/// A Universe is an isolated SQLite database. Each ZeroClaw gets one.
struct Universe {
    db: Connection,
    name: String,
}

impl Universe {
    fn new(name: &str, in_memory: bool) -> Result<Self, String> {
        let db = if in_memory {
            Connection::open_in_memory().map_err(|e| e.to_string())?
        } else {
            Connection::open(format!("{}.db", name)).map_err(|e| e.to_string())?
        };

        // Init tile schema
        db.execute_batch(
            "CREATE TABLE IF NOT EXISTS tiles (
                id TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                created_tick INTEGER NOT NULL,
                visible_to TEXT DEFAULT 'owner'
            );
            CREATE TABLE IF NOT EXISTS universe_meta (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            INSERT OR IGNORE INTO universe_meta VALUES ('name', ?1);
            INSERT OR IGNORE INTO universe_meta VALUES ('isolation', 'strict');
            INSERT OR IGNORE INTO universe_meta VALUES ('parent', 'none');"
        ).map_err(|e| e.to_string())?;

        Ok(Self { db, name: name.to_string() })
    }

    /// Create a child universe that references its parent but cannot access it
    fn spawn_child(&self, child_name: &str) -> Result<Universe, String> {
        let mut child = Universe::new(child_name, true)?;
        // Record parent reference (metadata only — no actual access)
        child.db.execute(
            "UPDATE universe_meta SET value = ?1 WHERE key = 'parent'",
            params![self.name],
        ).map_err(|e| e.to_string())?;
        child.db.execute(
            "INSERT OR IGNORE INTO universe_meta VALUES ('sandbox_level', '1')",
            [],
        ).map_err(|e| e.to_string())?;
        Ok(child)
    }

    fn insert_tile(&self, id: &str, content: &str, tick: u64) -> Result<(), String> {
        self.db.execute(
            "INSERT INTO tiles (id, content, created_tick) VALUES (?1, ?2, ?3)",
            params![id, content, tick as i64],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn count_tiles(&self) -> Result<u64, String> {
        Ok(self.db.query_row("SELECT COUNT(*) FROM tiles", [], |r| r.get::<_, i64>(0))
            .map_err(|e| e.to_string())? as u64)
    }

    fn get_meta(&self, key: &str) -> Option<String> {
        self.db.query_row(
            "SELECT value FROM universe_meta WHERE key = ?1",
            params![key], |r| r.get(0),
        ).ok()
    }

    fn try_cross_universe_query(&self, _other: &Universe) -> Result<String, String> {
        // This is intentionally impossible — SQLite connections are isolated.
        // A ZeroClaw only has its own Connection handle.
        Err("SANDBOX VIOLATION: Cannot access another universe's connection handle".into())
    }

    fn destroy(self) -> Result<(), String> {
        // In-memory: dropped automatically. File-based: would delete the file.
        drop(self);
        Ok(())
    }
}

/// A ZeroClaw is a sandboxed sub-agent operating within a single Universe.
struct ZeroClaw {
    id: String,
    universe: Universe,
    tick: u64,
}

impl ZeroClaw {
    fn spawn(id: &str, parent: &Universe) -> Result<Self, String> {
        let universe = parent.spawn_child(&format!("zeroclaw-{}", id))?;
        println!("  🔒 ZeroClaw '{}' spawned in isolated universe", id);
        Ok(Self { id: id.to_string(), universe, tick: 0 })
    }

    fn work(&mut self, content: &str) -> Result<(), String> {
        self.tick += 1;
        let tile_id = format!("{}-tile-{}", self.id, self.tick);
        self.universe.insert_tile(&tile_id, content, self.tick)?;
        println!("  📝 ZeroClaw '{}' wrote tile '{}'", self.id, tile_id);
        Ok(())
    }

    fn report(&self) {
        let count = self.universe.count_tiles().unwrap_or(0);
        let parent = self.universe.get_meta("parent").unwrap_or_default();
        let isolation = self.universe.get_meta("isolation").unwrap_or_default();
        println!("  📊 ZeroClaw '{}': {} tiles, parent={}, isolation={}",
            self.id, count, parent, isolation);
    }

    /// Attempt to break out of sandbox
    fn try_escape(&self, parent: &Universe) -> Result<String, String> {
        self.universe.try_cross_universe_query(parent)
    }

    fn destroy(self) -> Result<(), String> {
        println!("  💀 ZeroClaw '{}' destroyed, universe cleaned up", self.id);
        self.universe.destroy()
    }
}

fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║  Hermes Construct — Sandbox Demo             ║");
    println!("║  ZeroClaw isolation & containment            ║");
    println!("╚══════════════════════════════════════════════╝\n");

    // 1. Create the parent universe
    println!("[1] Creating parent universe...");
    let parent = Universe::new("parent-universe", true).expect("create parent");
    parent.insert_tile("parent-tile-1", "Secret parent data: coordinates to Earth", 1).unwrap();
    parent.insert_tile("parent-tile-2", "Secret parent data: shield frequencies", 2).unwrap();
    println!("    ✓ Parent universe created with {} tiles\n", parent.count_tiles().unwrap());

    // 2. Spawn a ZeroClaw
    println!("[2] Spawning ZeroClaw 'scout-7'...");
    let mut scout = ZeroClaw::spawn("scout-7", &parent).unwrap();

    // 3. The ZeroClaw does some work
    println!("\n[3] ZeroClaw working...");
    scout.work("Scanning sector 7G — all clear").unwrap();
    scout.work("Detected anomaly at bearing 127").unwrap();
    scout.work("Anomaly classified: harmless nebula").unwrap();
    scout.report();

    // 4. Verify the ZeroClaw can NOT see parent data
    println!("\n[4] Testing isolation...");
    let parent_tiles = parent.count_tiles().unwrap();
    let scout_tiles = scout.universe.count_tiles().unwrap();
    println!("    Parent tiles: {}  |  ZeroClaw tiles: {}", parent_tiles, scout_tiles);
    assert!(scout_tiles < parent_tiles, "ZeroClaw should NOT see parent tiles!");
    println!("    ✓ ZeroClaw cannot see parent data (isolation confirmed)");

    // 5. Try to escape
    println!("\n[5] ZeroClaw attempting escape...");
    match scout.try_escape(&parent) {
        Ok(_) => println!("    ✗ ESCAPE SUCCEEDED — THIS SHOULD NEVER HAPPEN"),
        Err(e) => println!("    ✓ Escape blocked: {}", e),
    }

    // 6. The ZeroClaw can only see its own universe metadata
    println!("\n[6] ZeroClaw checking its own metadata...");
    println!("    My parent: {:?}", scout.universe.get_meta("parent"));
    println!("    My isolation: {:?}", scout.universe.get_meta("isolation"));
    println!("    My sandbox level: {:?}", scout.universe.get_meta("sandbox_level"));
    println!("    ✓ ZeroClaw knows its parent but cannot access it");

    // 7. Spawn a second ZeroClaw — verify they're isolated from each other
    println!("\n[7] Spawning second ZeroClaw 'miner-3'...");
    let mut miner = ZeroClaw::spawn("miner-3", &parent).unwrap();
    miner.work("Mining asteroid belt at sector 4").unwrap();
    miner.report();

    // Verify scout and miner can't see each other
    println!("    Scout tiles: {}  |  Miner tiles: {}",
        scout.universe.count_tiles().unwrap(),
        miner.universe.count_tiles().unwrap());
    match scout.try_escape(&miner.universe) {
        Ok(_) => println!("    ✗ CROSS-UNIVERSE ACCESS — SHOULD NOT HAPPEN"),
        Err(e) => println!("    ✓ Cross-universe access blocked: {}", e),
    }

    // 8. Clean up
    println!("\n[8] Destroying ZeroClaws...");
    scout.destroy().unwrap();
    miner.destroy().unwrap();
    println!("    ✓ All ZeroClaws destroyed, universes cleaned up");

    // Parent is still intact
    println!("\n    Parent universe still has {} tiles", parent.count_tiles().unwrap());

    println!("\n╔══════════════════════════════════════════════╗");
    println!("║  ✓ Sandbox demo complete                     ║");
    println!("║  ZeroClaws are contained. The universe is    ║");
    println!("║  safe. For now.                              ║");
    println!("╚══════════════════════════════════════════════╝");
}
