# aisp

**AISP 5.1 document validation library** — AI Symbolic Protocol with <2% ambiguity.

[![Crates.io](https://img.shields.io/crates/v/aisp.svg)](https://crates.io/crates/aisp)
[![Documentation](https://docs.rs/aisp/badge.svg)](https://docs.rs/aisp)
[![License](https://img.shields.io/crates/l/aisp.svg)](LICENSE)

## Overview

AISP (AI Symbolic Protocol) is a formal specification language designed for high-density, low-ambiguity AI-to-AI communication. This crate provides:

- **Validation**: Validate AISP documents with semantic density scoring
- **Streaming**: Process large documents with streaming validation
- **Quality Tiers**: Automatic tier classification (⊘, ◊⁻, ◊, ◊⁺, ◊⁺⁺)
- **No-std Support**: Works without std (disable default features)

## Quick Start

```rust
use aisp::{validate, Tier};

let doc = r#"
𝔸1.0.example@2026-01-16
γ≔test

⟦Ω:Meta⟧{ ∀D:Ambig(D)<0.02 }
⟦Σ:Types⟧{ T≜ℕ }
⟦Γ:Rules⟧{ ∀x:T:x≥0 }
⟦Λ:Funcs⟧{ f≜λx.x }
⟦Ε⟧⟨δ≜0.75;φ≜100;τ≜◊⁺⁺⟩
"#;

let result = validate(doc);
assert!(result.valid);
assert!(result.tier >= Tier::Silver);
println!("Tier: {} (δ={:.2})", result.tier.symbol(), result.delta);
```

## Features

| Feature | Description | Default |
|---------|-------------|---------|
| `std` | Standard library support | ✓ |
| `streaming` | Streaming validation for large docs | |
| `serde` | Serialization support | |
| `wasm` | WebAssembly optimization | |

```toml
# Enable streaming
[dependencies]
aisp = { version = "0.1", features = ["streaming"] }

# Minimal no-std
[dependencies]
aisp = { version = "0.1", default-features = false }
```

## Streaming Validation

For documents larger than available memory:

```rust
use aisp::streaming::StreamValidator;

let mut validator = StreamValidator::new();

// Feed chunks as they arrive
validator.feed("𝔸1.0.test@2026-01-16\n")?;
validator.feed("⟦Ω:Meta⟧{ ... }\n")?;
// ... more chunks

let result = validator.finish();
```

## Quality Tiers

| Tier | Symbol | Density (δ) | Description |
|------|--------|-------------|-------------|
| Reject | ⊘ | < 0.20 | Invalid, insufficient formalization |
| Bronze | ◊⁻ | ≥ 0.20 | Minimum valid AISP |
| Silver | ◊ | ≥ 0.40 | Basic formal specification |
| Gold | ◊⁺ | ≥ 0.60 | Well-structured specification |
| Platinum | ◊⁺⁺ | ≥ 0.75 | Complete, proof-carrying spec |

## API Reference

### Core Functions

```rust
// Full validation with metrics
let result = aisp::validate(source);

// Quick boolean check
let is_valid = aisp::is_valid(source);

// Get tier only
let tier = aisp::get_tier(source);

// Get density only
let delta = aisp::get_density(source);
```

### Symbol Operations

```rust
use aisp::{is_aisp_char, count_symbols, lookup_symbol};

// Check if character is AISP symbol
assert!(is_aisp_char('∀'));
assert!(!is_aisp_char('x'));

// Count AISP symbols in text
let count = count_symbols("∀x∈S:P(x)");

// Look up symbol by glyph
let id = lookup_symbol("λ").unwrap();
```

## Required Blocks

Valid AISP documents must contain these 5 blocks:

| Block | Purpose |
|-------|---------|
| `⟦Ω⟧` | Meta/Foundation |
| `⟦Σ⟧` | Types/Glossary |
| `⟦Γ⟧` | Rules/Inference |
| `⟦Λ⟧` | Functions |
| `⟦Ε⟧` | Evidence |

## Supported Extensions

- `.aisp` - Primary AISP format
- `.md` - Markdown with embedded AISP
- `.txt` - Plain text AISP
- `.spec` - Specification files
- `.aisp5` - AISP 5.x format

## Size Limits

| Limit | Size |
|-------|------|
| Default max | 64 KB |
| Absolute max | 1 MB |
| WASM kernel | 1 KB |

## Author

Bradley Ross — [GitHub @bar181](https://github.com/bar181)

## License

Dual licensed under MIT OR Apache-2.0.

## Links

- **Repository**: https://github.com/bar181/aisp-open-core
- **Documentation**: https://docs.rs/aisp
- **AISP Specification**: See `AI_GUIDE.md` in repository
