//! Reference.md Formal Verification System
//! 
//! This module provides comprehensive formal verification of AISP documents
//! against the mathematical specifications in reference.md. The verification
//! system is split into focused modules for maintainability.

pub mod ambiguity_verification;
pub mod feature_verification;  
pub mod pipeline_verification;
pub mod trivector_verification;

use crate::ast::canonical::{CanonicalAispDocument as AispDocument, CanonicalAispBlock as AispBlock};
use crate::error::{AispResult};
use crate::semantic::{DeepVerificationResult};
use crate::z3_verification::{PropertyResult, Z3VerificationFacade};

use std::collections::HashMap;
use std::time::{Duration, Instant};

pub use ambiguity_verification::*;
pub use feature_verification::*;
pub use pipeline_verification::*;
pub use trivector_verification::*;

/// Main reference.md validator that coordinates all verification modules
pub struct ReferenceValidator {
    z3_verifier: Z3VerificationFacade,
    verification_stats: VerificationStats,
}

/// Overall verification statistics
#[derive(Debug, Clone)]
pub struct VerificationStats {
    pub features_verified: usize,
    pub features_failed: usize,
    pub smt_queries: usize,
    pub total_time: Duration,
}

/// Complete reference validation result
#[derive(Debug, Clone)]
pub struct ReferenceValidationResult {
    pub mathematical_foundations: MathematicalFoundationsResult,
    pub trivector_orthogonality: TriVectorOrthogonalityResult,
    pub feature_compliance: FeatureComplianceResult,
    pub pipeline_verification: PipelineVerificationResult,
    pub overall_compliance: ComplianceLevel,
    pub compliance_score: f64,
}

/// Compliance levels based on verification results
#[derive(Debug, Clone, PartialEq)]
pub enum ComplianceLevel {
    FullCompliance,
    PartialCompliance,
    MinimalCompliance, 
    Failed,
}

impl ReferenceValidator {
    /// Create new reference validator
    pub fn new() -> AispResult<Self> {
        let z3_verifier = Z3VerificationFacade::new()?;
        
        Ok(Self {
            z3_verifier,
            verification_stats: VerificationStats {
                features_verified: 0,
                features_failed: 0,
                smt_queries: 0,
                total_time: Duration::ZERO,
            },
        })
    }
    
    /// Validate an AISP document against reference.md specifications
    pub fn validate_document(
        &mut self,
        document: &AispDocument,
        semantic_result: &DeepVerificationResult,
    ) -> AispResult<ReferenceValidationResult> {
        let start_time = Instant::now();
        
        // Run all verification modules
        let mathematical_foundations = self.verify_mathematical_foundations(document, semantic_result)?;
        let trivector_orthogonality = self.verify_trivector_orthogonality(document, semantic_result)?;
        let feature_compliance = self.verify_feature_compliance(document)?;
        let pipeline_verification = self.verify_pipeline_success_rates()?;
        
        // Calculate overall compliance
        let compliance_score = self.calculate_compliance_score(
            &mathematical_foundations,
            &trivector_orthogonality, 
            &feature_compliance,
            &pipeline_verification,
        );
        
        let overall_compliance = self.determine_compliance_level(compliance_score);
        
        self.verification_stats.total_time += start_time.elapsed();
        
        Ok(ReferenceValidationResult {
            mathematical_foundations,
            trivector_orthogonality,
            feature_compliance,
            pipeline_verification,
            overall_compliance,
            compliance_score,
        })
    }
    
    /// Get current verification statistics
    pub fn get_stats(&self) -> &VerificationStats {
        &self.verification_stats
    }
}

// Delegate to specialized verification modules
impl ReferenceValidator {
    fn verify_mathematical_foundations(
        &mut self,
        document: &AispDocument,
        semantic_result: &DeepVerificationResult,
    ) -> AispResult<MathematicalFoundationsResult> {
        let mut ambiguity_verifier = AmbiguityVerifier::new(&mut self.z3_verifier);
        ambiguity_verifier.verify_ambiguity_calculation("", semantic_result)
    }
    
    fn verify_trivector_orthogonality(
        &mut self,
        document: &AispDocument,
        semantic_result: &DeepVerificationResult,
    ) -> AispResult<TriVectorOrthogonalityResult> {
        let mut trivector_verifier = TriVectorVerifier::new(&mut self.z3_verifier);
        trivector_verifier.verify_orthogonality(document, semantic_result)
    }
    
