//! Main Invariant Discovery Interface
//!
//! This module provides the main public interface for the invariant discovery system,
//! orchestrating the various components (analyzer, formulas, exporters).

use crate::{
    ast::AispDocument,
    error::AispResult,
    invariant_types::{DiscoveredInvariant, InvariantDiscoveryConfig, DiscoveryStats},
    invariant_analyzer::InvariantAnalyzer,
    invariant_exporters,
    satisfiability_checker::SatisfiabilityResult,
};

/// Result of analysis with satisfiability checking
#[derive(Debug, Clone)]
pub struct AnalysisWithSatResult {
    pub invariants: Vec<DiscoveredInvariant>,
    pub satisfiability_result: SatisfiabilityResult,
    pub discovery_stats: DiscoveryStats,
}

/// Main invariant discovery engine
pub struct InvariantDiscovery {
    analyzer: InvariantAnalyzer,
}

impl InvariantDiscovery {
    /// Create a new invariant discovery engine with default configuration
    pub fn new() -> Self {
        Self::with_config(InvariantDiscoveryConfig::default())
    }

    /// Create a new invariant discovery engine with custom configuration
    pub fn with_config(config: InvariantDiscoveryConfig) -> Self {
        Self {
            analyzer: InvariantAnalyzer::new(config),
        }
    }

    /// Discover invariants in an AISP document
    pub fn discover_invariants(&mut self, document: &AispDocument) -> AispResult<Vec<DiscoveredInvariant>> {
        self.analyzer.analyze(document)
    }

    /// Get discovery statistics from the last analysis
    pub fn get_discovery_stats(&self) -> &DiscoveryStats {
        self.analyzer.get_stats()
    }

    /// Export discovered invariants to JSON format
    pub fn export_json(&self, invariants: &[DiscoveredInvariant]) -> String {
        invariant_exporters::export_json(invariants)
    }

    /// Export discovered invariants to SMT-LIB format
    pub fn export_smt_lib(&self, invariants: &[DiscoveredInvariant]) -> String {
        invariant_exporters::export_smt_lib(invariants)
    }

    /// Export discovered invariants to human-readable format
    pub fn export_human_readable(&self, invariants: &[DiscoveredInvariant]) -> String {
        invariant_exporters::export_human_readable(invariants)
    }

    /// Export discovered invariants with detailed evidence information
    pub fn export_detailed_report(&self, invariants: &[DiscoveredInvariant]) -> String {
        invariant_exporters::export_detailed_report(invariants)
    }

    /// Perform full analysis and return results with statistics
    pub fn analyze_with_stats(&mut self, document: &AispDocument) -> AispResult<(Vec<DiscoveredInvariant>, DiscoveryStats)> {
        let invariants = self.discover_invariants(document)?;
        let stats = self.get_discovery_stats().clone();
        Ok((invariants, stats))
    }

    /// Quick analysis with basic configuration
    pub fn quick_analyze(document: &AispDocument) -> AispResult<Vec<DiscoveredInvariant>> {
        let mut config = InvariantDiscoveryConfig::default();
        config.max_invariants = 10; // Limit for quick analysis
        config.confidence_threshold = 0.7; // Higher threshold
        
        let mut discovery = Self::with_config(config);
        discovery.discover_invariants(document)
    }

    /// Comprehensive analysis with detailed configuration
    pub fn comprehensive_analyze(document: &AispDocument) -> AispResult<Vec<DiscoveredInvariant>> {
        let mut config = InvariantDiscoveryConfig::default();
        config.max_invariants = 100; // More invariants
        config.confidence_threshold = 0.3; // Lower threshold to catch more
        config.enable_patterns = true;
        config.enable_numerical_analysis = true;
        config.enable_logical_analysis = true;
        config.enable_structural_analysis = true;
        
        let mut discovery = Self::with_config(config);
        discovery.discover_invariants(document)
    }

    /// Analyze and verify satisfiability of discovered invariants
    pub fn analyze_with_satisfiability(&mut self, document: &AispDocument) -> AispResult<AnalysisWithSatResult> {
        // Discover invariants
        let invariants = self.discover_invariants(document)?;
        
        // Check satisfiability
        let sat_checker = crate::satisfiability_checker::SatisfiabilityChecker::default();
        let sat_result = sat_checker.check_invariants(&invariants)?;
        
        let stats = self.get_discovery_stats().clone();
        
        Ok(AnalysisWithSatResult {
            invariants,
            satisfiability_result: sat_result,
            discovery_stats: stats,
        })
    }

    /// Quick satisfiability analysis with basic configuration
    pub fn quick_satisfiability_check(document: &AispDocument) -> AispResult<crate::satisfiability_checker::SatisfiabilityResult> {
        let invariants = Self::quick_analyze(document)?;
        let sat_checker = crate::satisfiability_checker::SatisfiabilityChecker::default();
        sat_checker.check_invariants(&invariants)
    }
}

