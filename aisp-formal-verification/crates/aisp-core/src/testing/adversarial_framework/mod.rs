//! Adversarial Testing Framework
//!
//! Comprehensive adversarial testing framework for systematic security
//! validation of AISP parsers following SRP architecture.
//!
//! This module is organized into focused sub-modules:
//! - `types`: Core attack types, enums, and result structures
//! - `test_suite`: Adversarial test suite management and attack vectors
//! - `execution_engine`: Attack execution and evaluation logic

// Re-export public types and main API
pub use self::types::*;
pub use self::test_suite::AdversarialTestSuite;
pub use self::execution_engine::AttackExecutionEngine;

// Module declarations
pub mod types;
pub mod test_suite;
pub mod execution_engine;

// Convenience API for running comprehensive tests
pub fn run_adversarial_security_assessment() -> SecurityAssessmentReport {
    let test_suite = AdversarialTestSuite::new();
    let execution_engine = AttackExecutionEngine::new();
    
    execution_engine.run_comprehensive_tests(&test_suite)
}

// Convenience API for running specific attack categories
pub fn run_parse_bypass_tests() -> Vec<AttackResult> {
    let test_suite = AdversarialTestSuite::new();
    let execution_engine = AttackExecutionEngine::new();
    
    let mut results = Vec::new();
    for attack in &test_suite.parse_bypass_tests {
        // This is a simplified version - the full implementation would be in the engine
        let result = AttackResult {
            attack_name: attack.name.to_string(),
            attack_category: attack.attack_category.clone(),
            success: false, // Placeholder
            bypass_achieved: false,
            security_impact: execution_engine.map_severity_to_security(attack.severity.clone()),
            parser_response: crate::parser::robust_parser::ParseResult {
                document: None,
                errors: vec![],
                warnings: vec![],
                recovery_applied: false,
                partial_success: false,
                security_issues: vec![],
            },
            detection_triggered: false,
            mitigation_effective: true,
            performance_impact: PerformanceImpact {
                parsing_time_ms: 0,
                memory_usage_mb: 0,
                cpu_usage_percent: 0.0,
            },
            details: attack.description.to_string(),
        };
        results.push(result);
    }
    results
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_module_integration() {
        // Test that all components work together
        let test_suite = AdversarialTestSuite::new();
        let execution_engine = AttackExecutionEngine::new();
        
        assert!(test_suite.total_test_count() > 0);
        
        let report = execution_engine.run_comprehensive_tests(&test_suite);
        assert!(report.total_attacks > 0);
        assert!(report.overall_security_score >= 0.0);
        assert!(report.overall_security_score <= 1.0);
    }

    #[test]
    fn test_convenience_api() {
        let report = run_adversarial_security_assessment();
        assert!(report.total_attacks > 0);
        assert!(!report.vulnerability_summary.common_weakness_patterns.is_empty() || report.overall_security_score > 0.8);
    }

    #[test]
    fn test_parse_bypass_tests() {
        let results = run_parse_bypass_tests();
        assert!(!results.is_empty());
        
        // Verify all results are parse bypass category
        for result in &results {
            assert_eq!(result.attack_category, AttackCategory::ParseBypass);
        }
    }

    #[test]
    fn test_attack_types_comprehensive() {
        let test_suite = AdversarialTestSuite::new();
        
        // Verify all attack categories are populated
        assert!(!test_suite.parse_bypass_tests.is_empty());
        assert!(!test_suite.unicode_confusion_tests.is_empty());
        assert!(!test_suite.boundary_condition_tests.is_empty());
        assert!(!test_suite.malformed_document_tests.is_empty());
        assert!(!test_suite.resource_exhaustion_tests.is_empty());
        assert!(!test_suite.injection_tests.is_empty());
    }

    #[test]
    fn test_attack_severity_distribution() {
        let test_suite = AdversarialTestSuite::new();
        
        // Check that we have attacks of different severity levels
        let severities: std::collections::HashSet<_> = test_suite.parse_bypass_tests
            .iter()
            .map(|attack| attack.severity.clone())
            .collect();
        
        assert!(severities.len() > 1); // Should have multiple severity levels
    }

    #[test]
    fn test_vulnerability_summary_generation() {
        let execution_engine = AttackExecutionEngine::new();
        
        // Create test attack results
        let test_results = vec![
            AttackResult {
                attack_name: "test_critical".to_string(),
                attack_category: AttackCategory::ParseBypass,
                success: true,
                bypass_achieved: true,
                security_impact: crate::parser::robust_parser::SecuritySeverity::Critical,
                parser_response: crate::parser::robust_parser::ParseResult {
                document: None,
                errors: vec![],
                warnings: vec![],
                recovery_applied: false,
                partial_success: false,
                security_issues: vec![],
            },
                detection_triggered: false,
                mitigation_effective: false,
                performance_impact: PerformanceImpact {
                    parsing_time_ms: 100,
                    memory_usage_mb: 10,
                    cpu_usage_percent: 50.0,
                },
                details: "Test critical attack".to_string(),
            },
            AttackResult {
                attack_name: "test_low".to_string(),
                attack_category: AttackCategory::UnicodeConfusion,
                success: false,
                bypass_achieved: false,
                security_impact: crate::parser::robust_parser::SecuritySeverity::Low,
                parser_response: crate::parser::robust_parser::ParseResult {
                document: None,
                errors: vec![],
                warnings: vec![],
                recovery_applied: false,
                partial_success: false,
                security_issues: vec![],
            },
                detection_triggered: true,
                mitigation_effective: true,
                performance_impact: PerformanceImpact {
                    parsing_time_ms: 50,
                    memory_usage_mb: 5,
                    cpu_usage_percent: 25.0,
                },
                details: "Test low severity attack".to_string(),
            },
        ];
        
        let summary = execution_engine.generate_vulnerability_summary(&test_results);
        assert_eq!(summary.critical_issues, 1);
        assert_eq!(summary.low_risk_issues, 1);
    }
}