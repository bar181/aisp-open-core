//! AISP document validation
//!
//! Implements the AISP 5.1 validation rules including:
//! - Ambiguity check: Ambig(D) < 0.02
//! - Semantic density computation for tier assignment
//! - Required block verification

use crate::symbol::{count_symbols, count_tokens};
use crate::tier::Tier;
use crate::REQUIRED_BLOCKS;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Detailed density metrics
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DensityMetrics {
    /// Required blocks found (out of 5)
    pub blocks_found: u8,
    /// Definition count (≜)
    pub definitions: u16,
    /// Assignment count (≔)
    pub assignments: u16,
    /// Quantifier count (∀, ∃)
    pub quantifiers: u16,
    /// Lambda count (λ)
    pub lambdas: u16,
    /// Implication count (⇒, ⇔, →, ↔)
    pub implications: u16,
    /// Set operation count (∈, ⊆, ∩, ∪, ∅)
    pub set_ops: u16,
    /// Total AISP symbols
    pub symbol_count: u16,
    /// Total non-whitespace tokens
    pub token_count: u16,
}

impl DensityMetrics {
    /// Calculate from source
    pub fn from_source(source: &str) -> Self {
        let blocks_found = REQUIRED_BLOCKS
            .iter()
            .filter(|block| source.contains(*block))
            .count() as u8;

        let definitions = source.matches('≜').count() as u16;
        let assignments = source.matches('≔').count() as u16;
        let quantifiers = source.matches(|c| c == '∀' || c == '∃').count() as u16;
        let lambdas = source.matches('λ').count() as u16;
        let implications = source.matches(|c| matches!(c, '⇒' | '⇔' | '→' | '↔')).count() as u16;
        let set_ops = source.matches(|c| matches!(c, '∈' | '⊆' | '∩' | '∪' | '∅')).count() as u16;

        Self {
            blocks_found,
            definitions,
            assignments,
            quantifiers,
            lambdas,
            implications,
            set_ops,
            symbol_count: count_symbols(source) as u16,
            token_count: count_tokens(source) as u16,
        }
    }

    /// Total semantic bindings
    pub fn total_bindings(&self) -> u16 {
        self.definitions + self.assignments + self.quantifiers
            + self.lambdas + self.implications + self.set_ops
    }

    /// Block score (0.0 - 1.0)
    pub fn block_score(&self) -> f32 {
        self.blocks_found as f32 / 5.0
    }

    /// Binding score (0.0 - 1.0, capped at 20 bindings)
    pub fn binding_score(&self) -> f32 {
        (self.total_bindings() as f32 / 20.0).min(1.0)
    }

    /// Pure density (symbols per token)
    pub fn pure_density(&self) -> f32 {
        if self.token_count > 0 {
            self.symbol_count as f32 / self.token_count as f32
        } else {
            0.0
        }
    }
}

/// Validation result
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidationResult {
    /// Is document valid?
    pub valid: bool,
    /// Quality tier
    pub tier: Tier,
    /// Semantic density δ ∈ [0, 1]
    pub delta: f32,
    /// Pure density ρ (symbols/tokens)
    pub pure_density: f32,
    /// Ambiguity score ∈ [0, 1]
    pub ambiguity: f32,
    /// Optional error message
    pub error: Option<&'static str>,
    /// Detailed metrics (if computed)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub metrics: Option<DensityMetrics>,
}

impl ValidationResult {
    /// Create successful validation result
    pub fn success(tier: Tier, delta: f32, pure_density: f32) -> Self {
        Self {
            valid: true,
            tier,
            delta,
            pure_density,
            ambiguity: 0.01, // Valid AISP has near-zero ambiguity
            error: None,
            metrics: None,
        }
    }

    /// Create successful result with metrics
    pub fn success_with_metrics(tier: Tier, delta: f32, pure_density: f32, metrics: DensityMetrics) -> Self {
        Self {
            valid: true,
            tier,
            delta,
            pure_density,
            ambiguity: 0.01,
            error: None,
            metrics: Some(metrics),
        }
    }

