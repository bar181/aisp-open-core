//! Canonical AST Module - Single Source of Truth
//! 
//! This module provides the unified, production-ready AST representation
//! following SRP architecture with focused sub-modules:
//!
//! - `types`: Core AST types, spans, headers, and fundamental structures
//! - `blocks`: Block definitions, parsing, and structured operations  
//! - `document`: Document-level operations and management
//! - `conversions`: Migration utilities and backward compatibility

// Re-export all public types and functions
pub use self::types::*;
pub use self::blocks::*; 
pub use self::document::*;
pub use self::conversions::*;

// Explicit public re-exports for external visibility
pub use self::document::CanonicalAispDocument;
pub use self::blocks::CanonicalAispBlock;

// Module declarations
pub mod types;
pub mod blocks;
pub mod document;
pub mod conversions;

// Convenience functions for common operations
pub fn create_document(name: &str, version: &str, date: &str) -> CanonicalAispDocument {
    CanonicalAispDocument::new(name.to_string(), version.to_string(), date.to_string())
}

pub fn create_meta_block(raw_entries: Vec<String>) -> CanonicalAispBlock {
    CanonicalAispBlock::Meta(MetaBlock {
        entries: std::collections::HashMap::new(),
        raw_entries,
        span: None,
    })
}

pub fn create_types_block(raw_definitions: Vec<String>) -> CanonicalAispBlock {
    CanonicalAispBlock::Types(TypesBlock {
        definitions: std::collections::HashMap::new(),
        raw_definitions,
        span: None,
    })
}

pub fn create_rules_block(raw_rules: Vec<String>) -> CanonicalAispBlock {
    CanonicalAispBlock::Rules(RulesBlock {
        rules: Vec::new(),
        raw_rules,
        span: None,
    })
}

pub fn create_functions_block(raw_functions: Vec<String>) -> CanonicalAispBlock {
    CanonicalAispBlock::Functions(FunctionsBlock {
        functions: Vec::new(),
        raw_functions,
        span: None,
    })
}

pub fn create_evidence_block(raw_evidence: Vec<String>) -> CanonicalAispBlock {
    CanonicalAispBlock::Evidence(EvidenceBlock {
        delta: None,
        phi: None,
        tau: None,
        metrics: std::collections::HashMap::new(),
        raw_evidence,
        span: None,
    })
}

