//! Property-based testing for AISP validator
//!
//! This module implements comprehensive property-based testing using proptest
//! to generate thousands of test cases automatically and verify validator
//! correctness, consistency, and robustness.

use proptest::prelude::*;
use aisp_core::{
    ast::*,
    validator::{AispValidator, ValidationConfig},
    parser_new::AispParser,
    semantic::QualityTier,
};

/// Strategy for generating valid AISP identifiers
fn aisp_identifier() -> impl Strategy<Value = String> {
    prop::string::string_regex(r"[a-zA-Z][a-zA-Z0-9_]*")
        .unwrap()
        .prop_filter("Must be valid identifier", |s| {
            s.len() > 0 && s.len() <= 50 && !s.starts_with('_')
        })
}

/// Strategy for generating AISP version strings
fn aisp_version() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("5.1".to_string()),
        Just("5.0".to_string()),
        Just("4.9".to_string()),
    ]
}

/// Strategy for generating dates
fn aisp_date() -> impl Strategy<Value = String> {
    (2020u16..=2030, 1u8..=12, 1u8..=28)
        .prop_map(|(year, month, day)| format!("{:04}-{:02}-{:02}", year, month, day))
}

/// Strategy for generating AISP document headers
fn aisp_header() -> impl Strategy<Value = String> {
    (aisp_version(), aisp_identifier(), aisp_date())
        .prop_map(|(version, name, date)| {
            format!("𝔸{}.{}@{}", version, name, date)
        })
}

/// Strategy for generating meta entries
fn meta_entry() -> impl Strategy<Value = String> {
    prop_oneof![
        aisp_identifier().prop_map(|id| format!("  domain≜{}", id)),
        aisp_identifier().prop_map(|id| format!("  protocol≜\"{}\"", id)),
        aisp_identifier().prop_map(|id| format!("  version≜\"{}\"", id)),
        Just("  ∀D∈AISP:Ambig(D)<0.02".to_string()),
    ]
}

/// Strategy for generating meta blocks
fn meta_block() -> impl Strategy<Value = String> {
    prop::collection::vec(meta_entry(), 1..=5)
        .prop_map(|entries| {
            format!("⟦Ω:Meta⟧{{\n{}\n}}", entries.join("\n"))
        })
}

/// Strategy for generating type expressions
fn type_expression() -> impl Strategy<Value = String> {
    let leaf = prop_oneof![
        Just("ℕ".to_string()),
        Just("ℤ".to_string()),
        Just("ℝ".to_string()),
        Just("𝔹".to_string()),
        Just("𝕊".to_string()),
        aisp_identifier(),
        prop::collection::vec(aisp_identifier(), 1..=4)
            .prop_map(|ids| format!("{{{}}}", ids.join(","))),
    ];
    
    leaf.prop_recursive(3, 64, 10, |inner| {
        prop_oneof![
            inner.clone().prop_map(|s| format!("[{}]", s)),
            (inner.clone(), inner.clone())
                .prop_map(|(a, b)| format!("({},{})", a, b)),
            (inner.clone(), inner.clone())
                .prop_map(|(a, b)| format!("{}→{}", a, b)),
        ]
    })
}

/// Strategy for generating type definitions
fn type_definition() -> impl Strategy<Value = String> {
    (aisp_identifier(), type_expression())
        .prop_map(|(name, type_expr)| format!("  {}≜{}", name, type_expr))
}

/// Strategy for generating types blocks
fn types_block() -> impl Strategy<Value = String> {
    prop::collection::vec(type_definition(), 1..=6)
        .prop_map(|defs| {
            format!("⟦Σ:Types⟧{{\n{}\n}}", defs.join("\n"))
        })
}

