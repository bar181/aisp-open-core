//! AISP 5.1 Document Validation Library
//!
//! AISP (AI Symbolic Protocol) is a formal specification language designed for
//! precise AI-to-AI communication with <2% ambiguity.
//!
//! # Features
//!
//! - **Validation**: Validate AISP documents with semantic density scoring
//! - **Streaming**: Process large documents with streaming validation (feature: `streaming`)
//! - **Quality Tiers**: Automatic tier classification (⊘, ◊⁻, ◊, ◊⁺, ◊⁺⁺)
//! - **No-std Support**: Works without std (disable default features)
//!
//! # Quick Start
//!
//! ```rust
//! use aisp::{validate, Tier};
//!
//! let doc = r#"
//! 𝔸1.0.example@2026-01-16
//! γ≔test
//!
//! ⟦Ω:Meta⟧{ ∀D:Ambig(D)<0.02 }
//! ⟦Σ:Types⟧{ T≜ℕ }
//! ⟦Γ:Rules⟧{ ∀x:T:x≥0 }
//! ⟦Λ:Funcs⟧{ f≜λx.x }
//! ⟦Ε⟧⟨δ≜0.75;φ≜100;τ≜◊⁺⁺⟩
//! "#;
//!
//! let result = validate(doc);
//! assert!(result.valid);
//! assert!(result.tier >= Tier::Silver);
//! ```
//!
//! # Streaming Validation
//!
//! For large documents, use the streaming API:
//!
//! ```rust,ignore
//! use aisp::streaming::StreamValidator;
//!
//! let mut validator = StreamValidator::new();
//! validator.feed(chunk1);
//! validator.feed(chunk2);
//! let result = validator.finish();
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![deny(unsafe_code)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod symbol;
mod tier;
mod validate;

#[cfg(feature = "streaming")]
pub mod streaming;

#[cfg(feature = "z3")]
pub mod z3_validation;

// Re-exports
pub use symbol::{Category, Symbol, SymbolId, AISP_SYMBOLS, is_aisp_char, count_symbols, count_tokens, lookup_symbol, starts_with_symbol, get_glyph};
pub use tier::Tier;
pub use validate::{validate, is_valid, get_tier, get_density, ValidationResult, DensityMetrics};

#[cfg(feature = "z3")]
pub use z3_validation::{validate_with_z3, AispConstruct, Z3Context};

/// Required blocks for a valid AISP document
pub const REQUIRED_BLOCKS: [&str; 5] = ["⟦Ω", "⟦Σ", "⟦Γ", "⟦Λ", "⟦Ε"];

/// Supported file extensions
pub const SUPPORTED_EXTENSIONS: [&str; 5] = [".aisp", ".md", ".txt", ".spec", ".aisp5"];

/// Maximum default document size (64KB)
pub const DEFAULT_MAX_SIZE: usize = 64 * 1024;

/// Absolute maximum document size (1MB)
pub const ABSOLUTE_MAX_SIZE: usize = 1024 * 1024;

/// Check if a file extension is supported
pub fn is_extension_supported(ext: &str) -> bool {
    SUPPORTED_EXTENSIONS.iter().any(|&e| e.eq_ignore_ascii_case(ext))
}
