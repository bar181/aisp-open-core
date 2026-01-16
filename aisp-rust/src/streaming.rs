//! Streaming AISP validation for large documents
//!
//! This module provides streaming validation that processes documents in chunks,
//! enabling validation of documents larger than available memory.
//!
//! # Example
//!
//! ```rust,ignore
//! use aisp::streaming::StreamValidator;
//!
//! let mut validator = StreamValidator::new();
//!
//! // Feed chunks as they arrive
//! validator.feed("𝔸1.0.test@2026-01-16\nγ≔example\n");
//! validator.feed("⟦Ω:Meta⟧{ ∀D:Ambig(D)<0.02 }\n");
//! validator.feed("⟦Σ:Types⟧{ T≜ℕ }\n");
//! validator.feed("⟦Γ:Rules⟧{ ∀x:T:x≥0 }\n");
//! validator.feed("⟦Λ:Funcs⟧{ f≜λx.x }\n");
//! validator.feed("⟦Ε⟧⟨δ≜0.75;φ≜100;τ≜◊⁺⁺⟩");
//!
//! let result = validator.finish();
//! assert!(result.valid);
//! ```

use crate::symbol::is_aisp_char;
use crate::tier::Tier;
use crate::validate::ValidationResult;
use crate::REQUIRED_BLOCKS;

/// Streaming state for incremental validation
#[derive(Debug, Clone)]
pub struct StreamState {
    /// Total bytes processed
    pub bytes_processed: usize,
    /// Total AISP symbols found
    pub symbol_count: u32,
    /// Total non-whitespace tokens
    pub token_count: u32,
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
    /// Blocks found (bitmap: bit 0=Ω, 1=Σ, 2=Γ, 3=Λ, 4=Ε)
    pub blocks_found: u8,
    /// Whether AISP header (𝔸) was found
    pub has_header: bool,
    /// Partial token buffer (for tokens split across chunks)
    partial_token: String,
}

impl Default for StreamState {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamState {
    /// Create new streaming state
    pub fn new() -> Self {
        Self {
            bytes_processed: 0,
            symbol_count: 0,
            token_count: 0,
            definitions: 0,
            assignments: 0,
            quantifiers: 0,
            lambdas: 0,
            implications: 0,
            set_ops: 0,
            blocks_found: 0,
            has_header: false,
            partial_token: String::new(),
        }
    }

    /// Total semantic bindings
    pub fn total_bindings(&self) -> u32 {
        (self.definitions + self.assignments + self.quantifiers
            + self.lambdas + self.implications + self.set_ops) as u32
    }

    /// Block score (0.0 - 1.0)
    pub fn block_score(&self) -> f32 {
        self.blocks_found.count_ones() as f32 / 5.0
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

    /// Semantic density
    pub fn delta(&self) -> f32 {
        (self.block_score() * 0.4) + (self.binding_score() * 0.6)
    }
}

/// Streaming validator for processing AISP documents in chunks
///
/// This allows validation of documents larger than available memory
/// by processing them incrementally.
pub struct StreamValidator {
    state: StreamState,
    max_size: usize,
}

impl Default for StreamValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamValidator {
    /// Create a new streaming validator with default max size (64KB)
    pub fn new() -> Self {
        Self::with_max_size(crate::DEFAULT_MAX_SIZE)
    }

    /// Create a streaming validator with custom max size
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            state: StreamState::new(),
            max_size: max_size.min(crate::ABSOLUTE_MAX_SIZE),
        }
    }

    /// Get current state
    pub fn state(&self) -> &StreamState {
        &self.state
    }

    /// Get bytes processed so far
    pub fn bytes_processed(&self) -> usize {
        self.state.bytes_processed
    }

    /// Check if we've exceeded max size
    pub fn is_overflow(&self) -> bool {
        self.state.bytes_processed > self.max_size
    }

    /// Feed a chunk of data
    ///
    /// Returns `Ok(())` if successful, `Err` if max size exceeded
    pub fn feed(&mut self, chunk: &str) -> Result<(), &'static str> {
        let new_size = self.state.bytes_processed + chunk.len();
        if new_size > self.max_size {
            return Err("Document exceeds maximum size");
        }

