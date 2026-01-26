//! Formal verification integration tests (Level 5+)
//!
//! This module tests Z3-based formal verification including SMT solving,
//! property verification, and mathematical proof generation.

use aisp_core::{
    FormalVerifier, AispDocument, AispParser, FormalVerificationResult,
    VerificationResult, ProofResult, Z3Verifier, ValidationLevel,
    AispValidator, SMTFormula, PropertyType
};

/// Builder for creating formal verification test scenarios
pub struct FormalTestBuilder {
    document_source: String,
    expected_properties: usize,
    expected_verified: usize,
    expected_falsified: usize,
    expected_timeout: usize,
    verification_timeout: Option<u64>,
}

impl FormalTestBuilder {
    pub fn new(document_source: &str) -> Self {
        Self {
            document_source: document_source.to_string(),
            expected_properties: 0,
            expected_verified: 0,
            expected_falsified: 0,
            expected_timeout: 0,
            verification_timeout: None,
        }
    }

    pub fn expecting_properties(mut self, count: usize) -> Self {
        self.expected_properties = count;
        self
    }

    pub fn expecting_verified(mut self, count: usize) -> Self {
        self.expected_verified = count;
        self
    }

    pub fn expecting_falsified(mut self, count: usize) -> Self {
        self.expected_falsified = count;
        self
    }

    pub fn expecting_timeout(mut self, count: usize) -> Self {
        self.expected_timeout = count;
        self
    }

    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.verification_timeout = Some(timeout_ms);
        self
    }

    pub fn test_formal_verification(self) -> FormalResult {
        let parser = AispParser::new();
        let document = parser.parse(&self.document_source)
            .expect("Document should parse successfully for formal verification");

        let mut verifier = FormalVerifier::new();
        if let Some(timeout) = self.verification_timeout {
            verifier.set_timeout(timeout);
        }
        
        let result = verifier.verify(&document);

        // Verify property count
        if result.properties.len() != self.expected_properties {
            panic!("Expected {} properties but got {}: {:?}", 
                self.expected_properties, result.properties.len(), 
                result.properties);
        }

        // Count verification results
        let verified_count = result.properties.iter()
            .filter(|p| p.result == VerificationResult::Verified)
            .count();
        let falsified_count = result.properties.iter()
            .filter(|p| p.result == VerificationResult::Falsified)
            .count();
        let timeout_count = result.properties.iter()
            .filter(|p| p.result == VerificationResult::Timeout)
            .count();

        // Verify result counts
        if verified_count != self.expected_verified {
            panic!("Expected {} verified properties but got {}", 
                self.expected_verified, verified_count);
        }
        if falsified_count != self.expected_falsified {
            panic!("Expected {} falsified properties but got {}", 
                self.expected_falsified, falsified_count);
        }
        if timeout_count != self.expected_timeout {
            panic!("Expected {} timeout properties but got {}", 
                self.expected_timeout, timeout_count);
        }

        FormalResult::new(document, result)
    }
}

/// Helper for asserting formal verification results
pub struct FormalResult {
    _document: AispDocument,
    verification: FormalVerificationResult,
}

impl FormalResult {
    pub fn new(document: AispDocument, verification: FormalVerificationResult) -> Self {
        Self { _document: document, verification }
    }

    pub fn has_property(self, property_name: &str, expected_result: VerificationResult) -> Self {
        let property = self.verification.properties.iter()
            .find(|p| p.name.contains(property_name))
            .expect(&format!("Property '{}' not found", property_name));
        
        assert_eq!(property.result, expected_result,
            "Expected property '{}' to have result {:?} but got {:?}", 
            property_name, expected_result, property.result);
        self
    }

    pub fn has_proof(self, property_name: &str) -> Self {
        let property = self.verification.properties.iter()
            .find(|p| p.name.contains(property_name))
            .expect(&format!("Property '{}' not found", property_name));
        
        assert!(property.proof.is_some(),
            "Expected property '{}' to have a proof", property_name);
        self
    }

