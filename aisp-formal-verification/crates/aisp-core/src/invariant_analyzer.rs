//! Invariant Analysis Engine
//!
//! This module contains the core analysis logic for discovering
//! mathematical invariants from AISP document structures.

use crate::{
    ast::{AispDocument, AispBlock, TypeExpression, TypeDefinition, BasicType},
    error::AispResult,
    invariant_types::{
        DiscoveredInvariant, InvariantType, InvariantEvidence, EvidenceType,
        InvariantDiscoveryConfig, DiscoveryStats,
    },
    property_types::{SourceLocation},
};

/// Core invariant analysis engine
pub struct InvariantAnalyzer {
    config: InvariantDiscoveryConfig,
    discovered_invariants: Vec<DiscoveredInvariant>,
    discovery_stats: DiscoveryStats,
}

impl InvariantAnalyzer {
    /// Create a new invariant analyzer
    pub fn new(config: InvariantDiscoveryConfig) -> Self {
        Self {
            config,
            discovered_invariants: Vec::new(),
            discovery_stats: DiscoveryStats::default(),
        }
    }

    /// Analyze a document and discover invariants
    pub fn analyze(&mut self, document: &AispDocument) -> AispResult<Vec<DiscoveredInvariant>> {
        let start_time = std::time::Instant::now();
        
        // Clear previous results
        self.discovered_invariants.clear();
        self.discovery_stats = DiscoveryStats::default();
        
        // Analyze different aspects of the document
        self.analyze_type_invariants(document)?;
        if self.config.enable_patterns {
            self.analyze_function_invariants(document)?;
        }
        
        // Filter by confidence threshold
        let mut result: Vec<DiscoveredInvariant> = self.discovered_invariants.iter()
            .filter(|inv| inv.confidence >= self.config.confidence_threshold)
            .cloned()
            .collect();
        
        // Limit results
        if result.len() > self.config.max_invariants {
            result.truncate(self.config.max_invariants);
        }
        
        // Update statistics
        self.discovery_stats.total_time = start_time.elapsed();
        self.discovery_stats.type_invariants = result.iter()
            .filter(|inv| matches!(inv.invariant_type, InvariantType::TypeStructural | InvariantType::TypeMembership))
            .count();
        self.discovery_stats.functional_invariants = result.iter()
            .filter(|inv| matches!(inv.invariant_type, InvariantType::FunctionalProperty | InvariantType::FunctionalMonotonicity))
            .count();
        
        Ok(result)
    }

    /// Get analysis statistics
    pub fn get_stats(&self) -> &DiscoveryStats {
        &self.discovery_stats
    }
    
    /// Analyze type-related invariants
    fn analyze_type_invariants(&mut self, document: &AispDocument) -> AispResult<()> {
        for block in &document.blocks {
            if let AispBlock::Types(types_block) = block {
                for (type_name, type_def) in &types_block.definitions {
                    self.analyze_type_definition(type_name, &type_def.type_expr)?;
                }
            }
        }
        Ok(())
    }
    
    /// Analyze function-related invariants
    fn analyze_function_invariants(&mut self, _document: &AispDocument) -> AispResult<()> {
        // Simplified function analysis for now
        // In practice, this would analyze function definitions for patterns
        Ok(())
    }
    
    /// Analyze a single type definition
    fn analyze_type_definition(&mut self, type_name: &str, type_def: &TypeExpression) -> AispResult<()> {
        match type_def {
            TypeExpression::Basic(BasicType::Natural) => {
                self.add_natural_type_invariant(type_name)?;
            }
            TypeExpression::Enumeration(variants) => {
                self.add_enumeration_invariant(type_name, variants)?;
            }
            _ => {
                if self.config.enable_structural_analysis {
                    self.add_generic_type_invariant(type_name)?;
                }
            }
        }
        Ok(())
    }

