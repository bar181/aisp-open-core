//! Core types and configuration for enhanced Z3 verification
//!
//! This module defines the fundamental types, configurations, and results
//! used throughout the Z3 verification system.

use crate::{ast::*, error::*, tri_vector_validation::*};
use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime};

/// Enhanced verification configuration
#[derive(Debug, Clone)]
pub struct AdvancedVerificationConfig {
    /// Timeout for individual queries
    pub query_timeout_ms: u64,
    /// Enable incremental solving
    pub incremental: bool,
    /// Enable proof generation
    pub generate_proofs: bool,
    /// Enable model generation
    pub generate_models: bool,
    /// Enable unsat core generation
    pub generate_unsat_cores: bool,
    /// Z3 solver tactics
    pub solver_tactics: Vec<String>,
    /// Maximum memory usage (MB)
    pub max_memory_mb: usize,
    /// Random seed for reproducibility
    pub random_seed: Option<u64>,
}

impl Default for AdvancedVerificationConfig {
    fn default() -> Self {
        Self {
            query_timeout_ms: 30000,
            incremental: true,
            generate_proofs: true,
            generate_models: true,
            generate_unsat_cores: true,
            solver_tactics: vec![
                "simplify".to_string(),
                "solve-eqs".to_string(),
                "smt".to_string(),
            ],
            max_memory_mb: 4096,
            random_seed: Some(42),
        }
    }
}

/// Enhanced verification statistics
#[derive(Debug, Clone)]
pub struct EnhancedVerificationStats {
    /// Total verification time
    pub total_time: Duration,
    /// Verification time in milliseconds
    pub verification_time_ms: u128,
    /// Number of SMT queries executed
    pub smt_queries: usize,
    /// Number of successful proofs
    pub successful_proofs: usize,
    /// Number of counterexamples found
    pub counterexamples: usize,
    /// Number of timeouts
    pub timeouts: usize,
    /// Memory usage peak (bytes)
    pub peak_memory: usize,
    /// Z3 internal statistics
    pub z3_stats: HashMap<String, String>,
}

impl Default for EnhancedVerificationStats {
    fn default() -> Self {
        Self {
            total_time: Duration::ZERO,
            verification_time_ms: 0,
            smt_queries: 0,
            successful_proofs: 0,
            counterexamples: 0,
            timeouts: 0,
            peak_memory: 0,
            z3_stats: HashMap::new(),
        }
    }
}

/// Result of enhanced Z3 verification
#[derive(Debug, Clone)]
pub struct EnhancedVerificationResult {
    /// Overall verification status
    pub status: VerificationStatus,
    /// Verified properties with detailed results
    pub verified_properties: Vec<VerifiedProperty>,
    /// Generated formal proofs
    pub proofs: HashMap<String, FormalProof>,
    /// Counterexamples for disproven properties
    pub counterexamples: HashMap<String, CounterexampleModel>,
    /// Unsat cores for unsatisfiable constraints
    pub unsat_cores: HashMap<String, UnsatCore>,
    /// Verification statistics
    pub stats: EnhancedVerificationStats,
    /// Z3 solver diagnostics
    pub diagnostics: Vec<SolverDiagnostic>,
    /// Tri-vector validation result (optional)
    pub tri_vector_result: Option<TriVectorValidationResult>,
}


/// Status of verification process
#[derive(Debug, Clone, PartialEq)]
pub enum VerificationStatus {
    /// All properties successfully verified
    AllVerified,
    /// Some properties verified, others failed
    PartiallyVerified,
    /// Verification incomplete due to timeouts/limits
    Incomplete,
    /// Verification failed due to errors
    Failed(String),
    /// Z3 verification disabled
    Disabled,
}

/// Category of AISP property
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyCategory {
    /// Tri-vector orthogonality
    TriVectorOrthogonality,
    /// Temporal logic properties (LTL/CTL)
    TemporalLogic,
    /// Temporal safety property
    TemporalSafety,
    /// Temporal liveness property
    TemporalLiveness,
    /// Type safety invariant
    TypeSafety,
    /// Semantic consistency property
    SemanticConsistency,
    /// Functional correctness
    Correctness,
    /// Resource constraints
    ResourceConstraints,
    /// Protocol compliance
    ProtocolCompliance,
    /// Structural integrity
    StructuralIntegrity,
    /// Mathematical correctness
    MathematicalCorrectness,
}

/// Result of property verification
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyResult {
    /// Property proven valid
    Proven,
    /// Property disproven with counterexample
    Disproven,
    /// Property unknown (timeout/resource limit)
    Unknown,
    /// Verification error
    Error(String),
    /// Unsupported without Z3
    Unsupported,
}

/// Verified property with detailed information
#[derive(Debug, Clone)]
pub struct VerifiedProperty {
    /// Property identifier
    pub id: String,
    /// Property category
    pub category: PropertyCategory,
    /// Property description
    pub description: String,
    /// SMT-LIB formula
    pub smt_formula: String,
    /// Verification result
    pub result: PropertyResult,
    /// Verification time
    pub verification_time: Duration,
    /// Proof certificate (if available)
    pub proof_certificate: Option<String>,
}

/// Formal proof generated by Z3
#[derive(Debug, Clone)]
pub struct FormalProof {
    /// Proof identifier
    pub id: String,
    /// Proof format (Z3, TPTP, etc.)
    pub format: String,
    /// Proof content
    pub content: String,
    /// Proof size (number of steps)
    pub size: usize,
    /// Proof dependencies
    pub dependencies: Vec<String>,
    /// Proof validation status
    pub valid: bool,
}