    pub fn has_counterexample(self, property_name: &str) -> Self {
        let property = self.verification.properties.iter()
            .find(|p| p.name.contains(property_name))
            .expect(&format!("Property '{}' not found", property_name));
        
        assert!(property.counterexample.is_some(),
            "Expected property '{}' to have a counterexample", property_name);
        self
    }

    pub fn has_smt_formula_count(self, count: usize) -> Self {
        assert_eq!(self.verification.smt_formulas.len(), count,
            "Expected {} SMT formulas but got {}", count, self.verification.smt_formulas.len());
        self
    }

    pub fn has_verification_time_below(self, max_ms: u64) -> Self {
        assert!(self.verification.total_time_ms <= max_ms,
            "Expected verification time <= {}ms but got {}ms", 
            max_ms, self.verification.total_time_ms);
        self
    }

    pub fn has_solver_status(self, expected_status: &str) -> Self {
        assert_eq!(self.verification.solver_status, expected_status,
            "Expected solver status '{}' but got '{}'", 
            expected_status, self.verification.solver_status);
        self
    }
}

#[test]
fn test_basic_property_verification() {
    let document = r#"𝔸5.1.BasicVerification@2026-01-25

⟦Σ:Types⟧{
  Number≜ℕ
  Positive≜{x:Number | x>0}
}

⟦Γ:Rules⟧{
  ∀x:Number→x≥0              # Property 1: All numbers non-negative
  ∀p:Positive→p>0            # Property 2: All positive numbers > 0
  ∃x:Number→x=0              # Property 3: Zero exists
}

⟦Λ:Funcs⟧{
  double≜λx:Number.2*x
  isPositive≜λx:Number.x>0
}

⟦Ω:Meta⟧{
  domain≜basic_verification
  version≜"1.0.0"
  ∀f:Functions→Deterministic(f)
}

⟦Ε⟧⟨δ≜0.9;φ≜100⟩"#;

    FormalTestBuilder::new(document)
        .expecting_properties(3)
        .expecting_verified(3)
        .test_formal_verification()
        .has_property("Number", VerificationResult::Verified)
        .has_property("Positive", VerificationResult::Verified)
        .has_proof("Number")
        .has_smt_formula_count(3)
        .has_solver_status("sat");
}

#[test]
fn test_temporal_property_verification() {
    let document = r#"𝔸5.1.TemporalVerification@2026-01-25

⟦Σ:Types⟧{
  State≜{S0,S1,S2}
  Transition≜State→State
}

⟦Γ:Rules⟧{
  □(S0→◊S1)                  # Property 1: S0 eventually leads to S1
  □(S1→◊S2)                  # Property 2: S1 eventually leads to S2
  ◊□(S2)                     # Property 3: Eventually always S2
  □◊(S0)                     # Property 4: Infinitely often S0
}

⟦Λ:Funcs⟧{
  next≜λs:State.NextState(s)
  reachable≜λs:State.CanReach(s)
}

⟦Ω:Meta⟧{
  domain≜temporal_verification
  version≜"1.0.0"
  ∀s:State→Reachable(s)
  ∀t:Transition→Deterministic(t)
}

⟦Ε⟧⟨δ≜0.9;φ≜120;τ≜◊⁺⟩"#;

    FormalTestBuilder::new(document)
        .expecting_properties(4)
        .expecting_verified(4)
        .test_formal_verification()
        .has_property("S0→◊S1", VerificationResult::Verified)
        .has_property("S1→◊S2", VerificationResult::Verified)
        .has_property("◊□", VerificationResult::Verified)
        .has_property("□◊", VerificationResult::Verified)
        .has_smt_formula_count(4);
}

