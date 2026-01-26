//! Integration tests for tri-vector signal validation
//!
//! Tests the complete tri-vector validation pipeline including orthogonality
//! verification, safety isolation, and formal proof generation.

use aisp_core::{
    validator::{AispValidator, ValidationConfig},
    semantic::QualityTier,
    tri_vector_validation::{
        TriVectorValidator, TriVectorValidationConfig, OrthogonalityType,
        VerificationMethod, SafetyViolationType,
    },
};

/// Valid AISP document with proper tri-vector signal definition
const VALID_TRIVECTOR_DOCUMENT: &str = r#"
𝔸5.1.trivector-test@2026-01-26
γ≔trivector.validation.test
ρ≔⟨validation,trivector,orthogonality⟩
⊢ND∧CAT∧ΠΣ

⟦Ω:Meta⟧{
  ∀D∈AISP:Ambig(D)<0.02
  domain≜trivector-validation
  protocol≜"tri-vector-signal-test"
  Vision≜"Validate tri-vector signal decomposition with orthogonality constraints"
}

⟦Σ:Types⟧{
  ;; Core tri-vector signal definition
  Signal≜V_H⊕V_L⊕V_S
  V_H≜ℝ⁷⁶⁸:semantic
  V_L≜ℝ⁵¹²:structural
  V_S≜ℝ²⁵⁶:safety
  
  ;; Binding states for compatibility testing
  BindState≜{⊥:0:crash,∅:1:null,λ:2:adapt,⊤:3:zero-cost}
  
  ;; Additional types for completeness
  Pocket≜⟨ℋ:Header,ℳ:Membrane,𝒩:Nucleus⟩
}

⟦Γ:Rules⟧{
  ;; Core tri-vector orthogonality constraints
  V_H∩V_S≡∅
  V_L∩V_S≡∅
  V_H∩V_L≢∅
  
  ;; Signal decomposition uniqueness
  ∀signal:Signal→∃!(v_h,v_l,v_s):[v_h∈V_H∧v_l∈V_L∧v_s∈V_S∧signal≡v_h⊕v_l⊕v_s]
  
  ;; Safety isolation invariants
  ∀optimization∈SemanticOpt:¬affects(optimization,V_S)
  ∀modification∈StructuralMod:¬affects(modification,V_S)
  
  ;; Vector space axioms
  ∀V∈{V_H,V_L,V_S}:VectorSpace(V)
  ∀v1,v2∈V_S,v3∈V_H∪V_L:⟨v1,v2⟩∈ℝ∧⟨v1,v3⟩≡0
}

⟦Λ:Functions⟧{
  ;; Signal decomposition function
  decompose≜λs:Signal.⟨project_H(s),project_L(s),project_S(s)⟩
  project_H≜λs.π_H(s)
  project_L≜λs.π_L(s) 
  project_S≜λs.π_S(s)
  
  ;; Orthogonality verification
  orthogonal≜λ(V1,V2).∀v1∈V1,v2∈V2:⟨v1,v2⟩≡0
  verify_orthogonal≜λ(v1,v2).dot_product(v1,v2)≡0
  
  ;; Safety isolation verification
  safety_isolated≜λs:Signal.orthogonal(project_S(s),project_H(s)∪project_L(s))
}

⟦Ε⟧⟨
δ≜0.82
|𝔅|≜5/5
φ≜156
τ≜◊⁺⁺
⊢ND:tri_vector_orthogonality_proven
⊢CAT:signal_decomposition_functor
⊢ΠΣ:dependent_vector_types_checked
⊢𝕃:𝕃₀→𝕃₁→𝕃₂
⊢TRI:V_H⊥V_S∧V_L⊥V_S
⊢SAFETY:safety_isolation_verified
⊢Ambig(D)<0.02
⟩
"#;

/// Invalid document with orthogonality violations
const INVALID_ORTHOGONALITY_DOCUMENT: &str = r#"
𝔸5.1.invalid-trivector@2026-01-26

⟦Ω:Meta⟧{
  domain≜invalid-orthogonality-test
}

⟦Σ:Types⟧{
  ;; Incorrect tri-vector definition - wrong dimensions
  Signal≜V_H⊕V_L⊕V_S
  V_H≜ℝ⁵¹²:semantic  ;; Should be ℝ⁷⁶⁸
  V_L≜ℝ⁷⁶⁸:structural ;; Should be ℝ⁵¹²
  V_S≜ℝ²⁵⁶:safety     ;; Correct
}

