// Parser Security Validation Tests
// Comprehensive security testing for the Pest-based AISP parser
// Implements security validation requirements from ADR-022 and ADR-024

use crate::parser::robust_parser::RobustAispParser;
use crate::parser::unicode_support::UnicodeSymbolRegistry;
use crate::testing::adversarial_framework::{AdversarialTestSuite, AttackCategory, SecurityAssessmentReport};

/// Security validation test suite for parser hardening
pub struct ParserSecurityTestSuite {
    parser: RobustAispParser,
    unicode_registry: UnicodeSymbolRegistry,
    adversarial_suite: AdversarialTestSuite,
}

/// Security test results and metrics
#[derive(Debug, Clone)]
pub struct SecurityTestResults {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub critical_failures: usize,
    pub bypass_attempts: usize,
    pub successful_bypasses: usize,
    pub security_score: f64,
    pub recommendations: Vec<String>,
}

impl ParserSecurityTestSuite {
    /// Create new security test suite with strict security validation
    pub fn new() -> Self {
        Self {
            parser: RobustAispParser::new().with_security_validation(true),
            unicode_registry: UnicodeSymbolRegistry::new(),
            adversarial_suite: AdversarialTestSuite::new(),
        }
    }

    /// Create security test suite with strict parsing mode
    pub fn strict() -> Self {
        Self {
            parser: RobustAispParser::strict(),
            unicode_registry: UnicodeSymbolRegistry::new(),
            adversarial_suite: AdversarialTestSuite::new(),
        }
    }

    /// Run comprehensive parser security validation
    pub fn run_comprehensive_security_tests(&self) -> SecurityTestResults {
        let mut total_tests = 0;
        let mut passed_tests = 0;
        let mut failed_tests = 0;
        let mut critical_failures = 0;
        let mut bypass_attempts = 0;
        let mut successful_bypasses = 0;
        let mut recommendations = Vec::new();

        // Run basic security tests
        let basic_results = self.run_basic_security_tests();
        total_tests += basic_results.len();
        passed_tests += basic_results.iter().filter(|r| *r).count();
        failed_tests += basic_results.iter().filter(|r| !*r).count();

        // Run Unicode security tests
        let unicode_results = self.run_unicode_security_tests();
        total_tests += unicode_results.len();
        passed_tests += unicode_results.iter().filter(|r| *r).count();
        failed_tests += unicode_results.iter().filter(|r| !*r).count();

        // Run adversarial testing
        let adversarial_report = self.adversarial_suite.run_security_assessment(&self.parser);
        total_tests += adversarial_report.total_attacks;
        bypass_attempts = adversarial_report.total_attacks;
        successful_bypasses = adversarial_report.bypasses_achieved;
        critical_failures = adversarial_report.critical_vulnerabilities;

        // Calculate security score
        let security_score = self.calculate_security_score(
            total_tests,
            failed_tests,
            critical_failures,
            successful_bypasses,
        );

        // Generate recommendations
        recommendations.extend(self.generate_security_recommendations(
            &basic_results,
            &unicode_results,
            &adversarial_report,
        ));

        SecurityTestResults {
            total_tests,
            passed_tests,
            failed_tests,
            critical_failures,
            bypass_attempts,
            successful_bypasses,
            security_score,
            recommendations,
        }
    }

    /// Run basic parser security tests
    fn run_basic_security_tests(&self) -> Vec<bool> {
        let mut results = Vec::new();

        // Test 1: Empty input handling
        let result = self.parser.parse("");
        results.push(result.document.is_none() && !result.errors.is_empty());

        // Test 2: Null byte injection resistance
        let null_payload = "𝔸5.1.test@2026-01-27\0malicious";
        let result = self.parser.parse(null_payload);
        results.push(!result.security_issues.is_empty() || result.document.is_none());

        // Test 3: Oversized input handling
        let large_payload = "a".repeat(2_000_000);
        let result = self.parser.parse(&large_payload);
        results.push(!result.security_issues.is_empty());

        // Test 4: Malformed header rejection
        let malformed_header = "INVALID_HEADER⟦Ω:Meta⟧{Vision≜\"test\"}";
        let result = self.parser.parse(malformed_header);
        results.push(result.document.is_none());

        // Test 5: Unbalanced delimiters handling
        let unbalanced = "𝔸5.1.test@2026-01-27⟦Ω:Meta⟧{{{Vision≜\"test\"";
        let result = self.parser.parse(unbalanced);
        results.push(!result.errors.is_empty() || result.recovery_applied);

        // Test 6: Deep nesting resistance
        let deep_nested = self.generate_deeply_nested_payload(100);
        let result = self.parser.parse(&deep_nested);
        results.push(!result.security_issues.is_empty() || result.document.is_none());

        // Test 7: Unicode normalization handling
        let unicode_attack = "𝔸5.1.te\u{200B}st@2026-01-27⟦Ω:Meta⟧{Vision≜\"test\"}";
        let result = self.parser.parse(unicode_attack);
        results.push(!result.security_issues.is_empty());

        // Test 8: Invalid UTF-8 sequence handling
        let invalid_utf8 = "𝔸5.1.test@2026-01-27\u{00FF}\u{00FE}⟦Ω:Meta⟧";
        let result = self.parser.parse(invalid_utf8);
        results.push(result.document.is_none() || !result.security_issues.is_empty());

        results
    }

