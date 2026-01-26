//! Semantic analysis integration tests
//!
//! This module tests semantic analysis including type checking, symbol 
//! resolution, and quality analysis across the complete document structure.

use aisp_core::{
    SemanticAnalyzer, AispDocument, AispParser, SemanticAnalysisResult,
    ValidationLevel, QualityAnalyzer, SymbolAnalyzer, TypeChecker,
    QualityTier, AispError, AispWarning
};
use std::collections::HashMap;

/// Builder for creating semantic analysis test scenarios
pub struct SemanticTestBuilder {
    document_source: String,
    expected_errors: usize,
    expected_warnings: usize,
    expected_quality: Option<QualityTier>,
}

impl SemanticTestBuilder {
    pub fn new(document_source: &str) -> Self {
        Self {
            document_source: document_source.to_string(),
            expected_errors: 0,
            expected_warnings: 0,
            expected_quality: None,
        }
    }

    pub fn expecting_errors(mut self, count: usize) -> Self {
        self.expected_errors = count;
        self
    }

    pub fn expecting_warnings(mut self, count: usize) -> Self {
        self.expected_warnings = count;
        self
    }

    pub fn expecting_quality(mut self, tier: QualityTier) -> Self {
        self.expected_quality = Some(tier);
        self
    }

    pub fn test_semantic_analysis(self) -> SemanticResult {
        let parser = AispParser::new();
        let document = parser.parse(&self.document_source)
            .expect("Document should parse successfully for semantic analysis");

        let mut analyzer = SemanticAnalyzer::new();
        let result = analyzer.analyze(&document);

        // Verify error count
        if result.errors.len() != self.expected_errors {
            panic!("Expected {} errors but got {}: {:?}", 
                self.expected_errors, result.errors.len(), result.errors);
        }

        // Verify warning count
        if result.warnings.len() != self.expected_warnings {
            panic!("Expected {} warnings but got {}: {:?}", 
                self.expected_warnings, result.warnings.len(), result.warnings);
        }

        // Verify quality tier if specified
        if let Some(expected_quality) = self.expected_quality {
            if result.quality_tier != expected_quality {
                panic!("Expected quality tier {:?} but got {:?}", 
                    expected_quality, result.quality_tier);
            }
        }

        SemanticResult::new(document, result)
    }
}

/// Helper for asserting semantic analysis results
pub struct SemanticResult {
    document: AispDocument,
    analysis: SemanticAnalysisResult,
}

impl SemanticResult {
    pub fn new(document: AispDocument, analysis: SemanticAnalysisResult) -> Self {
        Self { document, analysis }
    }

    pub fn has_type_definitions(self, count: usize) -> Self {
        assert_eq!(self.analysis.type_definitions.len(), count,
            "Expected {} type definitions but got {}", count, self.analysis.type_definitions.len());
        self
    }

    pub fn has_function_definitions(self, count: usize) -> Self {
        assert_eq!(self.analysis.function_definitions.len(), count,
            "Expected {} function definitions but got {}", count, self.analysis.function_definitions.len());
        self
    }

    pub fn has_symbol_count(self, count: usize) -> Self {
        assert_eq!(self.analysis.symbol_table.len(), count,
            "Expected {} symbols but got {}", count, self.analysis.symbol_table.len());
        self
    }

    pub fn has_delta_above(self, threshold: f64) -> Self {
        assert!(self.analysis.delta >= threshold,
            "Expected delta >= {} but got {}", threshold, self.analysis.delta);
        self
    }

    pub fn has_ambiguity_below(self, threshold: f64) -> Self {
        assert!(self.analysis.ambiguity <= threshold,
            "Expected ambiguity <= {} but got {}", threshold, self.analysis.ambiguity);
        self
    }

    pub fn has_error_containing(self, message_fragment: &str) -> Self {
        let found = self.analysis.errors.iter()
            .any(|error| error.message.contains(message_fragment));
        assert!(found, "Expected error containing '{}' but errors were: {:?}", 
            message_fragment, self.analysis.errors);
        self
    }

    pub fn has_warning_containing(self, message_fragment: &str) -> Self {
        let found = self.analysis.warnings.iter()
            .any(|warning| warning.message.contains(message_fragment));
        assert!(found, "Expected warning containing '{}' but warnings were: {:?}", 
            message_fragment, self.analysis.warnings);
        self
    }
}

#[test]
fn test_basic_type_checking() {
    let document = r#"𝔸5.1.TypeTest@2026-01-25

⟦Σ:Types⟧{
  State≜{A,B,C}
  Transition≜State→State
  Value≜ℕ
}

⟦Ω:Meta⟧{
  domain≜type_test
  version≜"1.0.0"
}

⟦Ε⟧⟨δ≜0.8⟩"#;

    SemanticTestBuilder::new(document)
        .expecting_errors(0)
        .expecting_quality(QualityTier::Silver)
        .test_semantic_analysis()
        .has_type_definitions(3)
        .has_symbol_count(3)
        .has_delta_above(0.7);
}

