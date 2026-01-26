//! Main AISP validation engine
//! 
//! Provides the primary API for validating AISP documents with
//! comprehensive error handling and performance optimizations.

use crate::ast::*;
use crate::error::*;
use crate::parser_new::*;
use crate::semantic::*;
use crate::z3_integration::*;
use crate::tri_vector_validation::*;
use crate::enhanced_z3_verification::*;
use crate::ghost_intent_validation::*;
use crate::rossnet_scoring::*;
use crate::hebbian_learning::*;
use crate::anti_drift::*;
use crate::{MAX_DOCUMENT_SIZE, AISP_VERSION};
use std::time::{Duration, Instant};

/// Validation configuration options
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Maximum document size in bytes
    pub max_document_size: usize,
    /// Strict mode enables additional validations
    pub strict_mode: bool,
    /// Include detailed timing information
    pub include_timing: bool,
    /// Include AST in result (for debugging)
    pub include_ast: bool,
    /// Include symbol statistics
    pub include_symbol_stats: bool,
    /// Enable formal verification with Z3
    pub enable_formal_verification: bool,
    /// Z3 verification timeout
    pub z3_timeout: Duration,
    /// Enable tri-vector signal validation
    pub enable_trivector_validation: bool,
    /// Enable enhanced Z3 verification
    pub enable_enhanced_z3: bool,
    /// Enable ghost intent search validation
    pub enable_ghost_intent_validation: bool,
    /// Enable RossNet scoring validation
    pub enable_rossnet_scoring: bool,
    /// Enable Hebbian learning constraint validation
    pub enable_hebbian_learning: bool,
    /// Enable anti-drift protocol verification
    pub enable_anti_drift: bool,
    /// Strict formal verification mode - failures cause validation to fail instead of warnings
    pub strict_formal_verification: bool,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            max_document_size: MAX_DOCUMENT_SIZE,
            strict_mode: false,
            include_timing: false,
            include_ast: false,
            include_symbol_stats: false,
            enable_formal_verification: false,
            z3_timeout: Duration::from_secs(30),
            enable_trivector_validation: true,
            enable_enhanced_z3: Z3VerificationFacade::is_available(),
            enable_ghost_intent_validation: true,
            enable_rossnet_scoring: true,
            enable_hebbian_learning: true,
            enable_anti_drift: true,
            strict_formal_verification: true,  // Default to strict mode for sound verification
        }
    }
}

/// Complete validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Document is valid
    pub valid: bool,
    /// Quality tier
    pub tier: QualityTier,
    /// Tier symbol
    pub tier_symbol: String,
    /// Tier name
    pub tier_name: String,
    /// Tier numeric value (0-4)
    pub tier_value: u8,
    /// Semantic density (δ)
    pub delta: f64,
    /// Pure symbol density
    pub pure_density: f64,
    /// Calculated ambiguity level
    pub ambiguity: f64,
    /// Validation mode
    pub mode: String,
    /// Document size in bytes
    pub document_size: usize,
    /// Parsing timing
    pub parse_time: Option<Duration>,
    /// Semantic analysis timing
    pub semantic_time: Option<Duration>,
    /// Total validation timing
    pub total_time: Option<Duration>,
    /// Parsed AST (if requested)
    pub ast: Option<AispDocument>,
    /// Semantic analysis details
    pub semantic_analysis: Option<SemanticAnalysis>,
    /// Formal verification results
    pub formal_verification: Option<FormalVerificationResult>,
    /// Tri-vector validation results
    pub trivector_validation: Option<TriVectorValidationResult>,
    /// Enhanced Z3 verification results
    pub enhanced_z3_verification: Option<EnhancedVerificationResult>,
    /// Ghost intent search validation results
    pub ghost_intent_validation: Option<GhostIntentValidationResult>,
    /// RossNet scoring validation results
    pub rossnet_validation: Option<RossNetValidationResult>,
    /// Hebbian learning constraint validation results
    pub hebbian_validation: Option<HebbianValidationResult>,
    /// Anti-drift protocol verification results
    pub anti_drift_validation: Option<AntiDriftValidationResult>,
    /// All warnings collected
    pub warnings: Vec<AispWarning>,
    /// Error details (if validation failed)
    pub error: Option<AispError>,
}

