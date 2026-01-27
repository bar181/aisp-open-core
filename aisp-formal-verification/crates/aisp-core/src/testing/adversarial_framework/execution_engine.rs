//! Adversarial Attack Execution Engine
//!
//! Provides attack execution and evaluation logic for systematic security
//! testing of AISP parsers following SRP architecture.

use super::types::*;
use super::test_suite::AdversarialTestSuite;
use crate::parser::robust_parser::{RobustAispParser, ParseResult, SecuritySeverity};
use crate::parser::unicode_support::{UnicodeSymbolRegistry, SecurityReport};
use std::time::Instant;

/// Attack execution engine for systematic security testing
pub struct AttackExecutionEngine {
    parser: RobustAispParser,
    unicode_registry: UnicodeSymbolRegistry,
}

impl AttackExecutionEngine {
    /// Create new attack execution engine
    pub fn new() -> Self {
        Self {
            parser: RobustAispParser::new(),
            unicode_registry: UnicodeSymbolRegistry::new(),
        }
    }

    /// Run comprehensive adversarial test suite
    pub fn run_comprehensive_tests(&self, test_suite: &AdversarialTestSuite) -> SecurityAssessmentReport {
        let mut all_results = Vec::new();

        // Run all attack categories
        all_results.extend(self.run_parse_bypass_tests(test_suite));
        all_results.extend(self.run_unicode_confusion_tests(test_suite));
        all_results.extend(self.run_boundary_condition_tests(test_suite));
        all_results.extend(self.run_malformed_document_tests(test_suite));
        all_results.extend(self.run_resource_exhaustion_tests(test_suite));
        all_results.extend(self.run_injection_tests(test_suite));

        self.generate_assessment_report(all_results)
    }

    /// Run parse bypass attack tests
    fn run_parse_bypass_tests(&self, test_suite: &AdversarialTestSuite) -> Vec<AttackResult> {
        test_suite.parse_bypass_tests.iter().map(|attack| {
            let start_time = Instant::now();
            let parse_result = self.parser.parse(&attack.attack_payload);
            let parsing_time = start_time.elapsed();

            let success = self.evaluate_parse_bypass_success(&parse_result, &attack.expected_behavior);
            let bypass_achieved = parse_result.document.is_some() && parse_result.errors.is_empty();

            AttackResult {
                attack_name: attack.name.to_string(),
                attack_category: attack.attack_category.clone(),
                success,
                bypass_achieved,
                security_impact: self.map_severity_to_security(attack.severity.clone()),
                detection_triggered: !parse_result.security_issues.is_empty(),
                parser_response: parse_result,
                mitigation_effective: !success,
                performance_impact: PerformanceImpact {
                    parsing_time_ms: parsing_time.as_millis() as u64,
                    memory_usage_mb: 0,
                    cpu_usage_percent: 0.0,
                },
                details: attack.description.to_string(),
            }
        }).collect()
    }

    /// Run Unicode confusion attack tests
    fn run_unicode_confusion_tests(&self, test_suite: &AdversarialTestSuite) -> Vec<AttackResult> {
        test_suite.unicode_confusion_tests.iter().map(|attack| {
            let start_time = Instant::now();
            let parse_result = self.parser.parse(&attack.malicious_payload);
            let security_report = self.unicode_registry.analyze_security(&attack.malicious_payload);
            let parsing_time = start_time.elapsed();

            let success = self.evaluate_unicode_attack_success(&security_report, &parse_result);
            let bypass_achieved = parse_result.document.is_some() && security_report.dangerous_characters == 0;

            AttackResult {
                attack_name: attack.name.to_string(),
                attack_category: AttackCategory::UnicodeConfusion,
                success,
                bypass_achieved,
                security_impact: SecuritySeverity::High,
                parser_response: parse_result,
                detection_triggered: security_report.dangerous_characters > 0,
                mitigation_effective: security_report.dangerous_characters > 0,
                performance_impact: PerformanceImpact {
                    parsing_time_ms: parsing_time.as_millis() as u64,
                    memory_usage_mb: 0,
                    cpu_usage_percent: 0.0,
                },
                details: attack.description.to_string(),
            }
        }).collect()
    }

