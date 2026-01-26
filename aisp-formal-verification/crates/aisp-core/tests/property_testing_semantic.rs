//! Property-based tests for semantic analysis
//!
//! This module tests semantic analysis properties including type checking,
//! symbol analysis, and quality metrics calculation.

use proptest::prelude::*;
use aisp_core::{
    ast::*,
    semantic::{SemanticAnalyzer, QualityTier},
    parser_new::AispParser,
};

/// Strategy for generating valid type names
fn type_name() -> impl Strategy<Value = String> {
    prop::string::string_regex(r"[A-Z][a-zA-Z0-9]*")
        .unwrap()
        .prop_filter("Valid type name", |s| s.len() >= 1 && s.len() <= 20)
}

/// Strategy for generating variable names  
fn variable_name() -> impl Strategy<Value = String> {
    prop::string::string_regex(r"[a-z][a-zA-Z0-9]*")
        .unwrap()
        .prop_filter("Valid variable name", |s| s.len() >= 1 && s.len() <= 20)
}

/// Strategy for generating documents with known type definitions
fn document_with_types() -> impl Strategy<Value = String> {
    prop::collection::vec(type_name(), 1..=5)
        .prop_map(|type_names| {
            let type_defs = type_names.iter()
                .map(|name| format!("  {}≜ℕ", name))
                .collect::<Vec<_>>()
                .join("\n");
            
            let rules = type_names.iter()
                .map(|name| format!("  ∀x:{}→Valid(x)", name))
                .collect::<Vec<_>>()
                .join("\n");
            
            format!(r#"𝔸5.1.semantic@2026-01-26

⟦Ω:Meta⟧{{
  domain≜semantic-test
}}

⟦Σ:Types⟧{{
{}
}}

⟦Γ:Rules⟧{{
{}
}}

⟦Λ:Funcs⟧{{
  id≜λx.x
}}

⟦Ε⟧⟨δ≜0.7⟩"#, type_defs, rules)
        })
}

/// Strategy for generating documents with type conflicts
fn document_with_conflicts() -> impl Strategy<Value = String> {
    (type_name(), variable_name())
        .prop_map(|(type_name, var_name)| {
            format!(r#"𝔸5.1.conflicts@2026-01-26

⟦Ω:Meta⟧{{
  domain≜conflict-test
}}

⟦Σ:Types⟧{{
  {}≜ℕ
  {}≜𝔹
}}

⟦Γ:Rules⟧{{
  ∀{}:{}→Valid({})
  ∀{}:{}→Invalid({})
}}

⟦Λ:Funcs⟧{{
  test≜λx.Valid(x)
}}

⟦Ε⟧⟨δ≜0.5⟩"#, type_name, type_name, var_name, type_name, var_name, type_name, var_name, var_name)
        })
}

/// Strategy for generating documents with varying symbol density
fn document_with_symbol_density() -> impl Strategy<Value = String> {
    (1..=10usize, 0.1f64..=0.9)
        .prop_map(|(complexity, target_delta)| {
            let symbols = "∀∃∧∨⇒⇔¬≜≔≡≤≥∈∉⊆⊇∩∪⊕⊗λδφτΩΣΓΛΕℕℤℝ𝔹𝕊";
            let symbol_chars: Vec<char> = symbols.chars().collect();
            
            let meta_entries = (0..complexity)
                .map(|i| format!("  property{}≜\"{}\"", i, 
                    symbol_chars.iter().take(i % symbol_chars.len()).collect::<String>()))
                .collect::<Vec<_>>()
                .join("\n");
                
            let type_entries = (0..complexity)
                .map(|i| format!("  Type{}≜{{val{}}}", i, i))
                .collect::<Vec<_>>()
                .join("\n");
                
            format!(r#"𝔸5.1.density@2026-01-26

⟦Ω:Meta⟧{{
  domain≜density-test
{}
}}

⟦Σ:Types⟧{{
{}
}}

⟦Γ:Rules⟧{{
  ∀x:Type0→Valid(x)
}}

⟦Λ:Funcs⟧{{
  id≜λx.x
}}

⟦Ε⟧⟨δ≜{:.3}⟩"#, meta_entries, type_entries, target_delta)
        })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    /// Property: Type analysis should be consistent with document structure
    #[test]
    fn prop_type_analysis_consistency(doc in document_with_types()) {
        let mut parser = AispParser::new(doc.clone());
        if let Ok(parsed_doc) = parser.parse() {
            let mut analyzer = SemanticAnalyzer::new();
            if let Ok(analysis) = analyzer.analyze(&parsed_doc, &doc) {
                // Type definitions count should match AST
                let types_block = parsed_doc.blocks.iter()
                    .find_map(|b| match b {
                        AispBlock::Types(types) => Some(types),
                        _ => None,
                    });
                
                if let Some(types_block) = types_block {
                    prop_assert_eq!(
                        analysis.type_analysis.type_definitions.len(),
                        types_block.definitions.len(),
                        "Type definitions count should match"
                    );
                    
                    // All types in AST should be in analysis
                    for (type_name, _) in &types_block.definitions {
                        prop_assert!(
                            analysis.type_analysis.type_definitions.contains_key(type_name),
                            "Type '{}' should be in analysis", type_name
                        );
                    }
                }
            }
        }
    }

    /// Property: Symbol statistics should be monotonic with document size
    #[test]
    fn prop_symbol_stats_monotonic(doc in document_with_symbol_density()) {
        let mut parser = AispParser::new(doc.clone());
        if let Ok(parsed_doc) = parser.parse() {
            let mut analyzer = SemanticAnalyzer::new();
            if let Ok(analysis) = analyzer.analyze(&parsed_doc, &doc) {
                let stats = &analysis.symbol_stats;
                
                // Total symbols should be <= total tokens
                prop_assert!(stats.total_symbols <= stats.total_tokens,
                           "Symbol count ({}) should not exceed token count ({})", 
                           stats.total_symbols, stats.total_tokens);
                
                // Weighted score should be reasonable
                prop_assert!(stats.weighted_score >= 0.0 && stats.weighted_score <= 1.0,
                           "Weighted score should be in [0,1]");
                
                // Non-empty documents should have some tokens
                if !doc.trim().is_empty() {
                    prop_assert!(stats.total_tokens > 0, "Non-empty documents should have tokens");
                }
            }
        }
    }

    /// Property: Quality tier should reflect semantic density
    #[test]
    fn prop_quality_tier_reflects_density(doc in document_with_symbol_density()) {
        let mut parser = AispParser::new(doc.clone());
        if let Ok(parsed_doc) = parser.parse() {
            let mut analyzer = SemanticAnalyzer::new();
            if let Ok(analysis) = analyzer.analyze(&parsed_doc, &doc) {
                // Tier should be consistent with delta
                let expected_tier = QualityTier::from_delta(analysis.delta);
                prop_assert_eq!(analysis.tier, expected_tier, 
                              "Quality tier should match delta calculation");
                
                // Delta should be in valid range
                prop_assert!(analysis.delta >= 0.0 && analysis.delta <= 1.0,
                           "Delta should be in [0,1]");
                
                // Pure density should be reasonable
                prop_assert!(analysis.pure_density >= 0.0 && analysis.pure_density <= 1.0,
                           "Pure density should be in [0,1]");
            }
        }
    }

    /// Property: Ambiguity calculation should be stable
    #[test]
    fn prop_ambiguity_stable(doc in document_with_types()) {
        let mut parser = AispParser::new(doc.clone());
        if let Ok(parsed_doc) = parser.parse() {
            let mut analyzer1 = SemanticAnalyzer::new();
            let mut analyzer2 = SemanticAnalyzer::new();
            
            if let (Ok(analysis1), Ok(analysis2)) = (
                analyzer1.analyze(&parsed_doc, &doc),
                analyzer2.analyze(&parsed_doc, &doc)
            ) {
                // Ambiguity should be the same across multiple analyses
                prop_assert!((analysis1.ambiguity - analysis2.ambiguity).abs() < 1e-10,
                           "Ambiguity should be stable across analyses");
                
                // Ambiguity should be in valid range
                prop_assert!(analysis1.ambiguity >= 0.0 && analysis1.ambiguity <= 1.0,
                           "Ambiguity should be in [0,1]");
                
                // Block and binding scores should also be stable
                prop_assert!((analysis1.block_score - analysis2.block_score).abs() < 1e-10,
                           "Block score should be stable");
                prop_assert!((analysis1.binding_score - analysis2.binding_score).abs() < 1e-10,
                           "Binding score should be stable");
            }
        }
    }

    /// Property: Valid documents should have reasonable warning counts
    #[test]
    fn prop_reasonable_warning_counts(doc in document_with_types()) {
        let mut parser = AispParser::new(doc.clone());
        if let Ok(parsed_doc) = parser.parse() {
            let mut analyzer = SemanticAnalyzer::new();
            if let Ok(analysis) = analyzer.analyze(&parsed_doc, &doc) {
                // Warning count should be reasonable for document size
                let doc_size = doc.len();
                let max_expected_warnings = (doc_size / 100).max(10);
                
                prop_assert!(analysis.warnings.len() <= max_expected_warnings,
                           "Warning count ({}) should be reasonable for document size ({})", 
                           analysis.warnings.len(), doc_size);
                
                // If document is valid, it shouldn't have too many warnings
                if analysis.valid {
                    prop_assert!(analysis.warnings.len() <= 5,
                               "Valid documents should have few warnings");
                }
            }
        }
    }

    /// Property: Type conflicts should be detected properly
    #[test]
    fn prop_type_conflicts_detected(doc in document_with_conflicts()) {
        let mut parser = AispParser::new(doc.clone());
        if let Ok(parsed_doc) = parser.parse() {
            let mut analyzer = SemanticAnalyzer::new();
            if let Ok(analysis) = analyzer.analyze(&parsed_doc, &doc) {
                // Documents with intentional conflicts should have lower validity
                // or warnings about the conflicts
                
                let has_type_warnings = analysis.warnings.iter()
                    .any(|w| w.to_string().contains("type") || 
                             w.to_string().contains("conflict") ||
                             w.to_string().contains("redefin"));
                
                // Either the document is invalid or there are type-related warnings
                prop_assert!(
                    !analysis.valid || has_type_warnings || analysis.warnings.len() > 0,
                    "Documents with type conflicts should be invalid or have warnings"
                );
            }
        }
    }

    /// Property: Function signatures should be consistent
    #[test]
    fn prop_function_signatures_consistent(doc in document_with_types()) {
        let mut parser = AispParser::new(doc.clone());
        if let Ok(parsed_doc) = parser.parse() {
            let mut analyzer = SemanticAnalyzer::new();
            if let Ok(analysis) = analyzer.analyze(&parsed_doc, &doc) {
                // Function count in analysis should match AST
                let funcs_block = parsed_doc.blocks.iter()
                    .find_map(|b| match b {
                        AispBlock::Functions(funcs) => Some(funcs),
                        _ => None,
                    });
                
                if let Some(funcs_block) = funcs_block {
                    prop_assert_eq!(
                        analysis.type_analysis.function_signatures.len(),
                        funcs_block.functions.len(),
                        "Function signature count should match AST"
                    );
                    
                    // Each function in AST should have a signature
                    for (func_name, _) in &funcs_block.functions {
                        prop_assert!(
                            analysis.type_analysis.function_signatures.contains_key(func_name),
                            "Function '{}' should have a signature", func_name
                        );
                    }
                }
            }
        }
    }

    /// Property: Semantic analysis should handle empty blocks gracefully
    #[test]
    fn prop_empty_blocks_handled(name in prop::string::string_regex(r"[a-z]+").unwrap()) {
        let doc = format!(r#"𝔸5.1.{}@2026-01-26

⟦Ω:Meta⟧{{}}

⟦Σ:Types⟧{{}}

⟦Γ:Rules⟧{{}}

⟦Λ:Funcs⟧{{}}

⟦Ε⟧⟨⟩"#, name);

        let mut parser = AispParser::new(doc.clone());
        if let Ok(parsed_doc) = parser.parse() {
            let mut analyzer = SemanticAnalyzer::new();
            let result = analyzer.analyze(&parsed_doc, &doc);
            
            // Should not panic and should return reasonable values
            prop_assert!(result.is_ok(), "Empty blocks should be handled gracefully");
            
            if let Ok(analysis) = result {
                prop_assert!(analysis.delta >= 0.0 && analysis.delta <= 1.0,
                           "Delta should be in valid range for empty blocks");
                prop_assert!(analysis.ambiguity >= 0.0 && analysis.ambiguity <= 1.0,
                           "Ambiguity should be in valid range for empty blocks");
            }
        }
    }

    /// Property: Block score should reflect actual block presence
    #[test]
    fn prop_block_score_accurate(doc in document_with_types()) {
        let mut parser = AispParser::new(doc.clone());
        if let Ok(parsed_doc) = parser.parse() {
            let mut analyzer = SemanticAnalyzer::new();
            if let Ok(analysis) = analyzer.analyze(&parsed_doc, &doc) {
                // Count actual block types
                let mut block_types = std::collections::HashSet::new();
                for block in &parsed_doc.blocks {
                    block_types.insert(block.block_type());
                }
                
                let expected_score = block_types.len() as f64 / 5.0; // 5 total block types
                
                prop_assert!((analysis.block_score - expected_score).abs() < 0.01,
                           "Block score ({:.3}) should reflect actual blocks (expected {:.3})",
                           analysis.block_score, expected_score);
                
                // Block score should be in [0,1]
                prop_assert!(analysis.block_score >= 0.0 && analysis.block_score <= 1.0,
                           "Block score should be in [0,1]");
            }
        }
    }
}

/// Additional semantic property tests
#[cfg(test)]
mod semantic_edge_cases {
    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(25))]

        /// Property: Semantic analysis should handle Unicode properly
        #[test]
        fn prop_unicode_semantic_handling(name in prop::string::string_regex(r"[a-z]+").unwrap()) {
            let doc = format!(r#"𝔸5.1.{}@2026-01-26

⟦Ω:Meta⟧{{
  domain≜unicode-test
  protocol≜"∀∃∧∨⇒⇔¬"
}}

⟦Σ:Types⟧{{
  ℕ≜ℕ
  𝔹≜{{⊤,⊥}}
  𝔽≜{{α,β,γ,δ,ε,ζ,η,θ,ι,κ,λ,μ,ν,ξ,ο,π,ρ,σ,τ,υ,φ,χ,ψ,ω}}
}}

⟦Γ:Rules⟧{{
  ∀x:ℕ→x≥0
  ∀p:𝔹→p≡⊤∨p≡⊥
  ∀φ:𝔽→Valid(φ)
}}

⟦Λ:Funcs⟧{{
  ℵ≜λx.x
  ∀≜λP.∀x→P(x)
  ∃≜λP.∃x→P(x)
}}

⟦Ε⟧⟨δ≜0.8;φ≜90;τ≜◊⁺⟩"#, name);

            let mut parser = AispParser::new(doc.clone());
            if let Ok(parsed_doc) = parser.parse() {
                let mut analyzer = SemanticAnalyzer::new();
                let result = analyzer.analyze(&parsed_doc, &doc);
                
                prop_assert!(result.is_ok(), "Unicode documents should analyze properly");
                
                if let Ok(analysis) = result {
                    // Should have reasonable symbol statistics
                    prop_assert!(analysis.symbol_stats.total_symbols > 50,
                               "Unicode document should have substantial symbol count");
                    
                    // Should recognize the Unicode type definitions
                    prop_assert!(analysis.type_analysis.type_definitions.contains_key("ℕ"),
                               "Should recognize Unicode type ℕ");
                    prop_assert!(analysis.type_analysis.type_definitions.contains_key("𝔹"),
                               "Should recognize Unicode type 𝔹");
                    prop_assert!(analysis.type_analysis.type_definitions.contains_key("𝔽"),
                               "Should recognize Unicode type 𝔽");
                }
            }
        }

        /// Property: Recursive analysis should be bounded
        #[test]
        fn prop_recursive_analysis_bounded(depth in 1..10usize, name in prop::string::string_regex(r"[a-z]+").unwrap()) {
            // Create a document with nested type references
            let nested_types = (0..depth)
                .map(|i| {
                    if i == 0 {
                        format!("  Type{}≜ℕ", i)
                    } else {
                        format!("  Type{}≜Type{}", i, i-1)
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            
            let doc = format!(r#"𝔸5.1.{}@2026-01-26

⟦Ω:Meta⟧{{
  domain≜recursive-test
}}

⟦Σ:Types⟧{{
{}
}}

⟦Γ:Rules⟧{{
  ∀x:Type0→Valid(x)
}}

⟦Λ:Funcs⟧{{
  id≜λx.x
}}

⟦Ε⟧⟨δ≜0.6⟩"#, name, nested_types);

            let mut parser = AispParser::new(doc.clone());
            if let Ok(parsed_doc) = parser.parse() {
                let mut analyzer = SemanticAnalyzer::new();
                let start = std::time::Instant::now();
                let result = analyzer.analyze(&parsed_doc, &doc);
                let duration = start.elapsed();
                
                // Should complete within reasonable time even with nesting
                prop_assert!(duration.as_millis() < 1000,
                           "Recursive analysis should complete quickly ({}ms)", duration.as_millis());
                
                prop_assert!(result.is_ok(), "Recursive analysis should not fail");
            }
        }

        /// Property: Analysis should be deterministic under concurrent access
        #[test]
        fn prop_concurrent_analysis_deterministic(doc in document_with_types()) {
            use std::sync::{Arc, Mutex};
            use std::thread;
            
            let mut parser = AispParser::new(doc.clone());
            if let Ok(parsed_doc) = parser.parse() {
                let shared_doc = Arc::new(parsed_doc);
                let shared_source = Arc::new(doc);
                let results = Arc::new(Mutex::new(Vec::new()));
                
                let mut handles = vec![];
                
                // Run multiple analyses concurrently
                for _ in 0..3 {
                    let doc_clone = Arc::clone(&shared_doc);
                    let source_clone = Arc::clone(&shared_source);
                    let results_clone = Arc::clone(&results);
                    
                    let handle = thread::spawn(move || {
                        let mut analyzer = SemanticAnalyzer::new();
                        if let Ok(analysis) = analyzer.analyze(&*doc_clone, &*source_clone) {
                            let mut results = results_clone.lock().unwrap();
                            results.push((analysis.delta, analysis.ambiguity, analysis.tier.value()));
                        }
                    });
                    
                    handles.push(handle);
                }
                
                // Wait for all analyses to complete
                for handle in handles {
                    handle.join().unwrap();
                }
                
                let results = results.lock().unwrap();
                if results.len() >= 2 {
                    // All results should be identical
                    for i in 1..results.len() {
                        prop_assert!((results[0].0 - results[i].0).abs() < 1e-10,
                                   "Delta should be deterministic across concurrent analyses");
                        prop_assert!((results[0].1 - results[i].1).abs() < 1e-10,
                                   "Ambiguity should be deterministic across concurrent analyses");
                        prop_assert_eq!(results[0].2, results[i].2,
                                      "Tier should be deterministic across concurrent analyses");
                    }
                }
            }
        }
    }
}