#[test]
fn test_falsifiable_properties() {
    let document = r#"𝔸5.1.FalsifiableProps@2026-01-25

⟦Σ:Types⟧{
  Counter≜ℕ
  Bounded≜{x:Counter | x≤100}
}

⟦Γ:Rules⟧{
  ∀x:Counter→x<1000          # Property 1: Verifiable (assuming reasonable bounds)
  ∀x:Counter→x≠50            # Property 2: Falsifiable (50 exists)
  ∃x:Counter→x>200           # Property 3: May be falsifiable depending on bounds
}

⟦Λ:Funcs⟧{
  increment≜λx:Counter.x+1
  reset≜λx:Counter.0
  bounded≜λx:Counter.Min(x,100)
}

⟦Ω:Meta⟧{
  domain≜falsifiable_properties
  ∀x:Counter→x≤200
}

⟦Ε⟧⟨δ≜0.8⟩"#;

    FormalTestBuilder::new(document)
        .expecting_properties(3)
        .expecting_verified(2)   # Properties 1 and 3 should verify
        .expecting_falsified(1)  # Property 2 should be falsified
        .test_formal_verification()
        .has_property("x<1000", VerificationResult::Verified)
        .has_property("x≠50", VerificationResult::Falsified)
        .has_counterexample("x≠50");
}

#[test]
fn test_complex_mathematical_proofs() {
    let document = r#"𝔸5.1.MathProofs@2026-01-25

⟦Σ:Types⟧{
  Natural≜ℕ
  Even≜{x:Natural | x%2=0}
  Odd≜{x:Natural | x%2=1}
  Prime≜{p:Natural | p>1 ∧ ∀x:Natural→(x|p ⇒ x=1 ∨ x=p)}
}

⟦Γ:Rules⟧{
  ∀x:Natural→(Even(x) ∨ Odd(x))           # Property 1: Every natural is even or odd
  ∀x:Natural→¬(Even(x) ∧ Odd(x))          # Property 2: No natural is both even and odd
  ∀x:Even→∀y:Odd→Even(x+y+1)             # Property 3: Even + odd + 1 = even
  ∃p:Prime→p>2 ∧ Odd(p)                   # Property 4: Odd primes > 2 exist
}

⟦Λ:Funcs⟧{
  double≜λx:Natural.2*x
  successor≜λx:Natural.x+1
  isPrime≜λp:Natural.CheckPrimality(p)
  gcd≜λ(a:Natural,b:Natural).GreatestCommonDivisor(a,b)
}

⟦Ω:Meta⟧{
  domain≜mathematical_proofs
  version≜"2.0.0"
  description≜"Complex mathematical property verification"
  ∀f:Functions→Mathematical(f)
  ∀p:Properties→Provable(p)
}

⟦Ε⟧⟨δ≜0.95;φ≜200⟩"#;

    FormalTestBuilder::new(document)
        .expecting_properties(4)
        .expecting_verified(4)
        .with_timeout(5000) // Allow longer timeout for complex proofs
        .test_formal_verification()
        .has_property("Even(x) ∨ Odd(x)", VerificationResult::Verified)
        .has_property("¬(Even(x) ∧ Odd(x))", VerificationResult::Verified)
        .has_property("Even(x+y+1)", VerificationResult::Verified)
        .has_property("Odd(p)", VerificationResult::Verified)
        .has_proof("Even(x) ∨ Odd(x)")
        .has_proof("¬(Even(x) ∧ Odd(x))")
        .has_smt_formula_count(4);
}

