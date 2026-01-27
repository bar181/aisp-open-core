//! Deep Verifier Types and Structures
//!
//! Type definitions and supporting structures for deep semantic verification
//! Implements SRP by containing only type definitions

use crate::ast::canonical::*;
use std::collections::HashMap;
use std::fmt;

/// Deep verification result with comprehensive analysis
#[derive(Debug, Clone)]
pub struct DeepVerificationResult {
    pub overall_confidence: f64,
    pub semantic_score: f64,
    pub type_safety_score: f64,
    pub logic_consistency_score: f64,
    pub mathematical_correctness_score: f64,
    pub deception_risk_score: f64,
    pub verification_details: VerificationDetails,
    pub security_assessment: SecurityAssessment,
    pub recommendations: Vec<VerificationRecommendation>,
}

#[derive(Debug, Clone)]
pub struct VerificationDetails {
    pub verified_components: Vec<ComponentVerification>,
    pub failed_verifications: Vec<VerificationFailure>,
    pub warnings: Vec<VerificationWarning>,
    pub coverage_metrics: CoverageMetrics,
    pub performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Clone)]
pub struct SecurityAssessment {
    pub threat_level: ThreatLevel,
    pub vulnerability_count: usize,
    pub attack_surface_analysis: AttackSurfaceAnalysis,
    pub security_recommendations: Vec<SecurityRecommendation>,
    pub compliance_status: ComplianceStatus,
}

// Core type definitions

#[derive(Debug, Clone)]
pub struct TypeDefinition {
    pub name: String,
    pub structure: TypeStructure,
    pub constraints: Vec<TypeConstraint>,
    pub security_level: SecurityLevel,
    pub verification_status: VerificationStatus,
}

#[derive(Debug, Clone)]
pub enum TypeStructure {
    Primitive(PrimitiveType),
    Composite(CompositeType),
    Function(FunctionType),
    Generic(GenericType),
    Reference(String),
    Array(Box<TypeStructure>),
    Optional(Box<TypeStructure>),
}

