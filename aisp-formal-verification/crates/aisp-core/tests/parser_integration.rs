//! Updated Parser integration tests
//!
//! This module tests the complete parsing pipeline using the current
//! parser and validator API.

use aisp_core::{
    ast::*,
    parser_new::AispParser,
    validator::{AispValidator, ValidationConfig},
    semantic::QualityTier,
};

/// Builder for creating parser test cases
pub struct ParserTestBuilder {
    input: String,
    expected_blocks: usize,
    should_fail: bool,
}

impl ParserTestBuilder {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            expected_blocks: 0,
            should_fail: false,
        }
    }

    pub fn expecting_blocks(mut self, count: usize) -> Self {
        self.expected_blocks = count;
        self
    }

    pub fn should_fail(mut self) -> Self {
        self.should_fail = true;
        self
    }

    pub fn test_parse(self) -> ParseResult {
        let mut parser = AispParser::new(self.input.clone());
        let result = parser.parse();

        if self.should_fail {
            assert!(result.is_err(), "Expected parsing to fail but it succeeded");
            ParseResult::Failed
        } else {
            let document = result.expect("Parsing should succeed");
            assert_eq!(document.blocks.len(), self.expected_blocks, 
                "Expected {} blocks but got {}", self.expected_blocks, document.blocks.len());
            ParseResult::Success(document)
        }
    }
}

pub enum ParseResult {
    Success(AispDocument),
    Failed,
}

/// Assertion helper for documents
pub struct DocumentAssertion {
    document: AispDocument,
}

impl DocumentAssertion {
    pub fn new(document: AispDocument) -> Self {
        Self { document }
    }

    pub fn has_header_version(self, expected: &str) -> Self {
        assert_eq!(self.document.header.version, expected);
        self
    }

    pub fn has_header_name(self, expected: &str) -> Self {
        assert_eq!(self.document.header.name, expected);
        self
    }

    pub fn has_block_count(self, expected: usize) -> Self {
        assert_eq!(self.document.blocks.len(), expected);
        self
    }

    pub fn has_meta_block(self) -> MetaBlockAssertion {
        let meta_block = self.document.blocks.iter()
            .find_map(|block| match block {
                AispBlock::Meta(meta) => Some(meta.clone()),
                _ => None,
            })
            .expect("Expected meta block");
        
        MetaBlockAssertion::new(meta_block)
    }

    pub fn has_evidence_block(self) -> EvidenceBlockAssertion {
        let evidence_block = self.document.blocks.iter()
            .find_map(|block| match block {
                AispBlock::Evidence(evidence) => Some(evidence.clone()),
                _ => None,
            })
            .expect("Expected evidence block");
        
        EvidenceBlockAssertion::new(evidence_block)
    }
}

pub struct MetaBlockAssertion {
    meta: MetaBlock,
}

impl MetaBlockAssertion {
    pub fn new(meta: MetaBlock) -> Self {
        Self { meta }
    }

    pub fn has_entry_count(self, expected: usize) -> Self {
        assert_eq!(self.meta.entries.len(), expected);
        self
    }

    pub fn has_entry(self, key: &str) -> Self {
        assert!(self.meta.entries.contains_key(key), 
            "Expected entry '{}' not found", key);
        self
    }
}

pub struct EvidenceBlockAssertion {
    evidence: EvidenceBlock,
}

impl EvidenceBlockAssertion {
    pub fn new(evidence: EvidenceBlock) -> Self {
        Self { evidence }
    }

    pub fn has_delta(self, expected: f64) -> Self {
        let actual = self.evidence.delta.expect("Expected delta value");
        assert!((actual - expected).abs() < 0.001,
            "Expected delta {} but got {}", expected, actual);
        self
    }

    pub fn has_tau(self, expected: &str) -> Self {
        let actual = self.evidence.tau.as_ref().expect("Expected tau value");
        assert_eq!(actual, expected);
        self
    }
}