impl ValidationResult {
    /// Create a failed validation result
    pub fn failed(error: AispError, document_size: usize) -> Self {
        Self {
            valid: false,
            tier: QualityTier::Reject,
            tier_symbol: "⊘".to_string(),
            tier_name: "Reject".to_string(),
            tier_value: 0,
            delta: 0.0,
            pure_density: 0.0,
            ambiguity: 1.0,
            mode: "rust-pure".to_string(),
            document_size,
            parse_time: None,
            semantic_time: None,
            total_time: None,
            ast: None,
            semantic_analysis: None,
            formal_verification: None,
            trivector_validation: None,
            enhanced_z3_verification: None,
            ghost_intent_validation: None,
            rossnet_validation: None,
            hebbian_validation: None,
            anti_drift_validation: None,
            warnings: Vec::new(),
            error: Some(error),
        }
    }

    /// Create a successful validation result
    pub fn success(
        analysis: SemanticAnalysis,
        document_size: usize,
        parse_time: Duration,
        semantic_time: Duration,
        ast: Option<AispDocument>,
        formal_verification: Option<FormalVerificationResult>,
        trivector_validation: Option<TriVectorValidationResult>,
        enhanced_z3_verification: Option<EnhancedVerificationResult>,
        ghost_intent_validation: Option<GhostIntentValidationResult>,
        rossnet_validation: Option<RossNetValidationResult>,
        hebbian_validation: Option<HebbianValidationResult>,
        anti_drift_validation: Option<AntiDriftValidationResult>,
    ) -> Self {
        Self {
            valid: analysis.valid,
            tier: analysis.tier.clone(),
            tier_symbol: analysis.tier.symbol().to_string(),
            tier_name: analysis.tier.name().to_string(),
            tier_value: analysis.tier.value(),
            delta: analysis.delta,
            pure_density: analysis.pure_density,
            ambiguity: analysis.ambiguity,
            mode: "rust-pure".to_string(),
            document_size,
            parse_time: Some(parse_time),
            semantic_time: Some(semantic_time),
            total_time: Some(parse_time + semantic_time),
            ast,
            semantic_analysis: Some(analysis.clone()),
            formal_verification,
            trivector_validation,
            enhanced_z3_verification,
            ghost_intent_validation,
            rossnet_validation,
            hebbian_validation,
            anti_drift_validation,
            warnings: analysis.warnings,
            error: None,
        }
    }

    /// Check if document has acceptable quality
    pub fn is_acceptable(&self) -> bool {
        self.valid && self.tier != QualityTier::Reject && self.ambiguity < 0.02
    }

    /// Get detailed error message
    pub fn error_message(&self) -> Option<String> {
        self.error.as_ref().map(|e| e.to_string())
    }
}

/// Main AISP validator
pub struct AispValidator {
    config: ValidationConfig,
}

impl AispValidator {
    /// Create a new validator with default configuration
    pub fn new() -> Self {
        Self {
            config: ValidationConfig::default(),
        }
    }

    /// Create a new validator with custom configuration
    pub fn with_config(config: ValidationConfig) -> Self {
        Self { config }
    }

    /// Update validator configuration
    pub fn configure(&mut self, config: ValidationConfig) {
        self.config = config;
    }