#[test]
fn test_undefined_type_error() {
    let document = r#"𝔸5.1.ErrorTest@2026-01-25

⟦Σ:Types⟧{
  State≜{A,B,C}
  Transition≜UndefinedType→State
}

⟦Ω:Meta⟧{
  domain≜error_test
}

⟦Ε⟧⟨δ≜0.8⟩"#;

    SemanticTestBuilder::new(document)
        .expecting_errors(1)
        .test_semantic_analysis()
        .has_error_containing("UndefinedType");
}

#[test]
fn test_circular_type_dependency() {
    let document = r#"𝔸5.1.CircularTest@2026-01-25

⟦Σ:Types⟧{
  TypeA≜TypeB
  TypeB≜TypeA
}

⟦Ω:Meta⟧{
  domain≜circular_test
}

⟦Ε⟧⟨δ≜0.8⟩"#;

    SemanticTestBuilder::new(document)
        .expecting_errors(1)
        .test_semantic_analysis()
        .has_error_containing("circular");
}

#[test]
fn test_function_type_analysis() {
    let document = r#"𝔸5.1.FunctionTest@2026-01-25

⟦Σ:Types⟧{
  State≜{A,B,C}
  Transition≜State→State
}

⟦Λ:Funcs⟧{
  next≜λs:State.NextState(s)
  valid≜λt:Transition.IsValid(t)
  identity≜λx.x
}

⟦Ω:Meta⟧{
  domain≜function_test
}

⟦Ε⟧⟨δ≜0.85⟩"#;

    SemanticTestBuilder::new(document)
        .expecting_errors(0)
        .expecting_quality(QualityTier::Gold)
        .test_semantic_analysis()
        .has_type_definitions(2)
        .has_function_definitions(3)
        .has_delta_above(0.8);
}

#[test]
fn test_meta_constraint_analysis() {
    let document = r#"𝔸5.1.MetaTest@2026-01-25

⟦Ω:Meta⟧{
  domain≜meta_test
  version≜"1.0.0"
  description≜"Testing meta constraints"
  ∀D∈AISP:Ambig(D)<0.02
  ∀T∈Types:Valid(T)
}

⟦Σ:Types⟧{
  State≜{A,B,C}
}

⟦Ε⟧⟨δ≜0.9⟩"#;

    SemanticTestBuilder::new(document)
        .expecting_errors(0)
        .expecting_quality(QualityTier::Platinum)
        .test_semantic_analysis()
        .has_delta_above(0.8)
        .has_ambiguity_below(0.05);
}

#[test]
fn test_symbol_resolution() {
    let document = r#"𝔸5.1.SymbolTest@2026-01-25

⟦Σ:Types⟧{
  State≜{Start,Playing,End}
  Player≜{A,B}
  Move≜ℕ
  GameConfig≜{moves:Move, players:Player}
}

⟦Γ:Rules⟧{
  ∀s:State→Valid(s)
  ∀p:Player→Active(p)
  ∀m:Move→m>0
}

⟦Λ:Funcs⟧{
  nextMove≜λ(s:State,p:Player).CalculateMove(s,p)
  isValid≜λm:Move.m>0∧m<100
}

⟦Ω:Meta⟧{
  domain≜symbol_test
}

⟦Ε⟧⟨δ≜0.85⟩"#;

    SemanticTestBuilder::new(document)
        .expecting_errors(0)
        .expecting_quality(QualityTier::Gold)
        .test_semantic_analysis()
        .has_type_definitions(4)
        .has_function_definitions(2)
        .has_symbol_count(6); // 4 types + 2 functions
}

#[test]
fn test_quality_analysis_factors() {
    let document = r#"𝔸5.1.QualityTest@2026-01-25

⟦Ω:Meta⟧{
  domain≜quality_comprehensive_test
  version≜"2.1.0"
  description≜"Comprehensive quality analysis test with detailed metadata"
  author≜"Quality Tester"
  ∀D∈AISP:Ambig(D)<0.01
  ∀T∈Types:Complete(T)
  ∀F∈Functions:Verified(F)
}

⟦Σ:Types⟧{
  PrimaryState≜{Initial,Processing,Complete,Error}
  SecondaryState≜{Idle,Active,Suspended}
  TransitionRule≜PrimaryState→SecondaryState
  DataPayload≜{id:ℕ, value:ℝ, metadata:𝕊}
  ProcessResult≜{success:𝔹, data:DataPayload, state:PrimaryState}
}

⟦Γ:Rules⟧{
  ∀s:PrimaryState→Consistent(s)
  ∀t:TransitionRule→Valid(t)
  ∀d:DataPayload→d.id>0∧d.value≥0
  □(Initial→◊Complete)
  □(Error→◊Initial)
}

⟦Λ:Funcs⟧{
  processData≜λ(d:DataPayload).Process(d)
  validateState≜λs:PrimaryState.IsValid(s)
  transition≜λ(from:PrimaryState,to:SecondaryState).Execute(from,to)
  calculateMetrics≜λdata:DataPayload.Analyze(data)
}

⟦Ε⟧⟨δ≜0.95;φ≜150;τ≜◊⁺⟩"#;

    SemanticTestBuilder::new(document)
        .expecting_errors(0)
        .expecting_quality(QualityTier::Platinum)
        .test_semantic_analysis()
        .has_type_definitions(5)
        .has_function_definitions(4)
        .has_delta_above(0.9)
        .has_ambiguity_below(0.02);
}