        self.state.bytes_processed = new_size;
        self.process_chunk(chunk);
        Ok(())
    }

    /// Process a chunk of data
    fn process_chunk(&mut self, chunk: &str) {
        // Check for AISP header in first chunk
        if !self.state.has_header && self.state.bytes_processed == chunk.len() {
            self.state.has_header = chunk.trim_start().starts_with('𝔸');
        }

        // Process characters
        for c in chunk.chars() {
            if is_aisp_char(c) {
                self.state.symbol_count += 1;

                // Count specific operators
                match c {
                    '≜' => self.state.definitions += 1,
                    '≔' => self.state.assignments += 1,
                    '∀' | '∃' => self.state.quantifiers += 1,
                    'λ' => self.state.lambdas += 1,
                    '⇒' | '⇔' | '→' | '↔' => self.state.implications += 1,
                    '∈' | '⊆' | '∩' | '∪' | '∅' => self.state.set_ops += 1,
                    _ => {}
                }
            }
        }

        // Count tokens (split by whitespace)
        // Handle partial tokens at chunk boundaries
        let mut combined = std::mem::take(&mut self.state.partial_token);
        combined.push_str(chunk);

        let tokens: Vec<&str> = combined.split_whitespace().collect();
        if !tokens.is_empty() {
            // Count all but possibly last token (which might be partial)
            let complete_count = if combined.ends_with(char::is_whitespace) {
                tokens.len()
            } else {
                tokens.len().saturating_sub(1)
            };

            self.state.token_count += complete_count as u32;

            // Save partial token for next chunk
            if !combined.ends_with(char::is_whitespace) {
                if let Some(last) = tokens.last() {
                    self.state.partial_token = (*last).to_string();
                }
            }
        }

        // Check for required blocks
        for (i, block) in REQUIRED_BLOCKS.iter().enumerate() {
            if chunk.contains(block) {
                self.state.blocks_found |= 1 << i;
            }
        }
    }

    /// Finish validation and return result
    pub fn finish(mut self) -> ValidationResult {
        // Count any remaining partial token
        if !self.state.partial_token.is_empty() {
            self.state.token_count += 1;
        }

        // Check header
        if !self.state.has_header {
            return ValidationResult::failure("Missing AISP header (𝔸)");
        }

        // Check required blocks (all 5 must be present)
        if self.state.blocks_found.count_ones() < 5 {
            return ValidationResult::failure("Missing required blocks");
        }

        // Calculate density
        let delta = self.state.delta();
        let pure_density = self.state.pure_density();
        let tier = Tier::from_delta(delta);

        // Minimum Bronze tier required
        if tier == Tier::Reject {
            return ValidationResult {
                valid: false,
                tier,
                delta,
                pure_density,
                ambiguity: 0.5,
                error: Some("Document density too low (δ < 0.20)"),
                metrics: None,
            };
        }

        ValidationResult::success(tier, delta, pure_density)
    }

    /// Reset validator for reuse
    pub fn reset(&mut self) {
        self.state = StreamState::new();
    }
}

/// Convenience function to validate a document using streaming
pub fn validate_streaming(source: &str) -> ValidationResult {
    let mut validator = StreamValidator::new();
    if let Err(e) = validator.feed(source) {
        return ValidationResult::failure(e);
    }
    validator.finish()
}

/// Validate with progress callback
///
/// The callback receives (bytes_processed, total_bytes)
pub fn validate_with_progress<F>(source: &str, mut progress: F) -> ValidationResult
where
    F: FnMut(usize, usize),
{
    let total = source.len();
    let mut validator = StreamValidator::new();

    // Process in 4KB chunks
    const CHUNK_SIZE: usize = 4096;
    let mut offset = 0;

    while offset < source.len() {
        let end = (offset + CHUNK_SIZE).min(source.len());

        // Find a safe UTF-8 boundary
        let chunk_end = source[offset..end]
            .char_indices()
            .last()
            .map(|(i, c)| offset + i + c.len_utf8())
            .unwrap_or(end);

        let chunk = &source[offset..chunk_end];
        if let Err(e) = validator.feed(chunk) {
            return ValidationResult::failure(e);
        }

        progress(chunk_end, total);
        offset = chunk_end;
    }

    validator.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streaming_validation() {
        let mut validator = StreamValidator::new();

        validator.feed("𝔸1.0.test@2026-01-16\n").unwrap();
        validator.feed("γ≔example\n").unwrap();
        validator.feed("⟦Ω:Meta⟧{ ∀D:Ambig(D)<0.02 }\n").unwrap();
        validator.feed("⟦Σ:Types⟧{ T≜ℕ }\n").unwrap();
        validator.feed("⟦Γ:Rules⟧{ ∀x:T:x≥0 }\n").unwrap();
        validator.feed("⟦Λ:Funcs⟧{ f≜λx.x }\n").unwrap();
        validator.feed("⟦Ε⟧⟨δ≜0.75;φ≜100;τ≜◊⁺⁺⟩").unwrap();

        let result = validator.finish();
        assert!(result.valid);
        assert!(result.tier >= Tier::Silver);
    }

    #[test]
    fn test_streaming_overflow() {
        let mut validator = StreamValidator::with_max_size(100);
        let large_chunk = "x".repeat(200);

        assert!(validator.feed(&large_chunk).is_err());
    }

    #[test]
    fn test_convenience_function() {
        let doc = r#"𝔸1.0.test@2026-01-16
γ≔example
⟦Ω:Meta⟧{ ∀D:Ambig(D)<0.02 }
⟦Σ:Types⟧{ T≜ℕ }
⟦Γ:Rules⟧{ ∀x:T:x≥0 }
⟦Λ:Funcs⟧{ f≜λx.x }
⟦Ε⟧⟨δ≜0.75;φ≜100;τ≜◊⁺⁺⟩"#;

        let result = validate_streaming(doc);
        assert!(result.valid);
    }
}