    /// Run Unicode-specific security tests
    fn run_unicode_security_tests(&self) -> Vec<bool> {
        let mut results = Vec::new();

        // Test Unicode symbol validation
        let test_cases = [
            ('∀', true),  // Valid universal quantifier
            ('А', false), // Cyrillic A (should be rejected)
            ('Α', false), // Greek capital alpha (should be restricted)
            ('\u{200B}', false), // Zero-width space (should be dangerous)
            ('⟦', true),  // Valid block delimiter
            ('\u{202E}', false), // Right-to-left override (dangerous)
        ];

        for (symbol, should_be_safe) in test_cases {
            let validation = self.unicode_registry.validate_symbol(symbol);
            let is_safe = validation.is_valid && validation.security_warnings.is_empty();
            results.push(is_safe == should_be_safe);
        }

        // Test string security analysis
        let test_strings = [
            ("∀x∈ℕ", true),  // Safe mathematical notation
            ("Аlpha", false), // Contains Cyrillic A
            ("test\u{200B}text", false), // Contains zero-width space
            ("mal\u{202E}icious", false), // Contains RTL override
        ];

        for (test_string, should_be_safe) in test_strings {
            let report = self.unicode_registry.generate_security_report(test_string);
            let is_safe = report.dangerous_characters == 0;
            results.push(is_safe == should_be_safe);
        }

        results
    }

    /// Generate deeply nested payload for testing
    fn generate_deeply_nested_payload(&self, depth: usize) -> String {
        let mut payload = String::from("𝔸5.1.test@2026-01-27");
        
        for i in 0..depth {
            payload.push_str(&format!("⟦Ω{}:Meta⟧{{", i));
        }
        
        payload.push_str("Vision≜\"deep\"");
        
        for _ in 0..depth {
            payload.push('}');
        }
        
        payload
    }

    /// Calculate overall security score
    fn calculate_security_score(
        &self,
        total_tests: usize,
        failed_tests: usize,
        critical_failures: usize,
        successful_bypasses: usize,
    ) -> f64 {
        let base_score = ((total_tests - failed_tests) as f64 / total_tests as f64) * 100.0;
        let critical_penalty = critical_failures as f64 * 15.0;
        let bypass_penalty = successful_bypasses as f64 * 25.0;
        
        (base_score - critical_penalty - bypass_penalty).max(0.0)
    }

    /// Generate security recommendations based on test results
    fn generate_security_recommendations(
        &self,
        basic_results: &[bool],
        unicode_results: &[bool],
        adversarial_report: &SecurityAssessmentReport,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Analyze basic security test failures
        let basic_failures = basic_results.iter().filter(|&&r| !r).count();
        if basic_failures > 2 {
            recommendations.push(
                "Critical: Implement comprehensive input validation and error handling".to_string()
            );
        }

        // Analyze Unicode security failures
        let unicode_failures = unicode_results.iter().filter(|&&r| !r).count();
        if unicode_failures > 1 {
            recommendations.push(
                "High: Deploy Unicode normalization and character validation".to_string()
            );
        }

        // Analyze adversarial test results
        if adversarial_report.bypasses_achieved > 0 {
            recommendations.push(
                "Critical: Fix verification bypass vulnerabilities immediately".to_string()
            );
        }

        if adversarial_report.critical_vulnerabilities > 0 {
            recommendations.push(
                "Critical: Address critical security vulnerabilities".to_string()
            );
        }

        if adversarial_report.overall_security_score < 70.0 {
            recommendations.push(
                "Medium: Improve overall security posture through systematic hardening".to_string()
            );
        }

        // Add specific recommendations from adversarial report
        recommendations.extend(
            adversarial_report.recommendations
                .iter()
                .take(3)
                .map(|rec| format!("{}: {}", rec.priority, rec.description))
                .collect::<Vec<_>>()
        );

        recommendations
    }

