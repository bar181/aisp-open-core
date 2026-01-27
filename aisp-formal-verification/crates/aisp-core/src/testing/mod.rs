// Testing module for AISP security hardening
// Includes adversarial testing framework and security validation tests

pub mod adversarial_framework;
pub mod security_validation_tests;

pub use adversarial_framework::{
    AdversarialTestSuite,
    SecurityAssessmentReport,
    AttackResult,
    AttackCategory,
    SecurityRecommendation,
};

pub use security_validation_tests::{
    ParserSecurityTestSuite,
    SecurityTestResults,
    SecurityComplianceReport,
    ComplianceStatus,
};