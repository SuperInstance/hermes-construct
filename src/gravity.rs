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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gravity_negative_extreme_is_precise() {
        let p = gravity_to_params(-1.0);
        assert_eq!(p.prompt_style, "precise");
        assert!((p.temperature - 0.3).abs() < f64::EPSILON);
        assert_eq!(p.max_tokens, 500);
    }

    #[test]
    fn gravity_negative_moderate_is_balanced() {
        let p = gravity_to_params(-0.3);
        assert_eq!(p.prompt_style, "balanced");
        assert!((p.temperature - 0.5).abs() < f64::EPSILON);
        assert_eq!(p.max_tokens, 1000);
    }

    #[test]
    fn gravity_zero_is_creative() {
        let p = gravity_to_params(0.0);
        assert_eq!(p.prompt_style, "creative");
        assert!((p.temperature - 0.7).abs() < f64::EPSILON);
    }

    #[test]
    fn gravity_positive_moderate_is_creative() {
        let p = gravity_to_params(0.25);
        assert_eq!(p.prompt_style, "creative");
        assert_eq!(p.max_tokens, 2000);
    }

    #[test]
    fn gravity_positive_extreme_is_narrative() {
        let p = gravity_to_params(0.6);
        assert_eq!(p.prompt_style, "narrative");
        assert!((p.temperature - 0.9).abs() < f64::EPSILON);
        assert_eq!(p.max_tokens, 4000);
    }

    #[test]
    fn gravity_one_is_narrative() {
        let p = gravity_to_params(1.0);
        assert_eq!(p.prompt_style, "narrative");
    }

    #[test]
    fn gravity_clamps_above_one() {
        let p = gravity_to_params(5.0);
        assert_eq!(p.prompt_style, "narrative");
    }

    #[test]
    fn gravity_clamps_below_minus_one() {
        let p = gravity_to_params(-5.0);
        assert_eq!(p.prompt_style, "precise");
    }

    #[test]
    fn style_to_system_prompt_known_styles() {
        assert!(style_to_system_prompt("precise").contains("precise"));
        assert!(style_to_system_prompt("balanced").contains("balanced"));
        assert!(style_to_system_prompt("creative").contains("creative"));
        assert!(style_to_system_prompt("narrative").contains("narrative"));
    }

    #[test]
    fn style_to_system_prompt_unknown_falls_back() {
        let s = style_to_system_prompt("unknown");
        assert!(s.contains("helpful"));
    }
}