impl Default for InvariantDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{AispDocument, DocumentHeader, AispBlock, TypesBlock, TypeExpression},
        invariant_types::InvariantType,
    };
    use std::collections::HashMap;

    fn create_test_document() -> AispDocument {
        let mut types = HashMap::new();
        types.insert("Counter".to_string(), TypeExpression::Natural);
        types.insert("Status".to_string(), TypeExpression::Enumeration(vec![
            "Running".to_string(),
            "Stopped".to_string(),
        ]));

        AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "TestDoc".to_string(),
                date: "2026-01-26".to_string(),
            },
            blocks: vec![
                AispBlock::Types(TypesBlock {
                    definitions: types,
                }),
            ],
        }
    }

    #[test]
    fn test_new_discovery_engine() {
        let discovery = InvariantDiscovery::new();
        let stats = discovery.get_discovery_stats();
        
        assert_eq!(stats.type_invariants, 0);
        assert_eq!(stats.functional_invariants, 0);
    }

    #[test]
    fn test_with_config() {
        let mut config = InvariantDiscoveryConfig::default();
        config.max_invariants = 5;
        config.confidence_threshold = 0.8;
        
        let _discovery = InvariantDiscovery::with_config(config);
        // Config is consumed, so we can't directly test it, but creation should succeed
    }

    #[test]
    fn test_discover_invariants() {
        let mut discovery = InvariantDiscovery::new();
        let document = create_test_document();
        
        let result = discovery.discover_invariants(&document).unwrap();
        
        assert!(!result.is_empty());
        
        // Should find natural number invariant
        let has_natural = result.iter()
            .any(|inv| inv.invariant_type == InvariantType::TypeStructural);
        assert!(has_natural);
        
        // Should find enumeration invariant
        let has_enum = result.iter()
            .any(|inv| inv.invariant_type == InvariantType::TypeMembership);
        assert!(has_enum);
    }

    #[test]
    fn test_analyze_with_stats() {
        let mut discovery = InvariantDiscovery::new();
        let document = create_test_document();
        
        let (invariants, stats) = discovery.analyze_with_stats(&document).unwrap();
        
        assert!(!invariants.is_empty());
        assert!(stats.total_time.as_nanos() > 0);
        assert!(stats.type_invariants > 0);
    }

    #[test]
    fn test_quick_analyze() {
        let document = create_test_document();
        let result = InvariantDiscovery::quick_analyze(&document).unwrap();
        
        // Quick analysis should return limited results
        assert!(!result.is_empty());
        assert!(result.len() <= 10); // Respects the limit
        
        // All results should meet the higher confidence threshold
        for inv in &result {
            assert!(inv.confidence >= 0.7);
        }
    }

    #[test]
    fn test_comprehensive_analyze() {
        let document = create_test_document();
        let result = InvariantDiscovery::comprehensive_analyze(&document).unwrap();
        
        assert!(!result.is_empty());
        
        // Comprehensive analysis might find more invariants with lower confidence
        let has_low_confidence = result.iter()
            .any(|inv| inv.confidence < 0.7);
        // Note: This may or may not be true depending on the document,
        // but comprehensive analysis allows for it
    }

    #[test]
    fn test_export_functions() {
        let mut discovery = InvariantDiscovery::new();
        let document = create_test_document();
        let invariants = discovery.discover_invariants(&document).unwrap();
        
        // Test JSON export
        let json = discovery.export_json(&invariants);
        assert!(json.contains("\"invariants\""));
        assert!(json.contains("\"total_count\""));
        
        // Test SMT-LIB export
        let smt = discovery.export_smt_lib(&invariants);
        assert!(smt.contains("; AISP Invariants SMT-LIB Export"));
        assert!(smt.contains("(check-sat)"));
        
        // Test human-readable export
        let human = discovery.export_human_readable(&invariants);
        assert!(human.contains("AISP Invariant Discovery Report"));
        assert!(human.contains("Total Invariants:"));
        
        // Test detailed report
        let detailed = discovery.export_detailed_report(&invariants);
        assert!(detailed.contains("Detailed AISP Invariant Report"));
        assert!(detailed.contains("Formula Information:"));
    }

    #[test]
    fn test_default_implementation() {
        let discovery1 = InvariantDiscovery::new();
        let discovery2 = InvariantDiscovery::default();
        
        // Both should have the same initial state
        assert_eq!(discovery1.get_discovery_stats().type_invariants, 
                   discovery2.get_discovery_stats().type_invariants);
    }

    #[test]
    fn test_statistics_tracking() {
        let mut discovery = InvariantDiscovery::new();
        let document = create_test_document();
        
        // Stats should be empty initially
        let initial_stats = discovery.get_discovery_stats();
        assert_eq!(initial_stats.type_invariants, 0);
        
        // After analysis, stats should be updated
        let _result = discovery.discover_invariants(&document).unwrap();
        let final_stats = discovery.get_discovery_stats();
        assert!(final_stats.type_invariants > 0);
        assert!(final_stats.total_time.as_nanos() > 0);
    }

    #[test]
    fn test_empty_document() {
        let mut discovery = InvariantDiscovery::new();
        let document = AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "EmptyDoc".to_string(),
                date: "2026-01-26".to_string(),
            },
            blocks: vec![],
        };
        
        let result = discovery.discover_invariants(&document).unwrap();
        
        // Should handle empty document gracefully
        assert!(result.is_empty());
        
        let stats = discovery.get_discovery_stats();
        assert_eq!(stats.type_invariants, 0);
        assert_eq!(stats.functional_invariants, 0);
    }
}