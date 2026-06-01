//! basic_agent.rs — Full hermes-construct flow WITHOUT Telegram or external APIs
//!
//! Uses a local MockProvider. Runs standalone:
//!   cargo run --example basic_agent
//!
//! What it shows:
//!   1. Init SQLite (in-memory)
//!   2. Create rooms programmatically
//!   3. Deploy an ensign
//!   4. Route messages to rooms (keyword matching)
//!   5. Generate responses via MockProvider
//!   6. Record tiles (observation + action)
//!   7. Print full trace

use hermes_construct::conservation::{self, ConservationState};
use hermes_construct::ensign::{self, CompletionRequest, CompletionResponse, Ensign, Provider};
use hermes_construct::gravity;
use hermes_construct::room;
use hermes_construct::tile::{self, Tile, TileType};

use async_trait::async_trait;
use rusqlite::Connection;

// ---------------------------------------------------------------------------
// Mock provider (local to this example — not in the library)
// ---------------------------------------------------------------------------

struct MockProvider {
    response_text: String,
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

    fn name(&self) -> &str {
        "mock"
    }
}

// ---------------------------------------------------------------------------
// Demo
// ---------------------------------------------------------------------------

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
        let params = gravity::gravity_to_params(*g);
        let r = room::Room {
            id: id.to_string(),
            room_type: rt.clone(),
            gravity: *g,
            gravity_confidence: 0.5,
            temperature: params.temperature,
            max_tokens: params.max_tokens,
            prompt_style: params.prompt_style.clone(),
            deadband_tolerance: 0.1,
            ensign_id: None,
            config: None,
            created_tick: tick,
            updated_tick: tick,
        };
        room::upsert_room(&conn, &r).expect("upsert room");
        println!(
            "    ✓ Room '{}' — gravity={:+.1}, style={}, temp={}",
            id, g, params.prompt_style, params.temperature
        );
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
        println!(
            "    ✓ Ensign '{}' → model={}, room={}",
            id,
            model,
            room_id.unwrap_or("none")
        );
    }
    println!();

    // 4. Create mock provider
    println!("[4] Registering mock provider...");
    let mock = MockProvider {
        response_text: "All systems nominal. The warp core is stable at 97.3% efficiency.".into(),
    };
    println!("    ✓ MockProvider ready (returns canned response)\n");

    // 5. Simulate incoming messages and process them
    let messages = vec![
        ("Build me a new sensor array", "engineering"),
        ("Where is the nearest starbase?", "navigation"),
        ("Tell me a story about space", "social"),
    ];

    let mut cons = ConservationState {
        budget: 10000.0,
        used: 0.0,
        tick,
    };

    for (msg_text, expected_room) in &messages {
        let tick = conservation::advance_tick();
        cons.tick = tick;
        println!("[5] Processing message: \"{}\"", msg_text);

        // Create observation tile
        let obs = Tile::new(TileType::Observation, msg_text, tick);
        cons.spend(0.1).expect("spend");
        println!("    → Observation tile created (cost: 0.1)");

        // Route to room
        let routed = room::route_to_room(&conn, msg_text).expect("route");
        match &routed {
            Some(r) => println!(
                "    → Routed to room '{}' (expected: {}) {}",
                r.id,
                expected_room,
                if r.id == *expected_room { "✓" } else { "✗" }
            ),
            None => println!("    → No room found!"),
        }

        // Get ensign for room
        let room_id = routed.as_ref().map(|r| r.id.clone()).unwrap_or_default();
        let ensign_info = ensign::get_ensign_for_room(&conn, &room_id).expect("get ensign");
        if let Some(e) = &ensign_info {
            println!("    → Ensign '{}' on duty (model={})", e.id, e.model_name);
        }

        // Get model params
        let params = routed
            .as_ref()
            .map(|r| r.model_params())
            .unwrap_or_else(|| gravity::gravity_to_params(0.0));
        let sys_prompt = gravity::style_to_system_prompt(&params.prompt_style);

        // Call mock provider
        let request = CompletionRequest {
            prompt: msg_text.to_string(),
            model: ensign_info
                .as_ref()
                .map(|e| e.model_name.clone())
                .unwrap_or_default(),
            params: params.clone(),
            system_prompt: Some(sys_prompt),
        };
        let response = mock.complete(&request).await.expect("mock completion");
        println!(
            "    → Response: \"{}\"",
            response
                .text
                .chars()
                .take(80)
                .collect::<String>()
                .trim_end()
        );

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
        println!(
            "    → Action tile recorded (tokens: {}, cost: 0.6)",
            response.tokens_used
        );
        println!();
    }

    // 6. Summary
    println!("=== Summary ===");
    println!(
        "Conservation: {:.1} / {:.0} used ({:.1} remaining)",
        cons.used,
        cons.budget,
        cons.remaining()
    );
    println!("Tick: {}", cons.tick);

    // Count tiles
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM tiles", [], |r| r.get(0))
        .unwrap_or(0);
    println!("Tiles recorded: {}", count);
    println!("\n✓ Demo complete. No Telegram, no API keys, no network.");
}
