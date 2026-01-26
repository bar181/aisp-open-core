//! Automated Invariant Discovery System
//!
//! This module provides a unified interface to the AISP invariant discovery system,
//! which automatically identifies mathematical properties and constraints in AISP documents.
//!
//! The system is composed of several focused modules:
//! - `invariant_types`: Core data structures and configuration
//! - `invariant_formulas`: Mathematical formula construction
//! - `invariant_analyzer`: Analysis engine for discovering invariants
//! - `invariant_exporters`: Export utilities for various formats
//! - `invariant_discovery_main`: Main orchestration interface
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use aisp_core::{invariant_discovery::InvariantDiscovery, parser_new::AispParser};
//!
//! // Parse an AISP document
//! let mut parser = AispParser::new(document_text);
//! let document = parser.parse().unwrap();
//!
//! // Discover invariants
//! let mut discovery = InvariantDiscovery::new();
//! let invariants = discovery.discover_invariants(&document).unwrap();
//!
//! // Export results
//! println!("{}", discovery.export_human_readable(&invariants));
//! ```

pub use crate::invariant_types::*;
pub use crate::invariant_formulas::*;
pub use crate::invariant_analyzer::*;
pub use crate::invariant_exporters::*;
pub use crate::invariant_discovery_main::*;
pub use crate::satisfiability_checker::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{AispDocument, DocumentHeader, AispBlock, TypesBlock, TypeExpression},
        parser_new::AispParser,
    };
    use std::collections::HashMap;

    #[test]
    fn test_full_invariant_discovery_workflow() {
        // Create a test document
        let mut types = HashMap::new();
        types.insert("Counter".to_string(), TypeExpression::Natural);
        types.insert("State".to_string(), TypeExpression::Enumeration(vec![
            "Init".to_string(),
            "Running".to_string(),
            "Complete".to_string(),
        ]));

        let document = AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "WorkflowTest".to_string(),
                date: "2026-01-26".to_string(),
            },
            blocks: vec![
                AispBlock::Types(TypesBlock {
                    definitions: types,
                }),
            ],
        };

        // Test the full workflow
        let mut discovery = InvariantDiscovery::new();
        
        // Discover invariants
        let invariants = discovery.discover_invariants(&document).unwrap();
        assert!(!invariants.is_empty());
        
        // Test different export formats
        let json_export = discovery.export_json(&invariants);
        assert!(json_export.contains("\"invariants\""));
        
        let smt_export = discovery.export_smt_lib(&invariants);
        assert!(smt_export.contains("; AISP Invariants"));
        
        let human_export = discovery.export_human_readable(&invariants);
        assert!(human_export.contains("AISP Invariant Discovery Report"));
        
        let detailed_export = discovery.export_detailed_report(&invariants);
        assert!(detailed_export.contains("Detailed AISP Invariant Report"));
        
        // Test statistics
        let stats = discovery.get_discovery_stats();
        assert!(stats.type_invariants > 0);
        assert!(stats.total_time.as_nanos() > 0);
    }

    #[test]
    fn test_configuration_workflow() {
        // Test custom configuration
        let mut config = InvariantDiscoveryConfig::default();
        config.max_invariants = 5;
        config.confidence_threshold = 0.8;
        config.enable_patterns = true;
        config.enable_structural_analysis = false;

        let document = AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "ConfigTest".to_string(),
                date: "2026-01-26".to_string(),
            },
            blocks: vec![],
        };

        let mut discovery = InvariantDiscovery::with_config(config);
        let invariants = discovery.discover_invariants(&document).unwrap();
        
        // Empty document should result in no invariants
        assert!(invariants.is_empty());
    }

    #[test]
    fn test_quick_vs_comprehensive_analysis() {
        let mut types = HashMap::new();
        types.insert("Value".to_string(), TypeExpression::Natural);

        let document = AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "AnalysisTest".to_string(),
                date: "2026-01-26".to_string(),
            },
            blocks: vec![
                AispBlock::Types(TypesBlock {
                    definitions: types,
                }),
            ],
        };

        // Quick analysis
        let quick_result = InvariantDiscovery::quick_analyze(&document).unwrap();
        
        // Comprehensive analysis
        let comprehensive_result = InvariantDiscovery::comprehensive_analyze(&document).unwrap();
        
        // Both should find something, comprehensive might find more
        assert!(!quick_result.is_empty());
        assert!(!comprehensive_result.is_empty());
        
        // Quick analysis should have higher confidence threshold
        for inv in &quick_result {
            assert!(inv.confidence >= 0.7);
        }
    }

    #[test]
    fn test_evidence_tracking() {
        let mut types = HashMap::new();
        types.insert("Counter".to_string(), TypeExpression::Natural);

        let document = AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "EvidenceTest".to_string(),
                date: "2026-01-26".to_string(),
            },
            blocks: vec![
                AispBlock::Types(TypesBlock {
                    definitions: types,
                }),
            ],
        };

        let mut discovery = InvariantDiscovery::new();
        let invariants = discovery.discover_invariants(&document).unwrap();
        
        // Check that evidence is properly tracked
        for invariant in &invariants {
            assert!(!invariant.evidence.is_empty());
            assert!(!invariant.sources.is_empty());
            
            // Evidence should have reasonable strength
            for evidence in &invariant.evidence {
                assert!(evidence.strength > 0.0);
                assert!(evidence.strength <= 1.0);
                assert!(!evidence.description.is_empty());
            }
        }
    }

    #[test]
    fn test_formula_construction() {
        // Test individual formula construction functions
        let non_neg_formula = create_non_negativity_formula("TestType").unwrap();
        assert!(!non_neg_formula.predicates.is_empty());
        assert!(non_neg_formula.predicates.contains("≥"));
        
        let variants = vec!["A".to_string(), "B".to_string()];
        let membership_formula = create_membership_formula("TestEnum", &variants).unwrap();
        assert!(membership_formula.predicates.contains("∈"));
        
        let well_formed_formula = create_well_formed_formula("TestGeneric").unwrap();
        assert!(well_formed_formula.predicates.contains("WellFormed"));
        
        let range_formula = create_range_formula("TestRange", 0, 100).unwrap();
        assert!(range_formula.predicates.contains("≥"));
        assert!(range_formula.predicates.contains("≤"));
        
        let identity_formula = create_identity_formula("id", "Any").unwrap();
        assert!(identity_formula.predicates.contains("="));
        assert!(identity_formula.functions.contains("id"));
    }

    #[test] 
    fn test_parser_integration() {
        let aisp_text = r#"𝔸5.1.IntegrationTest@2026-01-26

⟦Ω:Meta⟧{
  domain≜integration-test
}

⟦Σ:Types⟧{
  Number≜ℕ
  Status≜{Active,Inactive}
}

⟦Γ:Rules⟧{
  ∀n:Number→n≥0
}

⟦Λ:Funcs⟧{
  id≜λx.x
}

⟦Ε⟧⟨δ≜0.8⟩"#;

        // Parse the document
        let mut parser = AispParser::new(aisp_text.to_string());
        let document = parser.parse().unwrap();
        
        // Discover invariants
        let mut discovery = InvariantDiscovery::new();
        let invariants = discovery.discover_invariants(&document).unwrap();
        
        assert!(!invariants.is_empty());
        
        // Should find natural number and enumeration invariants
        let has_natural = invariants.iter()
            .any(|inv| inv.invariant_type == InvariantType::TypeStructural);
        let has_enum = invariants.iter()
            .any(|inv| inv.invariant_type == InvariantType::TypeMembership);
            
        assert!(has_natural);
        assert!(has_enum);
        
        // Test export integration
        let report = discovery.export_human_readable(&invariants);
        assert!(report.contains("Number"));
        assert!(report.contains("Status"));
    }
}