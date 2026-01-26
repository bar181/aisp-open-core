//! Invariant Discovery Types and Structures
//!
//! This module defines the core types and structures used throughout
//! the invariant discovery system.

use crate::{
    property_types::{PropertyFormula, SourceLocation},
};
use std::time::Duration;

/// Discovered invariant with confidence scoring
#[derive(Debug, Clone)]
pub struct DiscoveredInvariant {
    /// Unique identifier for the invariant
    pub id: String,
    /// Human-readable name of the invariant
    pub name: String,
    /// Mathematical formula representation
    pub formula: PropertyFormula,
    /// Invariant classification
    pub invariant_type: InvariantType,
    /// Confidence score [0.0, 1.0]
    pub confidence: f64,
    /// Evidence supporting this invariant
    pub evidence: Vec<InvariantEvidence>,
    /// Source locations where this invariant was discovered
    pub sources: Vec<SourceLocation>,
    /// Whether this invariant has been verified
    pub verified: bool,
}

/// Classification of invariant types
#[derive(Debug, Clone, PartialEq)]
pub enum InvariantType {
    /// Type system invariants (e.g., ∀x:ℕ → x ≥ 0)
    TypeStructural,
    /// Type membership constraints
    TypeMembership,
    /// Functional invariants (e.g., f(x) = f(y) → x = y)
    FunctionalProperty,
    /// Function monotonicity properties
    FunctionalMonotonicity,
    /// Relational invariants (e.g., R(x,y) ∧ R(y,z) → R(x,z))
    RelationalInvariant,
    /// Numerical constraints (e.g., x + y = z → x ≤ z)
    NumericalInvariant,
    /// Logical consistency (e.g., P ∧ ¬P → ⊥)
    LogicalInvariant,
    /// Temporal properties (e.g., □(P → ◇Q))
    TemporalInvariant,
    /// Structural invariants (e.g., |S| ≥ 0)
    StructuralInvariant,
}

/// Evidence supporting an invariant
#[derive(Debug, Clone)]
pub struct InvariantEvidence {
    /// Type of evidence
    pub evidence_type: EvidenceType,
    /// Strength of evidence [0.0, 1.0]
    pub strength: f64,
    /// Human-readable description
    pub description: String,
    /// Source location of evidence
    pub location: SourceLocation,
}

/// Types of evidence that can support invariants
#[derive(Debug, Clone, PartialEq)]
pub enum EvidenceType {
    /// Evidence from type system enforcement
    TypeSystemEnforcement,
    /// Evidence from pattern matching
    PatternMatching,
    /// Evidence from mathematical analysis
    MathematicalAnalysis,
    /// Evidence from logical consistency
    LogicalConsistency,
    /// Evidence from empirical observation
    EmpiricalObservation,
    /// Evidence from formal verification
    FormalVerification,
}

/// Configuration for invariant discovery
#[derive(Debug, Clone)]
pub struct InvariantDiscoveryConfig {
    /// Maximum number of invariants to discover
    pub max_invariants: usize,
    /// Minimum confidence threshold for reporting invariants
    pub confidence_threshold: f64,
    /// Enable pattern-based discovery
    pub enable_patterns: bool,
    /// Enable numerical analysis
    pub enable_numerical_analysis: bool,
    /// Enable logical analysis
    pub enable_logical_analysis: bool,
    /// Enable structural analysis
    pub enable_structural_analysis: bool,
    /// Enable Z3 verification
    pub enable_z3_verification: bool,
    /// Verification timeout in milliseconds
    pub verification_timeout: u64,
}

impl Default for InvariantDiscoveryConfig {
    fn default() -> Self {
        Self {
            max_invariants: 50,
            confidence_threshold: 0.5,
            enable_patterns: true,
            enable_numerical_analysis: true,
            enable_logical_analysis: true,
            enable_structural_analysis: true,
            enable_z3_verification: false,
            verification_timeout: 5000,
        }
    }
}

/// Statistics about the discovery process
#[derive(Debug, Clone)]
pub struct DiscoveryStats {
    /// Total analysis time
    pub total_time: Duration,
    /// Number of type invariants discovered
    pub type_invariants: usize,
    /// Number of functional invariants discovered
    pub functional_invariants: usize,
    /// Number of verified invariants
    pub verified_correct: usize,
    /// Number of disproven invariants
    pub disproven: usize,
    /// Time spent on verification
    pub verification_time: Duration,
}

impl Default for DiscoveryStats {
    fn default() -> Self {
        Self {
            total_time: Duration::new(0, 0),
            type_invariants: 0,
            functional_invariants: 0,
            verified_correct: 0,
            disproven: 0,
            verification_time: Duration::new(0, 0),
        }
    }
}

impl DiscoveredInvariant {
    /// Create a new discovered invariant
    pub fn new(
        id: String,
        name: String,
        formula: PropertyFormula,
        invariant_type: InvariantType,
        confidence: f64,
    ) -> Self {
        Self {
            id,
            name,
            formula,
            invariant_type,
            confidence,
            evidence: Vec::new(),
            sources: Vec::new(),
            verified: false,
        }
    }

    /// Add evidence to this invariant
    pub fn add_evidence(&mut self, evidence: InvariantEvidence) {
        self.evidence.push(evidence);
    }

    /// Add source location to this invariant
    pub fn add_source(&mut self, source: SourceLocation) {
        self.sources.push(source);
    }

    /// Get the highest evidence strength
    pub fn max_evidence_strength(&self) -> f64 {
        self.evidence.iter()
            .map(|e| e.strength)
            .fold(0.0, f64::max)
    }

