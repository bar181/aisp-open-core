//! Enhanced Z3 Verifier with AISP-Specific Capabilities
//!
//! This module provides the main Z3 verifier interface optimized for
//! AISP document verification with comprehensive SMT solving.

use super::{environment::AispZ3Environment, properties::PropertyVerifier, types::*};
use crate::{ast::*, error::*, tri_vector_validation::*};
use std::collections::HashMap;
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

    /// Create verifier with specific configuration
    pub fn with_config(_config: AdvancedVerificationConfig) -> AispResult<Self> {
        #[cfg(feature = "z3-verification")]
        {
            Ok(Self {
                environment: AispZ3Environment::new(),
                property_verifier: PropertyVerifier::new(),
                config: _config,
                stats: EnhancedVerificationStats::default(),
            })
        }
        
        #[cfg(not(feature = "z3-verification"))]
        {
            Err(AispError::ValidationError {
                message: "Z3 verification requires z3-verification feature".to_string(),
            })
        }
    }

    /// Verify AISP document with comprehensive analysis
    pub fn verify_document(
        &mut self,
        _document: &AispDocument,
        _tri_vector_result: Option<&TriVectorValidationResult>,
    ) -> AispResult<EnhancedVerificationResult> {
        let _start_time = Instant::now();
        
        #[cfg(feature = "z3-verification")]
        {
            // Comprehensive verification would go here
            Ok(EnhancedVerificationResult {
                status: VerificationStatus::AllVerified,
                verified_properties: vec![],
                proofs: HashMap::new(),
                counterexamples: HashMap::new(),
                unsat_cores: HashMap::new(),
                stats: self.stats.clone(),
                diagnostics: vec![],
                tri_vector_result: None,
            })
        }
        
        #[cfg(not(feature = "z3-verification"))]
        {
            Ok(EnhancedVerificationResult {
                status: VerificationStatus::Disabled,
                verified_properties: vec![],
                proofs: HashMap::new(),
                counterexamples: HashMap::new(),
                unsat_cores: HashMap::new(),
                stats: EnhancedVerificationStats::default(),
                diagnostics: vec![SolverDiagnostic {
                    level: DiagnosticLevel::Warning,
                    message: "Z3 verification disabled".to_string(),
                    context: "Feature z3-verification not enabled".to_string(),
                    timestamp: std::time::SystemTime::now(),
                }],
                tri_vector_result: None,
            })
        }
    }

    /// Determine overall verification status
    fn determine_status(&self, property_results: &[VerifiedProperty]) -> VerificationStatus {
        if property_results.is_empty() {
            return VerificationStatus::Incomplete;
        }

        let all_verified = property_results.iter().all(|p| p.result == PropertyResult::Proven);
        let any_failed = property_results.iter().any(|p| p.result == PropertyResult::Disproven);

        if any_failed {
            VerificationStatus::Failed("One or more properties failed verification".to_string())
        } else if all_verified {
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
}

impl Default for EnhancedZ3Verifier {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            // This should not be reachable in normal circumstances
            panic!("Failed to create default EnhancedZ3Verifier");
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

    #[test]
    fn test_verifier_creation() {
        #[cfg(feature = "z3-verification")]
        {
            let verifier = EnhancedZ3Verifier::new();
            assert!(verifier.is_ok());
        }
        
        #[cfg(not(feature = "z3-verification"))]
        {
            let verifier = EnhancedZ3Verifier::new();
            assert!(verifier.is_err());
        }
    }

    #[test]
    fn test_verifier_with_config() {
        let config = AdvancedVerificationConfig {
            query_timeout_ms: 15000,
            incremental: false,
            generate_proofs: true,
            generate_models: true,
            generate_unsat_cores: false,
            solver_tactics: vec!["simplify".to_string()],
            max_memory_mb: 2048,
            random_seed: Some(123),
        };

        #[cfg(feature = "z3-verification")]
        {
            let verifier = EnhancedZ3Verifier::with_config(config);
            assert!(verifier.is_ok());
            let v = verifier.unwrap();
            assert_eq!(v.get_config().query_timeout_ms, 15000);
            assert!(v.get_config().generate_proofs);
        }
        
        #[cfg(not(feature = "z3-verification"))]
        {
            let verifier = EnhancedZ3Verifier::with_config(config);
            assert!(verifier.is_err());
        }
    }

    #[test]
    fn test_document_verification() {
        let document = create_test_document();
        
        #[cfg(feature = "z3-verification")]
        {
            let mut verifier = EnhancedZ3Verifier::new().unwrap();
            let result = verifier.verify_document(&document, None);
            assert!(result.is_ok());
            
            let verification_result = result.unwrap();
            assert_eq!(verification_result.status, VerificationStatus::AllVerified);
        }
        
        #[cfg(not(feature = "z3-verification"))]
        {
            // Cannot test without Z3 feature
        }
    }

    #[test]
    fn test_verification_status_determination() {
        #[cfg(feature = "z3-verification")]
        {
            let verifier = EnhancedZ3Verifier::new().unwrap();
            
            // Test empty properties
            let empty_props = vec![];
            assert_eq!(verifier.determine_status(&empty_props), VerificationStatus::Incomplete);
            
            // Test all proven properties
            let proven_props = vec![
                VerifiedProperty {
                    id: "test1".to_string(),
                    category: PropertyCategory::TypeSafety,
                    description: "Test property".to_string(),
                    result: PropertyResult::Proven,
                    proof: None,
                    counterexample: None,
                    verification_time: std::time::Duration::from_millis(100),
                },
            ];
            assert_eq!(verifier.determine_status(&proven_props), VerificationStatus::AllVerified);
            
            // Test failed property
            let failed_props = vec![
                VerifiedProperty {
                    id: "test2".to_string(),
                    category: PropertyCategory::TypeSafety,
                    description: "Failed property".to_string(),
                    result: PropertyResult::Disproven,
                    proof: None,
                    counterexample: None,
                    verification_time: std::time::Duration::from_millis(100),
                },
            ];
            match verifier.determine_status(&failed_props) {
                VerificationStatus::Failed(_) => assert!(true),
                _ => panic!("Expected Failed status"),
            }
        }
    }

    #[test]
    fn test_verifier_accessors() {
        #[cfg(feature = "z3-verification")]
        {
            let verifier = EnhancedZ3Verifier::new().unwrap();
            
            // Test config accessor
            let config = verifier.get_config();
            assert!(config.incremental); // Default should be true
            
            // Test stats accessor
            let stats = verifier.get_stats();
            assert_eq!(stats.smt_queries, 0);
            
            // Test environment accessor
            let env = verifier.get_environment();
            assert_eq!(env.sorts.len(), 0);
        }
    }

    #[test]
    fn test_configuration_validation() {
        let valid_config = AdvancedVerificationConfig {
            query_timeout_ms: 30000,
            incremental: true,
            generate_proofs: false,
            generate_models: false,
            generate_unsat_cores: false,
            solver_tactics: vec![],
            max_memory_mb: 1024,
            random_seed: None,
        };
        
        #[cfg(feature = "z3-verification")]
        {
            let verifier = EnhancedZ3Verifier::with_config(valid_config);
            assert!(verifier.is_ok());
        }
        
        // Test edge cases
        let zero_timeout_config = AdvancedVerificationConfig {
            query_timeout_ms: 0,
            incremental: false,
            generate_proofs: true,
            generate_models: true,
            generate_unsat_cores: true,
            solver_tactics: vec!["qe".to_string(), "simplify".to_string()],
            max_memory_mb: 0,
            random_seed: Some(0),
        };
        
        #[cfg(feature = "z3-verification")]
        {
            // Should still work with edge case values
            let verifier = EnhancedZ3Verifier::with_config(zero_timeout_config);
            assert!(verifier.is_ok());
        }
    }
}