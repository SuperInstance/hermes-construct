//! circuit_demo.rs — Deadband circuit monitoring
//!
//! Creates monitoring circuits, feeds values, shows status changes
//! as values go in and out of the deadband. Demonstrates:
//!   1. Creating circuits with setpoints and tolerances
//!   2. Feeding time-series values
//!   3. Detecting breaches
//!   4. Trend detection (stable, drifting, oscillating, diverging)
//!
//! Run: cargo run --example circuit_demo

use rusqlite::{params, Connection};

// ---- Inline deadband types ----

#[derive(Debug, Clone, PartialEq)]
enum Trend {
    Stable,
    Drifting,
    Oscillating,
    Diverging,
}

impl Trend {
    fn emoji(&self) -> &str {
        match self {
            Self::Stable => "🟢", Self::Drifting => "🟡",
            Self::Oscillating => "🟠", Self::Diverging => "🔴",
        }
    }
}

#[derive(Debug, Clone)]
struct DeadbandCircuit {
    id: String,
    name: String,
    room_id: String,
    setpoint: f64,
    tolerance: f64,
    action: String,
    last_value: Option<f64>,
    consecutive_breaches: u32,
    is_breached: bool,
    history: Vec<f64>,
}

impl DeadbandCircuit {
    fn new(id: &str, name: &str, room_id: &str, setpoint: f64, tolerance: f64, action: &str) -> Self {
        Self {
            id: id.to_string(), name: name.to_string(), room_id: room_id.to_string(),
            setpoint, tolerance, action: action.to_string(),
            last_value: None, consecutive_breaches: 0, is_breached: false, history: Vec::new(),
        }
    }

    fn check(&mut self, value: f64) -> (bool, Trend) {
        let lower = self.setpoint - self.tolerance;
        let upper = self.setpoint + self.tolerance;
        self.last_value = Some(value);
        self.history.push(value);

        let in_band = value >= lower && value <= upper;

        if !in_band {
            self.consecutive_breaches += 1;
            self.is_breached = true;
        } else {
            self.consecutive_breaches = 0;
            self.is_breached = false;
        }

        let trend = self.detect_trend();
        (in_band, trend)
    }

    fn detect_trend(&self) -> Trend {
        if self.history.len() < 3 { return Trend::Stable; }
        let recent: Vec<f64> = self.history.iter().rev().take(10).cloned().collect::<Vec<_>>().into_iter().rev().collect();
        let deltas: Vec<f64> = recent.windows(2).map(|w| w[1] - w[0]).collect();

        // Check oscillation
        let sign_changes: usize = deltas.windows(2)
            .filter(|w| w[0].signum() != w[1].signum()).count();
        if deltas.len() > 1 && sign_changes as f64 / (deltas.len() - 1) as f64 > 0.6 {
            return Trend::Oscillating;
        }

        // Check divergence
        if self.consecutive_breaches > 10 { return Trend::Diverging; }

        // Check drift
        if deltas.len() > 0 {
            let mean: f64 = deltas.iter().sum::<f64>() / deltas.len() as f64;
            if mean.abs() > 0.01 { return Trend::Drifting; }
        }

        Trend::Stable
    }

    fn status_line(&self) -> String {
        let v = self.last_value.map(|v| format!("{:.3}", v)).unwrap_or("---".into());
        let lower = self.setpoint - self.tolerance;
        let upper = self.setpoint + self.tolerance;
        let trend = self.detect_trend();
        let breach_mark = if self.is_breached { "⚠️" } else { "✓" };
        format!("{} {} = {:>8} [{:.1} .. {:.1}] {} breaches={}",
            breach_mark, self.name, v, lower, upper,
            trend.emoji(), self.consecutive_breaches)
    }
}

fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║  Hermes Construct — Circuit Demo             ║");
    println!("║  Deadband monitoring & trend detection        ║");
    println!("╚══════════════════════════════════════════════╝\n");

    let conn = Connection::open_in_memory().expect("db");
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS circuit_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            circuit_id TEXT, value REAL, in_band BOOLEAN, trend TEXT, tick INTEGER
        );"
    ).unwrap();

    // 1. Create monitoring circuits
    println!("[1] Creating monitoring circuits...\n");

    let mut circuits = vec![
        DeadbandCircuit::new("c1", "warp-core-temp", "engineering", 1000.0, 50.0, "alert"),
        DeadbandCircuit::new("c2", "shield-integrity", "security", 0.95, 0.05, "recalibrate"),
        DeadbandCircuit::new("c3", "crew-morale", "social", 0.7, 0.2, "organize_event"),
    ];

    for c in &circuits {
        let lower = c.setpoint - c.tolerance;
        let upper = c.setpoint + c.tolerance;
        println!("    📊 {} — setpoint={:.1}, band=[{:.1} .. {:.1}], action={}",
            c.name, c.setpoint, lower, upper, c.action);
    }

    // 2. Feed time-series values
    println!("\n[2] Feeding values (simulated time series)...\n");

    // Scenario: warp core drifts hot, then stabilizes
    let warp_values: Vec<f64> = vec![
        1000.0, 1005.0, 1012.0, 1020.0, 1030.0, // drifting up
        1042.0, 1050.0, 1051.0, 1048.0, 1040.0, // breach, then recovering
        1030.0, 1015.0, 1005.0, 1000.0, 998.0,  // stabilizing
    ];

    // Scenario: shields oscillating
    let shield_values: Vec<f64> = vec![
        0.95, 0.94, 0.96, 0.91, 0.97, 0.90, 0.93, 0.98,
        0.92, 0.95, 0.89, 0.94, 0.96, 0.93, 0.95,
    ];

    // Scenario: morale stable then sudden drop
    let morale_values: Vec<f64> = vec![
        0.72, 0.73, 0.71, 0.72, 0.70, 0.71,
        0.65, 0.55, 0.45, 0.40, 0.38,  // sudden drop
        0.42, 0.50, 0.58, 0.65,         // recovery
    ];

    // Feed warp core directly
    println!("    Warp Core Temperature:");
    for (tick, val) in warp_values.iter().enumerate() {
        let (in_band, trend) = circuits[0].check(*val);
        conn.execute(
            "INSERT INTO circuit_log (circuit_id, value, in_band, trend, tick) VALUES (?1,?2,?3,?4,?5)",
            params!["c1", val, in_band, format!("{:?}", trend), tick as i64],
        ).unwrap();
        println!("      t={:2}  {}", tick, circuits[0].status_line());
    }
    println!();

    // Feed shields
    println!("    Shield Integrity:");
    for (tick, val) in shield_values.iter().enumerate() {
        let (in_band, trend) = circuits[1].check(*val);
        conn.execute(
            "INSERT INTO circuit_log (circuit_id, value, in_band, trend, tick) VALUES (?1,?2,?3,?4,?5)",
            params!["c2", val, in_band, format!("{:?}", trend), tick as i64],
        ).unwrap();
        println!("      t={:2}  {}", tick, circuits[1].status_line());
    }
    println!();

    // Feed morale
    println!("    Crew Morale:");
    for (tick, val) in morale_values.iter().enumerate() {
        let (in_band, trend) = circuits[2].check(*val);
        conn.execute(
            "INSERT INTO circuit_log (circuit_id, value, in_band, trend, tick) VALUES (?1,?2,?3,?4,?5)",
            params!["c3", val, in_band, format!("{:?}", trend), tick as i64],
        ).unwrap();
        println!("      t={:2}  {}", tick, circuits[2].status_line());
    }

    // 3. Summary
    println!("\n[3] Circuit Summary:");
    for c in &circuits {
        let trend = c.detect_trend();
        let status = if c.is_breached { "BREACHED" } else { "NOMINAL" };
        println!("    {} [{}] — last={:?}, trend={:?} {}, breaches={}",
            c.name, status, c.last_value, trend, trend.emoji(), c.consecutive_breaches);
    }

    // 4. DB statistics
    let total_checks: i64 = conn.query_row("SELECT COUNT(*) FROM circuit_log", [], |r| r.get(0)).unwrap();
    let total_breaches: i64 = conn.query_row(
        "SELECT COUNT(*) FROM circuit_log WHERE in_band = 0", [], |r| r.get(0)
    ).unwrap();
    println!("\n[4] Statistics:");
    println!("    Total checks: {}", total_checks);
    println!("    Total breaches: {} ({:.1}%)", total_breaches, total_breaches as f64 / total_checks as f64 * 100.0);

    println!("\n╔══════════════════════════════════════════════╗");
    println!("║  ✓ Circuit demo complete                     ║");
    println!("║  {} circuits monitored, {} breaches detected     ║", circuits.len(), total_breaches);
    println!("╚══════════════════════════════════════════════╝");
}
