//! provenance_demo.rs — Full provenance flow
//!
//! Shows creating provenance entries, adding alternatives, validating,
//! and rendering as markdown. In hermes-construct, every tile carries
//! provenance: who generated it, with what model, at what cost.
//!
//! Run: cargo run --example provenance_demo

use rusqlite::{params, Connection};

// ---- Inline provenance types ----

#[derive(Debug, Clone)]
struct ProvenanceEntry {
    id: String,
    tile_id: String,
    ensign_id: String,
    model: String,
    provider: String,
    prompt_style: String,
    temperature: f64,
    tokens_used: u32,
    conservation_cost: f64,
    room_id: String,
    tick: u64,
    parent_provenance: Option<String>, // links to parent tile's provenance
    alternatives: Vec<ProvenanceAlternative>,
    is_valid: bool,
}

#[derive(Debug, Clone)]
struct ProvenanceAlternative {
    id: String,
    text: String,
    model: String,
    temperature: f64,
    tokens_used: u32,
    score: f64, // quality score
    rejected: bool,
    rejection_reason: Option<String>,
}

impl ProvenanceEntry {
    fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if self.tile_id.is_empty() { errors.push("missing tile_id".into()); }
        if self.ensign_id.is_empty() { errors.push("missing ensign_id".into()); }
        if self.model.is_empty() { errors.push("missing model".into()); }
        if self.provider.is_empty() { errors.push("missing provider".into()); }
        if self.temperature < 0.0 || self.temperature > 2.0 {
            errors.push(format!("invalid temperature: {}", self.temperature));
        }
        if self.conservation_cost < 0.0 {
            errors.push(format!("negative conservation cost: {}", self.conservation_cost));
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    fn render_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str(&format!("## Provenance: `{}`\n\n", &self.id[..8]));
        md.push_str(&format!("**Tile:** `{}`  \n", &self.tile_id[..8]));
        md.push_str(&format!("**Ensign:** `{}` ({})  \n", self.ensign_id, self.model));
        md.push_str(&format!("**Provider:** {}  \n", self.provider));
        md.push_str(&format!("**Room:** {}  \n", self.room_id));
        md.push_str(&format!("**Style:** {} (temp={:.2})  \n", self.prompt_style, self.temperature));
        md.push_str(&format!("**Tokens:** {}  \n", self.tokens_used));
        md.push_str(&format!("**Cost:** {:.3} energy units  \n", self.conservation_cost));
        md.push_str(&format!("**Tick:** {}  \n", self.tick));
        md.push_str(&format!("**Valid:** {}  \n", if self.is_valid { "✓" } else { "✗" }));

        if let Some(parent) = &self.parent_provenance {
            md.push_str(&format!("\n**Parent provenance:** `{}`  \n", &parent[..8]));
        }

        if !self.alternatives.is_empty() {
            md.push_str("\n### Alternatives\n\n");
            for (i, alt) in self.alternatives.iter().enumerate() {
                md.push_str(&format!("{}. **{}** (temp={:.2}, tokens={}) — score={:.2}",
                    i + 1, alt.model, alt.temperature, alt.tokens_used, alt.score));
                if alt.rejected {
                    md.push_str(&format!(" ❌ *rejected: {}*",
                        alt.rejection_reason.as_deref().unwrap_or("unknown")));
                } else {
                    md.push_str(" ✅ *selected*");
                }
                md.push('\n');
            }
        }

        md
    }
}

fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║  Hermes Construct — Provenance Demo          ║");
    println!("║  Full chain-of-custody tracking              ║");
    println!("╚══════════════════════════════════════════════╝\n");

    let conn = Connection::open_in_memory().expect("db");
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS provenance (
            id TEXT PRIMARY KEY,
            tile_id TEXT NOT NULL,
            ensign_id TEXT NOT NULL,
            model TEXT NOT NULL,
            provider TEXT NOT NULL,
            temperature REAL,
            tokens_used INTEGER,
            conservation_cost REAL,
            room_id TEXT,
            tick INTEGER,
            parent_provenance TEXT,
            is_valid BOOLEAN DEFAULT 1
        );
        CREATE TABLE IF NOT EXISTS provenance_alternatives (
            id TEXT PRIMARY KEY,
            provenance_id TEXT NOT NULL,
            text TEXT NOT NULL,
            model TEXT NOT NULL,
            temperature REAL,
            tokens_used INTEGER,
            score REAL,
            rejected BOOLEAN DEFAULT 0,
            rejection_reason TEXT
        );"
    ).unwrap();

    // 1. Create provenance entry for an observation tile
    println!("[1] Creating provenance entry for observation tile...");

    let obs_provenance = ProvenanceEntry {
        id: uuid::Uuid::new_v4().to_string(),
        tile_id: uuid::Uuid::new_v4().to_string(),
        ensign_id: "ensign-eng".into(),
        model: "seed-2.0-mini".into(),
        provider: "deepinfra".into(),
        prompt_style: "precise".into(),
        temperature: 0.3,
        tokens_used: 12,
        conservation_cost: 0.1,
        room_id: "engineering".into(),
        tick: 1,
        parent_provenance: None,
        alternatives: vec![],
        is_valid: true,
    };

    conn.execute(
        "INSERT INTO provenance (id,tile_id,ensign_id,model,provider,temperature,tokens_used,conservation_cost,room_id,tick,parent_provenance,is_valid) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)",
        params![obs_provenance.id, obs_provenance.tile_id, obs_provenance.ensign_id,
            obs_provenance.model, obs_provenance.provider, obs_provenance.temperature,
            obs_provenance.tokens_used as i64, obs_provenance.conservation_cost,
            obs_provenance.room_id, obs_provenance.tick as i64,
            obs_provenance.parent_provenance, obs_provenance.is_valid],
    ).unwrap();
    println!("    ✓ Observation provenance recorded\n");

    // 2. Create provenance for action tile with alternatives
    println!("[2] Creating provenance entry for action tile (with alternatives)...");

    let action_provenance = ProvenanceEntry {
        id: uuid::Uuid::new_v4().to_string(),
        tile_id: uuid::Uuid::new_v4().to_string(),
        ensign_id: "ensign-eng".into(),
        model: "seed-2.0-mini".into(),
        provider: "deepinfra".into(),
        prompt_style: "precise".into(),
        temperature: 0.3,
        tokens_used: 87,
        conservation_cost: 0.6,
        room_id: "engineering".into(),
        tick: 2,
        parent_provenance: Some(obs_provenance.id.clone()),
        alternatives: vec![
            ProvenanceAlternative {
                id: uuid::Uuid::new_v4().to_string(),
                text: "The sensor array requires recalibration. Estimated time: 4 hours.".into(),
                model: "seed-2.0-mini".into(),
                temperature: 0.3,
                tokens_used: 87,
                score: 0.92,
                rejected: false,
                rejection_reason: None,
            },
            ProvenanceAlternative {
                id: uuid::Uuid::new_v4().to_string(),
                text: "Sensors are broken. We should replace them.".into(),
                model: "glm-4-flash".into(),
                temperature: 0.7,
                tokens_used: 42,
                score: 0.45,
                rejected: true,
                rejection_reason: Some("Too vague, lacks specificity".into()),
            },
            ProvenanceAlternative {
                id: uuid::Uuid::new_v4().to_string(),
                text: "Recalibrating the main deflector dish could resolve the sensor issue.".into(),
                model: "seed-2.0-mini".into(),
                temperature: 0.3,
                tokens_used: 63,
                score: 0.78,
                rejected: true,
                rejection_reason: Some("Conflates unrelated systems".into()),
            },
        ],
        is_valid: true,
    };

    // Store main provenance
    conn.execute(
        "INSERT INTO provenance (id,tile_id,ensign_id,model,provider,temperature,tokens_used,conservation_cost,room_id,tick,parent_provenance,is_valid) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)",
        params![action_provenance.id, action_provenance.tile_id, action_provenance.ensign_id,
            action_provenance.model, action_provenance.provider, action_provenance.temperature,
            action_provenance.tokens_used as i64, action_provenance.conservation_cost,
            action_provenance.room_id, action_provenance.tick as i64,
            action_provenance.parent_provenance, action_provenance.is_valid],
    ).unwrap();

    // Store alternatives
    for alt in &action_provenance.alternatives {
        conn.execute(
            "INSERT INTO provenance_alternatives (id,provenance_id,text,model,temperature,tokens_used,score,rejected,rejection_reason) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
            params![alt.id, action_provenance.id, alt.text, alt.model, alt.temperature,
                alt.tokens_used as i64, alt.score, alt.rejected, alt.rejection_reason],
        ).unwrap();
    }
    println!("    ✓ Action provenance recorded with {} alternatives\n", action_provenance.alternatives.len());

    // 3. Validate
    println!("[3] Validating provenance entries...");
    match obs_provenance.validate() {
        Ok(()) => println!("    ✓ Observation provenance: VALID"),
        Err(e) => println!("    ✗ Observation provenance: INVALID — {:?}", e),
    }
    match action_provenance.validate() {
        Ok(()) => println!("    ✓ Action provenance: VALID"),
        Err(e) => println!("    ✗ Action provenance: INVALID — {:?}", e),
    }
    println!();

    // 4. Render markdown
    println!("[4] Markdown rendering:\n");
    println!("---");
    println!("{}", obs_provenance.render_markdown());
    println!();
    println!("{}", action_provenance.render_markdown());
    println!("---\n");

    // 5. Query the chain
    println!("[5] Provenance chain traversal:");
    let chain_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM provenance WHERE is_valid = 1", [], |r| r.get(0)
    ).unwrap();
    let alt_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM provenance_alternatives", [], |r| r.get(0)
    ).unwrap();
    let selected_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM provenance_alternatives WHERE rejected = 0", [], |r| r.get(0)
    ).unwrap();
    let rejected_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM provenance_alternatives WHERE rejected = 1", [], |r| r.get(0)
    ).unwrap();
    let total_cost: f64 = conn.query_row(
        "SELECT SUM(conservation_cost) FROM provenance", [], |r| r.get(0)
    ).unwrap_or(0.0);
    let total_tokens: i64 = conn.query_row(
        "SELECT SUM(tokens_used) FROM provenance", [], |r| r.get(0)
    ).unwrap_or(0);

    println!("    Valid provenance entries: {}", chain_count);
    println!("    Alternatives generated: {} (selected: {}, rejected: {})", alt_count, selected_count, rejected_count);
    println!("    Total conservation cost: {:.3} energy units", total_cost);
    println!("    Total tokens consumed: {}", total_tokens);

    println!("\n╔══════════════════════════════════════════════╗");
    println!("║  ✓ Provenance demo complete                  ║");
    println!("║  Every tile has a chain of custody.          ║");
    println!("║  Every alternative is tracked. Nothing is    ║");
    println!("║  lost. Nothing is free.                      ║");
    println!("╚══════════════════════════════════════════════╝");
}