    fn verify_feature_compliance(
        &mut self,
        document: &AispDocument,
    ) -> AispResult<FeatureComplianceResult> {
        let mut feature_verifier = FeatureVerifier::new(&mut self.z3_verifier);
        feature_verifier.verify_all_features(document)
    }
    
    fn verify_pipeline_success_rates(&mut self) -> AispResult<PipelineVerificationResult> {
        let mut pipeline_verifier = PipelineVerifier::new(&mut self.z3_verifier);
        pipeline_verifier.verify_success_rates()
    }
    
    fn calculate_compliance_score(
        &self,
        math: &MathematicalFoundationsResult,
        trivector: &TriVectorOrthogonalityResult,
        features: &FeatureComplianceResult,
        pipeline: &PipelineVerificationResult,
    ) -> f64 {
        let mut score = 0.0;
        let mut weight = 0.0;

        // Mathematical foundations (25% weight)
        if math.ambiguity_verified { score += 0.125; }
        if math.token_efficiency.meets_spec { score += 0.125; }
        weight += 0.25;

        // Tri-vector orthogonality (25% weight)
        if trivector.vh_vs_orthogonal { score += 0.125; }
        if trivector.vl_vs_orthogonal { score += 0.125; }
        weight += 0.25;

        // Feature compliance (35% weight)
        score += 0.35 * (features.compliance_percentage / 100.0);
        weight += 0.35;

        // Pipeline verification (15% weight)
        if pipeline.mathematical_proof_valid { score += 0.15; }
        weight += 0.15;

        if weight > 0.0 { score / weight } else { 0.0 }
    }
    
    fn determine_compliance_level(&self, score: f64) -> ComplianceLevel {
        match score {
            s if s >= 0.95 => ComplianceLevel::FullCompliance,
            s if s >= 0.80 => ComplianceLevel::PartialCompliance,
            s if s >= 0.60 => ComplianceLevel::MinimalCompliance,
            _ => ComplianceLevel::Failed,
        }
    }
}

impl Default for ReferenceValidator {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            // Fallback with disabled Z3
            Self {
                z3_verifier: Z3VerificationFacade::new().expect("Z3 verification required"),
                verification_stats: VerificationStats {
                    features_verified: 0,
                    features_failed: 0,
                    smt_queries: 0,
                    total_time: Duration::ZERO,
                },
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AispDocument, DocumentHeader, DocumentMetadata, Span, Position};
    
    fn create_test_document() -> AispDocument {
        AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "test".to_string(),
                date: "2026-01-26".to_string(),
                metadata: None,
            },
            metadata: DocumentMetadata {
                domain: Some("test".to_string()),
                protocol: None,
            },
            blocks: vec![],
            span: Span {
                start: Position { line: 1, column: 1, offset: 0 },
                end: Position { line: 1, column: 1, offset: 0 },
            },
        }
    }
    
    fn create_test_semantic_result() -> DeepVerificationResult {
        DeepVerificationResult {
            type_assignments: HashMap::new(),
            constraints: vec![],
            ambiguity: 0.01, // Low ambiguity
            warnings: vec![],
            errors: vec![],
            rule_coverage: 1.0,
            coherence_score: 0.95,
        }
    }
    
    #[test]
    fn test_validator_creation() {
        let validator = ReferenceValidator::new();
        assert!(validator.is_ok());
    }
    
    #[test] 
    fn test_compliance_level_determination() {
        let validator = ReferenceValidator::default();
        
        assert_eq!(validator.determine_compliance_level(0.99), ComplianceLevel::FullCompliance);
        assert_eq!(validator.determine_compliance_level(0.85), ComplianceLevel::PartialCompliance);
        assert_eq!(validator.determine_compliance_level(0.65), ComplianceLevel::MinimalCompliance);
        assert_eq!(validator.determine_compliance_level(0.30), ComplianceLevel::Failed);
    }
    
    #[test]
    fn test_document_validation() {
        let mut validator = ReferenceValidator::default();
        let document = create_test_document();
        let semantic_result = create_test_semantic_result();
        
        let result = validator.validate_document(&document, &semantic_result);
        assert!(result.is_ok());
        
        let validation_result = result.unwrap();
        assert!(validation_result.compliance_score >= 0.0);
        assert!(validation_result.compliance_score <= 1.0);
    }
    
    #[test]
    fn test_verification_stats_tracking() {
        let validator = ReferenceValidator::default();
        let stats = validator.get_stats();
        
        assert_eq!(stats.features_verified, 0);
        assert_eq!(stats.features_failed, 0);
        assert_eq!(stats.smt_queries, 0);
        assert_eq!(stats.total_time, Duration::ZERO);
    }
}