/// Counterexample model for disproven property
#[derive(Debug, Clone)]
pub struct CounterexampleModel {
    /// Model identifier
    pub id: String,
    /// Variable assignments
    pub assignments: HashMap<String, String>,
    /// Function interpretations
    pub function_interpretations: HashMap<String, FunctionInterpretation>,
    /// Model evaluation
    pub evaluation: String,
    /// Counterexample explanation
    pub explanation: String,
}

/// Function interpretation in counterexample
#[derive(Debug, Clone)]
pub struct FunctionInterpretation {
    /// Function name
    pub name: String,
    /// Domain types
    pub domain: Vec<String>,
    /// Codomain type
    pub codomain: String,
    /// Function mapping
    pub mapping: Vec<(Vec<String>, String)>,
    /// Default value (if partial function)
    pub default: Option<String>,
}

/// Unsat core for unsatisfiable constraints
#[derive(Debug, Clone)]
pub struct UnsatCore {
    /// Core identifier
    pub id: String,
    /// Minimal unsatisfiable subset of assertions
    pub core_assertions: Vec<String>,
    /// Explanation of unsatisfiability
    pub explanation: String,
    /// Suggestions for resolution
    pub suggestions: Vec<String>,
}

/// Solver diagnostic information
#[derive(Debug, Clone)]
pub struct SolverDiagnostic {
    /// Diagnostic level
    pub level: DiagnosticLevel,
    /// Diagnostic message
    pub message: String,
    /// Context information
    pub context: String,
    /// Timestamp
    pub timestamp: SystemTime,
}

/// Diagnostic severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticLevel {
    /// Information
    Info,
    /// Warning
    Warning,
    /// Error
    Error,
    /// Performance issue
    Performance,
}

impl EnhancedVerificationResult {
    /// Create a disabled result when Z3 is not available
    pub fn disabled() -> Self {
        Self {
            status: VerificationStatus::Disabled,
            verified_properties: vec![],
            proofs: HashMap::new(),
            counterexamples: HashMap::new(),
            unsat_cores: HashMap::new(),
            stats: EnhancedVerificationStats::default(),
            diagnostics: vec![],
            tri_vector_result: None,
        }
    }

    /// Create a failed result with error message
    pub fn failed(error: String) -> Self {
        Self {
            status: VerificationStatus::Failed(error),
            verified_properties: vec![],
            proofs: HashMap::new(),
            counterexamples: HashMap::new(),
            unsat_cores: HashMap::new(),
            stats: EnhancedVerificationStats::default(),
            diagnostics: vec![],
            tri_vector_result: None,
        }
    }
}

impl VerifiedProperty {
    /// Create a new verified property
    pub fn new(
        id: String,
        category: PropertyCategory,
        description: String,
        result: PropertyResult,
    ) -> Self {
        Self {
            id,
            category,
            description,
            smt_formula: String::new(),
            result,
            verification_time: Duration::ZERO,
            proof_certificate: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = AdvancedVerificationConfig::default();
        assert_eq!(config.query_timeout_ms, 30000);
        assert!(config.incremental);
        assert!(config.generate_proofs);
        assert_eq!(config.solver_tactics.len(), 3);
    }

    #[test]
    fn test_stats_default() {
        let stats = EnhancedVerificationStats::default();
        assert_eq!(stats.total_time, Duration::ZERO);
        assert_eq!(stats.smt_queries, 0);
        assert_eq!(stats.successful_proofs, 0);
    }

    #[test]
    fn test_verification_status() {
        assert_eq!(VerificationStatus::AllVerified, VerificationStatus::AllVerified);
        assert_ne!(VerificationStatus::AllVerified, VerificationStatus::Disabled);
    }

    #[test]
    fn test_property_categories() {
        let prop = VerifiedProperty::new(
            "test".to_string(),
            PropertyCategory::TriVectorOrthogonality,
            "Test property".to_string(),
            PropertyResult::Proven,
        );
        assert_eq!(prop.category, PropertyCategory::TriVectorOrthogonality);
        assert_eq!(prop.result, PropertyResult::Proven);
    }

    #[test]
    fn test_disabled_result() {
        let result = EnhancedVerificationResult::disabled();
        assert_eq!(result.status, VerificationStatus::Disabled);
        assert!(result.verified_properties.is_empty());
        assert!(result.proofs.is_empty());
    }

    #[test]
    fn test_failed_result() {
        let error_msg = "Test error";
        let result = EnhancedVerificationResult::failed(error_msg.to_string());
        
        match result.status {
            VerificationStatus::Failed(msg) => assert_eq!(msg, error_msg),
            _ => panic!("Expected failed status"),
        }
    }

    #[test]
    fn test_diagnostic_levels() {
        let diag = SolverDiagnostic {
            level: DiagnosticLevel::Warning,
            message: "Test warning".to_string(),
            context: "Test context".to_string(),
            timestamp: Instant::now(),
        };
        assert_eq!(diag.level, DiagnosticLevel::Warning);
    }

    #[test]
    fn test_property_result_matching() {
        let prop = VerifiedProperty::new(
            "test_prop".to_string(),
            PropertyCategory::TypeSafety,
            "Type safety test".to_string(),
            PropertyResult::Proven,
        );

        match prop.result {
            PropertyResult::Proven => assert!(true),
            _ => panic!("Expected proven result"),
        }
    }
}