#[test]
fn test_parse_minimal_document() {
    let input = r#"𝔸5.1.TestDoc@2026-01-25

⟦Ω:Meta⟧{
  domain≜test
}

⟦Σ:Types⟧{
  Unit≜{unit}
}

⟦Γ:Rules⟧{
  ∀x:Unit→Valid(x)
}

⟦Λ:Funcs⟧{
  id≜λx.x
}

⟦Ε⟧⟨δ≜0.7;τ≜◊⟩"#;

    if let ParseResult::Success(document) = ParserTestBuilder::new(input)
        .expecting_blocks(5)
        .test_parse()
    {
        DocumentAssertion::new(document)
            .has_header_version("5.1")
            .has_header_name("TestDoc")
            .has_block_count(5)
            .has_meta_block()
            .has_entry_count(1)
            .has_entry("domain");
    } else {
        panic!("Expected successful parse");
    }
}

#[test]
fn test_parse_evidence_block() {
    let input = r#"𝔸5.1.Evidence@2026-01-25

⟦Ω:Meta⟧{
  domain≜evidence-test
}

⟦Σ:Types⟧{
  Unit≜{unit}
}

⟦Γ:Rules⟧{
  ∀x:Unit→Valid(x)
}

⟦Λ:Funcs⟧{
  id≜λx.x
}

⟦Ε⟧⟨δ≜0.85;φ≜100;τ≜◊⁺⟩"#;

    if let ParseResult::Success(document) = ParserTestBuilder::new(input)
        .expecting_blocks(5)
        .test_parse()
    {
        DocumentAssertion::new(document)
            .has_evidence_block()
            .has_delta(0.85)
            .has_tau("◊⁺");
    } else {
        panic!("Expected successful parse");
    }
}

#[test]
fn test_parse_complex_types() {
    let input = r#"𝔸5.1.Types@2026-01-25

⟦Ω:Meta⟧{
  domain≜type-testing
}

⟦Σ:Types⟧{
  MyNat≜ℕ
  MyEnum≜{A,B,C}
}

⟦Γ:Rules⟧{
  ∀x:MyNat→Valid(x)
}

⟦Λ:Funcs⟧{
  id≜λx.x
}

⟦Ε⟧⟨δ≜0.6⟩"#;

    if let ParseResult::Success(document) = ParserTestBuilder::new(input)
        .expecting_blocks(5)
        .test_parse()
    {
        // Find types block and verify type definitions
        let types_block = document.blocks.iter()
            .find_map(|block| match block {
                AispBlock::Types(types) => Some(types),
                _ => None,
            })
            .expect("Expected types block");
        
        assert!(types_block.definitions.contains_key("MyNat"));
        assert!(types_block.definitions.contains_key("MyEnum"));
    } else {
        panic!("Expected successful parse");
    }
}

#[test]
fn test_validation_integration() {
    let input = r#"𝔸5.1.ValidationTest@2026-01-25

⟦Ω:Meta⟧{
  domain≜validation-test
  protocol≜"test-protocol"
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

⟦Ε⟧⟨δ≜0.75;φ≜95;τ≜◊⁺⟩"#;

    let validator = AispValidator::new();
    let result = validator.validate(input);
    
    // Just verify the validation runs without panic
    // Actual validity depends on semantic analysis implementation
    assert!(result.delta > 0.0);
    assert!(result.tier != QualityTier::Reject);
}

#[test]
fn test_invalid_syntax() {
    let input = r#"𝔸5.1.Invalid@2026-01-25

⟦Ω:Meta⟧{
  domain≜test
  INVALID_SYNTAX_HERE
}"#;

    ParserTestBuilder::new(input)
        .should_fail()
        .test_parse();
}

#[test]
fn test_empty_document() {
    let input = "";

    ParserTestBuilder::new(input)
        .should_fail()
        .test_parse();
}

#[test]
fn test_header_parsing() {
    let input = r#"𝔸5.1.HeaderTest@2026-01-25

⟦Ω:Meta⟧{
  domain≜header-test
}

⟦Σ:Types⟧{
  Unit≜{unit}
}

⟦Γ:Rules⟧{
  ∀x:Unit→Valid(x)
}

⟦Λ:Funcs⟧{
  id≜λx.x
}

⟦Ε⟧⟨δ≜0.5⟩"#;

    if let ParseResult::Success(document) = ParserTestBuilder::new(input)
        .expecting_blocks(5)
        .test_parse()
    {
        assert_eq!(document.header.version, "5.1");
        assert_eq!(document.header.name, "HeaderTest");
        assert_eq!(document.header.date, "2026-01-25");
    } else {
        panic!("Expected successful parse");
    }
}