#[derive(Debug, Clone)]
pub struct TypeConstraint {
    pub constraint_type: ConstraintType,
    pub expression: String,
    pub severity: ConstraintSeverity,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintType {
    Range,
    Pattern,
    Dependency,
    Security,
    Performance,
    Validation,
}

#[derive(Debug, Clone)]
pub struct LogicalAxiom {
    pub name: String,
    pub formula: String,
    pub axiom_type: AxiomType,
    pub priority: AxiomPriority,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AxiomType {
    Foundational,
    Derived,
    Domain,
}

#[derive(Debug, Clone)]
pub struct PlaceholderPattern {
    pub pattern_name: String,
    pub detection_regex: String,
    pub risk_level: RiskLevel,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThreatLevel {
    None,
    Minimal,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SecurityLevel {
    Public,
    Internal,
    Confidential,
    Secret,
    TopSecret,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VerificationStatus {
    Pending,
    Verified,
    Failed,
    Skipped,
    InProgress,
}

// Analysis result types

#[derive(Debug, Clone)]
pub struct TypeAnalysisResult {
    pub type_safety_score: f64,
    pub type_violations: Vec<String>,
    pub type_recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LogicAnalysisResult {
    pub consistency_score: f64,
    pub contradictions: Vec<String>,
    pub axiom_violations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DependencyAnalysisResult {
    pub circular_dependencies: Vec<String>,
    pub dependency_violations: Vec<String>,
    pub impact_score: f64,
}

#[derive(Debug, Clone)]
pub struct MathematicalAnalysisResult {
    pub correctness_score: f64,
    pub proof_violations: Vec<String>,
    pub mathematical_errors: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DeceptionAnalysisResult {
    pub deception_score: f64,
    pub placeholder_violations: Vec<String>,
    pub behavioral_inconsistencies: Vec<String>,
    pub authenticity_score: f64,
}

// Supporting types - consolidated from original file
#[derive(Debug, Clone)] pub struct InferredType { pub type_name: String, pub confidence: f64 }
#[derive(Debug, Clone)] pub struct TypeSecurityPolicy { pub policy_name: String, pub rules: Vec<String> }
#[derive(Debug, Clone)] pub struct InferenceRule { pub name: String, pub formula: String }
#[derive(Debug, Clone)] pub struct ContradictionDetector { pub detection_methods: Vec<String> }
#[derive(Debug, Clone)] pub struct ProofValidator { pub validation_rules: Vec<String> }
#[derive(Debug, Clone)] pub struct DependencyGraph { pub nodes: Vec<String>, pub edges: Vec<(String, String)> }
#[derive(Debug, Clone)] pub struct CircularDependencyDetector { pub algorithms: Vec<String> }
#[derive(Debug, Clone)] pub struct DependencyImpactAnalyzer { pub impact_metrics: Vec<String> }
#[derive(Debug, Clone)] pub struct SecurityBoundaryAnalyzer { pub boundary_rules: Vec<String> }
#[derive(Debug, Clone)] pub struct SMTSolverInterface { pub solver_type: String, pub timeout_ms: u64 }
#[derive(Debug, Clone)] pub struct MathematicalProperty { pub name: String, pub formula: String }
#[derive(Debug, Clone)] pub struct CorrectnessProof { pub proof_steps: Vec<String> }
#[derive(Debug, Clone)] pub struct VerificationResult { pub result: bool, pub confidence: f64 }
#[derive(Debug, Clone)] pub struct BehavioralAnalyzer { pub analysis_type: String }
#[derive(Debug, Clone)] pub struct ComplexityAnalyzer { pub metrics: Vec<String> }
#[derive(Debug, Clone)] pub struct CoverageAnalyzer { pub coverage_types: Vec<String> }
#[derive(Debug, Clone)] pub struct AuthenticityVerifier { pub verification_methods: Vec<String> }
#[derive(Debug, Clone)] pub struct ComponentVerification { pub component: String, pub status: VerificationStatus }
#[derive(Debug, Clone)] pub struct VerificationFailure { pub component: String, pub reason: String }
#[derive(Debug, Clone)] pub struct VerificationWarning { pub component: String, pub warning: String }
#[derive(Debug, Clone)] pub struct CoverageMetrics { pub line_coverage: f64, pub branch_coverage: f64 }
#[derive(Debug, Clone)] pub struct PerformanceMetrics { pub verification_time_ms: u64, pub memory_usage_mb: usize }
#[derive(Debug, Clone)] pub struct AttackSurfaceAnalysis { pub surface_area: f64, pub vulnerabilities: Vec<String> }
#[derive(Debug, Clone)] pub struct SecurityRecommendation { pub priority: String, pub action: String }
#[derive(Debug, Clone)] pub struct ComplianceStatus { pub compliant: bool, pub missing_requirements: Vec<String> }
#[derive(Debug, Clone)] pub struct VerificationRecommendation { pub priority: String, pub recommendation: String }
#[derive(Debug, Clone)] pub struct PrimitiveType { pub type_name: String }
#[derive(Debug, Clone)] pub struct CompositeType { pub fields: Vec<String> }
#[derive(Debug, Clone)] pub struct FunctionType { pub input_types: Vec<String>, pub output_type: String }
#[derive(Debug, Clone)] pub struct GenericType { pub base_type: String, pub type_parameters: Vec<String> }
#[derive(Debug, Clone, PartialEq)] pub enum ConstraintSeverity { Info, Warning, Error, Critical }
#[derive(Debug, Clone, PartialEq)] pub enum AxiomPriority { Low, Medium, High, Critical }

impl fmt::Display for DeepVerificationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Deep Semantic Verification Results")?;
        writeln!(f, "==================================")?;
        writeln!(f, "Overall Confidence: {:.2}%", self.overall_confidence * 100.0)?;
        writeln!(f, "Semantic Score: {:.2}%", self.semantic_score * 100.0)?;
        writeln!(f, "Type Safety Score: {:.2}%", self.type_safety_score * 100.0)?;
        writeln!(f, "Logic Consistency: {:.2}%", self.logic_consistency_score * 100.0)?;
        writeln!(f, "Mathematical Correctness: {:.2}%", self.mathematical_correctness_score * 100.0)?;
        writeln!(f, "Deception Risk: {:.2}%", self.deception_risk_score * 100.0)?;
        writeln!(f, "Threat Level: {:?}", self.security_assessment.threat_level)?;
        writeln!(f, "Vulnerabilities: {}", self.security_assessment.vulnerability_count)?;
        
        if !self.recommendations.is_empty() {
            writeln!(f, "\nRecommendations:")?;
            for rec in &self.recommendations {
                writeln!(f, "  - {}: {}", rec.priority, rec.recommendation)?;
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deep_verification_result_creation() {
        let result = DeepVerificationResult {
            overall_confidence: 0.95,
            semantic_score: 0.90,
            type_safety_score: 0.98,
            logic_consistency_score: 0.88,
            mathematical_correctness_score: 0.92,
            deception_risk_score: 0.05,
            verification_details: VerificationDetails {
                verified_components: vec![],
                failed_verifications: vec![],
                warnings: vec![],
                coverage_metrics: CoverageMetrics {
                    line_coverage: 0.95,
                    branch_coverage: 0.90,
                },
                performance_metrics: PerformanceMetrics {
                    verification_time_ms: 1500,
                    memory_usage_mb: 256,
                },
            },
            security_assessment: SecurityAssessment {
                threat_level: ThreatLevel::Low,
                vulnerability_count: 1,
                attack_surface_analysis: AttackSurfaceAnalysis {
                    surface_area: 0.1,
                    vulnerabilities: vec!["Minor input validation".to_string()],
                },
                security_recommendations: vec![],
                compliance_status: ComplianceStatus {
                    compliant: true,
                    missing_requirements: vec![],
                },
            },
            recommendations: vec![],
        };

        assert_eq!(result.overall_confidence, 0.95);
        assert_eq!(result.security_assessment.threat_level, ThreatLevel::Low);
    }

    #[test]
    fn test_type_structure_variants() {
        let primitive = TypeStructure::Primitive(PrimitiveType { type_name: "int".to_string() });
        let array = TypeStructure::Array(Box::new(primitive));
        let optional = TypeStructure::Optional(Box::new(array));

        match optional {
            TypeStructure::Optional(inner) => {
                match *inner {
                    TypeStructure::Array(_) => assert!(true),
                    _ => panic!("Expected Array inside Optional"),
                }
            },
            _ => panic!("Expected Optional type"),
        }
    }

    #[test]
    fn test_constraint_types() {
        let constraints = vec![
            ConstraintType::Range,
            ConstraintType::Pattern,
            ConstraintType::Security,
            ConstraintType::Performance,
        ];
        assert_eq!(constraints.len(), 4);
        assert_eq!(constraints[0], ConstraintType::Range);
    }

    #[test]
    fn test_verification_status_enum() {
        assert_eq!(VerificationStatus::Pending, VerificationStatus::Pending);
        assert_ne!(VerificationStatus::Verified, VerificationStatus::Failed);
    }

    #[test]
    fn test_threat_level_ordering() {
        let levels = vec![
            ThreatLevel::None,
            ThreatLevel::Low,
            ThreatLevel::Medium,
            ThreatLevel::High,
            ThreatLevel::Critical,
        ];
        assert_eq!(levels.len(), 5);
        assert_eq!(levels[4], ThreatLevel::Critical);
    }
}