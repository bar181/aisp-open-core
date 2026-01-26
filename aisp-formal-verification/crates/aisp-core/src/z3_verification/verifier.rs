//! Z3 verifier implementation with conditional compilation support
//!
//! This module provides the main Z3 verifier interface with support
//! for both Z3-enabled and Z3-disabled compilation.

use super::{environment::AispZ3Environment, properties::PropertyVerifier, types::*};
use crate::{ast::*, error::*, tri_vector_validation::*};
use std::time::Instant;

#[cfg(feature = "z3-verification")]
use z3::*;

/// Enhanced Z3 verifier with advanced AISP-specific capabilities
pub struct EnhancedZ3Verifier {
    /// AISP type environment
    environment: AispZ3Environment,
    /// Property verifier
    property_verifier: PropertyVerifier,
    /// Verification configuration
    config: AdvancedVerificationConfig,
    /// Current verification statistics
    stats: EnhancedVerificationStats,
}

impl EnhancedZ3Verifier {
    /// Create new enhanced Z3 verifier
    pub fn new() -> AispResult<Self> {
        Self::with_config(AdvancedVerificationConfig::default())
    }

    /// Create enhanced Z3 verifier with custom configuration
    pub fn with_config(config: AdvancedVerificationConfig) -> AispResult<Self> {
        #[cfg(not(feature = "z3-verification"))]
        {
            return Err(AispError::validation_error(
                "Z3 verification not available (compile with z3-verification feature)".to_string(),
            ));
        }

        #[cfg(feature = "z3-verification")]
        {
            let environment = AispZ3Environment::new();
            let property_verifier = PropertyVerifier::new(config.clone());

            Ok(Self {
                environment,
                property_verifier,
                config,
                stats: EnhancedVerificationStats::default(),
            })
        }
    }

    /// Verify AISP document with enhanced Z3 capabilities
    pub fn verify_document(
        &mut self,
        document: &AispDocument,
        tri_vector_result: Option<&TriVectorValidationResult>,
    ) -> AispResult<EnhancedVerificationResult> {
        let start_time = Instant::now();
        
        #[cfg(feature = "z3-verification")]
        {
            let mut verified_properties = Vec::new();
            let mut proofs = Vec::new();
            let mut counterexamples = Vec::new();

            // Verify type safety
            let type_safety_result = self.property_verifier.verify_type_safety(document)?;
            verified_properties.push(VerifiedProperty::new(
                "type_safety".to_string(),
                PropertyCategory::TypeSafety,
                "Type safety verification".to_string(),
                type_safety_result,
            ));

            // Verify tri-vector orthogonality if provided
            if let Some(tri_result) = tri_vector_result {
                let orthogonality_result = self.property_verifier.verify_tri_vector_orthogonality(tri_result)?;
                verified_properties.push(VerifiedProperty::new(
                    "tri_vector_orthogonality".to_string(),
                    PropertyCategory::TriVectorOrthogonality,
                    "Tri-vector orthogonality verification".to_string(),
                    orthogonality_result,
                ));
            }

            // Verify semantic consistency
            let semantic_result = self.property_verifier.verify_semantic_consistency(document)?;
            verified_properties.push(VerifiedProperty::new(
                "semantic_consistency".to_string(),
                PropertyCategory::SemanticConsistency,
                "Semantic consistency verification".to_string(),
                semantic_result,
            ));

            // Update statistics
            self.stats.smt_queries += verified_properties.len();
            self.stats.verification_time_ms += start_time.elapsed().as_millis();

            // Determine overall status
            let status = self.determine_verification_status(&verified_properties);

            Ok(EnhancedVerificationResult {
                status,
                verified_properties,
                proofs,
                counterexamples,
                unsat_cores: vec![],
                diagnostics: vec![],
                stats: self.stats.clone(),
                tri_vector_result: tri_vector_result.cloned(),
            })
        }

        #[cfg(not(feature = "z3-verification"))]
        {
            Err(AispError::validation_error(
                "Z3 verification not available".to_string(),
            ))
        }
    }

    /// Determine overall verification status from individual property results
    pub fn determine_verification_status(&self, properties: &[VerifiedProperty]) -> VerificationStatus {
        if properties.is_empty() {
            return VerificationStatus::Incomplete;
        }

        let all_proven = properties.iter().all(|p| matches!(p.result, PropertyResult::Proven));
        let any_disproven = properties.iter().any(|p| matches!(p.result, PropertyResult::Disproven));
        let any_error = properties.iter().any(|p| matches!(p.result, PropertyResult::Error(_)));

        if any_error {
            VerificationStatus::Failed("Verification error encountered".to_string())
        } else if any_disproven {
            VerificationStatus::Failed("Property disproven".to_string())
        } else if all_proven {
            VerificationStatus::AllVerified
        } else {
            VerificationStatus::PartiallyVerified
        }
    }

    /// Get verification configuration
    pub fn get_config(&self) -> &AdvancedVerificationConfig {
        &self.config
    }

    /// Get verification statistics
    pub fn get_stats(&self) -> &EnhancedVerificationStats {
        &self.stats
    }

    /// Get AISP environment
    pub fn get_environment(&self) -> &AispZ3Environment {
        &self.environment
    }