#[test]
fn test_concurrent_system_verification() {
    let document = r#"𝔸5.1.ConcurrentVerification@2026-01-25

⟦Σ:Types⟧{
  ProcessState≜{Idle,Running,Blocked,Terminated}
  Resource≜{Available,Locked}
  Lock≜{Acquired,Released}
}

⟦Γ:Rules⟧{
  □(Running→◊(Blocked∨Terminated))        # Property 1: Running processes eventually block or terminate
  □¬(Acquired∧Available)                  # Property 2: Mutual exclusion
  □(Locked→◊Available)                    # Property 3: No permanent resource locking
  □◊(Idle→Running)                        # Property 4: Progress - idle processes eventually run
}

⟦Λ:Funcs⟧{
  acquire≜λr:Resource.Lock(r)
  release≜λr:Resource.Unlock(r)
  schedule≜λp:ProcessState.NextSchedule(p)
  terminate≜λp:ProcessState.Cleanup(p)
}

⟦Ω:Meta⟧{
  domain≜concurrent_verification
  version≜"1.0.0"
  description≜"Concurrent system property verification"
  ∀p:Process→WellFormed(p)
  ∀r:Resource→Accessible(r)
  ∀synchronization:Correct(synchronization)
}

⟦Ε⟧⟨δ≜0.88;φ≜150;τ≜◊⁺⟩"#;

    FormalTestBuilder::new(document)
        .expecting_properties(4)
        .expecting_verified(4)
        .with_timeout(3000)
        .test_formal_verification()
        .has_property("Running→◊", VerificationResult::Verified)
        .has_property("¬(Acquired∧Available)", VerificationResult::Verified)
        .has_property("Locked→◊Available", VerificationResult::Verified)
        .has_property("□◊(Idle→Running)", VerificationResult::Verified)
        .has_smt_formula_count(4);
}

#[test]
fn test_verification_timeout_handling() {
    let document = r#"𝔸5.1.TimeoutTest@2026-01-25

⟦Σ:Types⟧{
  ComplexType≜{a:ℕ, b:ℕ, c:ℕ, d:ℕ, e:ℕ}
  VeryComplex≜ComplexType[]
}

⟦Γ:Rules⟧{
  # Intentionally complex properties that may timeout
  ∀x:ComplexType→∀y:ComplexType→∀z:ComplexType→
    (x.a*y.b*z.c + x.d*y.e*z.a > 0 → ∃w:ComplexType→w.a>x.a∧w.b>y.b∧w.c>z.c)
  
  ∀v:VeryComplex→(Length(v)>100 → 
    ∃subset:VeryComplex→Length(subset)≤10 ∧ ∀item∈subset→Valid(item))
}

⟦Λ:Funcs⟧{
  compute≜λx:ComplexType.HeavyComputation(x)
  analyze≜λv:VeryComplex.DeepAnalysis(v)
}

⟦Ω:Meta⟧{
  domain≜timeout_test
  version≜"1.0.0"
  description≜"Testing verification timeout handling"
}

⟦Ε⟧⟨δ≜0.7⟩"#;

    FormalTestBuilder::new(document)
        .expecting_properties(2)
        .expecting_timeout(2) // Both properties expected to timeout
        .with_timeout(100) // Very short timeout to force timeout
        .test_formal_verification()
        .has_verification_time_below(1000); // Should abort quickly
}

#[test]
fn test_smt_formula_generation() {
    let document = r#"𝔸5.1.SMTFormulas@2026-01-25

⟦Σ:Types⟧{
  Integer≜ℤ
  Boolean≜𝔹
  Array≜Integer[10]
}

⟦Γ:Rules⟧{
  ∀x:Integer→(x>0 ⇒ x*x>0)               # SMT Formula 1: Quadratic positivity
  ∀a:Array→∀i:ℕ→(i<10 ⇒ a[i]∈Integer)   # SMT Formula 2: Array bounds and types
  ∀b:Boolean→(b ∨ ¬b)                    # SMT Formula 3: Law of excluded middle
  ∀x:Integer→∀y:Integer→(x<y ⇒ x+1≤y)    # SMT Formula 4: Integer ordering
}

⟦Λ:Funcs⟧{
  square≜λx:Integer.x*x
  arrayGet≜λ(a:Array,i:ℕ).a[i]
  negate≜λb:Boolean.¬b
  compare≜λ(x:Integer,y:Integer).x<y
}

⟦Ω:Meta⟧{
  domain≜smt_formulas
  version≜"1.0.0"
  description≜"Testing SMT formula generation and solving"
  ∀formula:WellFormed(formula)
  ∀encoding:Correct(encoding)
}

⟦Ε⟧⟨δ≜0.92;φ≜180⟩"#;

    FormalTestBuilder::new(document)
        .expecting_properties(4)
        .expecting_verified(4)
        .test_formal_verification()
        .has_smt_formula_count(4)
        .has_property("x*x>0", VerificationResult::Verified)
        .has_property("a[i]∈Integer", VerificationResult::Verified)  
        .has_property("b ∨ ¬b", VerificationResult::Verified)
        .has_property("x+1≤y", VerificationResult::Verified)
        .has_solver_status("sat")
        .has_verification_time_below(2000);
}

