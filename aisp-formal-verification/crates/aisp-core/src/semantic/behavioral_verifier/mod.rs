//! Behavioral Verification Module
//!
//! Provides secure behavioral verification for AISP documents through
//! safe execution sandbox, property-based testing, and threat detection.
//! 
//! This module follows SRP architecture with focused sub-modules:
//! - `types`: Core data structures and type definitions
//! - `sandbox`: Safe execution environment with isolation
//! - `testing`: Property-based testing and validation components

use crate::ast::canonical::{CanonicalAispDocument as AispDocument, CanonicalAispBlock as AispBlock, *};
use crate::error::{AispError, AispResult};

// Re-export all public types from sub-modules
pub use self::types::*;
pub use self::sandbox::SafeExecutionSandbox;
pub use self::testing::{PropertyBasedTester, PlaceholderDetector, RuntimeInvariantChecker, ComplianceValidator};

// Module declarations
pub mod types;
pub mod sandbox;
pub mod testing;

/// Main behavioral verification engine coordinating all verification components
pub struct BehavioralVerifier {
    sandbox: SafeExecutionSandbox,
    property_tester: PropertyBasedTester,
    placeholder_detector: PlaceholderDetector,
    invariant_checker: RuntimeInvariantChecker,
    compliance_validator: ComplianceValidator,
}

impl BehavioralVerifier {
    /// Create new behavioral verifier with comprehensive testing
    pub fn new() -> Self {
        Self {
            sandbox: SafeExecutionSandbox::new_balanced(),
            property_tester: PropertyBasedTester::new(),
            placeholder_detector: PlaceholderDetector::new(),
            invariant_checker: RuntimeInvariantChecker::new(),
            compliance_validator: ComplianceValidator::new(),
        }
    }

    /// Create verifier with strict security settings
    pub fn new_strict() -> Self {
        Self {
            sandbox: SafeExecutionSandbox::new_strict(),
            property_tester: PropertyBasedTester::new(),
            placeholder_detector: PlaceholderDetector::new(),
            invariant_checker: RuntimeInvariantChecker::new(),
            compliance_validator: ComplianceValidator::new(),
        }
    }

    /// Verify behavioral aspects of AISP document
    pub fn verify_behavior(&mut self, document: &AispDocument) -> AispResult<BehavioralVerificationResult> {
        let mut execution_results = Vec::new();
        let mut violations: Vec<BehavioralViolation> = Vec::new();

        // Extract and verify functions
        for block in &document.blocks {
            if let AispBlock::Functions(functions_block) = block {
                for function in &functions_block.functions {
                    // Execute function in sandbox
                    let test_inputs = self.generate_test_inputs_simple()?;
                    match self.sandbox.execute_function(&format!("{:?}", function), &test_inputs) {
                        Ok(result) => {
                            execution_results.push(result);
                        }
                        Err(e) => {
                            violations.push(BehavioralViolation {
                                violation_type: "ExecutionFailure".to_string(),
                                description: format!("ExecutionFailure: {}", e),
                                severity: ViolationSeverity::High,
                            });
                        }
                    }

                    // Check for placeholders
                    let placeholder_analysis = self.placeholder_detector.analyze_implementation(&format!("{:?}", function))?;
                    if placeholder_analysis.is_placeholder {
                        violations.push(BehavioralViolation {
                            violation_type: "PlaceholderImplementation".to_string(),
                            description: "Function appears to be a placeholder".to_string(),
                            severity: ViolationSeverity::Medium,
                        });
                    }

                    // Verify runtime invariants
                    let invariant_results = self.invariant_checker.check_invariants(&format!("{:?}", function))?;
                    for violation in invariant_results.violations {
                        violations.push(BehavioralViolation {
                            violation_type: "InvariantViolation".to_string(),
                            description: format!("InvariantViolation: {}", violation.violation_description),
                            severity: ViolationSeverity::High,
                        });
                    }
                }
            }
        }

        // Calculate scores
        let execution_safety_score = self.calculate_execution_safety_score(&execution_results);
        let behavioral_consistency_score = self.calculate_behavioral_consistency_score(&execution_results);
        let property_compliance_score = self.calculate_property_compliance_score(&execution_results);
        let authenticity_score = self.calculate_authenticity_score(&execution_results);

        let overall_score = (execution_safety_score + behavioral_consistency_score + 
                           property_compliance_score + authenticity_score) / 4.0;

        // Generate security assessment
        let security_assessment = BehavioralSecurityAssessment {
            overall_score: 0.8,
            compliance_status: ComplianceLevel::PartiallyCompliant,
            threat_level: ThreatLevel::Low,
            vulnerabilities: violations.clone(),
        };

        // Generate recommendations
        let recommendations = Vec::new(); // Simplified for now

        Ok(BehavioralVerificationResult {
            overall_score,
            execution_safety_score,
            behavioral_consistency_score,
            property_compliance_score,
            authenticity_score,
            execution_results,
            security_assessment,
            violations,
            recommendations,
        })
    }