    /// Validate AISP document from source text
    pub fn validate(&self, source: &str) -> ValidationResult {
        let start_time = Instant::now();
        let document_size = source.len();

        // Check document size
        if document_size > self.config.max_document_size {
            return ValidationResult::failed(
                AispError::DocumentTooLarge {
                    size: document_size,
                    max: self.config.max_document_size,
                },
                document_size,
            );
        }

        // Check for empty document
        if source.trim().is_empty() {
            return ValidationResult::failed(
                AispError::validation_error("Empty document"),
                document_size,
            );
        }

        // Parse document
        let parse_start = Instant::now();
        let mut parser = AispParser::new(source.to_string());
        let document = match parser.parse() {
            Ok(doc) => doc,
            Err(error) => {
                return ValidationResult::failed(error, document_size);
            }
        };
        let parse_time = parse_start.elapsed();

        // Collect parser warnings
        let mut all_warnings = parser.warnings().to_vec();

        // Check AISP version compatibility
        if document.header.version != AISP_VERSION {
            all_warnings.push(AispWarning::warning(
                format!(
                    "Document version {} may not be fully compatible with validator version {}",
                    document.header.version, AISP_VERSION
                ),
            ));
        }

        // Semantic analysis
        let semantic_start = Instant::now();
        let mut analyzer = SemanticAnalyzer::new();
        let mut analysis = match analyzer.analyze(&document, source) {
            Ok(analysis) => analysis,
            Err(error) => {
                return ValidationResult::failed(error, document_size);
            }
        };
        let semantic_time = semantic_start.elapsed();

        // Merge warnings
        all_warnings.extend(analysis.warnings.clone());
        analysis.warnings = all_warnings;

        // Apply strict mode checks
        if self.config.strict_mode {
            self.apply_strict_checks(&mut analysis);
        }

        // Perform formal verification if enabled  
        let formal_verification = if self.config.enable_formal_verification {
            match self.perform_formal_verification(&document, &analysis) {
                Ok(verification_result) => Some(verification_result),
                Err(err) => {
                    // Add warning to analysis warnings since all_warnings is already merged
                    analysis.warnings.push(AispWarning::warning(
                        format!("Formal verification failed: {}", err)
                    ));
                    None
                }
            }
        } else {
            None
        };

        // Perform tri-vector validation if enabled
        let trivector_validation = if self.config.enable_trivector_validation {
            match self.perform_trivector_validation(&document) {
                Ok(trivector_result) => Some(trivector_result),
                Err(err) => {
                    // Add warning for tri-vector validation failure
                    analysis.warnings.push(AispWarning::warning(
                        format!("Tri-vector validation failed: {}", err)
                    ));
                    None
                }
            }
        } else {
            None
        };

        // Perform enhanced Z3 verification if enabled
        let enhanced_z3_verification = if self.config.enable_enhanced_z3 {
            match self.perform_enhanced_z3_verification(&document, trivector_validation.as_ref()) {
                Ok(z3_result) => Some(z3_result),
                Err(err) => {
                    // Formal verification failure should cause validation to fail, not just warn
                    if self.config.strict_formal_verification {
                        return ValidationResult::failed(
                            AispError::validation_error(
                                format!("Enhanced Z3 verification failed: {}. Enable 'strict_formal_verification: false' to downgrade to warnings.", err)
                            ),
                            document_size,
                        );
                    } else {
                        // Add warning only if not in strict mode
                        analysis.warnings.push(AispWarning::warning(
                            format!("Enhanced Z3 verification failed: {}", err)
                        ));
                        None
                    }
                }
            }
        } else {
            None
        };

        // Perform ghost intent validation if enabled
        let ghost_intent_validation = if self.config.enable_ghost_intent_validation {
            match self.perform_ghost_intent_validation(&document) {
                Ok(ghost_result) => Some(ghost_result),
                Err(err) => {
                    // Formal verification failure should cause validation to fail, not just warn
                    if self.config.strict_formal_verification {
                        return ValidationResult::failed(
                            AispError::validation_error(
                                format!("Ghost intent validation failed: {}. Enable 'strict_formal_verification: false' to downgrade to warnings.", err)
                            ),
                            document_size,
                        );
                    } else {
                        // Add warning only if not in strict mode
                        analysis.warnings.push(AispWarning::warning(
                            format!("Ghost intent validation failed: {}", err)
                        ));
                        None
                    }
                }
            }
        } else {
            None
        };

        // Perform RossNet scoring validation if enabled
        let rossnet_validation = if self.config.enable_rossnet_scoring {
            match self.perform_rossnet_validation(&document, &analysis) {
                Ok(rossnet_result) => Some(rossnet_result),
                Err(err) => {
                    // Formal verification failure should cause validation to fail, not just warn
                    if self.config.strict_formal_verification {
                        return ValidationResult::failed(
                            AispError::validation_error(
                                format!("RossNet scoring validation failed: {}. Enable 'strict_formal_verification: false' to downgrade to warnings.", err)
                            ),
                            document_size,
                        );
                    } else {
                        // Add warning only if not in strict mode
                        analysis.warnings.push(AispWarning::warning(
                            format!("RossNet scoring validation failed: {}", err)
                        ));
                        None
                    }
                }
            }
        } else {
            None
        };

        // Perform Hebbian learning validation if enabled
        let hebbian_validation = if self.config.enable_hebbian_learning {
            match self.perform_hebbian_validation(&document, &analysis) {
                Ok(hebbian_result) => Some(hebbian_result),
                Err(err) => {
                    // Formal verification failure should cause validation to fail, not just warn
                    if self.config.strict_formal_verification {
                        return ValidationResult::failed(
                            AispError::validation_error(
                                format!("Hebbian learning validation failed: {}. Enable 'strict_formal_verification: false' to downgrade to warnings.", err)
                            ),
                            document_size,
                        );
                    } else {
                        // Add warning only if not in strict mode
                        analysis.warnings.push(AispWarning::warning(
                            format!("Hebbian learning validation failed: {}", err)
                        ));
                        None
                    }
                }
            }
        } else {
            None
        };

        // Perform anti-drift protocol validation if enabled
        let anti_drift_validation = if self.config.enable_anti_drift {
            match self.perform_anti_drift_validation(&document, &analysis) {
                Ok(anti_drift_result) => Some(anti_drift_result),
                Err(err) => {
                    // Formal verification failure should cause validation to fail, not just warn
                    if self.config.strict_formal_verification {
                        return ValidationResult::failed(
                            AispError::validation_error(
                                format!("Anti-drift protocol validation failed: {}. Enable 'strict_formal_verification: false' to downgrade to warnings.", err)
                            ),
                            document_size,
                        );
                    } else {
                        // Add warning only if not in strict mode
                        analysis.warnings.push(AispWarning::warning(
                            format!("Anti-drift protocol validation failed: {}", err)
                        ));
                        None
                    }
                }
            }
        } else {
            None
        };

        // Create result
        let ast = if self.config.include_ast {
            Some(document)
        } else {
            None
        };

        let mut result = ValidationResult::success(
            analysis,
            document_size,
            parse_time,
            semantic_time,
            ast,
            formal_verification,
            trivector_validation,
            enhanced_z3_verification,
            ghost_intent_validation,
            rossnet_validation,
            hebbian_validation,
            anti_drift_validation,
        );

        // Apply timing configuration
        if !self.config.include_timing {
            result.parse_time = None;
            result.semantic_time = None;
            result.total_time = None;
        }

        // Apply symbol stats configuration
        if !self.config.include_symbol_stats {
            if let Some(ref mut analysis) = result.semantic_analysis {
                // Clear detailed symbol statistics if not requested
                analysis.symbol_stats.category_counts.clear();
            }
        }

        result
    }