    /// Add natural number type invariant
    fn add_natural_type_invariant(&mut self, type_name: &str) -> AispResult<()> {
        let formula = crate::invariant_formulas::create_non_negativity_formula(type_name)?;
        let mut invariant = DiscoveredInvariant::new(
            format!("nat_nonneg_{}", type_name),
            format!("Non-negativity of {}", type_name),
            formula,
            InvariantType::TypeStructural,
            0.95,
        );

        invariant.add_evidence(InvariantEvidence::new(
            EvidenceType::TypeSystemEnforcement,
            0.95,
            "Natural number types are non-negative by definition".to_string(),
            SourceLocation {
                block_type: "Types".to_string(),
                line: None,
                column: None,
                source_text: Some(format!("{}≜ℕ", type_name)),
            },
        ));

        invariant.add_source(SourceLocation {
            block_type: "Types".to_string(),
            line: None,
            column: None,
            source_text: Some(format!("{}≜ℕ", type_name)),
        });

        self.discovered_invariants.push(invariant);
        Ok(())
    }

    /// Add enumeration type invariant
    fn add_enumeration_invariant(&mut self, type_name: &str, variants: &[String]) -> AispResult<()> {
        let formula = crate::invariant_formulas::create_membership_formula(type_name, variants)?;
        let mut invariant = DiscoveredInvariant::new(
            format!("enum_member_{}", type_name),
            format!("Membership constraint for {}", type_name),
            formula,
            InvariantType::TypeMembership,
            0.90,
        );

        invariant.add_evidence(InvariantEvidence::new(
            EvidenceType::TypeSystemEnforcement,
            0.90,
            format!("Enumeration {} must be one of {:?}", type_name, variants),
            SourceLocation {
                block_type: "Types".to_string(),
                line: None,
                column: None,
                source_text: Some(format!("{}≜{{{}}}", type_name, variants.join(","))),
            },
        ));

        invariant.add_source(SourceLocation {
            block_type: "Types".to_string(),
            line: None,
            column: None,
            source_text: Some(format!("{}≜{{{}}}", type_name, variants.join(","))),
        });

        self.discovered_invariants.push(invariant);
        Ok(())
    }

    /// Add generic type invariant
    fn add_generic_type_invariant(&mut self, type_name: &str) -> AispResult<()> {
        let formula = crate::invariant_formulas::create_well_formed_formula(type_name)?;
        let mut invariant = DiscoveredInvariant::new(
            format!("generic_type_{}", type_name),
            format!("Type consistency for {}", type_name),
            formula,
            InvariantType::TypeStructural,
            0.75,
        );

        invariant.add_evidence(InvariantEvidence::new(
            EvidenceType::TypeSystemEnforcement,
            0.75,
            format!("Type {} must be well-formed", type_name),
            SourceLocation {
                block_type: "Types".to_string(),
                line: None,
                column: None,
                source_text: Some(format!("{}≜...", type_name)),
            },
        ));

        invariant.add_source(SourceLocation {
            block_type: "Types".to_string(),
            line: None,
            column: None,
            source_text: Some(format!("{}≜...", type_name)),
        });

        self.discovered_invariants.push(invariant);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{AispDocument, DocumentHeader, AispBlock, TypesBlock, TypeDefinition, 
              TypeExpression, BasicType},
        invariant_types::InvariantDiscoveryConfig,
    };
    use std::{collections::HashMap, time::Duration};

