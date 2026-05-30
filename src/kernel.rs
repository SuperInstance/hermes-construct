//! kernel.rs — ShellKernel tying it all together
//!
//! The main tick loop, message routing, and conservation budget management.

use rusqlite::Connection;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::conservation::{self, ConservationState, costs};
use crate::deadband;
use crate::ensign::{self, CompletionRequest, Provider};
use crate::gravity;
use crate::penrose;
use crate::port::{Port, PortMessage, PortResponse};
use crate::room;
use crate::tile::{self, Tile, TileType};

/// Helper to convert rusqlite errors to String
fn sql_err(e: rusqlite::Error) -> String {
    format!("{}", e)
}

// ---------------------------------------------------------------------------
// ShellKernel
// ---------------------------------------------------------------------------

pub struct ShellKernel {
    pub db: Arc<Mutex<Connection>>,
    pub providers: Vec<(String, Box<dyn Provider>)>,
    pub ports: Vec<Arc<Mutex<dyn Port>>>,
    pub conservation: Arc<Mutex<ConservationState>>,
    pub gravity_history: Arc<Mutex<HashMap<String, Vec<f64>>>>,
    pub tick_interval_ms: u64,
}

impl ShellKernel {
    /// Bootstrap: init all SQLite schemas, load rooms/ensigns
    pub async fn bootstrap(
        db_path: &str,
        rooms_dir: &str,
        ensigns_dir: &str,
    ) -> Result<Self, String> {
        // Open SQLite with WAL mode
        let conn = Connection::open(db_path)
            .map_err(|e| format!("open db: {}", e))?;

        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA busy_timeout=5000;")
            .map_err(|e| format!("set WAL mode: {}", e))?;

        // Init all schemas
        conservation::init_schema(&conn).map_err(sql_err)?;
        tile::init_schema(&conn).map_err(sql_err)?;
        room::init_schema(&conn).map_err(sql_err)?;
        ensign::init_schema(&conn).map_err(sql_err)?;
        penrose::init_schema(&conn).map_err(sql_err)?;
        deadband::init_schema(&conn).map_err(sql_err)?;

        // Shell metadata
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS shell_meta (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            INSERT OR IGNORE INTO shell_meta (key, value) VALUES ('shell_id', 'hermes-construct');
            INSERT OR IGNORE INTO shell_meta (key, value) VALUES ('shell_type', 'hermes');
            INSERT OR IGNORE INTO shell_meta (key, value) VALUES ('autonomy_level', '1');
            INSERT OR IGNORE INTO shell_meta (key, value) VALUES ('conservation_budget', '10000');"
        ).map_err(|e| format!("shell_meta init: {}", e))?;

        let tick = conservation::current_tick();

        // Load rooms from JSON
        let rooms = room::load_rooms_from_dir(&conn, rooms_dir, tick)?;
        log::info!("Loaded {} rooms from {}", rooms.len(), rooms_dir);

        // Load ensigns from JSON
        let ensigns = ensign::load_ensigns_from_dir(&conn, ensigns_dir)?;
        log::info!("Loaded {} ensigns from {}", ensigns.len(), ensigns_dir);

        // Load conservation state
        let mut cons_state = conservation::load_state(&conn).map_err(sql_err)?;
        cons_state.tick = tick;

        // Initialize gravity history
        let mut gravity_history = HashMap::new();
        for r in &rooms {
            gravity_history.entry(r.id.clone())
                .or_insert_with(|| vec![r.gravity]);
        }

        Ok(Self {
            db: Arc::new(Mutex::new(conn)),
            providers: Vec::new(),
            ports: Vec::new(),
            conservation: Arc::new(Mutex::new(cons_state)),
            gravity_history: Arc::new(Mutex::new(gravity_history)),
            tick_interval_ms: 30_000,
        })
    }

    /// Register a provider
    pub fn add_provider(&mut self, name: &str, provider: Box<dyn Provider>) {
        self.providers.push((name.to_string(), provider));
    }

    /// Register a port
    pub fn add_port(&mut self, port: Arc<Mutex<dyn Port>>) {
        self.ports.push(port);
    }