/// Strategy for generating logical expressions
fn logical_expression() -> impl Strategy<Value = String> {
    let leaf = prop_oneof![
        aisp_identifier(),
        aisp_identifier().prop_map(|id| format!("Valid({})", id)),
        aisp_identifier().prop_map(|id| format!("{}(x)", id)),
        Just("true".to_string()),
        Just("false".to_string()),
    ];
    
    leaf.prop_recursive(3, 32, 5, |inner| {
        prop_oneof![
            (aisp_identifier(), aisp_identifier())
                .prop_map(|(var, domain)| format!("∀{}:{}→Valid({})", var, domain, var)),
            (inner.clone(), inner.clone())
                .prop_map(|(a, b)| format!("{}∧{}", a, b)),
            (inner.clone(), inner.clone())
                .prop_map(|(a, b)| format!("{}∨{}", a, b)),
            (inner.clone(), inner.clone())
                .prop_map(|(a, b)| format!("{}⇒{}", a, b)),
            inner.clone().prop_map(|s| format!("¬{}", s)),
        ]
    })
}

/// Strategy for generating rules
fn rule() -> impl Strategy<Value = String> {
    logical_expression()
        .prop_map(|expr| format!("  {}", expr))
}

/// Strategy for generating rules blocks
fn rules_block() -> impl Strategy<Value = String> {
    prop::collection::vec(rule(), 1..=4)
        .prop_map(|rules| {
            format!("⟦Γ:Rules⟧{{\n{}\n}}", rules.join("\n"))
        })
}

/// Strategy for generating lambda expressions
fn lambda_expression() -> impl Strategy<Value = String> {
    (aisp_identifier(), logical_expression())
        .prop_map(|(param, body)| format!("λ{}.{}", param, body))
}

/// Strategy for generating function definitions
fn function_definition() -> impl Strategy<Value = String> {
    (aisp_identifier(), lambda_expression())
        .prop_map(|(name, lambda)| format!("  {}≜{}", name, lambda))
}

/// Strategy for generating functions blocks
fn functions_block() -> impl Strategy<Value = String> {
    prop::collection::vec(function_definition(), 1..=4)
        .prop_map(|funcs| {
            format!("⟦Λ:Funcs⟧{{\n{}\n}}", funcs.join("\n"))
        })
}

/// Strategy for generating evidence entries
fn evidence_entry() -> impl Strategy<Value = String> {
    prop_oneof![
        (0.0f64..1.0).prop_map(|delta| format!("δ≜{:.3}", delta)),
        (0.0f64..100.0).prop_map(|phi| format!("φ≜{:.0}", phi)),
        prop_oneof![
            Just("τ≜⊘".to_string()),
            Just("τ≜◊⁻".to_string()),
            Just("τ≜◊".to_string()),
            Just("τ≜◊⁺".to_string()),
            Just("τ≜◊⁺⁺".to_string()),
        ],
        Just("⊢ND:natural_deduction_valid".to_string()),
        Just("⊢CAT:functors_verified".to_string()),
        Just("⊢Ambig(D)<0.02".to_string()),
    ]
}

/// Strategy for generating evidence blocks
fn evidence_block() -> impl Strategy<Value = String> {
    prop::collection::vec(evidence_entry(), 1..=6)
        .prop_map(|entries| {
            format!("⟦Ε⟧⟨{}⟩", entries.join(";"))
        })
}

/// Strategy for generating complete AISP documents
fn aisp_document() -> impl Strategy<Value = String> {
    (
        aisp_header(),
        meta_block(),
        types_block(),
        rules_block(),
        functions_block(),
        evidence_block(),
    ).prop_map(|(header, meta, types, rules, funcs, evidence)| {
        format!("{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}", 
                header, meta, types, rules, funcs, evidence)
    })
}

