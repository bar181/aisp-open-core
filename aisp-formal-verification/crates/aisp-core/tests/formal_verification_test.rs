//! Comprehensive tests for the formal verification implementation
//!
//! This test suite validates that the formal verification system
//! actually generates mathematical proofs and validates logical properties.

use aisp_core::{
    ast::*,
    error::AispResult,
    formal_verification::*,
    property_types::*,
    validator::AispValidator,
    semantic::SemanticAnalyzer,
};
use std::collections::{HashMap, HashSet};

/// Test basic formal verification functionality
#[test]
fn test_formal_verification_basic() {
    let mut verifier = FormalVerifier::new();
    
    // Create a simple property: True ∧ True
    let simple_property = PropertyFormula {
        structure: FormulaStructure::Conjunction(vec![
            FormulaStructure::Atomic(AtomicFormula {
                predicate: "True".to_string(),
                terms: vec![],
                type_signature: None,
            }),
            FormulaStructure::Atomic(AtomicFormula {
                predicate: "True".to_string(),
                terms: vec![],
                type_signature: None,
            }),
        ]),
        quantifiers: vec![],
        free_variables: HashSet::new(),
        predicates: {
            let mut set = HashSet::new();
            set.insert("True".to_string());
            set
        },
        functions: HashSet::new(),
        constants: HashSet::new(),
    };
    
    let result = verifier.verify_property(&simple_property);
    assert!(result.is_ok(), "Simple conjunction should be verifiable: {:?}", result.err());
    
    let proof = result.unwrap();
    assert!(!proof.id.is_empty(), "Proof should have an ID");
    assert!(!proof.proof_steps.is_empty(), "Proof should have actual steps");
    assert_eq!(proof.validation, ProofValidation::Valid, "Proof should be valid");
}

/// Test ambiguity calculation with various document types
#[test] 
fn test_ambiguity_measurement() {
    let validator = AispValidator::new();
    
    // High-ambiguity document (mostly natural language)
    let ambiguous_doc = r#"
    𝔸5.1.Test@2026-01-26
    ⟦Ω:Meta⟧{
      description≜"this is mostly english text with few symbols"
    }
    "#;
    
    let result = validator.validate(ambiguous_doc).unwrap();
    assert!(result.ambiguity > 0.1, "Ambiguous document should have high ambiguity: {}", result.ambiguity);
    
    // Low-ambiguity document (formal AISP)
    let formal_doc = r#"
    𝔸5.1.Formal@2026-01-26
    ⟦Σ:Types⟧{
      State≜{Start,End}
      Event≜ℕ
    }
    ⟦Γ:Rules⟧{
      ∀s:State→Valid(s)
    }
    "#;
    
    let result = validator.validate(formal_doc);
    assert!(result.ambiguity < 0.05, "Formal document should have low ambiguity: {}", result.ambiguity);
    assert!(result.ambiguity < 0.02, "Should satisfy AISP ambiguity invariant: {}", result.ambiguity);
}

/// Test enumeration parsing fix
#[test]
fn test_enumeration_parsing() {
    let validator = AispValidator::new();
    
    // Test space-separated enumeration (previously failed)
    let doc = r#"
    𝔸5.1.EnumTest@2026-01-26
    ⟦Σ:Types⟧{
      GameState≜{Start Playing GameOver}
      Player≜{PlayerA PlayerB}
    }
    "#;
    
    let result = validator.validate(doc);
    assert!(result.is_ok(), "Space-separated enumerations should parse: {:?}", result.err());
    
    let validation = result.unwrap();
    assert!(validation.valid, "Enumeration document should be valid");
    
    // Test comma-separated enumeration (should still work)  
    let doc2 = r#"
    𝔸5.1.EnumTest2@2026-01-26
    ⟦Σ:Types⟧{
      Status≜{Active, Inactive, Pending}
    }
    "#;
    
    let result2 = validator.validate(doc2);
    assert!(result2.is_ok(), "Comma-separated enumerations should parse: {:?}", result2.err());
}