    fn create_test_document() -> AispDocument {
        let mut types = HashMap::new();
        types.insert("Natural".to_string(), TypeDefinition {
            name: "Natural".to_string(),
            type_expr: TypeExpression::Basic(BasicType::Natural),
            span: crate::ast::Span { start: 0, end: 0 },
        });
        types.insert("Status".to_string(), TypeDefinition {
            name: "Status".to_string(),
            type_expr: TypeExpression::Enumeration(vec![
                "Active".to_string(),
                "Inactive".to_string(),
            ]),
            span: crate::ast::Span { start: 0, end: 0 },
        });

        AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "TestDoc".to_string(),
                date: "2026-01-26".to_string(),
            },
            blocks: vec![
                AispBlock::Types(TypesBlock {
                    definitions: types,
                    span: crate::ast::Span { start: 0, end: 0 },
                }),
            ],
        }
    }

    #[test]
    fn test_analyzer_creation() {
        let config = InvariantDiscoveryConfig::default();
        let analyzer = InvariantAnalyzer::new(config);
        
        assert_eq!(analyzer.discovered_invariants.len(), 0);
        assert_eq!(analyzer.discovery_stats.total_time, Duration::new(0, 0));
    }

    #[test]
    fn test_analyze_type_invariants() {
        let config = InvariantDiscoveryConfig::default();
        let mut analyzer = InvariantAnalyzer::new(config);
        let document = create_test_document();
        
        let result = analyzer.analyze(&document).unwrap();
        
        // Should discover at least one invariant
        assert!(!result.is_empty());
        
        // Should have natural number non-negativity invariant
        let has_natural_invariant = result.iter()
            .any(|inv| inv.id.contains("nat_nonneg") && inv.invariant_type == InvariantType::TypeStructural);
        assert!(has_natural_invariant);
        
        // Should have enumeration membership invariant
        let has_enum_invariant = result.iter()
            .any(|inv| inv.id.contains("enum_member") && inv.invariant_type == InvariantType::TypeMembership);
        assert!(has_enum_invariant);
    }

    #[test]
    fn test_confidence_threshold_filtering() {
        let mut config = InvariantDiscoveryConfig::default();
        config.confidence_threshold = 0.9; // High threshold
        
        let mut analyzer = InvariantAnalyzer::new(config);
        let document = create_test_document();
        
        let result = analyzer.analyze(&document).unwrap();
        
        // All results should meet the confidence threshold
        for invariant in &result {
            assert!(invariant.confidence >= 0.9);
        }
    }

    #[test]
    fn test_max_invariants_limit() {
        let mut config = InvariantDiscoveryConfig::default();
        config.max_invariants = 1; // Limit to 1
        
        let mut analyzer = InvariantAnalyzer::new(config);
        let document = create_test_document();
        
        let result = analyzer.analyze(&document).unwrap();
        
        // Should not exceed the limit
        assert!(result.len() <= 1);
    }

    #[test]
    fn test_statistics_tracking() {
        let config = InvariantDiscoveryConfig::default();
        let mut analyzer = InvariantAnalyzer::new(config);
        let document = create_test_document();
        
        let _result = analyzer.analyze(&document).unwrap();
        let stats = analyzer.get_stats();
        
        // Should have recorded some analysis time
        assert!(stats.total_time > Duration::new(0, 0));
        
        // Should have discovered some type invariants
        assert!(stats.type_invariants > 0);
    }

    #[test]
    fn test_structural_analysis_config() {
        let mut config = InvariantDiscoveryConfig::default();
        config.enable_structural_analysis = false;
        
        let mut analyzer = InvariantAnalyzer::new(config);
        
        // Create document with only generic types (not natural or enum)
        let mut types = HashMap::new();
        types.insert("CustomType".to_string(), TypeDefinition {
            name: "CustomType".to_string(),
            type_expr: TypeExpression::Basic(BasicType::Boolean),
            span: crate::ast::Span { start: 0, end: 0 },
        });

        let document = AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "TestDoc".to_string(),
                date: "2026-01-26".to_string(),
            },
            blocks: vec![
                AispBlock::Types(TypesBlock {
                    definitions: types,
                    span: crate::ast::Span { start: 0, end: 0 },
                }),
            ],
        };
        
        let result = analyzer.analyze(&document).unwrap();
        
        // Should not discover generic type invariants when structural analysis is disabled
        let has_generic_invariant = result.iter()
            .any(|inv| inv.id.contains("generic_type"));
        assert!(!has_generic_invariant);
    }
}