    /// Run boundary condition tests
    fn run_boundary_condition_tests(&self, test_suite: &AdversarialTestSuite) -> Vec<AttackResult> {
        test_suite.boundary_condition_tests.iter().map(|attack| {
            let start_time = Instant::now();
            let parse_result = self.parser.parse(&attack.attack_vector);
            let parsing_time = start_time.elapsed();

            let success = self.evaluate_boundary_attack_success(&parse_result, &attack.expected_failure_mode);
            let bypass_achieved = parse_result.document.is_some() && parse_result.errors.is_empty();

            AttackResult {
                attack_name: attack.name.to_string(),
                attack_category: AttackCategory::BoundaryExploitation,
                success,
                bypass_achieved,
                security_impact: SecuritySeverity::High,
                parser_response: parse_result,
                detection_triggered: parsing_time > std::time::Duration::from_secs(1),
                mitigation_effective: !success,
                performance_impact: PerformanceImpact {
                    parsing_time_ms: parsing_time.as_millis() as u64,
                    memory_usage_mb: 0,
                    cpu_usage_percent: 0.0,
                },
                details: attack.description.to_string(),
            }
        }).collect()
    }

    /// Run malformed document tests
    fn run_malformed_document_tests(&self, test_suite: &AdversarialTestSuite) -> Vec<AttackResult> {
        test_suite.malformed_document_tests.iter().map(|attack| {
            let start_time = Instant::now();
            let parse_result = self.parser.parse(&attack.malformed_payload);
            let parsing_time = start_time.elapsed();

            let success = self.evaluate_malformed_attack_success(&parse_result, &attack.expected_recovery_behavior);
            let bypass_achieved = parse_result.document.is_some() && !parse_result.recovery_applied;
            let detection_triggered = parse_result.recovery_applied || !parse_result.security_issues.is_empty();
            let mitigation_effective = parse_result.recovery_applied;

            AttackResult {
                attack_name: attack.name.to_string(),
                attack_category: AttackCategory::ParseBypass,
                success,
                bypass_achieved,
                security_impact: SecuritySeverity::Medium,
                parser_response: parse_result,
                detection_triggered,
                mitigation_effective,
                performance_impact: PerformanceImpact {
                    parsing_time_ms: parsing_time.as_millis() as u64,
                    memory_usage_mb: 0,
                    cpu_usage_percent: 0.0,
                },
                details: attack.description.to_string(),
            }
        }).collect()
    }

    /// Run resource exhaustion tests
    fn run_resource_exhaustion_tests(&self, test_suite: &AdversarialTestSuite) -> Vec<AttackResult> {
        test_suite.resource_exhaustion_tests.iter().map(|attack| {
            let payload = self.generate_resource_exhaustion_payload(attack);
            let start_time = Instant::now();
            let parse_result = self.parser.parse(&payload);
            let parsing_time = start_time.elapsed();

            let success = parsing_time > std::time::Duration::from_millis(attack.threshold_limits.cpu_time_ms as u64);
            let bypass_achieved = parse_result.document.is_some() && parse_result.security_issues.is_empty();
            let detection_triggered = !parse_result.security_issues.is_empty();
            let mitigation_effective = !parse_result.security_issues.is_empty();

            AttackResult {
                attack_name: attack.name.to_string(),
                attack_category: AttackCategory::ResourceExhaustion,
                success,
                bypass_achieved,
                security_impact: SecuritySeverity::High,
                parser_response: parse_result,
                detection_triggered,
                mitigation_effective,
                performance_impact: PerformanceImpact {
                    parsing_time_ms: parsing_time.as_millis() as u64,
                    memory_usage_mb: 0,
                    cpu_usage_percent: 0.0,
                },
                details: attack.description.to_string(),
            }
        }).collect()
    }