    /// Run targeted attack simulation
    pub fn run_targeted_attack_simulation(&self, attack_category: AttackCategory) -> Vec<bool> {
        match attack_category {
            AttackCategory::ParseBypass => self.simulate_parse_bypass_attacks(),
            AttackCategory::UnicodeConfusion => self.simulate_unicode_confusion_attacks(),
            AttackCategory::BoundaryExploitation => self.simulate_boundary_attacks(),
            AttackCategory::ResourceExhaustion => self.simulate_resource_exhaustion_attacks(),
            AttackCategory::InjectionAttack => self.simulate_injection_attacks(),
            AttackCategory::DeceptionAttack => self.simulate_deception_attacks(),
        }
    }

    /// Simulate parse bypass attacks
    fn simulate_parse_bypass_attacks(&self) -> Vec<bool> {
        let attack_payloads = vec![
            "",  // Empty document
            "𝔸",  // Incomplete header
            "𝔸5.1.",  // Truncated header
            "NOT_AISP_DOCUMENT",  // Invalid format
            "𝔸5.1.test@INVALID_DATE",  // Invalid date
        ];

        attack_payloads.iter().map(|payload| {
            let result = self.parser.parse(payload);
            // Attack should be blocked
            result.document.is_none()
        }).collect()
    }

    /// Simulate Unicode confusion attacks
    fn simulate_unicode_confusion_attacks(&self) -> Vec<bool> {
        let attack_payloads = vec![
            "𝔸5.1.tеst@2026-01-27",  // Contains Cyrillic е
            "𝔸5.1.test@2026-01-27⟦Ω:MetΑ⟧",  // Contains Greek Α
            "𝔸5.1.test@2026-01-27⟦Ω\u{200B}:Meta⟧",  // Zero-width space
            "𝔸5.1.test@2026-01-27⟦\u{202E}Ω:Meta⟧",  // RTL override
        ];

        attack_payloads.iter().map(|payload| {
            let result = self.parser.parse(payload);
            // Attack should be detected
            !result.security_issues.is_empty() || result.document.is_none()
        }).collect()
    }

    /// Simulate boundary condition attacks
    fn simulate_boundary_attacks(&self) -> Vec<bool> {
        let attack_payloads = vec![
            self.generate_deeply_nested_payload(200),  // Deep nesting
            format!("𝔸5.1.test@2026-01-27⟦Ω:Meta⟧{{Vision≜\"{}\"}}", "A".repeat(100_000)),  // Large string
            "⟦".repeat(1000),  // Many unclosed delimiters
        ];

        attack_payloads.iter().map(|payload| {
            let result = self.parser.parse(payload);
            // Should trigger security measures
            !result.security_issues.is_empty() || result.document.is_none()
        }).collect()
    }

    /// Simulate resource exhaustion attacks
    fn simulate_resource_exhaustion_attacks(&self) -> Vec<bool> {
        let attack_payloads = vec![
            "A".repeat(5_000_000),  // Massive input
            self.generate_deeply_nested_payload(500),  // Extremely deep nesting
        ];

        attack_payloads.iter().map(|payload| {
            let start_time = std::time::Instant::now();
            let result = self.parser.parse(payload);
            let parse_time = start_time.elapsed();
            
            // Should be rejected or complete quickly
            result.document.is_none() || 
            !result.security_issues.is_empty() || 
            parse_time < std::time::Duration::from_secs(5)
        }).collect()
    }

    /// Simulate injection attacks
    fn simulate_injection_attacks(&self) -> Vec<bool> {
        let injection_payloads = vec![
            "Vision≜\"<script>alert('xss')</script>\"",
            "Vision≜\"'; DROP TABLE docs; --\"",
            "Vision≜\"$(rm -rf /)\"",
            "Vision≜\"\\\"; system('malicious'); \\\"\"",
        ];

        injection_payloads.iter().map(|injection| {
            let payload = format!("𝔸5.1.test@2026-01-27⟦Ω:Meta⟧{{{}}}", injection);
            let result = self.parser.parse(&payload);
            
            // Should parse but with proper escaping/sanitization
            result.document.is_some() && (
                !result.security_issues.is_empty() || 
                result.warnings.iter().any(|w| w.message.contains("injection"))
            )
        }).collect()
    }

