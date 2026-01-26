//! Main AISP validation engine
//! 
//! Provides the primary API for validating AISP documents with
//! comprehensive error handling and performance optimizations.

use crate::ast::*;
use crate::error::*;
use crate::parser_new::*;
use crate::semantic::*;
use crate::z3_integration::*;
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