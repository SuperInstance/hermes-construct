//! gravity.rs — JEPA gravity per room → model params
//!
//! Maps a single gravity scalar (-1.0 to +1.0) to algorithmic model parameters.
//! Negative = precise, zero = balanced, positive = creative/narrative.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelParams {
    pub temperature: f64,
    pub prompt_style: String,
    pub max_tokens: u32,
    pub top_p: f64,
    pub frequency_penalty: f64,
    pub presence_penalty: f64,
}

/// Map gravity value to model parameters.
///
/// | Gravity Range  | Temp | Style      | Tokens | Top P |
/// |----------------|------|------------|--------|-------|
/// | -1.0 to -0.5   | 0.3  | precise    | 500    | 0.9   |
/// | -0.5 to 0.0    | 0.5  | balanced   | 1000   | 0.95  |
/// | 0.0 to 0.5     | 0.7  | creative   | 2000   | 0.95  |
/// | 0.5 to 1.0     | 0.9  | narrative  | 4000   | 0.95  |
pub fn gravity_to_params(gravity: f64) -> ModelParams {
    let g = gravity.clamp(-1.0, 1.0);

    if g < -0.5 {
        ModelParams {
            temperature: 0.3,
            prompt_style: "precise".to_string(),
            max_tokens: 500,
            top_p: 0.9,
            frequency_penalty: 0.3,
            presence_penalty: 0.1,
        }
    } else if g < 0.0 {
        ModelParams {
            temperature: 0.5,
            prompt_style: "balanced".to_string(),
            max_tokens: 1000,
            top_p: 0.95,
            frequency_penalty: 0.1,
            presence_penalty: 0.1,
        }
    } else if g < 0.5 {
        ModelParams {
            temperature: 0.7,
            prompt_style: "creative".to_string(),
            max_tokens: 2000,
            top_p: 0.95,
            frequency_penalty: 0.0,
            presence_penalty: 0.2,
        }
    } else {
        ModelParams {
            temperature: 0.9,
            prompt_style: "narrative".to_string(),
            max_tokens: 4000,
            top_p: 0.95,
            frequency_penalty: 0.0,
            presence_penalty: 0.3,
        }
    }
}

/// Build a system prompt prefix based on prompt style
pub fn style_to_system_prompt(style: &str) -> String {
    match style {
        "precise" => "Be precise and concise. State facts. No hedging.".to_string(),
        "balanced" => "Be balanced and helpful. Provide clear explanations.".to_string(),
        "creative" => "Be creative and thoughtful. Explore ideas freely.".to_string(),
        "narrative" => "Tell stories. Use rich narrative. Be expressive.".to_string(),
        _ => "Be helpful and conversational.".to_string(),
    }
}