    /// Process an incoming message through the full pipeline
    pub async fn process_message(&self, msg: &PortMessage) -> Result<(), String> {
        let tick = conservation::advance_tick();

        // 1. Create observation tile
        let mut obs_tile = Tile::new(TileType::Observation, &msg.text, tick);
        obs_tile.conservation_delta = costs::TILE_CREATE;

        {
            let mut cons = self.conservation.lock().await;
            cons.spend(costs::TILE_CREATE)?;
            cons.tick = tick;
        }

        // 2. Route to room
        let room = {
            let db = self.db.lock().await;
            room::route_to_room(&db, &msg.text).map_err(sql_err)?
        };

        let room_id = room.as_ref().map(|r| r.id.clone())
            .unwrap_or_else(|| "default".to_string());

        obs_tile.room_id = Some(room_id.clone());

        // 3. Get ensign for room
        let ensign_info = {
            let db = self.db.lock().await;
            ensign::get_ensign_for_room(&db, &room_id).map_err(sql_err)?
        };

        // 4. Determine model params from room gravity
        let model_params = room.as_ref()
            .map(|r| r.model_params())
            .unwrap_or_else(|| gravity::gravity_to_params(0.0));

        // 5. Find provider and model
        let model_name = ensign_info.as_ref()
            .map(|e| e.model_name.clone())
            .unwrap_or_else(|| "seed-2.0-mini".to_string());

        let provider_name = ensign_info.as_ref()
            .map(|e| e.provider.clone())
            .unwrap_or_else(|| "deepinfra".to_string());

        let system_prompt = gravity::style_to_system_prompt(&model_params.prompt_style);

        // 6. Call API
        let completion_result = {
            let provider = self.providers.iter()
                .find(|(n, _)| n == &provider_name)
                .map(|(_, p)| p.as_ref());

            match provider {
                Some(p) => {
                    let request = CompletionRequest {
                        prompt: msg.text.clone(),
                        model: model_name.clone(),
                        params: model_params.clone(),
                        system_prompt: Some(system_prompt),
                    };
                    p.complete(&request).await
                }
                None => Err(format!("no provider '{}' available", provider_name)),
            }
        };

        // 7. Create action tile
        let (response_text, tokens_used, model_used) = match completion_result {
            Ok(resp) => (resp.text.clone(), resp.tokens_used, resp.model.clone()),
            Err(e) => {
                log::error!("completion error: {}", e);
                ("I encountered an error processing your request.".to_string(), 0, "none".to_string())
            }
        };

        let mut action_tile = Tile::new(TileType::Action, &response_text, tick);
        action_tile.room_id = Some(room_id.clone());
        action_tile.parent_id = Some(obs_tile.id.clone());
        action_tile.model_used = Some(model_used);
        action_tile.tokens_used = tokens_used;
        action_tile.ensign_id = ensign_info.as_ref().map(|e| e.id.clone());

        {
            let mut cons = self.conservation.lock().await;
            let _ = cons.spend(costs::TILE_CREATE + costs::ENSIGN_TILE);
        }

        // 8. Update room gravity
        if let Some(ref room) = room {
            let signal = if tokens_used > 0 { 0.05 } else { -0.05 };
            let mut updated_room = room.clone();
            updated_room.nudge_gravity(signal, 0.1, tick);

            let db = self.db.lock().await;
            let _ = room::upsert_room(&db, &updated_room).map_err(sql_err);

            // Track gravity history
            let mut gh = self.gravity_history.lock().await;
            gh.entry(room.id.clone())
                .or_default()
                .push(updated_room.gravity);
        }

        // 9. Persist tiles
        {
            let db = self.db.lock().await;
            let _ = tile::insert_tile(&db, &obs_tile);
            action_tile.complete(tick);
            let _ = tile::insert_tile(&db, &action_tile);
        }

        // 10. Send response
        for port in &self.ports {
            let p = port.lock().await;
            if p.is_active() {
                let response = PortResponse {
                    text: response_text.clone(),
                    reply_to: msg.chat_id.to_string(),
                };
                let _ = p.send(&response).await;
            }
        }

        // 11. Save conservation state
        {
            let db = self.db.lock().await;
            let cons = self.conservation.lock().await;
            let _ = conservation::save_state(&db, &cons);
        }

        Ok(())
    }

    /// Background tick: decay gravities, scan correlations, check deadbands
    pub async fn background_tick(&self) -> Result<(), String> {
        let tick = conservation::advance_tick();

        let db = self.db.lock().await;

        // Decay room gravities
        let rooms = room::get_all_rooms(&db)
            .map_err(|e| format!("get rooms: {}", e))?;

        for mut r in rooms {
            r.decay_gravity(0.01, tick);
            room::upsert_room(&db, &r).map_err(sql_err)?;

            // Track gravity
            let mut gh = self.gravity_history.lock().await;
            gh.entry(r.id.clone()).or_default().push(r.gravity);
        }

        // Scan correlations (only if we have enough history)
        let gh = self.gravity_history.lock().await;
        let histories: HashMap<String, Vec<f64>> = gh.clone();
        drop(gh);

        if !histories.is_empty() {
            let corrs = penrose::scan_correlations(&db, &histories, tick)
                .map_err(sql_err)?;
            if !corrs.is_empty() {
                log::info!("Detected {} correlations", corrs.len());
            }
        }

        // Check deadbands
        let circuits = deadband::run_checks(&db, tick)
            .map_err(sql_err)?;
        for circuit in &circuits {
            if circuit.is_breached {
                log::warn!(
                    "Deadband breach: {} in room {}",
                    circuit.name, circuit.room_id
                );
            }
        }

        // Update conservation
        {
            let mut cons = self.conservation.lock().await;
            let _ = cons.spend(costs::GRAVITY_UPDATE + costs::CORRELATION_COMPUTE + costs::DEADBAND_CHECK);
            cons.tick = tick;
            let _ = conservation::save_state(&db, &cons);
        }

        Ok(())
    }

    /// Run the main event loop
    pub async fn run(&self) -> Result<(), String> {
        log::info!("Hermes Construct kernel starting...");

        let mut tick_interval = tokio::time::interval(
            std::time::Duration::from_millis(self.tick_interval_ms)
        );

        loop {
            tokio::select! {
                // Poll ports for messages
                _ = async {
                    for port in &self.ports {
                        let p = port.lock().await;
                        if let Some(msg) = p.receive().await {
                            drop(p);
                            if let Err(e) = self.process_message(&msg).await {
                                log::error!("message processing error: {}", e);
                            }
                        }
                    }
                } => {}

                // Background tick
                _ = tick_interval.tick() => {
                    if let Err(e) = self.background_tick().await {
                        log::error!("background tick error: {}", e);
                    }
                }
            }
        }
    }
}
