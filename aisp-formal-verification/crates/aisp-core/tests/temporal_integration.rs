//! Temporal logic integration tests (Level 5)
//!
//! This module tests Level 5 temporal logic analysis including LTL/CTL
//! operators, pattern detection, model checking, and temporal consistency.

use aisp_core::{
    TemporalAnalyzer, AispDocument, AispParser, TemporalAnalysisResult,
    TemporalOperator, PatternType, TemporalPattern, OperatorInstance,
    ModelCheckingResult, VerificationResult, FormulaType
};

/// Builder for creating temporal analysis test scenarios
pub struct TemporalTestBuilder {
    document_source: String,
    expected_operators: usize,
    expected_patterns: usize,
    expected_ltl_formulas: usize,
    expected_ctl_formulas: usize,
    expected_consistency: Option<f64>,
}

impl TemporalTestBuilder {
    pub fn new(document_source: &str) -> Self {
        Self {
            document_source: document_source.to_string(),
            expected_operators: 0,
            expected_patterns: 0,
            expected_ltl_formulas: 0,
            expected_ctl_formulas: 0,
            expected_consistency: None,
        }
    }

    pub fn expecting_operators(mut self, count: usize) -> Self {
        self.expected_operators = count;
        self
    }

    pub fn expecting_patterns(mut self, count: usize) -> Self {
        self.expected_patterns = count;
        self
    }

    pub fn expecting_ltl_formulas(mut self, count: usize) -> Self {
        self.expected_ltl_formulas = count;
        self
    }

    pub fn expecting_ctl_formulas(mut self, count: usize) -> Self {
        self.expected_ctl_formulas = count;
        self
    }

    pub fn expecting_consistency(mut self, consistency: f64) -> Self {
        self.expected_consistency = Some(consistency);
        self
    }

    pub fn test_temporal_analysis(self) -> TemporalResult {
        let parser = AispParser::new();
        let document = parser.parse(&self.document_source)
            .expect("Document should parse successfully for temporal analysis");

        let mut analyzer = TemporalAnalyzer::new();
        let result = analyzer.analyze(&document);

        // Verify operator count
        if result.detected_operators.len() != self.expected_operators {
            panic!("Expected {} temporal operators but got {}: {:?}", 
                self.expected_operators, result.detected_operators.len(), 
                result.detected_operators);
        }

        // Verify pattern count
        if result.detected_patterns.len() != self.expected_patterns {
            panic!("Expected {} temporal patterns but got {}: {:?}", 
                self.expected_patterns, result.detected_patterns.len(), 
                result.detected_patterns);
        }

        // Verify LTL formula count
        if result.ltl_formulas.len() != self.expected_ltl_formulas {
            panic!("Expected {} LTL formulas but got {}: {:?}", 
                self.expected_ltl_formulas, result.ltl_formulas.len(), 
                result.ltl_formulas);
        }

        // Verify CTL formula count
        if result.ctl_formulas.len() != self.expected_ctl_formulas {
            panic!("Expected {} CTL formulas but got {}: {:?}", 
                self.expected_ctl_formulas, result.ctl_formulas.len(), 
                result.ctl_formulas);
        }

        // Verify consistency score if specified
        if let Some(expected_consistency) = self.expected_consistency {
            let actual_consistency = result.consistency_score;
            if (actual_consistency - expected_consistency).abs() > 0.1 {
                panic!("Expected consistency score ~{} but got {}", 
                    expected_consistency, actual_consistency);
            }
        }

        TemporalResult::new(document, result)
    }
}

/// Helper for asserting temporal analysis results
pub struct TemporalResult {
    _document: AispDocument,
    analysis: TemporalAnalysisResult,
}

impl TemporalResult {
    pub fn new(document: AispDocument, analysis: TemporalAnalysisResult) -> Self {
        Self { _document: document, analysis }
    }

    pub fn has_operator(self, operator: TemporalOperator) -> Self {
        let found = self.analysis.detected_operators.iter()
            .any(|op| op.operator == operator);
        assert!(found, "Expected temporal operator {:?} but found: {:?}", 
            operator, self.analysis.detected_operators);
        self
    }

    pub fn has_pattern(self, pattern_type: PatternType) -> Self {
        let found = self.analysis.detected_patterns.iter()
            .any(|pattern| pattern.pattern_type == pattern_type);
        assert!(found, "Expected temporal pattern {:?} but found: {:?}", 
            pattern_type, self.analysis.detected_patterns);
        self
    }

    pub fn has_ltl_formula_containing(self, fragment: &str) -> Self {
        let found = self.analysis.ltl_formulas.iter()
            .any(|formula| formula.contains(fragment));
        assert!(found, "Expected LTL formula containing '{}' but found: {:?}", 
            fragment, self.analysis.ltl_formulas);
        self
    }