    /// Create failed validation result
    pub fn failure(error: &'static str) -> Self {
        Self {
            valid: false,
            tier: Tier::Reject,
            delta: 0.0,
            pure_density: 0.0,
            ambiguity: 0.5,
            error: Some(error),
            metrics: None,
        }
    }
}

/// Validate an AISP document
///
/// # Arguments
/// * `source` - AISP document source code
///
/// # Returns
/// `ValidationResult` containing validity, tier, density metrics, and ambiguity
///
/// # Example
/// ```rust
/// use aisp::validate;
///
/// let doc = r#"𝔸1.0.test@2026-01-16
/// γ≔example
/// ⟦Ω:Meta⟧{ ∀D:Ambig(D)<0.02 }
/// ⟦Σ:Types⟧{ T≜ℕ }
/// ⟦Γ:Rules⟧{ ∀x:T:x≥0 }
/// ⟦Λ:Funcs⟧{ f≜λx.x }
/// ⟦Ε⟧⟨δ≜0.75;φ≜100;τ≜◊⁺⁺⟩"#;
///
/// let result = validate(doc);
/// assert!(result.valid);
/// ```
pub fn validate(source: &str) -> ValidationResult {
    // Check for AISP header
    let trimmed = source.trim();
    if !trimmed.starts_with('𝔸') {
        return ValidationResult::failure("Missing AISP header (𝔸)");
    }

    // Compute density metrics
    let metrics = DensityMetrics::from_source(source);

    // Check required blocks
    if metrics.blocks_found < 5 {
        return ValidationResult::failure("Missing required blocks");
    }

    // Calculate semantic density: δ = (blockScore × 0.4) + (bindingScore × 0.6)
    let delta = (metrics.block_score() * 0.4) + (metrics.binding_score() * 0.6);
    let pure_density = metrics.pure_density();

    // Get tier
    let tier = Tier::from_delta(delta);

    // AISP requires minimum Bronze tier (δ ≥ 0.20)
    if tier == Tier::Reject {
        return ValidationResult {
            valid: false,
            tier,
            delta,
            pure_density,
            ambiguity: 0.5,
            error: Some("Document density too low (δ < 0.20)"),
            metrics: Some(metrics),
        };
    }

    ValidationResult::success_with_metrics(tier, delta, pure_density, metrics)
}

/// Quick validation check (returns only boolean)
pub fn is_valid(source: &str) -> bool {
    validate(source).valid
}

/// Get the quality tier of a document
pub fn get_tier(source: &str) -> Tier {
    validate(source).tier
}

/// Get the semantic density of a document
pub fn get_density(source: &str) -> f32 {
    validate(source).delta
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_DOC: &str = r#"𝔸1.0.test@2026-01-16
γ≔example
⟦Ω:Meta⟧{ ∀D:Ambig(D)<0.02 }
⟦Σ:Types⟧{ T≜ℕ }
⟦Γ:Rules⟧{ ∀x:T:x≥0 }
⟦Λ:Funcs⟧{ f≜λx.x }
⟦Ε⟧⟨δ≜0.75;φ≜100;τ≜◊⁺⁺⟩"#;

    #[test]
    fn test_valid_document() {
        let result = validate(VALID_DOC);
        assert!(result.valid);
        assert!(result.tier >= Tier::Silver);
    }

    #[test]
    fn test_missing_header() {
        let result = validate("no header here");
        assert!(!result.valid);
        assert_eq!(result.error, Some("Missing AISP header (𝔸)"));
    }

    #[test]
    fn test_missing_blocks() {
        let result = validate("𝔸1.0.test@2026-01-16\nsome content");
        assert!(!result.valid);
        assert_eq!(result.error, Some("Missing required blocks"));
    }

    #[test]
    fn test_density_metrics() {
        let metrics = DensityMetrics::from_source(VALID_DOC);
        assert_eq!(metrics.blocks_found, 5);
        assert!(metrics.definitions >= 3);
        assert!(metrics.quantifiers >= 2);
        assert!(metrics.lambdas >= 1);
    }
}