    /// Validate AISP document from file
    pub fn validate_file(&self, file_path: &str) -> ValidationResult {
        match std::fs::read_to_string(file_path) {
            Ok(content) => self.validate(&content),
            Err(error) => ValidationResult::failed(
                AispError::IoError {
                    message: format!("Failed to read file {}: {}", file_path, error),
                },
                0,
            ),
        }
    }

    /// Quick validation check (returns only validity)
    pub fn is_valid(&self, source: &str) -> bool {
        self.validate(source).valid
    }

    /// Get document quality tier
    pub fn get_tier(&self, source: &str) -> QualityTier {
        self.validate(source).tier
    }

    /// Get semantic density
    pub fn get_delta(&self, source: &str) -> f64 {
        self.validate(source).delta
    }

    /// Parse document without full validation (for syntax checking)
    pub fn parse_only(&self, source: &str) -> AispResult<AispDocument> {
        if source.len() > self.config.max_document_size {
            return Err(AispError::DocumentTooLarge {
                size: source.len(),
                max: self.config.max_document_size,
            });
        }

        let mut parser = AispParser::new(source.to_string());
        parser.parse()
    }

    /// Get validator configuration
    pub fn config(&self) -> &ValidationConfig {
        &self.config
    }

    fn apply_strict_checks(&self, analysis: &mut SemanticAnalysis) {
        // Apply additional strict mode validations
        
        // Require higher quality threshold in strict mode
        if analysis.delta < 0.6 {
            analysis.warnings.push(AispWarning::warning(
                "Strict mode: Semantic density below recommended threshold (0.6)"
            ));
        }

        // Require very low ambiguity in strict mode
        if analysis.ambiguity > 0.01 {
            analysis.warnings.push(AispWarning::warning(
                "Strict mode: Ambiguity above strict threshold (0.01)"
            ));
            analysis.valid = false;
        }

        // Check for undefined types
        if !analysis.type_analysis.undefined_types.is_empty() {
            analysis.warnings.push(AispWarning::error(
                format!(
                    "Strict mode: Undefined types detected: {:?}",
                    analysis.type_analysis.undefined_types
                )
            ));
            analysis.valid = false;
        }
    }

    /// Perform formal verification using Z3
    fn perform_formal_verification(
        &self,
        document: &AispDocument,
        analysis: &SemanticAnalysis,
    ) -> AispResult<FormalVerificationResult> {
        let mut z3_verifier = Z3Verifier::new()?;
        z3_verifier.set_timeout(self.config.z3_timeout);

        // Extract analysis components
        let relational_analysis = analysis.relational_analysis.as_ref();
        let temporal_analysis = analysis.temporal_analysis.as_ref();

        // Perform verification
        z3_verifier.verify_document(document, relational_analysis, temporal_analysis)
    }