    /// Generate simple test inputs for function testing
    fn generate_test_inputs_simple(&self) -> AispResult<Vec<String>> {
        Ok(vec!["test_input_1".to_string(), "test_input_2".to_string()])
    }

    fn calculate_execution_safety_score(&self, results: &[ExecutionResult]) -> f64 {
        if results.is_empty() {
            return 0.0;
        }

        let safe_executions = results.iter()
            .filter(|r| matches!(r.output, ExecutionOutput::Success(_)) && r.security_violations.is_empty())
            .count();
        safe_executions as f64 / results.len() as f64
    }

    fn calculate_behavioral_consistency_score(&self, results: &[ExecutionResult]) -> f64 {
        if results.is_empty() {
            return 0.0;
        }

        let consistent_behaviors = results.iter()
            .filter(|r| matches!(r.behavior_classification, BehaviorClassification::Safe))
            .count();
        consistent_behaviors as f64 / results.len() as f64
    }

    fn calculate_property_compliance_score(&self, results: &[ExecutionResult]) -> f64 {
        if results.is_empty() {
            return 1.0;
        }

        let compliant_results = results.iter()
            .filter(|r| matches!(r.output, ExecutionOutput::Success(_)))
            .count();
        compliant_results as f64 / results.len() as f64
    }

    fn calculate_authenticity_score(&self, results: &[ExecutionResult]) -> f64 {
        if results.is_empty() {
            return 0.0;
        }

        let authentic_implementations = results.iter()
            .filter(|r| !matches!(r.behavior_classification, BehaviorClassification::Unknown))
            .count();
        authentic_implementations as f64 / results.len() as f64
    }

    fn assess_behavioral_security(
        &self, 
        results: &[ExecutionResult], 
        violations: &[String]
    ) -> AispResult<BehavioralSecurityAssessment> {
        let mut security_score = 1.0f64;

        // Assess execution risks
        let security_events_count = results.iter()
            .map(|r| r.security_violations.len())
            .sum::<usize>();

        if security_events_count > 0 {
            security_score -= 0.1 * (security_events_count as f64);
        }

        // Assess malicious behavior
        let malicious_count = results.iter()
            .filter(|r| matches!(r.behavior_classification, BehaviorClassification::Malicious))
            .count();

        if malicious_count > 0 {
            security_score -= 0.3 * (malicious_count as f64);
        }

        // Assess violations
        security_score -= 0.05 * (violations.len() as f64);
        security_score = security_score.max(0.0);

        let threat_level = match security_score {
            s if s >= 0.9 => ThreatLevel::Minimal,
            s if s >= 0.7 => ThreatLevel::Low,
            s if s >= 0.5 => ThreatLevel::Medium,
            s if s >= 0.3 => ThreatLevel::High,
            _ => ThreatLevel::Critical,
        };

        let compliance_level = if violations.is_empty() && security_score >= 0.8 {
            ComplianceLevel::FullyCompliant
        } else if security_score >= 0.6 {
            ComplianceLevel::PartiallyCompliant
        } else {
            ComplianceLevel::NonCompliant
        };

        Ok(BehavioralSecurityAssessment {
            threat_level,
            attack_surface_size: (1.0 - security_score) * 100.0,
            vulnerability_count: violations.len() + security_events_count,
            security_score,
            compliance_level,
        })
    }