// Type aliases for backward compatibility during migration
pub type AispBlock = CanonicalAispBlock;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_module_integration() {
        // Test that all components work together seamlessly
        let mut doc = create_document("integration_test", "5.1", "2026-01-27");
        
        // Add various block types
        doc.add_block(create_meta_block(vec!["Vision≜\"Integration Test\"".to_string()]));
        doc.add_block(create_types_block(vec!["TestType≜Natural".to_string()]));
        doc.add_block(create_rules_block(vec!["∀x ∈ ℕ: x ≥ 0".to_string()]));
        doc.add_block(create_functions_block(vec!["f≜λx.x + 1".to_string()]));
        doc.add_block(create_evidence_block(vec!["δ≜0.001".to_string()]));
        
        // Test document has all blocks
        assert_eq!(doc.blocks.len(), 5);
        
        // Parse structured data
        doc.parse_structured_data();
        
        // Verify parsing worked
        let meta_blocks = doc.get_meta_blocks();
        assert_eq!(meta_blocks.len(), 1);
        assert!(meta_blocks[0].entries.contains_key("Vision"));
        
        let evidence_blocks = doc.get_evidence_blocks();
        assert_eq!(evidence_blocks.len(), 1);
        assert_eq!(evidence_blocks[0].delta, Some(0.001));
    }

    #[test]
    fn test_backward_compatibility_aliases() {
        let doc: AispDocument = create_document("test", "5.1", "2026-01-27");
        assert_eq!(doc.header.name, "test");
        
        let block: AispBlock = create_meta_block(vec!["test≜value".to_string()]);
        match block {
            AispBlock::Meta(_) => {}, // Success
            _ => panic!("Expected Meta block"),
        }
    }

    #[test]
    fn test_conversion_integration() {
        use crate::parser::robust_parser;
        
        // Create a robust parser document
        let robust_doc = robust_parser::AispDocument {
            header: robust_parser::DocumentHeader {
                version: "5.1".to_string(),
                name: "conversion_test".to_string(),
                date: "2026-01-27".to_string(),
                metadata: None,
            },
            metadata: robust_parser::DocumentMetadata {
                domain: Some("test".to_string()),
                protocol: Some("aisp".to_string()),
            },
            blocks: vec![
                robust_parser::AispBlock::Meta(robust_parser::MetaBlock {
                    entries: vec!["Vision≜\"Converted Document\"".to_string()],
                }),
            ],
        };

        // Convert to canonical
        let canonical = robust_doc.into_canonical();
        assert_eq!(canonical.header.name, "conversion_test");
        assert_eq!(canonical.metadata.domain, Some("test".to_string()));
        assert_eq!(canonical.blocks.len(), 1);
    }

    #[test]
    fn test_document_statistics_integration() {
        let mut doc = create_document("stats_test", "5.1", "2026-01-27");
        
        doc.add_block(create_meta_block(vec!["key1≜value1".to_string(), "key2≜value2".to_string()]));
        doc.add_block(create_types_block(vec!["Type1≜Natural".to_string()]));
        doc.add_block(create_rules_block(vec!["rule1".to_string(), "rule2".to_string()]));
        
        doc.parse_structured_data();
        
        let stats = doc.get_statistics();
        assert_eq!(stats.meta_blocks, 1);
        assert_eq!(stats.meta_entries, 2);
        assert_eq!(stats.types_blocks, 1);
        assert_eq!(stats.type_definitions, 1);
        assert_eq!(stats.rules_blocks, 1);
        assert_eq!(stats.logical_rules, 2);
    }

    #[test]
    fn test_validation_integration() {
        let mut doc = create_document("", "5.1", "2026-01-27"); // Invalid empty name
        
        let validation = doc.validate_structure();
        assert!(!validation.is_valid());
        assert!(validation.errors.iter().any(|e| e.contains("name is empty")));
        
        // Fix the name
        doc.header.name = "valid_name".to_string();
        let validation = doc.validate_structure();
        assert!(validation.is_valid());
    }

    #[test]
    fn test_extraction_utilities_integration() {
        let mut doc = create_document("extract_test", "5.1", "2026-01-27");
        
        doc.add_block(create_meta_block(vec!["meta_key≜meta_value".to_string()]));
        doc.add_block(create_types_block(vec!["TestType≜Integer".to_string()]));
        doc.add_block(create_evidence_block(vec!["δ≜0.5".to_string()]));
        
        doc.parse_structured_data();
        
        // Test extraction utilities
        let meta_entries = extract_all_meta_entries(&doc);
        assert_eq!(meta_entries.len(), 1);
        assert!(meta_entries.contains_key("meta_key"));
        
        let type_defs = extract_all_type_definitions(&doc);
        assert_eq!(type_defs.len(), 1);
        assert!(type_defs.contains_key("TestType"));
        
        let evidence_metrics = extract_all_evidence_metrics(&doc);
        // Evidence metrics are different from delta/phi/tau values
        assert_eq!(evidence_metrics.len(), 0); // No custom metrics added
    }

    #[test]
    fn test_convenience_functions() {
        // Test all convenience block creation functions
        let meta = create_meta_block(vec!["test≜value".to_string()]);
        let types = create_types_block(vec!["T≜Natural".to_string()]);
        let rules = create_rules_block(vec!["rule".to_string()]);
        let functions = create_functions_block(vec!["f≜x".to_string()]);
        let evidence = create_evidence_block(vec!["δ≜0.1".to_string()]);
        
        assert_eq!(meta.block_type(), "Meta");
        assert_eq!(types.block_type(), "Types");
        assert_eq!(rules.block_type(), "Rules");
        assert_eq!(functions.block_type(), "Functions");
        assert_eq!(evidence.block_type(), "Evidence");
    }

    #[test]
    fn test_comprehensive_document_lifecycle() {
        // Create document
        let mut doc = create_document("lifecycle_test", "5.1", "2026-01-27");
        
        // Set metadata
        doc.set_domain("mathematics".to_string());
        doc.set_protocol("aisp".to_string());
        
        // Add blocks
        doc.add_block(create_meta_block(vec![
            "Author≜\"Test Author\"".to_string(),
            "Version≜\"1.0\"".to_string(),
        ]));
        
        doc.add_block(create_types_block(vec![
            "Natural≜ℕ".to_string(),
            "Real≜ℝ".to_string(),
        ]));
        
        doc.add_block(create_rules_block(vec![
            "∀x ∈ ℕ: x ≥ 0".to_string(),
            "∀x ∈ ℝ: x ∈ ℝ".to_string(),
        ]));
        
        doc.add_block(create_functions_block(vec![
            "successor≜λn.n + 1".to_string(),
            "identity≜λx.x".to_string(),
        ]));
        
        doc.add_block(create_evidence_block(vec![
            "δ≜0.001".to_string(),
            "φ≜42".to_string(),
            "τ≜formal".to_string(),
        ]));
        
        // Parse structured data
        doc.parse_structured_data();
        
        // Validate
        let validation = doc.validate_structure();
        assert!(validation.is_valid());
        
        // Get statistics
        let stats = doc.get_statistics();
        assert_eq!(stats.meta_blocks, 1);
        assert_eq!(stats.meta_entries, 2);
        assert_eq!(stats.types_blocks, 1);
        assert_eq!(stats.type_definitions, 2);
        assert_eq!(stats.rules_blocks, 1);
        assert_eq!(stats.logical_rules, 2);
        assert_eq!(stats.functions_blocks, 1);
        assert_eq!(stats.function_definitions, 2);
        assert_eq!(stats.evidence_blocks, 1);
        
        // Verify specific content
        let meta_blocks = doc.get_meta_blocks();
        assert!(meta_blocks[0].entries.contains_key("Author"));
        assert!(meta_blocks[0].entries.contains_key("Version"));
        
        let evidence_blocks = doc.get_evidence_blocks();
        assert_eq!(evidence_blocks[0].delta, Some(0.001));
        assert_eq!(evidence_blocks[0].phi, Some(42));
        assert_eq!(evidence_blocks[0].tau, Some("formal".to_string()));
    }
}