/// Test theorem prover with actual logical reasoning
#[test]
fn test_theorem_prover_logic() {
    use aisp_core::theorem_prover::TheoremProver;
    
    let mut prover = TheoremProver::new();
    
    // Test simple implication: P → P
    let tautology = PropertyFormula {
        structure: FormulaStructure::Implication(
            Box::new(FormulaStructure::Atomic(AtomicFormula {
                predicate: "P".to_string(),
                terms: vec![Term::Variable("x".to_string(), None)],
                type_signature: None,
            })),
            Box::new(FormulaStructure::Atomic(AtomicFormula {
                predicate: "P".to_string(),
                terms: vec![Term::Variable("x".to_string(), None)],
                type_signature: None,
            }))
        ),
        quantifiers: vec![],
        free_variables: {
            let mut set = HashSet::new();
            set.insert("x".to_string());
            set
        },
        predicates: {
            let mut set = HashSet::new();
            set.insert("P".to_string());
            set
        },
        functions: HashSet::new(),
        constants: HashSet::new(),
    };
    
    let result = prover.prove_formula(&tautology);
    assert!(result.is_ok(), "Simple tautology P → P should be provable: {:?}", result.err());
}

/// Test quality assessment improvements
#[test]
fn test_quality_assessment() {
    let validator = AispValidator::new();
    
    // Well-formed AISP document should get high quality score
    let well_formed = r#"
    𝔸5.1.WellFormed@2026-01-26
    γ≔⟨formal,verified⟩
    ⟦Ω:Meta⟧{
      ∀D∈AISP:Ambig(D)<0.02
    }
    ⟦Σ:Types⟧{
      State≜{Initial,Processing,Final}
      Transition≜State×State
    }
    ⟦Γ:Rules⟧{
      ∀s₁,s₂:State→Valid(Transition(s₁,s₂))
      □(Initial→◊Final)
    }
    "#;
    
    let result = validator.validate(well_formed);
    assert!(result.delta > 0.8, "Well-formed document should have high delta: {}", result.delta);
    assert!(result.tier.value() >= 3, "Should achieve high quality tier: {:?}", result.tier);
    assert!(result.is_acceptable(), "Should be acceptable quality");
}

/// Test property extraction and verification pipeline
#[test]
fn test_property_extraction_pipeline() {
    let validator = AispValidator::new();
    
    let doc_with_properties = r#"
    𝔸5.1.Properties@2026-01-26
    ⟦Γ:Rules⟧{
      ∀x:Natural→x≥0
      ∀f:Function→Deterministic(f)
    }
    ⟦Ε⟧⟨δ≜0.9;φ≜99⟩
    "#;
    
    let result = validator.validate(doc_with_properties);
    
    // Should extract and attempt verification of universal properties
    assert!(result.valid || !result.error.unwrap_or_default().is_empty(), 
           "Should either succeed or provide meaningful error");
           
    // Quality should reflect formal properties
    assert!(result.delta > 0.5, "Document with formal properties should have decent delta");
}

/// Test error handling in formal verification
#[test]
fn test_verification_error_handling() {
    let mut verifier = FormalVerifier::new();
    
    // Invalid property structure
    let invalid_property = PropertyFormula {
        structure: FormulaStructure::Conjunction(vec![]), // Empty conjunction - invalid
        quantifiers: vec![],
        free_variables: HashSet::new(),
        predicates: HashSet::new(), 
        functions: HashSet::new(),
        constants: HashSet::new(),
    };
    
    let result = verifier.verify_property(&invalid_property);
    assert!(result.is_err(), "Invalid property should fail verification");
    
    match result {
        Err(aisp_core::error::AispError::VerificationFailed(msg)) => {
            assert!(msg.contains("Conjunction must have at least 2 conjuncts"));
        }
        _ => panic!("Expected VerificationFailed error"),
    }
}