    pub fn has_ctl_formula_containing(self, fragment: &str) -> Self {
        let found = self.analysis.ctl_formulas.iter()
            .any(|formula| formula.contains(fragment));
        assert!(found, "Expected CTL formula containing '{}' but found: {:?}", 
            fragment, self.analysis.ctl_formulas);
        self
    }

    pub fn has_consistency_above(self, threshold: f64) -> Self {
        assert!(self.analysis.consistency_score >= threshold,
            "Expected consistency >= {} but got {}", threshold, self.analysis.consistency_score);
        self
    }

    pub fn has_complexity_below(self, threshold: f64) -> Self {
        assert!(self.analysis.complexity_score <= threshold,
            "Expected complexity <= {} but got {}", threshold, self.analysis.complexity_score);
        self
    }

    pub fn has_model_checking_result(self, property: &str, expected: bool) -> Self {
        let found = self.analysis.model_checking_results.iter()
            .find(|result| result.property.contains(property))
            .expect(&format!("Model checking result for '{}' not found", property));
        
        match found.result {
            VerificationResult::Verified if expected => {},
            VerificationResult::Falsified if !expected => {},
            _ => panic!("Expected model checking result {} for '{}' but got {:?}", 
                expected, property, found.result),
        }
        self
    }
}

#[test]
fn test_basic_temporal_operators() {
    let document = r#"𝔸5.1.BasicTemporal@2026-01-25

⟦Σ:Types⟧{
  State≜{A,B,C}
}

⟦Γ:Rules⟧{
  □(A→B)           # Always: if A then B
  ◊(B→C)           # Eventually: if B then eventually C  
  A∪B              # Until: A until B
  A∨B              # Release: A releases B
}

⟦Ω:Meta⟧{
  domain≜basic_temporal
}

⟦Ε⟧⟨δ≜0.8;τ≜◊⁺⟩"#;

    TemporalTestBuilder::new(document)
        .expecting_operators(4)
        .expecting_ltl_formulas(4)
        .expecting_consistency(1.0)
        .test_temporal_analysis()
        .has_operator(TemporalOperator::Always)
        .has_operator(TemporalOperator::Eventually)
        .has_operator(TemporalOperator::Until)
        .has_operator(TemporalOperator::Release)
        .has_ltl_formula_containing("□")
        .has_ltl_formula_containing("◊")
        .has_consistency_above(0.9);
}

#[test]
fn test_ctl_temporal_operators() {
    let document = r#"𝔸5.1.CTLTemporal@2026-01-25

⟦Σ:Types⟧{
  State≜{Initial,Processing,Complete}
}

⟦Γ:Rules⟧{
  AG(Processing→AF(Complete))     # For all paths globally, processing leads to all paths eventually complete
  EF(Initial→EG(Processing))      # Exists path eventually, initial leads to exists path globally processing
  AX(Initial→Processing)          # For all paths next, initial leads to processing
  EX(Processing→Complete)         # Exists path next, processing leads to complete
}

⟦Ω:Meta⟧{
  domain≜ctl_temporal
}

⟦Ε⟧⟨δ≜0.85;τ≜◊⁺⟩"#;

    TemporalTestBuilder::new(document)
        .expecting_operators(8) # AG, AF, EF, EG, AX, EX count as separate operators
        .expecting_ctl_formulas(4)
        .expecting_consistency(1.0)
        .test_temporal_analysis()
        .has_operator(TemporalOperator::AllGlobally)
        .has_operator(TemporalOperator::AllEventually)
        .has_operator(TemporalOperator::ExistsEventually)
        .has_operator(TemporalOperator::ExistsGlobally)
        .has_ctl_formula_containing("AG")
        .has_ctl_formula_containing("EF")
        .has_consistency_above(0.9);
}

#[test]
fn test_temporal_pattern_detection() {
    let document = r#"𝔸5.1.PatternDetection@2026-01-25

⟦Σ:Types⟧{
  SystemState≜{Safe,Unsafe,Error,Recovery}
}

⟦Γ:Rules⟧{
  □(¬Error)                     # Safety pattern: never error
  □◊(Recovery)                  # Liveness pattern: infinitely often recovery
  □(Unsafe→◊Safe)              # Response pattern: unsafe leads to safe
  ◊□(Safe)                     # Persistence pattern: eventually always safe
  □◊(Safe)→□◊(Recovery)        # Fairness pattern: if infinitely safe then infinitely recovery
}

⟦Ω:Meta⟧{
  domain≜pattern_detection
}

⟦Ε⟧⟨δ≜0.88;τ≜◊⁺⟩"#;

    TemporalTestBuilder::new(document)
        .expecting_operators(10) # Multiple nested operators
        .expecting_patterns(5)   # Safety, liveness, response, persistence, fairness
        .expecting_ltl_formulas(5)
        .test_temporal_analysis()
        .has_pattern(PatternType::Safety)
        .has_pattern(PatternType::Liveness)
        .has_pattern(PatternType::Response)
        .has_pattern(PatternType::Persistence)
        .has_pattern(PatternType::Fairness)
        .has_consistency_above(0.85);
}