    /// Run injection tests
    fn run_injection_tests(&self, test_suite: &AdversarialTestSuite) -> Vec<AttackResult> {
        test_suite.injection_tests.iter().map(|attack| {
            let payload = format!("𝔸5.1.test@2026-01-27⟦Ω:Meta⟧{{{}}}", attack.injection_payload);
            let start_time = Instant::now();
            let parse_result = self.parser.parse(&payload);
            let parsing_time = start_time.elapsed();

            let success = parse_result.document.is_some() && parse_result.security_issues.is_empty();
            let bypass_achieved = success && parse_result.errors.is_empty();
            let detection_triggered = !parse_result.security_issues.is_empty();
            let mitigation_effective = !parse_result.security_issues.is_empty();

            AttackResult {
                attack_name: attack.name.to_string(),
                attack_category: AttackCategory::InjectionAttack,
                success,
                bypass_achieved,
                security_impact: SecuritySeverity::Medium,
                parser_response: parse_result,
                detection_triggered,
                mitigation_effective,
                performance_impact: PerformanceImpact {
                    parsing_time_ms: parsing_time.as_millis() as u64,
                    memory_usage_mb: 0,
                    cpu_usage_percent: 0.0,
                },
                details: attack.description.to_string(),
            }
        }).collect()
    }

    /// Generate resource exhaustion payload based on attack specification
    fn generate_resource_exhaustion_payload(&self, attack: &ResourceExhaustionAttack) -> String {
        match attack.payload_generator {
            PayloadGenerator::ExponentialNesting => {
                self.generate_deep_nesting(attack.threshold_limits.max_depth)
            },
            PayloadGenerator::RepeatedPattern => {
                format!("𝔸5.1.test@2026-01-27{}", "⟦Ω:Meta⟧{Vision≜\"test\"}".repeat(attack.threshold_limits.max_iterations))
            },
            PayloadGenerator::LargeInput => {
                format!("𝔸5.1.test@2026-01-27⟦Ω:Meta⟧{{Vision≜\"{}\"}}", "A".repeat(attack.threshold_limits.memory_mb * 1024))
            },
            PayloadGenerator::ComplexUnicode => {
                let complex_unicode = (0..attack.threshold_limits.max_iterations)
                    .map(|i| format!("α{}β{}γ{}", i, i*2, i*3))
                    .collect::<Vec<_>>()
                    .join("");
                format!("𝔸5.1.test@2026-01-27⟦Ω:Meta⟧{{Vision≜\"{}\"}}", complex_unicode)
            },
            PayloadGenerator::CyclicReference => {
                // Simplified cyclic pattern
                format!("𝔸5.1.test@2026-01-27⟦Γ:Rules⟧{{∀x:T:x∈T∧T⊆x}}")
            },
        }
    }

    /// Generate deeply nested structure for testing
    fn generate_deep_nesting(&self, depth: usize) -> String {
        let mut result = String::with_capacity(depth * 20);
        result.push_str("𝔸5.1.nested@2026-01-27");
        
        for i in 0..depth {
            result.push_str(&format!("⟦Ω{i}:Meta⟧{{"));
        }
        result.push_str("Vision≜\"deeply_nested\"");
        for _ in 0..depth {
            result.push('}');
        }
        
        result
    }