    /// Perform tri-vector signal validation
    fn perform_trivector_validation(
        &self,
        document: &AispDocument,
    ) -> AispResult<TriVectorValidationResult> {
        let mut trivector_validator = TriVectorValidator::with_config(
            TriVectorValidationConfig {
                require_formal_proofs: self.config.strict_mode,
                orthogonality_tolerance: 1e-10,
                verify_safety_isolation: true,
                z3_timeout_ms: self.config.z3_timeout.as_millis() as u64,
                max_dimension: 2048,
            }
        );

        trivector_validator.validate_document(document)
    }

    /// Perform enhanced Z3 verification
    fn perform_enhanced_z3_verification(
        &self,
        document: &AispDocument,
        trivector_result: Option<&TriVectorValidationResult>,
    ) -> AispResult<EnhancedVerificationResult> {
        let mut z3_facade = Z3VerificationFacade::new()?;
        z3_facade.verify_document(document, trivector_result)
    }

    /// Perform ghost intent search validation
    fn perform_ghost_intent_validation(
        &self,
        document: &AispDocument,
    ) -> AispResult<GhostIntentValidationResult> {
        let config = GhostIntentConfig {
            min_confidence_threshold: 0.6,
            max_analysis_time: self.config.z3_timeout,
            enable_formal_verification: self.config.enable_formal_verification,
            z3_timeout_ms: (self.config.z3_timeout.as_millis() as u32).min(30000),
        };
        
        let mut validator = GhostIntentValidator::new(config);
        validator.validate_ghost_intents(document)
    }

    /// Perform RossNet scoring validation
    fn perform_rossnet_validation(
        &self,
        document: &AispDocument,
        analysis: &SemanticAnalysis,
    ) -> AispResult<RossNetValidationResult> {
        let config = RossNetConfig {
            min_rossnet_score: if self.config.strict_mode { 0.8 } else { 0.7 },
            max_analysis_time: Duration::from_secs(10),
            enable_caching: true,
            similarity_weight: 0.4,
            fitness_weight: 0.35,
            affinity_weight: 0.25,
            reference_document: None,
        };
        
        let semantic_result = SemanticAnalysisResult {
            delta: analysis.delta,
            ambiguity: analysis.ambiguity,
            completeness: analysis.completeness,
            tier: analysis.tier.clone(),
            quality_score: analysis.quality_score,
            validation_errors: analysis.errors.iter().map(|e| e.to_string()).collect(),
            warnings: analysis.warnings.iter().map(|w| w.to_string()).collect(),
        };
        
        let mut validator = RossNetValidator::new(config);
        validator.validate_rossnet_scoring(document, &semantic_result)
    }

    /// Perform Hebbian learning constraint validation
    fn perform_hebbian_validation(
        &self,
        document: &AispDocument,
        analysis: &SemanticAnalysis,
    ) -> AispResult<HebbianValidationResult> {
        let config = HebbianConfig {
            target_penalty_ratio: 10.0,
            penalty_ratio_tolerance: if self.config.strict_mode { 0.5 } else { 1.0 },
            min_learning_rate: 0.001,
            max_learning_rate: 0.1,
            max_weight_update: 1.0,
            min_temporal_consistency: if self.config.strict_mode { 0.9 } else { 0.8 },
            max_analysis_time: Duration::from_secs(5),
            enable_plasticity_analysis: true,
        };
        
        let semantic_result = SemanticAnalysisResult {
            delta: analysis.delta,
            ambiguity: analysis.ambiguity,
            completeness: analysis.completeness,
            tier: analysis.tier.clone(),
            quality_score: analysis.quality_score,
            validation_errors: analysis.errors.iter().map(|e| e.to_string()).collect(),
            warnings: analysis.warnings.iter().map(|w| w.to_string()).collect(),
        };
        
        let mut validator = HebbianValidator::new(config);
        validator.validate_hebbian_learning(document, &semantic_result)
    }