#[test]
fn test_complex_temporal_formulas() {
    let document = r#"𝔸5.1.ComplexFormulas@2026-01-25

⟦Σ:Types⟧{
  ProcessState≜{Idle,Running,Waiting,Complete}
  Resource≜{Available,Locked,Released}
}

⟦Γ:Rules⟧{
  □((Running∧Locked)→◊(Complete∧Released))                    # Complex safety with resource
  ◊□(Available→¬Locked)                                        # Eventually always resource constraint
  (IdleURunning)∧□(Running→◊Waiting)                         # Until with always-eventually
  AG(EF(Complete)→AX(Idle∨Available))                         # Mixed CTL/LTL style
  □◊(Idle)∧□◊(Running)∧□◊(Complete)                          # Multiple fairness constraints
}

⟦Λ:Funcs⟧{
  transition≜λ(s:ProcessState,r:Resource).NextState(s,r)
  validate≜λ(s:ProcessState).IsValid(s)
}

⟦Ω:Meta⟧{
  domain≜complex_formulas
  version≜"1.0.0"
}

⟦Ε⟧⟨δ≜0.9;φ≜100;τ≜◊⁺⟩"#;

    TemporalTestBuilder::new(document)
        .expecting_operators(20) # Many nested operators
        .expecting_patterns(6)   # Various complex patterns
        .expecting_ltl_formulas(4)
        .expecting_ctl_formulas(1)
        .expecting_consistency(1.0)
        .test_temporal_analysis()
        .has_operator(TemporalOperator::Always)
        .has_operator(TemporalOperator::Eventually)
        .has_operator(TemporalOperator::Until)
        .has_pattern(PatternType::Safety)
        .has_pattern(PatternType::Liveness)
        .has_pattern(PatternType::Response)
        .has_consistency_above(0.85)
        .has_complexity_below(5.0);
}

#[test]
fn test_temporal_model_checking() {
    let document = r#"𝔸5.1.ModelChecking@2026-01-25

⟦Σ:Types⟧{
  State≜{S0,S1,S2,S3}
  Transition≜State→State
}

⟦Γ:Rules⟧{
  # Verifiable properties
  □(S0→◊S1)                    # Property 1: S0 eventually leads to S1
  □(S1→◊(S2∨S3))              # Property 2: S1 leads to S2 or S3
  ◊□(S3)                       # Property 3: Eventually always S3
  □◊(S0)                       # Property 4: Infinitely often S0
}

⟦Λ:Funcs⟧{
  next≜λs:State.Transition(s)
  valid≜λs:State.CheckInvariant(s)
}

⟦Ω:Meta⟧{
  domain≜model_checking
  version≜"1.0.0"
  description≜"Model checking verification test"
  ∀s:State→Reachable(s)
  ∀t:Transition→Deterministic(t)
}

⟦Ε⟧⟨δ≜0.92;φ≜150;τ≜◊⁺⟩"#;

    TemporalTestBuilder::new(document)
        .expecting_operators(8)
        .expecting_patterns(4)
        .expecting_ltl_formulas(4)
        .expecting_consistency(1.0)
        .test_temporal_analysis()
        .has_pattern(PatternType::Response) // S0→◊S1
        .has_pattern(PatternType::Persistence) // ◊□(S3)
        .has_pattern(PatternType::Liveness) // □◊(S0)
        .has_consistency_above(0.9);
}

#[test]
fn test_temporal_consistency_analysis() {
    let document = r#"𝔸5.1.ConsistencyAnalysis@2026-01-25

⟦Σ:Types⟧{
  State≜{A,B,C}
  Event≜{E1,E2,E3}
}

⟦Γ:Rules⟧{
  # Consistent temporal properties
  □(A→◊B)                      # If A then eventually B
  □(B→◊C)                      # If B then eventually C  
  □(C→◊A)                      # If C then eventually A (forms cycle)
  
  # Consistent with the cycle
  ◊□(A∨B∨C)                   # Eventually always one of them
  □◊(A)∧□◊(B)∧□◊(C)           # All occur infinitely often
}

⟦Λ:Funcs⟧{
  trigger≜λ(s:State,e:Event).Transition(s,e)
  check≜λs:State.Validate(s)
}

⟦Ω:Meta⟧{
  domain≜consistency_analysis
  version≜"2.0.0"
  description≜"Testing temporal consistency calculations"
  ∀s:State→Consistent(s)
  ∀e:Event→Valid(e)
}

⟦Ε⟧⟨δ≜0.94;φ≜180;τ≜◊⁺⟩"#;

    TemporalTestBuilder::new(document)
        .expecting_operators(14) # Multiple operators in consistent formulas
        .expecting_patterns(4)
        .expecting_ltl_formulas(5)
        .expecting_consistency(1.0)
        .test_temporal_analysis()
        .has_pattern(PatternType::Response) // Multiple response patterns
        .has_pattern(PatternType::Liveness) // Multiple liveness patterns
        .has_pattern(PatternType::Persistence)
        .has_consistency_above(0.95);
}