/// Integration test: end-to-end formal verification
#[test]
fn test_end_to_end_formal_verification() {
    let validator = AispValidator::new();
    
    let complete_doc = r#"
    𝔸5.1.Complete@2026-01-26
    γ≔⟨system,verified⟩
    ρ≔⟨protocol,state-machine⟩
    
    ⟦Ω:Meta⟧{
      domain≜formal_systems
      ∀D∈AISP:Ambig(D)<0.02
      ∀P∈Properties:Verifiable(P)
    }
    
    ⟦Σ:Types⟧{
      State≜{Idle Active Error}
      Event≜{Start Stop Reset}
      Machine≜State×Event→State
    }
    
    ⟦Γ:Rules⟧{
      ∀s:State→Reachable(s)
      ∀e:Event→Handled(e)
      □(Active→◊Idle)
      ◊□Stable(Machine)
    }
    
    ⟦Λ:Funcs⟧{
      transition≜λ(s,e).NextState(s,e)
      isValid≜λm.WellFormed(m)
    }
    
    ⟦Ε⟧⟨δ≜0.95;φ≜100;τ≜◊⁺⁺⟩
    "#;
    
    let result = validator.validate(complete_doc);
    assert!(result.is_ok(), "Complete formal document should validate: {:?}", result.err());
    
    let validation = result.unwrap();
    assert!(validation.valid || validation.warnings.len() < 5, 
           "Should be valid or have minimal warnings");
    assert!(validation.ambiguity < 0.02, "Should satisfy AISP ambiguity invariant");
    assert!(validation.delta > 0.9, "Should achieve high semantic density");
}

#[test]
fn test_verification_performance() {
    let mut verifier = FormalVerifier::new();
    let start = std::time::Instant::now();
    
    // Create moderately complex property
    let complex_property = PropertyFormula {
        structure: FormulaStructure::Universal(
            Quantifier {
                variable: "x".to_string(),
                quantifier_type: QuantifierType::Forall,
                domain: Some("Natural".to_string()),
            },
            Box::new(FormulaStructure::Implication(
                Box::new(FormulaStructure::Atomic(AtomicFormula {
                    predicate: "Positive".to_string(),
                    terms: vec![Term::Variable("x".to_string(), Some("Natural".to_string()))],
                    type_signature: None,
                })),
                Box::new(FormulaStructure::Atomic(AtomicFormula {
                    predicate: "Valid".to_string(),
                    terms: vec![Term::Variable("x".to_string(), Some("Natural".to_string()))],
                    type_signature: None,
                }))
            ))
        ),
        quantifiers: vec![
            Quantifier {
                variable: "x".to_string(),
                quantifier_type: QuantifierType::Forall,
                domain: Some("Natural".to_string()),
            }
        ],
        free_variables: HashSet::new(),
        predicates: {
            let mut set = HashSet::new();
            set.insert("Positive".to_string());
            set.insert("Valid".to_string());
            set
        },
        functions: HashSet::new(),
        constants: HashSet::new(),
    };
    
    let _result = verifier.verify_property(&complex_property);
    let duration = start.elapsed();
    
    // Should complete reasonably quickly (< 1 second for this complexity)
    assert!(duration.as_millis() < 1000, "Verification should complete quickly: {}ms", duration.as_millis());
}

/// Test the formal verification demonstration system integration
#[test] 
fn test_formal_verification_demonstration() {
    // This test validates that our implementation matches the claims in ADR-009
    let validator = AispValidator::new();
    
    let demonstration_doc = r#"
    𝔸5.1.Demo@2026-01-26
    γ≔⟨demonstration,formal-methods⟩
    
    ⟦Ω:Meta⟧{
      purpose≜"Demonstrate formal verification capabilities"
      ∀D∈AISP:Ambig(D)<0.02
      ∀P∈Properties:Provable(P)∨Disprovable(P)
    }
    
    ⟦Σ:Types⟧{
      Counter≜ℕ
      State≜{Zero Positive}
    }
    
    ⟦Γ:Rules⟧{
      ∀n:Counter→n≥0
      ∀s:State→Consistent(s)
      Zero⇔(Counter=0)
    }
    "#;
    
    let result = validator.validate(demonstration_doc);
    
    // Validate the formal verification demonstration criteria from ADR-009
    assert!(result.delta > 0.8, "Demo should have high semantic density");
    assert!(result.ambiguity < 0.02, "Demo should satisfy ambiguity invariant");
    assert!(result.tier.value() >= 3, "Demo should achieve high quality tier");
}