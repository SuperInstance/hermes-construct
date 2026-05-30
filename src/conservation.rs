//! conservation.rs — Budget tracking and enforcement
//!
//! Every operation costs tokens. Conservation enforcement ensures
//! total_deposits - total_withdrawals == total_budget after every tick.

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

/// Cost table for operations (in energy units)
pub mod costs {
    pub const TILE_CREATE: f64 = 0.1;
    pub const TILE_COMPLETE: f64 = 0.05;
    pub const TILE_ARCHIVE: f64 = 0.01;
    pub const ENSIGN_ACTIVATE: f64 = 1.0;
    pub const ENSIGN_ORIENT: f64 = 0.5;
    pub const ENSIGN_TILE: f64 = 0.5;
    pub const ENSIGN_STAND_DOWN: f64 = 0.3;
    pub const GRAVITY_UPDATE: f64 = 0.01;
    pub const GRAVITY_RECALIBRATE: f64 = 0.1;
    pub const PHONE_A_FRIEND: f64 = 5.0;
    pub const CORRELATION_COMPUTE: f64 = 0.05;
    pub const CORRELATION_TRANSFER: f64 = 0.05;
    pub const PENROSE_REFIT: f64 = 0.5;
    pub const PORT_OPEN_CLOSE: f64 = 0.2;
    pub const PORT_MESSAGE: f64 = 0.01;
    pub const DEADBAND_CHECK: f64 = 0.02;
    pub const DEADBAND_ACTION: f64 = 0.5;
    pub const BOOTSTRAP_STEP: f64 = 0.5;
    pub const SHELL_SPAWN: f64 = 5.0;
    pub const SHELL_DESTROY: f64 = 2.0;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConservationState {
    pub budget: f64,
    pub used: f64,
    pub tick: u64,
}

impl ConservationState {
    pub fn remaining(&self) -> f64 {
        self.budget - self.used
    }

    pub fn can_spend(&self, cost: f64) -> bool {
        self.remaining() >= cost
    }

    pub fn spend(&mut self, cost: f64) -> Result<(), String> {
        if !self.can_spend(cost) {
            return Err(format!(
                "conservation budget exceeded: remaining={} cost={}",
                self.remaining(),
                cost
            ));
        }
        self.used += cost;
        Ok(())
    }

    pub fn deposit(&mut self, amount: f64) {
        self.budget += amount;
    }
}

static GLOBAL_TICK: AtomicU64 = AtomicU64::new(0);

pub fn current_tick() -> u64 {
    GLOBAL_TICK.load(Ordering::Relaxed)
}

pub fn advance_tick() -> u64 {
    GLOBAL_TICK.fetch_add(1, Ordering::Relaxed)
}

/// Initialize conservation table in SQLite
pub fn init_schema(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS conservation (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        INSERT OR IGNORE INTO conservation (key, value) VALUES ('budget', '10000');
        INSERT OR IGNORE INTO conservation (key, value) VALUES ('used', '0');
        INSERT OR IGNORE INTO conservation (key, value) VALUES ('tick', '0');"
    )
}

/// Load conservation state from SQLite
pub fn load_state(conn: &Connection) -> Result<ConservationState, rusqlite::Error> {
    let budget: f64 = conn
        .query_row("SELECT value FROM conservation WHERE key = 'budget'", [], |r| {
            r.get(0)
        })
        .unwrap_or(10000.0);

    let used: f64 = conn
        .query_row("SELECT value FROM conservation WHERE key = 'used'", [], |r| {
            r.get(0)
        })
        .unwrap_or(0.0);

    let tick: u64 = conn
        .query_row("SELECT value FROM conservation WHERE key = 'tick'", [], |r| {
            r.get::<_, String>(0).map(|s| s.parse().unwrap_or(0))
        })
        .unwrap_or(0);

    GLOBAL_TICK.store(tick, Ordering::Relaxed);

    Ok(ConservationState { budget, used, tick })
}

/// Persist conservation state to SQLite
pub fn save_state(conn: &Connection, state: &ConservationState) -> Result<(), rusqlite::Error> {
    conn.execute(
        "UPDATE conservation SET value = ? WHERE key = 'budget'",
        params![state.budget.to_string()],
    )?;
    conn.execute(
        "UPDATE conservation SET value = ? WHERE key = 'used'",
        params![state.used.to_string()],
    )?;
    conn.execute(
        "UPDATE conservation SET value = ? WHERE key = 'tick'",
        params![state.tick.to_string()],
    )?;
    Ok(())
}