    /// Check if this invariant has evidence of a specific type
    pub fn has_evidence_type(&self, evidence_type: &EvidenceType) -> bool {
        self.evidence.iter()
            .any(|e| &e.evidence_type == evidence_type)
    }
}

impl InvariantEvidence {
    /// Create new evidence
    pub fn new(
        evidence_type: EvidenceType,
        strength: f64,
        description: String,
        location: SourceLocation,
    ) -> Self {
        Self {
            evidence_type,
            strength,
            description,
            location,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::property_types::{PropertyFormula, FormulaStructure, AtomicFormula, Term};
    use std::collections::HashSet;

    fn create_test_formula() -> PropertyFormula {
        PropertyFormula {
            structure: FormulaStructure::Atomic(AtomicFormula {
                predicate: "test".to_string(),
                terms: vec![Term::Constant("x".to_string(), "ℕ".to_string())],
                type_signature: None,
            }),
            quantifiers: vec![],
            free_variables: HashSet::new(),
            predicates: HashSet::new(),
            functions: HashSet::new(),
            constants: HashSet::new(),
        }
    }

    fn create_test_location() -> SourceLocation {
        SourceLocation {
            block_type: "Test".to_string(),
            line: Some(1),
            column: Some(1),
            source_text: Some("test".to_string()),
        }
    }

    #[test]
    fn test_discovered_invariant_new() {
        let formula = create_test_formula();
        let invariant = DiscoveredInvariant::new(
            "test_id".to_string(),
            "Test Invariant".to_string(),
            formula,
            InvariantType::TypeStructural,
            0.85,
        );

        assert_eq!(invariant.id, "test_id");
        assert_eq!(invariant.name, "Test Invariant");
        assert_eq!(invariant.invariant_type, InvariantType::TypeStructural);
        assert_eq!(invariant.confidence, 0.85);
        assert!(!invariant.verified);
        assert!(invariant.evidence.is_empty());
        assert!(invariant.sources.is_empty());
    }

    #[test]
    fn test_add_evidence() {
        let formula = create_test_formula();
        let mut invariant = DiscoveredInvariant::new(
            "test_id".to_string(),
            "Test Invariant".to_string(),
            formula,
            InvariantType::TypeStructural,
            0.85,
        );

        let evidence = InvariantEvidence::new(
            EvidenceType::TypeSystemEnforcement,
            0.9,
            "Test evidence".to_string(),
            create_test_location(),
        );

        invariant.add_evidence(evidence);
        assert_eq!(invariant.evidence.len(), 1);
        assert_eq!(invariant.evidence[0].strength, 0.9);
    }

    #[test]
    fn test_max_evidence_strength() {
        let formula = create_test_formula();
        let mut invariant = DiscoveredInvariant::new(
            "test_id".to_string(),
            "Test Invariant".to_string(),
            formula,
            InvariantType::TypeStructural,
            0.85,
        );

        // No evidence initially
        assert_eq!(invariant.max_evidence_strength(), 0.0);

        // Add evidence with different strengths
        invariant.add_evidence(InvariantEvidence::new(
            EvidenceType::TypeSystemEnforcement,
            0.7,
            "Evidence 1".to_string(),
            create_test_location(),
        ));

        invariant.add_evidence(InvariantEvidence::new(
            EvidenceType::MathematicalAnalysis,
            0.95,
            "Evidence 2".to_string(),
            create_test_location(),
        ));

        assert_eq!(invariant.max_evidence_strength(), 0.95);
    }

    #[test]
    fn test_has_evidence_type() {
        let formula = create_test_formula();
        let mut invariant = DiscoveredInvariant::new(
            "test_id".to_string(),
            "Test Invariant".to_string(),
            formula,
            InvariantType::TypeStructural,
            0.85,
        );

        assert!(!invariant.has_evidence_type(&EvidenceType::TypeSystemEnforcement));

        invariant.add_evidence(InvariantEvidence::new(
            EvidenceType::TypeSystemEnforcement,
            0.9,
            "Test evidence".to_string(),
            create_test_location(),
        ));

        assert!(invariant.has_evidence_type(&EvidenceType::TypeSystemEnforcement));
        assert!(!invariant.has_evidence_type(&EvidenceType::MathematicalAnalysis));
    }

    #[test]
    fn test_invariant_discovery_config_default() {
        let config = InvariantDiscoveryConfig::default();
        assert_eq!(config.max_invariants, 50);
        assert_eq!(config.confidence_threshold, 0.5);
        assert!(config.enable_patterns);
        assert!(config.enable_numerical_analysis);
        assert!(config.enable_logical_analysis);
        assert!(config.enable_structural_analysis);
        assert!(!config.enable_z3_verification);
        assert_eq!(config.verification_timeout, 5000);
    }

    #[test]
    fn test_discovery_stats_default() {
        let stats = DiscoveryStats::default();
        assert_eq!(stats.total_time, Duration::new(0, 0));
        assert_eq!(stats.type_invariants, 0);
        assert_eq!(stats.functional_invariants, 0);
        assert_eq!(stats.verified_correct, 0);
        assert_eq!(stats.disproven, 0);
        assert_eq!(stats.verification_time, Duration::new(0, 0));
    }

    #[test]
    fn test_invariant_type_equality() {
        assert_eq!(InvariantType::TypeStructural, InvariantType::TypeStructural);
        assert_ne!(InvariantType::TypeStructural, InvariantType::TypeMembership);
    }

    #[test]
    fn test_evidence_type_equality() {
        assert_eq!(EvidenceType::TypeSystemEnforcement, EvidenceType::TypeSystemEnforcement);
        assert_ne!(EvidenceType::TypeSystemEnforcement, EvidenceType::PatternMatching);
    }
}