    /// Perform anti-drift protocol validation
    fn perform_anti_drift_validation(
        &self,
        document: &AispDocument,
        analysis: &SemanticAnalysis,
    ) -> AispResult<AntiDriftValidationResult> {
        let config = AntiDriftConfig {
            max_drift_velocity: if self.config.strict_mode { 0.05 } else { 0.1 },
            severity_threshold: if self.config.strict_mode { 0.2 } else { 0.3 },
            min_stability_score: if self.config.strict_mode { 0.9 } else { 0.8 },
            analysis_time_window: Duration::from_secs(3600),
            max_analysis_time: Duration::from_secs(10),
            enable_auto_correction: true,
            reference_baseline: None,
        };
        
        let semantic_result = SemanticAnalysisResult {
            delta: analysis.delta,
            ambiguity: analysis.ambiguity,
            completeness: analysis.completeness,
            tier: analysis.tier.clone(),
            quality_score: analysis.quality_score,
            validation_errors: analysis.errors.iter().map(|e| e.to_string()).collect(),
            warnings: analysis.warnings.iter().map(|w| w.to_string()).collect(),
        };
        
        let mut validator = AntiDriftValidator::new(config);
        validator.validate_anti_drift(document, &semantic_result)
    }
}

impl Default for AispValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function for quick validation
pub fn validate(source: &str) -> ValidationResult {
    AispValidator::new().validate(source)
}

/// Convenience function for quick validity check
pub fn is_valid(source: &str) -> bool {
    AispValidator::new().is_valid(source)
}

/// Convenience function to get document tier
pub fn get_tier(source: &str) -> QualityTier {
    AispValidator::new().get_tier(source)
}

/// Convenience function to get semantic density
pub fn get_delta(source: &str) -> f64 {
    AispValidator::new().get_delta(source)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DOCUMENT: &str = r#"
𝔸5.1.test@2026-01-25

⟦Ω:Meta⟧{
  domain≜test
  version≜1.0.0
  ∀D∈AISP:Ambig(D)<0.02
}

⟦Σ:Types⟧{
  State≜{Start,Active,End}
  Event≜{Begin,Process,Finish}
}

⟦Γ:Rules⟧{
  ∀s:State→NextState(s)
  ∀e:Event⇒StateTransition
}

⟦Λ:Funcs⟧{
  transition≜λ(s,e).NextState(s,e)
}

⟦Ε⟧⟨δ≜0.85;φ≜100;τ≜◊⁺⟩
    "#;

    #[test]
    fn test_basic_validation() {
        let validator = AispValidator::new();
        let result = validator.validate(TEST_DOCUMENT);
        
        assert!(result.valid, "Validation should succeed: {:?}", result.error);
        assert_ne!(result.tier, QualityTier::Reject);
        assert!(result.delta > 0.0);
        assert!(result.ambiguity < 1.0);
    }

    #[test]
    fn test_invalid_document() {
        let validator = AispValidator::new();
        let result = validator.validate("invalid content");
        
        assert!(!result.valid);
        assert_eq!(result.tier, QualityTier::Reject);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_document_too_large() {
        let mut config = ValidationConfig::default();
        config.max_document_size = 100; // Very small limit
        
        let validator = AispValidator::with_config(config);
        let result = validator.validate(TEST_DOCUMENT);
        
        assert!(!result.valid);
        assert!(matches!(
            result.error.unwrap(),
            AispError::DocumentTooLarge { .. }
        ));
    }

    #[test]
    fn test_strict_mode() {
        let mut config = ValidationConfig::default();
        config.strict_mode = true;
        
        let validator = AispValidator::with_config(config);
        let result = validator.validate(TEST_DOCUMENT);
        
        // Should still be valid but may have additional warnings
        assert!(!result.warnings.is_empty());
    }

    #[test]
    fn test_timing_configuration() {
        let mut config = ValidationConfig::default();
        config.include_timing = true;
        
        let validator = AispValidator::with_config(config);
        let result = validator.validate(TEST_DOCUMENT);
        
        assert!(result.parse_time.is_some());
        assert!(result.semantic_time.is_some());
        assert!(result.total_time.is_some());
    }

    #[test]
    fn test_convenience_functions() {
        assert!(is_valid(TEST_DOCUMENT));
        assert_ne!(get_tier(TEST_DOCUMENT), QualityTier::Reject);
        assert!(get_delta(TEST_DOCUMENT) > 0.0);
    }

    #[test]
    fn test_empty_document() {
        let validator = AispValidator::new();
        let result = validator.validate("");
        
        assert!(!result.valid);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_parse_only() {
        let validator = AispValidator::new();
        let doc = validator.parse_only(TEST_DOCUMENT);
        
        assert!(doc.is_ok());
        let doc = doc.unwrap();
        assert_eq!(doc.header.name, "test");
        assert_eq!(doc.blocks.len(), 5);
    }
}