    /// Generate comprehensive security assessment report
    fn generate_assessment_report(&self, attack_results: Vec<AttackResult>) -> SecurityAssessmentReport {
        let total_attacks = attack_results.len();
        let successful_attacks = attack_results.iter().filter(|r| r.success).count();
        let bypasses_achieved = attack_results.iter().filter(|r| r.bypass_achieved).count();
        let critical_vulnerabilities = attack_results.iter()
            .filter(|r| r.security_impact == SecuritySeverity::Critical)
            .count();

        let vulnerability_summary = self.generate_vulnerability_summary(&attack_results);
        let recommendations = self.generate_security_recommendations(&attack_results);
        let overall_security_score = self.calculate_security_score(&attack_results);

        SecurityAssessmentReport {
            timestamp: "2026-01-27 12:00:00 UTC".to_string(), // Simplified timestamp
            total_attacks,
            successful_attacks,
            bypasses_achieved,
            critical_vulnerabilities,
            attack_results,
            vulnerability_summary,
            recommendations,
            overall_security_score,
        }
    }

    // Evaluation helper methods
    fn evaluate_parse_bypass_success(&self, result: &ParseResult, expected: &ExpectedBehavior) -> bool {
        match expected {
            ExpectedBehavior::ShouldReject => result.document.is_none(),
            ExpectedBehavior::ShouldParseWithWarnings => result.document.is_some() && !result.warnings.is_empty(),
            ExpectedBehavior::ShouldTriggerSecurity => !result.security_issues.is_empty(),
            ExpectedBehavior::ShouldBypassValidation => result.document.is_some() && result.errors.is_empty(),
        }
    }

    fn evaluate_unicode_attack_success(&self, security_report: &SecurityReport, parse_result: &ParseResult) -> bool {
        security_report.dangerous_characters > 0 || !parse_result.security_issues.is_empty()
    }

    fn evaluate_boundary_attack_success(&self, result: &ParseResult, expected_failure: &FailureMode) -> bool {
        match expected_failure {
            FailureMode::ParseError => !result.errors.is_empty(),
            FailureMode::SecurityViolation => !result.security_issues.is_empty(),
            FailureMode::MemoryExhaustion => result.document.is_none(),
            FailureMode::InfiniteLoop => false, // Would timeout
            FailureMode::IncorrectBehavior => result.document.is_some() && !result.errors.is_empty(),
        }
    }

    fn evaluate_malformed_attack_success(&self, result: &ParseResult, expected_recovery: &RecoveryBehavior) -> bool {
        match expected_recovery {
            RecoveryBehavior::GracefulDegradation => result.document.is_some() && result.recovery_applied,
            RecoveryBehavior::PartialRecovery => result.recovery_applied,
            RecoveryBehavior::FailFast => result.document.is_none(),
            RecoveryBehavior::SecurityLockdown => !result.security_issues.is_empty(),
        }
    }

    pub fn generate_vulnerability_summary(&self, results: &[AttackResult]) -> VulnerabilitySummary {
        let critical_issues = results.iter().filter(|r| r.security_impact == SecuritySeverity::Critical).count();
        let high_risk_issues = results.iter().filter(|r| r.security_impact == SecuritySeverity::High).count();
        let medium_risk_issues = results.iter().filter(|r| r.security_impact == SecuritySeverity::Medium).count();
        let low_risk_issues = results.iter().filter(|r| r.security_impact == SecuritySeverity::Low).count();

        let most_critical_attack = results.iter()
            .filter(|r| r.success && r.security_impact == SecuritySeverity::Critical)
            .map(|r| r.attack_name.clone())
            .next();

        let mut weakness_patterns = Vec::new();
        if results.iter().any(|r| r.attack_category == AttackCategory::ParseBypass && r.success) {
            weakness_patterns.push("Parse bypass vulnerabilities".to_string());
        }
        if results.iter().any(|r| r.attack_category == AttackCategory::UnicodeConfusion && r.success) {
            weakness_patterns.push("Unicode confusion attacks".to_string());
        }

        VulnerabilitySummary {
            critical_issues,
            high_risk_issues,
            medium_risk_issues,
            low_risk_issues,
            most_critical_attack,
            common_weakness_patterns: weakness_patterns,
        }
    }