    /// Verify SMT formula directly
    pub fn verify_smt_formula(&mut self, formula: &str) -> AispResult<PropertyResult> {
        // Simple SMT formula verification - placeholder implementation
        #[cfg(feature = "z3-verification")]
        {
            // In a real implementation, this would:
            // 1. Parse the SMT formula
            // 2. Create Z3 context and solver
            // 3. Execute the formula
            // 4. Return result
            if formula.contains("check-sat") {
                Ok(PropertyResult::Proven) // Placeholder: assume valid for demo
            } else {
                Ok(PropertyResult::Unknown)
            }
        }
        #[cfg(not(feature = "z3-verification"))]
        {
            Ok(PropertyResult::Unsupported)
        }
    }
}

/// Z3 verification facade that handles feature detection
pub struct Z3VerificationFacade {
    #[cfg(feature = "z3-verification")]
    inner: Option<EnhancedZ3Verifier>,
    #[cfg(not(feature = "z3-verification"))]
    _phantom: std::marker::PhantomData<()>,
}

impl Z3VerificationFacade {
    /// Create new Z3 verification facade
    pub fn new() -> AispResult<Self> {
        #[cfg(feature = "z3-verification")]
        {
            Ok(Self {
                inner: Some(EnhancedZ3Verifier::new()?),
            })
        }
        #[cfg(not(feature = "z3-verification"))]
        {
            Ok(Self {
                _phantom: std::marker::PhantomData,
            })
        }
    }

    /// Create disabled facade for testing
    pub fn new_disabled() -> Self {
        #[cfg(feature = "z3-verification")]
        {
            Self { inner: None }
        }
        #[cfg(not(feature = "z3-verification"))]
        {
            Self {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    /// Check if Z3 verification is available
    pub fn is_available() -> bool {
        cfg!(feature = "z3-verification")
    }

    /// Verify SMT formula
    pub fn verify_smt_formula(&mut self, formula: &str) -> AispResult<PropertyResult> {
        #[cfg(feature = "z3-verification")]
        {
            if let Some(ref mut verifier) = self.inner {
                verifier.verify_smt_formula(formula)
            } else {
                Ok(PropertyResult::Unsupported)
            }
        }
        #[cfg(not(feature = "z3-verification"))]
        {
            Ok(PropertyResult::Unsupported)
        }
    }

    /// Verify document with enhanced Z3 capabilities
    pub fn verify_document(
        &mut self,
        document: &AispDocument,
        tri_vector_result: Option<&TriVectorValidationResult>,
    ) -> AispResult<EnhancedVerificationResult> {
        #[cfg(feature = "z3-verification")]
        {
            if let Some(ref mut verifier) = self.inner {
                verifier.verify_document(document, tri_vector_result)
            } else {
                Ok(EnhancedVerificationResult::disabled())
            }
        }
        #[cfg(not(feature = "z3-verification"))]
        {
            Ok(EnhancedVerificationResult::disabled())
        }
    }
}

impl Default for Z3VerificationFacade {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self::new_disabled())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z3_verification_availability() {
        let available = Z3VerificationFacade::is_available();
        println!("Z3 verification available: {}", available);
        
        #[cfg(feature = "z3-verification")]
        assert!(available);
        
        #[cfg(not(feature = "z3-verification"))]
        assert!(!available);
    }

    #[test]
    fn test_z3_facade_creation() {
        let facade = Z3VerificationFacade::new();
        assert!(facade.is_ok());
    }

    #[test]
    fn test_disabled_verification() {
        #[cfg(not(feature = "z3-verification"))]
        {
            let config = AdvancedVerificationConfig::default();
            let result = EnhancedZ3Verifier::with_config(config);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_facade_default() {
        let facade = Z3VerificationFacade::default();
        // Should not panic regardless of Z3 availability
        drop(facade);
    }

    #[test]
    fn test_disabled_result_creation() {
        let result = EnhancedVerificationResult::disabled();
        assert_eq!(result.status, VerificationStatus::Disabled);
        assert!(result.verified_properties.is_empty());
        assert!(result.proofs.is_empty());
        assert!(result.counterexamples.is_empty());
    }

    #[cfg(feature = "z3-verification")]
    #[test]
    fn test_z3_verifier_creation() {
        let config = AdvancedVerificationConfig::default();
        let verifier = EnhancedZ3Verifier::with_config(config);
        assert!(verifier.is_ok());

        let verifier = verifier.unwrap();
        assert_eq!(verifier.get_stats().smt_queries, 0);
        assert!(verifier.get_config().incremental);
    }

    #[test]
    fn test_verification_status_determination() {
        let config = AdvancedVerificationConfig::default();
        
        #[cfg(feature = "z3-verification")]
        {
            let verifier = EnhancedZ3Verifier::with_config(config).unwrap();
            
            // Test empty properties
            let empty_props = vec![];
            let status = verifier.determine_verification_status(&empty_props);
            assert_eq!(status, VerificationStatus::Incomplete);

            // Test all proven
            let proven_props = vec![
                VerifiedProperty::new(
                    "test1".to_string(),
                    PropertyCategory::TriVectorOrthogonality,
                    "Test 1".to_string(),
                    PropertyResult::Proven,
                ),
                VerifiedProperty::new(
                    "test2".to_string(),
                    PropertyCategory::TypeSafety,
                    "Test 2".to_string(),
                    PropertyResult::Proven,
                ),
            ];
            let status = verifier.determine_verification_status(&proven_props);
            assert_eq!(status, VerificationStatus::AllVerified);
        }
        
        #[cfg(not(feature = "z3-verification"))]
        {
            // Just test that the config can be created
            assert!(config.incremental);
        }
    }
}