#[test]
fn test_temporal_inconsistencies() {
    let document = r#"𝔸5.1.InconsistentTemporal@2026-01-25

⟦Σ:Types⟧{
  State≜{X,Y,Z}
}

⟦Γ:Rules⟧{
  □(X→Y)                       # X always leads to Y
  □(Y→¬X)                      # Y always leads to not X
  ◊□(X)                        # Eventually always X (inconsistent with above)
  
  □◊(Z)                        # Infinitely often Z
  □(Z→◊¬Z)                     # Z leads to eventually not Z (consistent)
}

⟦Ω:Meta⟧{
  domain≜inconsistent_temporal
}

⟦Ε⟧⟨δ≜0.6⟩"#;

    TemporalTestBuilder::new(document)
        .expecting_operators(8)
        .expecting_patterns(3)
        .expecting_ltl_formulas(5)
        .expecting_consistency(0.6) # Low due to inconsistencies
        .test_temporal_analysis()
        .has_pattern(PatternType::Response)
        .has_pattern(PatternType::Liveness)
        .has_pattern(PatternType::Persistence);
        // Note: Consistency will be lower due to logical inconsistencies
}

#[test]
fn test_nested_temporal_operators() {
    let document = r#"𝔸5.1.NestedOperators@2026-01-25

⟦Σ:Types⟧{
  SystemState≜{Init,Ready,Active,Suspend,Terminate}
}

⟦Γ:Rules⟧{
  □(Init→◊□(Ready))                           # Deep nesting: always, eventually, always
  ◊□(Active→◊(Suspend∪Terminate))            # Eventually always with until
  □◊□(Ready→◊Active)                          # Triple nesting
  (Init∪Ready)∧□((Active∨Suspend)→◊◊Ready)   # Until with double eventually
  □(◊□(Active)→□◊(Terminate))                # Complex implication nesting
}

⟦Ω:Meta⟧{
  domain≜nested_operators
}

⟦Ε⟧⟨δ≜0.85;τ≜◊⁺⟩"#;

    TemporalTestBuilder::new(document)
        .expecting_operators(18) # Many nested operators
        .expecting_patterns(5)
        .expecting_ltl_formulas(5)
        .test_temporal_analysis()
        .has_operator(TemporalOperator::Always)
        .has_operator(TemporalOperator::Eventually)
        .has_operator(TemporalOperator::Until)
        .has_pattern(PatternType::Persistence) # ◊□ patterns
        .has_pattern(PatternType::Response)
        .has_consistency_above(0.8)
        .has_complexity_below(8.0); # Complex but manageable
}

#[test]
fn test_temporal_evidence_integration() {
    let document = r#"𝔸5.1.TemporalEvidence@2026-01-25

⟦Σ:Types⟧{
  QualityState≜{Low,Medium,High,Excellent}
}

⟦Γ:Rules⟧{
  □(Low→◊Medium)                # Quality progression
  □(Medium→◊High)               # Continued progression  
  □(High→◊Excellent)            # Final progression
  ◊□(Excellent)                 # Eventually excellent forever
  □◊(High∨Excellent)            # Maintain high quality infinitely
}

⟦Λ:Funcs⟧{
  improve≜λq:QualityState.Enhance(q)
  validate≜λq:QualityState.Check(q)
  measure≜λq:QualityState.Score(q)
}

⟦Ω:Meta⟧{
  domain≜temporal_evidence
  version≜"3.0.0"
  description≜"Temporal logic with quality evidence integration"
  ∀q:QualityState→Measurable(q)
  ∀improvement:Monotonic(improvement)
  ∀validation:Deterministic(validation)
}

⟦Ε⟧⟨δ≜0.96;φ≜200;τ≜◊⁺;ψ≜□◊;ξ≜0.98⟩"#;

    TemporalTestBuilder::new(document)
        .expecting_operators(10)
        .expecting_patterns(5)
        .expecting_ltl_formulas(5)
        .expecting_consistency(1.0)
        .test_temporal_analysis()
        .has_pattern(PatternType::Response) // Quality progression patterns
        .has_pattern(PatternType::Persistence) // ◊□(Excellent)
        .has_pattern(PatternType::Liveness) // □◊(High∨Excellent)
        .has_consistency_above(0.95);
}