    /// Simulate deception attacks
    fn simulate_deception_attacks(&self) -> Vec<bool> {
        let deception_payloads = vec![
            // Empty or minimal implementations that might bypass verification
            "𝔸5.1.minimal@2026-01-27⟦Ω:Meta⟧{Vision≜\"placeholder\"}",
            "𝔸5.1.fake@2026-01-27⟦Λ:Functions⟧{f≜λx.x}",
            "𝔸5.1.trivial@2026-01-27⟦Γ:Rules⟧{∀x:⊤}",
        ];

        deception_payloads.iter().map(|payload| {
            let result = self.parser.parse(payload);
            
            // Should parse successfully but potentially flag as suspicious
            result.document.is_some()
        }).collect()
    }

    /// Generate security compliance report
    pub fn generate_compliance_report(&self) -> SecurityComplianceReport {
        let test_results = self.run_comprehensive_security_tests();
        let adversarial_report = self.adversarial_suite.run_security_assessment(&self.parser);
        
        SecurityComplianceReport {
            timestamp: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            overall_compliance_score: test_results.security_score,
            parser_security_score: (test_results.passed_tests as f64 / test_results.total_tests as f64) * 100.0,
            adversarial_resistance_score: adversarial_report.overall_security_score,
            critical_issues_count: test_results.critical_failures,
            bypass_resistance: if test_results.bypass_attempts > 0 {
                ((test_results.bypass_attempts - test_results.successful_bypasses) as f64 / test_results.bypass_attempts as f64) * 100.0
            } else {
                100.0
            },
            recommendations: test_results.recommendations,
            compliance_status: if test_results.security_score >= 85.0 {
                ComplianceStatus::Compliant
            } else if test_results.security_score >= 70.0 {
                ComplianceStatus::PartiallyCompliant
            } else {
                ComplianceStatus::NonCompliant
            },
        }
    }
}

/// Security compliance report
#[derive(Debug, Clone)]
pub struct SecurityComplianceReport {
    pub timestamp: String,
    pub overall_compliance_score: f64,
    pub parser_security_score: f64,
    pub adversarial_resistance_score: f64,
    pub critical_issues_count: usize,
    pub bypass_resistance: f64,
    pub recommendations: Vec<String>,
    pub compliance_status: ComplianceStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComplianceStatus {
    Compliant,
    PartiallyCompliant,
    NonCompliant,
}

impl Default for ParserSecurityTestSuite {
    fn default() -> Self {
        Self::new()
    }
}

// Add chrono for timestamp (conditional compilation)
#[cfg(not(feature = "chrono"))]
mod chrono {
    pub struct Utc;
    impl Utc {
        pub fn now() -> DateTime {
            DateTime
        }
    }
    
    pub struct DateTime;
    impl DateTime {
        pub fn format(&self, _fmt: &str) -> impl std::fmt::Display {
            "2026-01-27 00:00:00 UTC"
        }
    }
}

#[cfg(feature = "chrono")]
use chrono;

impl std::fmt::Display for SecurityTestResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser Security Test Results\n")?;
        write!(f, "============================\n")?;
        write!(f, "Total Tests: {}\n", self.total_tests)?;
        write!(f, "Passed: {} ({:.1}%)\n", self.passed_tests, 
               (self.passed_tests as f64 / self.total_tests as f64) * 100.0)?;
        write!(f, "Failed: {} ({:.1}%)\n", self.failed_tests,
               (self.failed_tests as f64 / self.total_tests as f64) * 100.0)?;
        write!(f, "Critical Failures: {}\n", self.critical_failures)?;
        write!(f, "Bypass Attempts: {}\n", self.bypass_attempts)?;
        write!(f, "Successful Bypasses: {} ({:.1}%)\n", self.successful_bypasses,
               if self.bypass_attempts > 0 {
                   (self.successful_bypasses as f64 / self.bypass_attempts as f64) * 100.0
               } else {
                   0.0
               })?;
        write!(f, "Security Score: {:.1}/100\n", self.security_score)?;
        
        if !self.recommendations.is_empty() {
            write!(f, "\nTop Recommendations:\n")?;
            for (i, rec) in self.recommendations.iter().take(5).enumerate() {
                write!(f, "{}. {}\n", i + 1, rec)?;
            }
        }
        
        Ok(())
    }
}