⟦Γ:Rules⟧{
  ;; Violates orthogonality - incorrectly claims overlap
  V_H∩V_S≢∅  ;; Should be V_H∩V_S≡∅
  V_L∩V_S≢∅  ;; Should be V_L∩V_S≡∅
}

⟦Λ:Functions⟧{
  id≜λx.x
}

⟦Ε⟧⟨δ≜0.25;φ≜45⟩
"#;

/// Document missing tri-vector definition entirely
const MISSING_TRIVECTOR_DOCUMENT: &str = r#"
𝔸5.1.no-trivector@2026-01-26

⟦Ω:Meta⟧{
  domain≜missing-trivector-test
}

⟦Σ:Types⟧{
  ;; No tri-vector signal definition
  BasicType≜{unit,bool}
}

⟦Γ:Rules⟧{
  ∀x:BasicType→Valid(x)
}

⟦Λ:Functions⟧{
  identity≜λx.x
}

⟦Ε⟧⟨δ≜0.6;φ≜78⟩
"#;

#[test]
fn test_valid_trivector_document_validation() {
    let mut config = ValidationConfig::default();
    config.enable_trivector_validation = true;
    config.strict_mode = true;
    
    let validator = AispValidator::with_config(config);
    let result = validator.validate(VALID_TRIVECTOR_DOCUMENT);
    
    // Should pass basic validation
    assert!(result.valid, "Document should be valid: {:?}", result.error);
    assert!(result.tier != QualityTier::Reject, "Should not be rejected");
    
    // Should have tri-vector validation results
    assert!(result.trivector_validation.is_some(), "Should have tri-vector validation results");
    
    let trivector_result = result.trivector_validation.unwrap();
    assert!(trivector_result.valid, "Tri-vector validation should pass");
    
    // Should detect tri-vector signal
    assert!(trivector_result.signal.is_some(), "Should detect tri-vector signal");
    let signal = trivector_result.signal.unwrap();
    
    assert_eq!(signal.semantic.dimension, 768);
    assert_eq!(signal.structural.dimension, 512);
    assert_eq!(signal.safety.dimension, 256);
    
    // Should verify orthogonality constraints
    assert!(!trivector_result.orthogonality_results.is_empty(), "Should have orthogonality results");
    
    // Check specific orthogonality results
    if let Some(vh_vs_result) = trivector_result.orthogonality_results.get("V_H ⊥ V_S") {
        assert_eq!(vh_vs_result.orthogonality_type, OrthogonalityType::CompletelyOrthogonal);
        assert!(vh_vs_result.confidence > 0.9);
    }
    
    if let Some(vl_vs_result) = trivector_result.orthogonality_results.get("V_L ⊥ V_S") {
        assert_eq!(vl_vs_result.orthogonality_type, OrthogonalityType::CompletelyOrthogonal);
        assert!(vl_vs_result.confidence > 0.9);
    }
    
    // Should verify safety isolation
    assert!(trivector_result.safety_isolation.isolated, "Safety should be isolated");
    assert!(trivector_result.safety_isolation.violations.is_empty(), "Should have no safety violations");
    
    // Should generate proof certificates
    assert!(!trivector_result.proof_certificates.is_empty(), "Should have proof certificates");
    
    let has_orthogonality_cert = trivector_result.proof_certificates.iter()
        .any(|cert| cert.id == "tri-vector-orthogonality");
    assert!(has_orthogonality_cert, "Should have orthogonality proof certificate");
    
    let has_safety_cert = trivector_result.proof_certificates.iter()
        .any(|cert| cert.id == "safety-isolation");
    assert!(has_safety_cert, "Should have safety isolation certificate");
}

#[test]
fn test_invalid_orthogonality_document() {
    let mut config = ValidationConfig::default();
    config.enable_trivector_validation = true;
    config.strict_mode = true;
    
    let validator = AispValidator::with_config(config);
    let result = validator.validate(INVALID_ORTHOGONALITY_DOCUMENT);
    
    // Document might pass basic parsing but tri-vector validation should detect issues
    if let Some(trivector_result) = result.trivector_validation {
        // Should detect errors in tri-vector definition
        assert!(!trivector_result.errors.is_empty(), "Should detect tri-vector errors");
        
        // Should detect dimension errors
        let has_dimension_error = trivector_result.errors.iter()
            .any(|err| matches!(err, aisp_core::tri_vector_validation::TriVectorError::InvalidDimension { .. }));
        assert!(has_dimension_error, "Should detect invalid dimensions");
    } else {
        // Should at least generate warnings about tri-vector validation failure
        let has_trivector_warning = result.warnings.iter()
            .any(|warning| warning.message.contains("Tri-vector validation failed"));
        assert!(has_trivector_warning, "Should warn about tri-vector validation failure");
    }
}

