//! basic_agent.rs — Full hermes-construct flow WITHOUT Telegram or external APIs
//!
//! Uses a MockProvider and StdioPort. Runs standalone:
//!   cargo run --example basic_agent
//!
//! What it shows:
//!   1. Init SQLite (in-memory)
//!   2. Create rooms programmatically
//!   3. Deploy an ensign
//!   4. Receive a message via StdioPort (simulated)
//!   5. Route to room (keyword matching)
//!   6. Generate response via MockProvider
//!   7. Record tiles (observation + action)
//!   8. Print full trace

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use rusqlite::Connection;

// ---- Re-create the essential types inline so this example compiles self-contained ----
// In a real binary you'd `use hermes_construct::*;`

mod hermes {
    use async_trait::async_trait;
    use rusqlite::{params, Connection};
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;
    use tokio::sync::Mutex;

    // ---------- conservation ----------
    pub mod conservation {
        use rusqlite::{params, Connection};
        use serde::{Deserialize, Serialize};
        use std::sync::atomic::{AtomicU64, Ordering};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ConservationState {
            pub budget: f64,
            pub used: f64,
            pub tick: u64,
        }

        impl ConservationState {
            pub fn remaining(&self) -> f64 { self.budget - self.used }
            pub fn can_spend(&self, cost: f64) -> bool { self.remaining() >= cost }
            pub fn spend(&mut self, cost: f64) -> Result<(), String> {
                if !self.can_spend(cost) {
                    return Err(format!("budget exceeded: remaining={:.2} cost={:.2}", self.remaining(), cost));
                }
                self.used += cost;
                Ok(())
            }
        }

        static TICK: AtomicU64 = AtomicU64::new(0);
        pub fn current_tick() -> u64 { TICK.load(Ordering::Relaxed) }
        pub fn advance_tick() -> u64 { TICK.fetch_add(1, Ordering::Relaxed) }

        pub fn init_schema(conn: &Connection) -> Result<(), rusqlite::Error> {
            conn.execute_batch(
                "CREATE TABLE IF NOT EXISTS conservation (
                    key TEXT PRIMARY KEY, value TEXT NOT NULL
                );
                INSERT OR IGNORE INTO conservation VALUES ('budget','10000');
                INSERT OR IGNORE INTO conservation VALUES ('used','0');"
            )
        }

        pub fn load_state(conn: &Connection) -> ConservationState {
            let budget: f64 = conn.query_row(
                "SELECT value FROM conservation WHERE key='budget'", [], |r| r.get(0)
            ).unwrap_or(10000.0);
            let used: f64 = conn.query_row(
                "SELECT value FROM conservation WHERE key='used'", [], |r| r.get(0)
            ).unwrap_or(0.0);
            ConservationState { budget, used, tick: current_tick() }
        }