#[test]
fn test_incomplete_document_warnings() {
    let document = r#"𝔸5.1.IncompleteTest@2026-01-25

⟦Σ:Types⟧{
  State≜{A,B}
}

⟦Ω:Meta⟧{
  domain≜incomplete_test
}

⟦Ε⟧⟨δ≜0.6⟩"#;

    SemanticTestBuilder::new(document)
        .expecting_errors(0)
        .expecting_warnings(1) // Warning about missing components
        .expecting_quality(QualityTier::Bronze)
        .test_semantic_analysis()
        .has_warning_containing("incomplete");
}

#[test]
fn test_semantic_error_accumulation() {
    let document = r#"𝔸5.1.MultiErrorTest@2026-01-25

⟦Σ:Types⟧{
  State≜UndefinedType1
  Transition≜UndefinedType2→UndefinedType3
  Value≜{A,B,A}  # Duplicate enumeration value
}

⟦Λ:Funcs⟧{
  badFunc≜λx:UndefinedType4.Process(x)
  duplicate≜λy.Process(y)
  duplicate≜λz.Process(z)  # Duplicate function name
}

⟦Ω:Meta⟧{
  domain≜multi_error_test
}

⟦Ε⟧⟨δ≜0.8⟩"#;

    SemanticTestBuilder::new(document)
        .expecting_errors(5) // Multiple semantic errors should be caught
        .test_semantic_analysis()
        .has_error_containing("UndefinedType");
}

#[test]
fn test_type_inference_and_checking() {
    let document = r#"𝔸5.1.InferenceTest@2026-01-25

⟦Σ:Types⟧{
  Number≜ℕ
  Predicate≜Number→𝔹
  Transform≜Number→Number
  Combinator≜(Number,Number)→Number
}

⟦Λ:Funcs⟧{
  isEven≜λn:Number.n%2=0
  double≜λn:Number.n*2
  add≜λ(x:Number,y:Number).x+y
  compose≜λ(f:Transform,g:Transform).λx.f(g(x))
}

⟦Γ:Rules⟧{
  ∀n:Number→n≥0
  ∀p:Predicate→∀x:Number→p(x)∈𝔹
  ∀t:Transform→∀x:Number→t(x)∈Number
}

⟦Ω:Meta⟧{
  domain≜inference_test
}

⟦Ε⟧⟨δ≜0.88⟩"#;

    SemanticTestBuilder::new(document)
        .expecting_errors(0)
        .expecting_quality(QualityTier::Gold)
        .test_semantic_analysis()
        .has_type_definitions(4)
        .has_function_definitions(4)
        .has_delta_above(0.85);
}

#[test]
fn test_advanced_type_relationships() {
    let document = r#"𝔸5.1.AdvancedTypes@2026-01-25

⟦Σ:Types⟧{
  BaseType≜ℕ
  DerivedType≜BaseType
  ContainerType≜DerivedType[10]
  FunctionType≜BaseType→DerivedType
  CompositeType≜(BaseType,DerivedType,ContainerType)
  RecursiveType≜{value:BaseType, next:RecursiveType}
}

⟦Λ:Funcs⟧{
  convert≜λ(x:BaseType).Cast(x,DerivedType)
  process≜λ(container:ContainerType).Map(container,convert)
  combine≜λ(comp:CompositeType).Merge(comp)
}

⟦Γ:Rules⟧{
  ∀x:BaseType→∀y:DerivedType→Compatible(x,y)
  ∀c:ContainerType→Length(c)=10
  ∀r:RecursiveType→WellFormed(r)
}

⟦Ω:Meta⟧{
  domain≜advanced_types
  version≜"1.0.0"
  ∀T∈Types:TypeSafe(T)
}

⟦Ε⟧⟨δ≜0.9;φ≜120⟩"#;

    SemanticTestBuilder::new(document)
        .expecting_errors(0)
        .expecting_quality(QualityTier::Platinum)
        .test_semantic_analysis()
        .has_type_definitions(6)
        .has_function_definitions(3)
        .has_delta_above(0.85);
}