#[test]
fn test_end_to_end_formal_validation() {
    let document = r#"𝔸5.1.EndToEndFormal@2026-01-25

⟦Ω:Meta⟧{
  domain≜end_to_end_formal
  version≜"3.0.0"
  description≜"Complete end-to-end formal verification test"
  author≜"Formal Verification Team"
  ∀D∈AISP:Verified(D)
  ∀P∈Properties:Provable(P)
  ∀F∈Functions:Correct(F)
}

⟦Σ:Types⟧{
  State≜{Initial,Processing,Validated,Complete}
  Quality≜{Low,Medium,High,Excellent}
  Metric≜{precision:ℝ, recall:ℝ, accuracy:ℝ}
  Result≜{state:State, quality:Quality, metrics:Metric}
}

⟦Γ:Rules⟧{
  # Temporal properties
  □(Initial→◊Processing)
  □(Processing→◊Validated)  
  □(Validated→◊Complete)
  ◊□(Complete)
  
  # Quality constraints
  ∀m:Metric→(m.precision≥0 ∧ m.precision≤1)
  ∀m:Metric→(m.recall≥0 ∧ m.recall≤1)
  ∀m:Metric→(m.accuracy≥0 ∧ m.accuracy≤1)
  ∀r:Result→(r.quality=Excellent ⇒ r.metrics.accuracy>0.95)
}

⟦Λ:Funcs⟧{
  process≜λs:State.NextState(s)
  validate≜λs:State.CheckValidation(s)
  assess≜λr:Result.EvaluateQuality(r)
  measure≜λr:Result.CalculateMetrics(r)
}

⟦Ε⟧⟨δ≜0.98;φ≜250;τ≜◊⁺;ψ≜□◊;ξ≜0.99⟩"#;

    FormalTestBuilder::new(document)
        .expecting_properties(8) # 4 temporal + 4 quality constraints
        .expecting_verified(8)
        .with_timeout(5000)
        .test_formal_verification()
        .has_property("Initial→◊Processing", VerificationResult::Verified)
        .has_property("◊□(Complete)", VerificationResult::Verified)
        .has_property("precision≥0", VerificationResult::Verified)
        .has_property("accuracy>0.95", VerificationResult::Verified)
        .has_smt_formula_count(8)
        .has_solver_status("sat")
        .has_verification_time_below(5000);
}

#[test]
fn test_integration_with_validation_levels() {
    let document = r#"𝔸5.1.IntegrationTest@2026-01-25

⟦Σ:Types⟧{
  ProcessState≜{Ready,Running,Complete}
}

⟦Γ:Rules⟧{
  □(Ready→◊Running)
  □(Running→◊Complete)
}

⟦Ω:Meta⟧{
  domain≜integration_test
  version≜"1.0.0"
}

⟦Ε⟧⟨δ≜0.9⟩"#;

    // Test that formal verification integrates with the main validation pipeline
    let validator = AispValidator::new();
    let result = validator.validate_document(document, ValidationLevel::Formal)
        .expect("Formal validation should succeed");

    assert!(result.is_valid, "Document should be valid at formal level");
    assert!(result.delta >= 0.85, "Delta should be high for valid formal document");
    
    // Verify that formal verification results are included
    assert!(result.formal_verification_result.is_some(), 
        "Formal verification result should be present");
    
    let formal_result = result.formal_verification_result.unwrap();
    assert!(formal_result.properties.len() >= 2, 
        "Should have extracted temporal properties for verification");
}