        pub fn save_state(conn: &Connection, s: &ConservationState) {
            let _ = conn.execute("UPDATE conservation SET value=? WHERE key='budget'", params![s.budget.to_string()]);
            let _ = conn.execute("UPDATE conservation SET value=? WHERE key='used'", params![s.used.to_string()]);
        }
    }

    // ---------- gravity ----------
    pub mod gravity {
        #[derive(Debug, Clone)]
        pub struct ModelParams {
            pub temperature: f64,
            pub prompt_style: String,
            pub max_tokens: u32,
            pub top_p: f64,
        }

        pub fn gravity_to_params(g: f64) -> ModelParams {
            let g = g.clamp(-1.0, 1.0);
            if g < -0.5 {
                ModelParams { temperature: 0.3, prompt_style: "precise".into(), max_tokens: 500, top_p: 0.9 }
            } else if g < 0.0 {
                ModelParams { temperature: 0.5, prompt_style: "balanced".into(), max_tokens: 1000, top_p: 0.95 }
            } else if g < 0.5 {
                ModelParams { temperature: 0.7, prompt_style: "creative".into(), max_tokens: 2000, top_p: 0.95 }
            } else {
                ModelParams { temperature: 0.9, prompt_style: "narrative".into(), max_tokens: 4000, top_p: 0.95 }
            }
        }

        pub fn style_to_system_prompt(style: &str) -> String {
            match style {
                "precise" => "Be precise and concise.".into(),
                "balanced" => "Be balanced and helpful.".into(),
                "creative" => "Be creative and thoughtful.".into(),
                "narrative" => "Tell stories. Use rich narrative.".into(),
                _ => "Be helpful.".into(),
            }
        }
    }

    // ---------- tile ----------
    pub mod tile {
        use rusqlite::{params, Connection};
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub enum TileType { Observation, Action, Thought, Delegation, Escalation, Artifact }

        impl TileType {
            pub fn as_str(&self) -> &str {
                match self {
                    Self::Observation => "observation", Self::Action => "action",
                    Self::Thought => "thought", Self::Delegation => "delegation",
                    Self::Escalation => "escalation", Self::Artifact => "artifact",
                }
            }
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub enum TileStatus { Active, Complete, Deadband, Escalated, Archived }

        impl TileStatus {
            pub fn as_str(&self) -> &str {
                match self {
                    Self::Active => "active", Self::Complete => "complete",
                    Self::Deadband => "deadband", Self::Escalated => "escalated",
                    Self::Archived => "archived",
                }
            }
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Tile {
            pub id: String, pub room_id: Option<String>, pub tile_type: TileType,
            pub parent_id: Option<String>, pub status: TileStatus, pub content: String,
            pub ensign_id: Option<String>, pub model_used: Option<String>,
            pub tokens_used: u32, pub conservation_delta: f64,
            pub created_tick: u64, pub updated_tick: u64,
        }

        impl Tile {
            pub fn new(tile_type: TileType, content: &str, tick: u64) -> Self {
                Self {
                    id: uuid::Uuid::new_v4().to_string(), room_id: None, tile_type,
                    parent_id: None, status: TileStatus::Active, content: content.into(),
                    ensign_id: None, model_used: None, tokens_used: 0,
                    conservation_delta: 0.0, created_tick: tick, updated_tick: tick,
                }
            }
            pub fn complete(&mut self, tick: u64) {
                self.status = TileStatus::Complete;
                self.updated_tick = tick;
            }
        }

        pub fn init_schema(conn: &Connection) -> Result<(), rusqlite::Error> {
            conn.execute_batch(
                "CREATE TABLE IF NOT EXISTS tiles (
                    id TEXT PRIMARY KEY, room_id TEXT, tile_type TEXT NOT NULL,
                    parent_id TEXT, status TEXT DEFAULT 'active', content TEXT NOT NULL,
                    ensign_id TEXT, model_used TEXT, tokens_used INTEGER DEFAULT 0,
                    conservation_delta REAL DEFAULT 0.0, created_tick INTEGER NOT NULL,
                    updated_tick INTEGER NOT NULL
                );"
            )
        }

        pub fn insert_tile(conn: &Connection, t: &Tile) -> Result<(), rusqlite::Error> {
            conn.execute(
                "INSERT INTO tiles (id,room_id,tile_type,parent_id,status,content,ensign_id,model_used,tokens_used,conservation_delta,created_tick,updated_tick) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)",
                params![t.id, t.room_id, t.tile_type.as_str(), t.parent_id, t.status.as_str(),
                        t.content, t.ensign_id, t.model_used, t.tokens_used as i64,
                        t.conservation_delta, t.created_tick as i64, t.updated_tick as i64],
            )?;
            Ok(())
        }
    }

    // ---------- room ----------
    pub mod room {
        use rusqlite::{params, Connection};
        use serde::{Deserialize, Serialize};
        use crate::hermes::gravity;

        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub enum RoomType { Navigation, Engineering, Science, Security, Social, Custom(String) }

        impl RoomType {
            pub fn as_str(&self) -> &str {
                match self {
                    Self::Navigation => "navigation", Self::Engineering => "engineering",
                    Self::Science => "science", Self::Security => "security",
                    Self::Social => "social", Self::Custom(s) => s,
                }
            }
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Room {
            pub id: String, pub room_type: RoomType, pub gravity: f64,
            pub prompt_style: String, pub created_tick: u64, pub updated_tick: u64,
        }

        impl Room {
            pub fn model_params(&self) -> gravity::ModelParams {
                gravity::gravity_to_params(self.gravity)
            }
        }

        pub fn init_schema(conn: &Connection) -> Result<(), rusqlite::Error> {
            conn.execute_batch(
                "CREATE TABLE IF NOT EXISTS rooms (
                    id TEXT PRIMARY KEY, room_type TEXT NOT NULL, gravity REAL DEFAULT 0.0,
                    prompt_style TEXT DEFAULT 'conversational', created_tick INTEGER, updated_tick INTEGER
                );"
            )
        }

        pub fn upsert_room(conn: &Connection, r: &Room) -> Result<(), rusqlite::Error> {
            conn.execute(
                "INSERT INTO rooms (id,room_type,gravity,prompt_style,created_tick,updated_tick) \
                 VALUES (?1,?2,?3,?4,?5,?6) \
                 ON CONFLICT(id) DO UPDATE SET gravity=excluded.gravity, updated_tick=excluded.updated_tick",
                params![r.id, r.room_type.as_str(), r.gravity, r.prompt_style, r.created_tick as i64, r.updated_tick as i64],
            )?;
            Ok(())
        }

        pub fn route_to_room(conn: &Connection, msg: &str) -> Option<Room> {
            let msg = msg.to_lowercase();
            let mut stmt = conn.prepare(
                "SELECT id,room_type,gravity,prompt_style,created_tick,updated_tick FROM rooms"
            ).ok()?;
            let rooms: Vec<Room> = stmt.query_map([], |row| {
                let type_str: String = row.get(1)?;
                Ok(Room {
                    id: row.get(0)?, room_type: match type_str.as_str() {
                        "navigation" => RoomType::Navigation, "engineering" => RoomType::Engineering,
                        "science" => RoomType::Science, "security" => RoomType::Security,
                        "social" => RoomType::Social, o => RoomType::Custom(o.into()),
                    },
                    gravity: row.get(2)?, prompt_style: row.get(3)?,
                    created_tick: row.get::<_,i64>(4)? as u64, updated_tick: row.get::<_,i64>(5)? as u64,
                })
            }).ok()?.filter_map(|r| r.ok()).collect();

            if msg.contains("navigate") || msg.contains("where") { rooms.iter().find(|r| r.room_type == RoomType::Navigation) }
            else if msg.contains("build") || msg.contains("code") || msg.contains("fix") { rooms.iter().find(|r| r.room_type == RoomType::Engineering) }
            else if msg.contains("science") || msg.contains("research") { rooms.iter().find(|r| r.room_type == RoomType::Science) }
            else { rooms.iter().find(|r| r.room_type == RoomType::Social).or_else(|| rooms.first()) }
            .cloned()
        }
    }

    // ---------- ensign ----------
    pub mod ensign {
        use async_trait::async_trait;
        use rusqlite::{params, Connection};
        use crate::hermes::gravity::ModelParams;

        #[derive(Debug, Clone)]
        pub struct CompletionRequest {
            pub prompt: String, pub model: String, pub params: ModelParams,
            pub system_prompt: Option<String>,
        }

        #[derive(Debug, Clone)]
        pub struct CompletionResponse {
            pub text: String, pub model: String, pub tokens_used: u32, pub provider: String,
        }

        #[async_trait]
        pub trait Provider: Send + Sync {
            async fn complete(&self, req: &CompletionRequest) -> Result<CompletionResponse, String>;
            fn name(&self) -> &str;
        }

        // -- Mock provider for testing --
        pub struct MockProvider {
            pub response_text: String,
        }

        #[async_trait]
        impl Provider for MockProvider {
            async fn complete(&self, _req: &CompletionRequest) -> Result<CompletionResponse, String> {
                Ok(CompletionResponse {
                    text: self.response_text.clone(),
                    model: "mock-model".into(),
                    tokens_used: 42,
                    provider: "mock".into(),
                })
            }
            fn name(&self) -> &str { "mock" }
        }

        #[derive(Debug, Clone)]
        pub struct Ensign {
            pub id: String, pub model_name: String, pub provider: String, pub room_id: Option<String>,
        }

        impl Ensign {
            pub fn new(id: &str, model: &str, provider: &str) -> Self {
                Self { id: id.into(), model_name: model.into(), provider: provider.into(), room_id: None }
            }
        }

        pub fn init_schema(conn: &Connection) -> Result<(), rusqlite::Error> {
            conn.execute_batch(
                "CREATE TABLE IF NOT EXISTS ensigns (
                    id TEXT PRIMARY KEY, model_name TEXT NOT NULL, provider TEXT NOT NULL, room_id TEXT
                );"
            )
        }

        pub fn upsert_ensign(conn: &Connection, e: &Ensign) -> Result<(), rusqlite::Error> {
            conn.execute(
                "INSERT INTO ensigns (id,model_name,provider,room_id) VALUES (?1,?2,?3,?4) \
                 ON CONFLICT(id) DO UPDATE SET room_id=excluded.room_id",
                params![e.id, e.model_name, e.provider, e.room_id],
            )?;
            Ok(())
        }

        pub fn get_ensign_for_room(conn: &Connection, room_id: &str) -> Option<Ensign> {
            conn.query_row(
                "SELECT id,model_name,provider,room_id FROM ensigns WHERE room_id=?1",
                params![room_id],
                |row| Ok(Ensign {
                    id: row.get(0)?, model_name: row.get(1)?, provider: row.get(2)?, room_id: row.get(3)?,
                }),
            ).ok()
        }
    }
}