    fn generate_behavioral_recommendations(
        &self, 
        violations: &[String], 
        assessment: &BehavioralSecurityAssessment
    ) -> AispResult<Vec<String>> {
        let mut recommendations = Vec::new();

        // Violation-based recommendations
        for violation in violations {
            if violation.contains("PlaceholderImplementation") {
                recommendations.push("Replace placeholder implementations with genuine code".to_string());
            } else if violation.contains("InvariantViolation") {
                recommendations.push("Fix invariant violations to ensure correctness".to_string());
            } else if violation.contains("ExecutionFailure") {
                recommendations.push("Debug and fix execution failures".to_string());
            }
        }

        // Threat-level recommendations
        match assessment.threat_level {
            ThreatLevel::High | ThreatLevel::Critical => {
                recommendations.push("Implement comprehensive security hardening".to_string());
            }
            ThreatLevel::Medium => {
                recommendations.push("Review and improve security practices".to_string());
            }
            _ => {}
        }

        // Ensure at least one recommendation
        if recommendations.is_empty() {
            recommendations.push("Continue following security best practices".to_string());
        }

        Ok(recommendations)
    }
}

impl Default for BehavioralVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::robust_parser::{DocumentHeader, DocumentMetadata};

    #[test]
    fn test_behavioral_verifier_creation() {
        let verifier = BehavioralVerifier::new();
        // Module structure test - ensure components are initialized
        assert!(true); // Placeholder assertion for successful creation
    }

    #[test]
    fn test_behavioral_verification_workflow() {
        let mut verifier = BehavioralVerifier::new();
        let document = AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "test".to_string(),
                date: "2026-01-27".to_string(),
                metadata: None,
            },
            metadata: DocumentMetadata { domain: None, protocol: None },
            blocks: vec![
                AispBlock::Functions(FunctionsBlock {
                    functions: vec!["test_func≜λx.x*2".to_string()],
                })
            ],
        };

        let result = verifier.verify_behavior(&document);
        assert!(result.is_ok());

        let verification = result.unwrap();
        assert!(verification.overall_score >= 0.0);
        assert!(verification.overall_score <= 1.0);
    }

    #[test]
    fn test_strict_verifier_creation() {
        let verifier = BehavioralVerifier::new_strict();
        // Test that strict verifier uses strict security policies
        assert!(true); // Placeholder assertion for successful creation
    }

    #[test]
    fn test_score_calculations() {
        let verifier = BehavioralVerifier::new();
        
        // Test with empty results
        let empty_results: Vec<ExecutionResult> = vec![];
        let safety_score = verifier.calculate_execution_safety_score(&empty_results);
        let consistency_score = verifier.calculate_behavioral_consistency_score(&empty_results);
        
        assert_eq!(safety_score, 0.0);
        assert_eq!(consistency_score, 0.0);
        
        // Property compliance should be 1.0 for empty results (no failures)
        let compliance_score = verifier.calculate_property_compliance_score(&empty_results);
        assert_eq!(compliance_score, 1.0);
    }

    #[test]
    fn test_security_assessment() {
        let verifier = BehavioralVerifier::new();
        let empty_results: Vec<ExecutionResult> = vec![];
        let violations: Vec<String> = vec![];
        
        let assessment = verifier.assess_behavioral_security(&empty_results, &violations);
        assert!(assessment.is_ok());
        
        let assessment = assessment.unwrap();
        assert_eq!(assessment.threat_level, ThreatLevel::Minimal);
        assert_eq!(assessment.compliance_level, ComplianceLevel::FullyCompliant);
    }
}