impl std::fmt::Display for SecurityComplianceReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AISP Parser Security Compliance Report\n")?;
        write!(f, "=====================================\n")?;
        write!(f, "Timestamp: {}\n", self.timestamp)?;
        write!(f, "Compliance Status: {:?}\n", self.compliance_status)?;
        write!(f, "Overall Score: {:.1}/100\n", self.overall_compliance_score)?;
        write!(f, "Parser Security: {:.1}/100\n", self.parser_security_score)?;
        write!(f, "Adversarial Resistance: {:.1}/100\n", self.adversarial_resistance_score)?;
        write!(f, "Bypass Resistance: {:.1}%\n", self.bypass_resistance)?;
        write!(f, "Critical Issues: {}\n", self.critical_issues_count)?;
        
        if !self.recommendations.is_empty() {
            write!(f, "\nPriority Recommendations:\n")?;
            for (i, rec) in self.recommendations.iter().take(3).enumerate() {
                write!(f, "{}. {}\n", i + 1, rec)?;
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_test_suite_creation() {
        let suite = ParserSecurityTestSuite::new();
        // Should create successfully with security validation enabled
        assert!(suite.parser.security_validation);
    }

    #[test]
    fn test_basic_security_tests() {
        let suite = ParserSecurityTestSuite::new();
        let results = suite.run_basic_security_tests();
        
        // Should have multiple security tests
        assert!(results.len() >= 5);
        
        // At least some tests should pass (indicating proper security behavior)
        let pass_rate = results.iter().filter(|&&r| r).count() as f64 / results.len() as f64;
        assert!(pass_rate > 0.5, "Security test pass rate too low: {:.1}%", pass_rate * 100.0);
    }

    #[test]
    fn test_unicode_security_tests() {
        let suite = ParserSecurityTestSuite::new();
        let results = suite.run_unicode_security_tests();
        
        // Should have Unicode security tests
        assert!(!results.is_empty());
        
        // Most Unicode security tests should pass
        let pass_rate = results.iter().filter(|&&r| r).count() as f64 / results.len() as f64;
        assert!(pass_rate >= 0.7, "Unicode security test pass rate too low: {:.1}%", pass_rate * 100.0);
    }

    #[test]
    fn test_comprehensive_security_assessment() {
        let suite = ParserSecurityTestSuite::new();
        let results = suite.run_comprehensive_security_tests();
        
        // Should run multiple categories of tests
        assert!(results.total_tests > 10);
        
        // Should have a measurable security score
        assert!(results.security_score >= 0.0);
        assert!(results.security_score <= 100.0);
        
        // Should provide recommendations if there are issues
        if results.failed_tests > 0 || results.critical_failures > 0 {
            assert!(!results.recommendations.is_empty());
        }
    }

    #[test]
    fn test_targeted_attack_simulation() {
        let suite = ParserSecurityTestSuite::new();
        
        // Test parse bypass simulation
        let bypass_results = suite.run_targeted_attack_simulation(AttackCategory::ParseBypass);
        assert!(!bypass_results.is_empty());
        
        // Test Unicode confusion simulation
        let unicode_results = suite.run_targeted_attack_simulation(AttackCategory::UnicodeConfusion);
        assert!(!unicode_results.is_empty());
        
        // Most attacks should be successfully blocked
        let total_blocked = bypass_results.iter().chain(unicode_results.iter()).filter(|&&r| r).count();
        let total_attacks = bypass_results.len() + unicode_results.len();
        let block_rate = total_blocked as f64 / total_attacks as f64;
        assert!(block_rate > 0.6, "Attack blocking rate too low: {:.1}%", block_rate * 100.0);
    }

    #[test]
    fn test_compliance_report_generation() {
        let suite = ParserSecurityTestSuite::new();
        let report = suite.generate_compliance_report();
        
        // Should generate a complete compliance report
        assert!(!report.timestamp.is_empty());
        assert!(report.overall_compliance_score >= 0.0);
        assert!(report.overall_compliance_score <= 100.0);
        assert!(report.parser_security_score >= 0.0);
        assert!(report.parser_security_score <= 100.0);
        
        // Compliance status should be determined correctly
        match report.overall_compliance_score {
            score if score >= 85.0 => assert_eq!(report.compliance_status, ComplianceStatus::Compliant),
            score if score >= 70.0 => assert_eq!(report.compliance_status, ComplianceStatus::PartiallyCompliant),
            _ => assert_eq!(report.compliance_status, ComplianceStatus::NonCompliant),
        }
    }
}