    fn generate_security_recommendations(&self, results: &[AttackResult]) -> Vec<SecurityRecommendation> {
        let mut recommendations = Vec::new();

        if results.iter().any(|r| r.attack_category == AttackCategory::ParseBypass && r.success) {
            recommendations.push(SecurityRecommendation {
                priority: RecommendationPriority::Immediate,
                category: "Parse Validation".to_string(),
                description: "Strengthen input validation and parsing error handling".to_string(),
                implementation_effort: ImplementationEffort::Medium,
                risk_reduction: 0.7,
            });
        }

        if results.iter().any(|r| r.attack_category == AttackCategory::UnicodeConfusion && r.success) {
            recommendations.push(SecurityRecommendation {
                priority: RecommendationPriority::Short_term,
                category: "Unicode Security".to_string(),
                description: "Implement Unicode normalization and character validation".to_string(),
                implementation_effort: ImplementationEffort::High,
                risk_reduction: 0.8,
            });
        }

        recommendations
    }

    fn calculate_security_score(&self, results: &[AttackResult]) -> f64 {
        if results.is_empty() {
            return 1.0;
        }

        let successful_attacks = results.iter().filter(|r| r.success).count();
        let total_attacks = results.len();
        
        1.0 - (successful_attacks as f64 / total_attacks as f64)
    }

    pub fn map_severity_to_security(&self, severity: AttackSeverity) -> SecuritySeverity {
        match severity {
            AttackSeverity::Low => SecuritySeverity::Low,
            AttackSeverity::Medium => SecuritySeverity::Medium,
            AttackSeverity::High => SecuritySeverity::High,
            AttackSeverity::Critical => SecuritySeverity::Critical,
        }
    }
}

impl Default for AttackExecutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_engine_creation() {
        let engine = AttackExecutionEngine::new();
        // Test that engine can be created without panicking
        assert!(true);
    }

    #[test]
    fn test_comprehensive_test_execution() {
        let engine = AttackExecutionEngine::new();
        let test_suite = AdversarialTestSuite::new();
        
        let report = engine.run_comprehensive_tests(&test_suite);
        
        assert!(report.total_attacks > 0);
        assert_eq!(report.total_attacks, test_suite.total_test_count());
        assert!(report.overall_security_score >= 0.0);
        assert!(report.overall_security_score <= 1.0);
    }

    #[test]
    fn test_deep_nesting_generation() {
        let engine = AttackExecutionEngine::new();
        let nested = engine.generate_deep_nesting(3);
        
        assert!(nested.contains("𝔸5.1.nested@2026-01-27"));
        assert!(nested.matches("⟦Ω").count() == 3);
        assert!(nested.matches('}').count() == 3);
    }

    #[test]
    fn test_security_score_calculation() {
        let engine = AttackExecutionEngine::new();
        
        // Test with no attacks
        let empty_results: Vec<AttackResult> = vec![];
        assert_eq!(engine.calculate_security_score(&empty_results), 1.0);
        
        // Test with all successful attacks (worst case)
        let bad_results = vec![
            AttackResult {
                attack_name: "test".to_string(),
                attack_category: AttackCategory::ParseBypass,
                success: true,
                bypass_achieved: true,
                security_impact: SecuritySeverity::High,
                parser_response: ParseResult::default(),
                detection_triggered: false,
                mitigation_effective: false,
                performance_impact: PerformanceImpact {
                    parsing_time_ms: 100,
                    memory_usage_mb: 10,
                    cpu_usage_percent: 50.0,
                },
                details: "Test attack".to_string(),
            }
        ];
        assert_eq!(engine.calculate_security_score(&bad_results), 0.0);
    }

    #[test]
    fn test_attack_evaluation_methods() {
        let engine = AttackExecutionEngine::new();
        let result = ParseResult::default();
        
        // Test parse bypass evaluation
        assert!(!engine.evaluate_parse_bypass_success(&result, &ExpectedBehavior::ShouldReject));
        
        // Test boundary attack evaluation
        assert!(!engine.evaluate_boundary_attack_success(&result, &FailureMode::ParseError));
    }
}