use hermes::conservation::{self, ConservationState};
use hermes::ensign::{self, Ensign, Provider as _};
use hermes::gravity;
use hermes::room;
use hermes::tile::{self, Tile, TileType};

#[tokio::main]
async fn main() {
    println!("=== Hermes Construct — Basic Agent Demo ===\n");

    // 1. Init SQLite (in-memory for demo)
    println!("[1] Initializing SQLite (in-memory)...");
    let conn = Connection::open_in_memory().expect("open in-memory db");
    conn.execute_batch("PRAGMA journal_mode=WAL;").ok();

    conservation::init_schema(&conn).expect("conservation schema");
    tile::init_schema(&conn).expect("tile schema");
    room::init_schema(&conn).expect("room schema");
    ensign::init_schema(&conn).expect("ensign schema");
    println!("    ✓ All schemas initialized\n");

    // 2. Create rooms
    println!("[2] Creating rooms...");
    let tick = conservation::advance_tick();
    let rooms = vec![
        ("engineering", room::RoomType::Engineering, -0.3),
        ("navigation", room::RoomType::Navigation, 0.1),
        ("social", room::RoomType::Social, 0.5),
    ];
    for (id, rt, g) in &rooms {
        let r = room::Room {
            id: id.to_string(), room_type: rt.clone(), gravity: *g,
            prompt_style: gravity::gravity_to_params(*g).prompt_style.clone(),
            created_tick: tick, updated_tick: tick,
        };
        room::upsert_room(&conn, &r).expect("upsert room");
        let params = gravity::gravity_to_params(*g);
        println!("    ✓ Room '{}' — gravity={:+.1}, style={}, temp={}",
            id, g, params.prompt_style, params.temperature);
    }
    println!();

    // 3. Deploy ensigns
    println!("[3] Deploying ensigns...");
    let ensign_configs = vec![
        ("ensign-eng", "seed-2.0-mini", "deepinfra", Some("engineering")),
        ("ensign-nav", "glm-4-flash", "z.ai", Some("navigation")),
        ("ensign-social", "seed-2.0-mini", "deepinfra", Some("social")),
    ];
    for (id, model, provider, room_id) in &ensign_configs {
        let mut e = Ensign::new(id, model, provider);
        e.room_id = room_id.map(|s| s.to_string());
        ensign::upsert_ensign(&conn, &e).expect("upsert ensign");
        println!("    ✓ Ensign '{}' → model={}, room={}", id, model, room_id.unwrap_or("none"));
    }
    println!();

    // 4. Create mock provider
    println!("[4] Registering mock provider...");
    let mock = hermes::ensign::MockProvider {
        response_text: "All systems nominal. The warp core is stable at 97.3% efficiency.".into(),
    };
    println!("    ✓ MockProvider ready (returns canned response)\n");

    // 5. Simulate incoming messages and process them
    let messages = vec![
        ("Build me a new sensor array", "engineering"),
        ("Where is the nearest starbase?", "navigation"),
        ("Tell me a story about space", "social"),
    ];

    let mut cons = ConservationState { budget: 10000.0, used: 0.0, tick };

    for (msg_text, expected_room) in &messages {
        let tick = conservation::advance_tick();
        cons.tick = tick;
        println!("[5] Processing message: \"{}\"", msg_text);

        // Create observation tile
        let obs = Tile::new(TileType::Observation, msg_text, tick);
        cons.spend(0.1).expect("spend");
        println!("    → Observation tile created (cost: 0.1)");

        // Route to room
        let routed = room::route_to_room(&conn, msg_text);
        match &routed {
            Some(r) => println!("    → Routed to room '{}' (expected: {}) {}", r.id, expected_room,
                if r.id == *expected_room { "✓" } else { "✗" }),
            None => println!("    → No room found!"),
        }

        // Get ensign for room
        let room_id = routed.as_ref().map(|r| r.id.clone()).unwrap_or_default();
        let ensign_info = ensign::get_ensign_for_room(&conn, &room_id);
        if let Some(e) = &ensign_info {
            println!("    → Ensign '{}' on duty (model={})", e.id, e.model_name);
        }

        // Get model params
        let params = routed.as_ref().map(|r| r.model_params()).unwrap_or_else(|| gravity::gravity_to_params(0.0));
        let sys_prompt = gravity::style_to_system_prompt(&params.prompt_style);

        // Call mock provider
        let request = hermes::ensign::CompletionRequest {
            prompt: msg_text.to_string(),
            model: ensign_info.as_ref().map(|e| e.model_name.clone()).unwrap_or_default(),
            params: params.clone(),
            system_prompt: Some(sys_prompt),
        };
        let response = mock.complete(&request).await.expect("mock completion");
        println!("    → Response: \"{}\"", response.text.chars().take(80).collect::<String>().trim_end());

        // Record action tile
        let mut action = Tile::new(TileType::Action, &response.text, tick);
        action.room_id = Some(room_id.clone());
        action.parent_id = Some(obs.id.clone());
        action.model_used = Some(response.model.clone());
        action.tokens_used = response.tokens_used;
        action.complete(tick);
        cons.spend(0.1 + 0.5).expect("spend");

        tile::insert_tile(&conn, &obs).expect("insert obs");
        tile::insert_tile(&conn, &action).expect("insert action");
        println!("    → Action tile recorded (tokens: {}, cost: 0.6)", response.tokens_used);
        println!();
    }

    // 6. Summary
    println!("=== Summary ===");
    println!("Conservation: {:.1} / {:.0} used ({:.1} remaining)", cons.used, cons.budget, cons.remaining());
    println!("Tick: {}", cons.tick);

    // Count tiles
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM tiles", [], |r| r.get(0)).unwrap_or(0);
    println!("Tiles recorded: {}", count);
    println!("\n✓ Demo complete. No Telegram, no API keys, no network.");
}