/// Strategy for generating minimal valid AISP documents
fn minimal_aisp_document() -> impl Strategy<Value = String> {
    (aisp_header(), aisp_identifier())
        .prop_map(|(header, domain)| {
            format!(r#"{}

⟦Ω:Meta⟧{{
  domain≜{}
}}

⟦Σ:Types⟧{{
  Unit≜{{unit}}
}}

⟦Γ:Rules⟧{{
  ∀x:Unit→Valid(x)
}}

⟦Λ:Funcs⟧{{
  id≜λx.x
}}

⟦Ε⟧⟨δ≜0.5⟩"#, header, domain)
        })
}

/// Strategy for generating malformed documents (for negative testing)
fn malformed_document() -> impl Strategy<Value = String> {
    prop_oneof![
        // Missing header
        Just("⟦Ω:Meta⟧{ domain≜test }".to_string()),
        // Invalid characters
        Just("𝔸5.1.test@2026 INVALID CHARS".to_string()),
        // Unclosed blocks
        Just("𝔸5.1.test@2026-01-26\n⟦Ω:Meta⟧{ domain≜test".to_string()),
        // Invalid block headers
        Just("𝔸5.1.test@2026-01-26\n⟦Invalid:Block⟧{ }".to_string()),
        // Empty document
        Just("".to_string()),
        // Only whitespace
        Just("   \n\n\t  ".to_string()),
    ]
}

// Property Tests

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property: All minimal documents should parse successfully
    #[test]
    fn prop_minimal_documents_parse(doc in minimal_aisp_document()) {
        let mut parser = AispParser::new(doc.clone());
        let result = parser.parse();
        
        prop_assert!(result.is_ok(), "Minimal document should parse: {}", doc);
        
        if let Ok(parsed) = result {
            prop_assert_eq!(parsed.blocks.len(), 5, "Should have exactly 5 blocks");
            prop_assert!(parsed.header.version.len() > 0, "Should have version");
            prop_assert!(parsed.header.name.len() > 0, "Should have name");
            prop_assert!(parsed.header.date.len() > 0, "Should have date");
        }
    }

    /// Property: Parser should be deterministic (same input = same output)
    #[test]
    fn prop_parser_deterministic(doc in minimal_aisp_document()) {
        let mut parser1 = AispParser::new(doc.clone());
        let mut parser2 = AispParser::new(doc.clone());
        
        let result1 = parser1.parse();
        let result2 = parser2.parse();
        
        prop_assert_eq!(result1.is_ok(), result2.is_ok(), "Parser should be deterministic");
        
        if result1.is_ok() && result2.is_ok() {
            let doc1 = result1.unwrap();
            let doc2 = result2.unwrap();
            prop_assert_eq!(doc1.blocks.len(), doc2.blocks.len(), "Block count should match");
            prop_assert_eq!(doc1.header.name, doc2.header.name, "Header name should match");
        }
    }

    /// Property: Validator should be consistent across multiple runs
    #[test]
    fn prop_validator_consistent(doc in minimal_aisp_document()) {
        let validator = AispValidator::new();
        
        let result1 = validator.validate(&doc);
        let result2 = validator.validate(&doc);
        
        prop_assert_eq!(result1.valid, result2.valid, "Validation should be consistent");
        prop_assert_eq!(result1.tier, result2.tier, "Quality tier should be consistent");
        prop_assert!((result1.delta - result2.delta).abs() < 1e-10, "Delta should be consistent");
        prop_assert!((result1.ambiguity - result2.ambiguity).abs() < 1e-10, "Ambiguity should be consistent");
    }

    /// Property: Valid documents should have reasonable quality metrics
    #[test]
    fn prop_valid_documents_reasonable_metrics(doc in minimal_aisp_document()) {
        let validator = AispValidator::new();
        let result = validator.validate(&doc);
        
        if result.valid {
            prop_assert!(result.delta >= 0.0 && result.delta <= 1.0, "Delta should be in [0,1]");
            prop_assert!(result.ambiguity >= 0.0 && result.ambiguity <= 1.0, "Ambiguity should be in [0,1]");
            prop_assert!(result.pure_density >= 0.0 && result.pure_density <= 1.0, "Pure density should be in [0,1]");
            prop_assert!(result.tier_value >= 0 && result.tier_value <= 4, "Tier value should be in [0,4]");
        }
    }

    /// Property: Quality tier should be monotonic with delta
    #[test]
    fn prop_quality_tier_monotonic(delta in 0.0f64..1.0) {
        let tier = QualityTier::from_delta(delta);
        let tier_value = tier.value();
        
        // Test monotonicity: higher delta should never give lower tier
        let higher_delta = (delta + 0.1).min(1.0);
        let higher_tier = QualityTier::from_delta(higher_delta);
        let higher_tier_value = higher_tier.value();
        
        prop_assert!(higher_tier_value >= tier_value, 
                    "Higher delta ({}) should give higher or equal tier ({} >= {})", 
                    higher_delta, higher_tier_value, tier_value);
    }

    /// Property: Malformed documents should fail gracefully
    #[test]
    fn prop_malformed_documents_fail_gracefully(doc in malformed_document()) {
        let validator = AispValidator::new();
        let result = validator.validate(&doc);
        
        // Should either fail validation or have clear error
        prop_assert!(
            !result.valid || result.error.is_some(),
            "Malformed documents should fail validation or have clear errors"
        );
        
        // Should never panic - this property is enforced by the test not panicking
        prop_assert!(result.tier_value <= 4, "Tier value should be bounded even for invalid docs");
    }

    /// Property: Document size should correlate with complexity metrics
    #[test]
    fn prop_document_size_correlates_complexity(doc in aisp_document()) {
        let validator = AispValidator::new();
        let result = validator.validate(&doc);
        
        let doc_size = doc.len();
        
        // Larger documents should generally have higher symbol counts (if valid)
        if let Some(analysis) = result.semantic_analysis {
            if doc_size > 500 {
                prop_assert!(analysis.symbol_stats.total_tokens > 50, 
                           "Large documents should have substantial token counts");
            }
            
            // Symbol ratio should be reasonable
            if analysis.symbol_stats.total_tokens > 0 {
                let symbol_ratio = analysis.symbol_stats.total_symbols as f64 / 
                                 analysis.symbol_stats.total_tokens as f64;
                prop_assert!(symbol_ratio >= 0.0 && symbol_ratio <= 1.0, 
                           "Symbol ratio should be in [0,1]");
            }
        }
    }

    /// Property: Evidence block values should be preserved in validation result
    #[test]
    fn prop_evidence_preservation(
        delta in 0.0f64..1.0,
        phi in 0.0f64..100.0,
        domain in aisp_identifier()
    ) {
        let doc = format!(r#"𝔸5.1.evidence@2026-01-26

⟦Ω:Meta⟧{{
  domain≜{}
}}

⟦Σ:Types⟧{{
  Unit≜{{unit}}
}}

⟦Γ:Rules⟧{{
  ∀x:Unit→Valid(x)
}}

⟦Λ:Funcs⟧{{
  id≜λx.x
}}

⟦Ε⟧⟨δ≜{:.3};φ≜{:.0}⟩"#, domain, delta, phi);

        let mut parser = AispParser::new(doc.clone());
        if let Ok(parsed) = parser.parse() {
            // Find evidence block
            if let Some(AispBlock::Evidence(evidence)) = parsed.blocks.iter()
                .find(|b| matches!(b, AispBlock::Evidence(_))) {
                
                if let Some(parsed_delta) = evidence.delta {
                    prop_assert!((parsed_delta - delta).abs() < 0.01, 
                               "Delta should be preserved in parsing");
                }
                
                if let Some(parsed_phi) = evidence.phi {
                    prop_assert!((parsed_phi - phi).abs() < 1.0, 
                               "Phi should be preserved in parsing");
                }
            }
        }
    }

    /// Property: Validation with different configurations should be stable
    #[test]
    fn prop_validation_config_stability(doc in minimal_aisp_document()) {
        let default_config = ValidationConfig::default();
        let strict_config = ValidationConfig {
            strict_mode: true,
            ..ValidationConfig::default()
        };
        
        let validator_default = AispValidator::with_config(default_config);
        let validator_strict = AispValidator::with_config(strict_config);
        
        let result_default = validator_default.validate(&doc);
        let result_strict = validator_strict.validate(&doc);
        
        // Core metrics should be the same
        if result_default.valid && result_strict.valid {
            prop_assert!((result_default.delta - result_strict.delta).abs() < 1e-6, 
                       "Delta should be stable across configs");
        }
        
        // Strict mode should never be more lenient
        if result_strict.valid {
            prop_assert!(result_default.valid, 
                       "If strict mode passes, default should also pass");
        }
    }
}

/// Additional property tests for edge cases
#[cfg(test)]
mod edge_case_properties {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]

        /// Property: Empty blocks should be handled gracefully
        #[test]
        fn prop_empty_blocks_handled(name in aisp_identifier()) {
            let doc = format!(r#"𝔸5.1.{}@2026-01-26

⟦Ω:Meta⟧{{}}

⟦Σ:Types⟧{{}}

⟦Γ:Rules⟧{{}}

⟦Λ:Funcs⟧{{}}

⟦Ε⟧⟨⟩"#, name);

            let validator = AispValidator::new();
            let result = validator.validate(&doc);
            
            // Should not panic and should have clear validation result
            prop_assert!(result.tier_value <= 4);
            prop_assert!(result.delta >= 0.0 && result.delta <= 1.0);
        }

        /// Property: Large documents should not cause performance issues
        #[test]
        fn prop_large_documents_performant(repetitions in 1..50usize) {
            let large_doc = format!(r#"𝔸5.1.large@2026-01-26

⟦Ω:Meta⟧{{
  domain≜large-test
}}

⟦Σ:Types⟧{{{}
}}

⟦Γ:Rules⟧{{{}
}}

⟦Λ:Funcs⟧{{{}
}}

⟦Ε⟧⟨δ≜0.5⟩"#, 
                (0..repetitions).map(|i| format!("  Type{}≜ℕ", i)).collect::<Vec<_>>().join("\n"),
                (0..repetitions).map(|i| format!("  ∀x:Type{}→Valid(x)", i)).collect::<Vec<_>>().join("\n"),
                (0..repetitions).map(|i| format!("  func{}≜λx.x", i)).collect::<Vec<_>>().join("\n")
            );

            let validator = AispValidator::new();
            let start = std::time::Instant::now();
            let result = validator.validate(&large_doc);
            let duration = start.elapsed();
            
            // Should complete within reasonable time (adjust threshold as needed)
            prop_assert!(duration.as_secs() < 5, "Large documents should validate within 5 seconds");
            prop_assert!(result.tier_value <= 4);
        }

        /// Property: Documents with special Unicode characters should be handled
        #[test]
        fn prop_unicode_handling(name in aisp_identifier()) {
            let doc = format!(r#"𝔸5.1.{}@2026-01-26

⟦Ω:Meta⟧{{
  domain≜unicode-test
  protocol≜"∀∃∧∨⇒⇔¬"
}}

⟦Σ:Types⟧{{
  ℕ≜ℕ
  𝔹≜{{⊤,⊥}}
}}

⟦Γ:Rules⟧{{
  ∀x:ℕ→x≥0
  ∀p:𝔹→p≡⊤∨p≡⊥
}}

⟦Λ:Funcs⟧{{
  id≜λx.x
}}

⟦Ε⟧⟨δ≜0.7;τ≜◊⁺⟩"#, name);

            let validator = AispValidator::new();
            let result = validator.validate(&doc);
            
            // Should handle Unicode properly without panicking
            prop_assert!(result.tier_value <= 4);
            
            if result.valid {
                prop_assert!(result.delta >= 0.0 && result.delta <= 1.0);
            }
        }
    }
}