#[test]
fn test_missing_trivector_document() {
    let mut config = ValidationConfig::default();
    config.enable_trivector_validation = true;
    
    let validator = AispValidator::with_config(config);
    let result = validator.validate(MISSING_TRIVECTOR_DOCUMENT);
    
    // Should either fail tri-vector validation or generate appropriate warnings
    if let Some(trivector_result) = result.trivector_validation {
        assert!(!trivector_result.valid, "Should fail tri-vector validation");
        assert!(trivector_result.signal.is_none(), "Should not detect tri-vector signal");
        
        let has_missing_space_error = trivector_result.errors.iter()
            .any(|err| matches!(err, aisp_core::tri_vector_validation::TriVectorError::MissingVectorSpace(_)));
        assert!(has_missing_space_error, "Should detect missing vector spaces");
    } else {
        // Should warn about missing tri-vector validation
        let has_warning = result.warnings.iter()
            .any(|w| w.message.contains("Tri-vector") || w.message.contains("tri-vector"));
        assert!(has_warning, "Should warn about tri-vector validation issues");
    }
}

#[test]
fn test_trivector_validator_direct_usage() {
    let mut validator = TriVectorValidator::new();
    
    // This would require creating a proper AispDocument from the test strings
    // For now, test validator configuration
    assert!(validator.config.require_formal_proofs);
    assert_eq!(validator.config.orthogonality_tolerance, 1e-10);
    assert!(validator.config.verify_safety_isolation);
    assert_eq!(validator.config.z3_timeout_ms, 30000);
    assert_eq!(validator.config.max_dimension, 2048);
}

#[test]
fn test_trivector_validation_config_customization() {
    let custom_config = TriVectorValidationConfig {
        require_formal_proofs: false,
        orthogonality_tolerance: 1e-8,
        verify_safety_isolation: false,
        z3_timeout_ms: 15000,
        max_dimension: 1024,
    };
    
    let validator = TriVectorValidator::with_config(custom_config);
    
    assert!(!validator.config.require_formal_proofs);
    assert_eq!(validator.config.orthogonality_tolerance, 1e-8);
    assert!(!validator.config.verify_safety_isolation);
    assert_eq!(validator.config.z3_timeout_ms, 15000);
    assert_eq!(validator.config.max_dimension, 1024);
}

#[test]
fn test_integration_with_main_validation_pipeline() {
    // Test that tri-vector validation integrates properly with main validation
    let mut config = ValidationConfig::default();
    config.enable_trivector_validation = true;
    config.strict_mode = false;
    config.include_timing = true;
    
    let validator = AispValidator::with_config(config);
    let result = validator.validate(VALID_TRIVECTOR_DOCUMENT);
    
    // Should have timing information
    assert!(result.parse_time.is_some());
    assert!(result.semantic_time.is_some());
    assert!(result.total_time.is_some());
    
    // Should have semantic analysis
    assert!(result.semantic_analysis.is_some());
    
    // Should have delta calculation
    assert!(result.delta > 0.0);
    assert!(result.delta <= 1.0);
    
    // Should have tier assignment
    assert_ne!(result.tier, QualityTier::Reject);
    assert!(!result.tier_symbol.is_empty());
    assert!(!result.tier_name.is_empty());
    
    // Should have tri-vector validation results when enabled
    assert!(result.trivector_validation.is_some());
}

#[test]
fn test_trivector_disabled_validation() {
    // Test validation with tri-vector disabled
    let mut config = ValidationConfig::default();
    config.enable_trivector_validation = false;
    
    let validator = AispValidator::with_config(config);
    let result = validator.validate(VALID_TRIVECTOR_DOCUMENT);
    
    // Should still validate successfully but without tri-vector results
    assert!(result.valid);
    assert!(result.trivector_validation.is_none(), "Should not have tri-vector results when disabled");
}