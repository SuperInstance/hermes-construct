//! correlation_demo.rs — Cross-room correlation detection (Penrose)
//!
//! Feeds correlated signals to two rooms, runs Pearson correlation,
//! classifies spline types, and shows the resulting splines.
//!
//! Run: cargo run --example correlation_demo

use rusqlite::{params, Connection};

// ---- Inline correlation types ----

#[derive(Debug, Clone, PartialEq)]
enum SplineType {
    Causal,
    Resonant,
    Predictive,
    Synergistic,
    Redundant,
}

impl SplineType {
    fn as_str(&self) -> &str {
        match self {
            Self::Causal => "causal", Self::Resonant => "resonant",
            Self::Predictive => "predictive", Self::Synergistic => "synergistic",
            Self::Redundant => "redundant",
        }
    }
}

#[derive(Debug, Clone)]
struct Correlation {
    room_a: String,
    room_b: String,
    coeff: f64,
    spline_type: SplineType,
}

/// Pearson correlation coefficient
fn pearson(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.len() < 2 { return 0.0; }
    let n = x.len() as f64;
    let mx: f64 = x.iter().sum::<f64>() / n;
    let my: f64 = y.iter().sum::<f64>() / n;
    let (mut cov, mut vx, mut vy) = (0.0, 0.0, 0.0);
    for i in 0..x.len() {
        let dx = x[i] - mx;
        let dy = y[i] - my;
        cov += dx * dy;
        vx += dx * dx;
        vy += dy * dy;
    }
    if vx == 0.0 || vy == 0.0 { return 0.0; }
    cov / (vx.sqrt() * vy.sqrt())
}

fn classify(coeff: f64) -> SplineType {
    let a = coeff.abs();
    if a > 0.9 { SplineType::Causal }
    else if a > 0.7 { SplineType::Predictive }
    else if a > 0.5 { SplineType::Synergistic }
    else if a > 0.3 { SplineType::Resonant }
    else { SplineType::Redundant }
}

fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║  Hermes Construct — Correlation Demo         ║");
    println!("║  Penrose cross-room correlation detection     ║");
    println!("╚══════════════════════════════════════════════╝\n");

    // Create in-memory DB
    let conn = Connection::open_in_memory().expect("db");
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS correlations (
            id TEXT PRIMARY KEY, room_a TEXT, room_b TEXT,
            correlation REAL, spline_type TEXT, confidence REAL
        );
        CREATE TABLE IF NOT EXISTS room_gravity_history (
            room_id TEXT, tick INTEGER, gravity REAL
        );"
    ).unwrap();

    // Simulate gravity histories for 4 rooms
    println!("[1] Generating room gravity histories (20 ticks)...\n");

    let rooms = vec!["engineering", "science", "social", "navigation"];

    // Engineering and Science: highly correlated (both drift together)
    // Social: anti-correlated with Engineering
    // Navigation: random noise (no correlation)

    let ticks: Vec<f64> = (0..20).map(|i| i as f64).collect();

    let eng_gravity: Vec<f64> = ticks.iter().map(|t| 0.3 * (t * 0.1).sin() + 0.01 * t).collect();
    let sci_gravity: Vec<f64> = ticks.iter().map(|t| 0.28 * (t * 0.1).sin() + 0.009 * t).collect(); // highly correlated
    let soc_gravity: Vec<f64> = ticks.iter().map(|t| -0.3 * (t * 0.1).sin() - 0.01 * t).collect(); // anti-correlated
    let nav_gravity: Vec<f64> = ticks.iter().map(|t| 0.1 * (t * 0.37).sin()).collect(); // different frequency

    let histories = vec![
        ("engineering", eng_gravity.clone()),
        ("science", sci_gravity.clone()),
        ("social", soc_gravity.clone()),
        ("navigation", nav_gravity.clone()),
    ];

    // Print gravity traces
    for (room, gravities) in &histories {
        let sparkline: String = gravities.iter().map(|g| {
            let normalized = ((g + 1.0) / 2.0 * 8.0) as usize;
            "▁▂▃▄▅▆▇█".chars().nth(normalized.min(7)).unwrap()
        }).collect();
        println!("    {} [{:>12}] {}", room, sparkline, format!("{:+.3}", gravities[0]));
    }

    // Store in DB
    for (room, gravities) in &histories {
        for (i, g) in gravities.iter().enumerate() {
            conn.execute(
                "INSERT INTO room_gravity_history (room_id, tick, gravity) VALUES (?1, ?2, ?3)",
                params![room, i as i64, g],
            ).unwrap();
        }
    }

    // 2. Compute pairwise correlations
    println!("\n[2] Computing pairwise Pearson correlations...\n");

    let room_ids: Vec<String> = histories.iter().map(|(r, _)| r.to_string()).collect();
    let grav_map: std::collections::HashMap<String, &Vec<f64>> = histories.iter()
        .map(|(r, g)| (r.to_string(), g)).collect();

    let mut correlations: Vec<Correlation> = Vec::new();

    for i in 0..room_ids.len() {
        for j in (i+1)..room_ids.len() {
            let a = &room_ids[i];
            let b = &room_ids[j];
            let ga = grav_map[a];
            let gb = grav_map[b];
            let coeff = pearson(ga, gb);
            let spline_type = classify(coeff);

            let corr = Correlation {
                room_a: a.clone(), room_b: b.clone(),
                coeff, spline_type: spline_type.clone(),
            };

            let id = uuid::Uuid::new_v4().to_string();
            conn.execute(
                "INSERT INTO correlations (id,room_a,room_b,correlation,spline_type,confidence) VALUES (?1,?2,?3,?4,?5,?6)",
                params![id, a, b, coeff, spline_type.as_str(), coeff.abs()],
            ).unwrap();

            println!("    {} ↔ {}  Pearson={:+.4}  → {:?}",
                a, b, coeff, spline_type);

            correlations.push(corr);
        }
    }

    // 3. Show detected splines
    println!("\n[3] Detected splines (|r| > 0.3):\n");

    let mut spline_count = 0;
    for c in &correlations {
        if c.coeff.abs() > 0.3 {
            spline_count += 1;
            let strength = if c.coeff > 0.0 { "positive" } else { "negative" };
            println!("    🔗 Spline #{}: {} ↔ {}", spline_count, c.room_a, c.room_b);
            println!("       Type: {:?}, Strength: {}, Coefficient: {:+.4}",
                c.spline_type, strength, c.coeff);
        }
    }

    if spline_count == 0 {
        println!("    (no significant correlations detected)");
    }

    // 4. Summary statistics
    println!("\n[4] Summary:");
    let total: i64 = conn.query_row("SELECT COUNT(*) FROM correlations", [], |r| r.get(0)).unwrap();
    let significant: i64 = conn.query_row(
        "SELECT COUNT(*) FROM correlations WHERE ABS(correlation) > 0.3", [], |r| r.get(0)
    ).unwrap();
    let causal: i64 = conn.query_row(
        "SELECT COUNT(*) FROM correlations WHERE spline_type='causal'", [], |r| r.get(0)
    ).unwrap();

    println!("    Total pairs analyzed: {}", total);
    println!("    Significant (|r| > 0.3): {}", significant);
    println!("    Causal (|r| > 0.9): {}", causal);

    // Verify our expectations
    let eng_sci = correlations.iter().find(|c|
        (c.room_a == "engineering" && c.room_b == "science") ||
        (c.room_a == "science" && c.room_b == "engineering")
    ).unwrap();
    let eng_soc = correlations.iter().find(|c|
        (c.room_a == "engineering" && c.room_b == "social") ||
        (c.room_a == "social" && c.room_b == "engineering")
    ).unwrap();

    println!("\n[5] Verification:");
    println!("    Engineering ↔ Science: {:+.4} (expected ~+0.99) {}",
        eng_sci.coeff, if eng_sci.coeff > 0.9 { "✓" } else { "✗" });
    println!("    Engineering ↔ Social:  {:+.4} (expected ~-0.99) {}",
        eng_soc.coeff, if eng_soc.coeff < -0.9 { "✓" } else { "✗" });

    println!("\n╔══════════════════════════════════════════════╗");
    println!("║  ✓ Correlation demo complete                 ║");
    println!("║  Penrose detected {} significant splines across {} rooms ║", significant, rooms.len());
    println!("╚══════════════════════════════════════════════╝");
}
