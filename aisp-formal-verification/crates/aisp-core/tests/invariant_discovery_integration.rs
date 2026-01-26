//! Invariant Discovery Integration Tests
//!
//! This module tests the complete automated invariant discovery system
//! for mathematical property detection and verification.

use aisp_core::{
    invariant_discovery::{
        InvariantDiscovery, InvariantDiscoveryConfig, InvariantType,
        DiscoveredInvariant,
    },
    ast::*,
    parser_new::AispParser,
    validator::AispValidator,
    error::AispResult,
};
use std::collections::HashMap;

/// Test document builder for invariant discovery tests
pub struct DiscoveryTestDocumentBuilder {
    header: String,
    blocks: HashMap<String, String>,
}

impl DiscoveryTestDocumentBuilder {
    pub fn new() -> Self {
        Self {
            header: "𝔸5.1.DiscoveryTest@2026-01-26".to_string(),
            blocks: HashMap::new(),
        }
    }

    pub fn with_meta_block(mut self, content: &str) -> Self {
        self.blocks.insert("meta".to_string(), format!("⟦Ω:Meta⟧{{{}}}", content));
        self
    }

    pub fn with_types_block(mut self, content: &str) -> Self {
        self.blocks.insert("types".to_string(), format!("⟦Σ:Types⟧{{{}}}", content));
        self
    }

    pub fn with_rules_block(mut self, content: &str) -> Self {
        self.blocks.insert("rules".to_string(), format!("⟦Γ:Rules⟧{{{}}}", content));
        self
    }

    pub fn with_functions_block(mut self, content: &str) -> Self {
        self.blocks.insert("functions".to_string(), format!("⟦Λ:Funcs⟧{{{}}}", content));
        self
    }

    pub fn with_evidence_block(mut self, content: &str) -> Self {
        self.blocks.insert("evidence".to_string(), format!("⟦Ε⟧{}", content));
        self
    }

    pub fn build(self) -> String {
        let mut document = format!("{}\n\n", self.header);
        
        let block_order = ["meta", "types", "rules", "functions", "evidence"];
        for block_name in &block_order {
            if let Some(block_content) = self.blocks.get(*block_name) {
                document.push_str(&format!("{}\n\n", block_content));
            }
        }
        
        document.trim().to_string()
    }
}

/// Parse a test document for invariant discovery
fn parse_test_document(content: &str) -> AispResult<AispDocument> {
    let mut parser = AispParser::new(content.to_string());
    parser.parse()
}

/// Assertion helper for discovered invariants
pub struct InvariantAssertion {
    invariants: Vec<DiscoveredInvariant>,
}

impl InvariantAssertion {
    pub fn new(invariants: Vec<DiscoveredInvariant>) -> Self {
        Self { invariants }
    }

    pub fn has_count(self, expected: usize) -> Self {
        assert_eq!(self.invariants.len(), expected, 
            "Expected {} invariants, but found {}", expected, self.invariants.len());
        self
    }

    pub fn has_type_invariant(self) -> Self {
        assert!(self.invariants.iter().any(|inv| 
            matches!(inv.invariant_type, InvariantType::TypeStructural | InvariantType::TypeMembership)),
            "Expected at least one type invariant");
        self
    }

    pub fn has_functional_invariant(self) -> Self {
        assert!(self.invariants.iter().any(|inv| 
            matches!(inv.invariant_type, InvariantType::FunctionalProperty | InvariantType::FunctionalMonotonicity)),
            "Expected at least one functional invariant");
        self
    }

    pub fn has_high_confidence_invariant(self) -> Self {
        assert!(self.invariants.iter().any(|inv| 
            inv.confidence >= 0.8),
            "Expected at least one high-confidence invariant");
        self
    }

    pub fn all_verified(self) -> Self {
        for inv in &self.invariants {
            assert!(inv.verified, "All discovered invariants should be verified");
        }
        self
    }
}

#[test]
fn test_basic_invariant_discovery() {
    let document_content = DiscoveryTestDocumentBuilder::new()
        .with_meta_block("domain≜basic-test")
        .with_types_block("Natural≜ℕ\nPositive≜{x∈ℕ|x>0}")
        .with_rules_block("∀x:Natural→x≥0")
        .with_functions_block("square≜λx.x×x")
        .with_evidence_block("⟨δ≜0.8⟩")
        .build();

    let document = parse_test_document(&document_content)
        .expect("Failed to parse test document");

    let config = InvariantDiscoveryConfig::default();
    let mut discovery = InvariantDiscovery::new(config);
    
    let invariants = discovery.discover_invariants(&document)
        .expect("Failed to discover invariants");

    InvariantAssertion::new(invariants)
        .has_type_invariant()
        .all_verified();
}

#[test]
fn test_numerical_invariant_discovery() {
    let document_content = DiscoveryTestDocumentBuilder::new()
        .with_meta_block("domain≜numerical-test")
        .with_types_block("Range≜{x∈ℕ|0≤x≤100}\nCounter≜ℕ")
        .with_rules_block("∀x:Range→0≤x≤100\n∀c:Counter→c≥0")
        .with_functions_block("increment≜λx.x+1\nvalidate≜λx.x∈Range")
        .with_evidence_block("⟨δ≜0.9⟩")
        .build();

    let document = parse_test_document(&document_content)
        .expect("Failed to parse test document");

    let mut config = InvariantDiscoveryConfig::default();
    config.enable_numerical_analysis = true;
    config.verification_timeout = 5000;
    
    let mut discovery = InvariantDiscovery::new(config);
    
    let invariants = discovery.discover_invariants(&document)
        .expect("Failed to discover invariants");

    InvariantAssertion::new(invariants)
        .has_type_invariant()
        .has_functional_invariant()
        .has_high_confidence_invariant()
        .all_verified();
}

#[test]
fn test_pattern_based_invariant_discovery() {
    let document_content = DiscoveryTestDocumentBuilder::new()
        .with_meta_block("domain≜pattern-test")
        .with_types_block("State≜{Init,Active,Done}\nTransition≜State→State")
        .with_rules_block("∀s:State→NextState(s)\n∀t:Transition→Valid(t)")
        .with_functions_block("next≜λs.case[Init⇒Active,Active⇒Done,Done⇒Done]\nid≜λx.x")
        .with_evidence_block("⟨δ≜0.85⟩")
        .build();

    let document = parse_test_document(&document_content)
        .expect("Failed to parse test document");

    let mut config = InvariantDiscoveryConfig::default();
    config.enable_patterns = true;
    config.max_invariants = 10;
    
    let mut discovery = InvariantDiscovery::new(config);
    
    let invariants = discovery.discover_invariants(&document)
        .expect("Failed to discover invariants");

    InvariantAssertion::new(invariants)
        .has_type_invariant()
        .has_functional_invariant()
        .all_verified();
}

#[test]
fn test_logical_invariant_discovery() {
    let document_content = DiscoveryTestDocumentBuilder::new()
        .with_meta_block("domain≜logic-test")
        .with_types_block("Prop≜{True,False}\nFormula≜Prop∧Prop∨¬Prop")
        .with_rules_block("∀p:Prop→p∨¬p\n∀f:Formula→Satisfiable(f)")
        .with_functions_block("and≜λ(p,q).p∧q\nnot≜λp.¬p")
        .with_evidence_block("⟨δ≜0.75⟩")
        .build();

    let document = parse_test_document(&document_content)
        .expect("Failed to parse test document");

    let mut config = InvariantDiscoveryConfig::default();
    config.enable_logical_analysis = true;
    config.confidence_threshold = 0.7;
    
    let mut discovery = InvariantDiscovery::new(config);
    
    let invariants = discovery.discover_invariants(&document)
        .expect("Failed to discover invariants");

    InvariantAssertion::new(invariants)
        .has_type_invariant()
        .has_functional_invariant();
}

#[test]
fn test_structural_invariant_discovery() {
    let document_content = DiscoveryTestDocumentBuilder::new()
        .with_meta_block("domain≜structural-test")
        .with_types_block("List≜{Nil,Cons(ℕ,List)}\nTree≜{Leaf(ℕ),Branch(Tree,Tree)}")
        .with_rules_block("∀l:List→WellFormed(l)\n∀t:Tree→Balanced(t)")
        .with_functions_block("length≜λl.case[Nil⇒0,Cons(x,xs)⇒1+length(xs)]")
        .with_evidence_block("⟨δ≜0.8⟩")
        .build();

    let document = parse_test_document(&document_content)
        .expect("Failed to parse test document");

    let mut config = InvariantDiscoveryConfig::default();
    config.enable_structural_analysis = true;
    
    let mut discovery = InvariantDiscovery::new(config);
    
    let invariants = discovery.discover_invariants(&document)
        .expect("Failed to discover invariants");

    InvariantAssertion::new(invariants)
        .has_type_invariant()
        .has_functional_invariant()
        .all_verified();
}

#[test]
fn test_complex_document_invariant_discovery() {
    let document_content = DiscoveryTestDocumentBuilder::new()
        .with_meta_block("domain≜complex-test\nprotocol≜\"advanced-aisp\"")
        .with_types_block(r#"
            Signal≜V_H⊕V_L⊕V_S
            V_H≜ℝ⁷⁶⁸
            V_L≜ℝ⁵¹²
            V_S≜ℝ²⁵⁶
            State≜{Active,Idle,Error}
        "#)
        .with_rules_block(r#"
            ∀s:Signal→WellFormed(s)
            V_H∩V_S≡∅
            V_L∩V_S≡∅
            ∀x:V_H→|x|=768
        "#)
        .with_functions_block(r#"
            validate≜λs.CheckDimensions(s)∧CheckDisjoint(s)
            transform≜λ(h,l).Combine(h,l)
        "#)
        .with_evidence_block("⟨δ≜0.9;φ≜150;τ≜◊⁺⁺⟩")
        .build();

    let document = parse_test_document(&document_content)
        .expect("Failed to parse test document");

    let mut config = InvariantDiscoveryConfig::default();
    config.enable_patterns = true;
    config.enable_numerical_analysis = true;
    config.enable_logical_analysis = true;
    config.enable_structural_analysis = true;
    config.max_invariants = 20;
    
    let mut discovery = InvariantDiscovery::new(config);
    
    let invariants = discovery.discover_invariants(&document)
        .expect("Failed to discover invariants");

    InvariantAssertion::new(invariants)
        .has_type_invariant()
        .has_functional_invariant()
        .has_high_confidence_invariant();
}

#[test]
fn test_invariant_discovery_with_z3_verification() {
    let document_content = DiscoveryTestDocumentBuilder::new()
        .with_meta_block("domain≜z3-test")
        .with_types_block("Natural≜ℕ\nEven≜{x∈ℕ|x%2=0}")
        .with_rules_block("∀x:Natural→x≥0\n∀e:Even→e%2=0")
        .with_functions_block("double≜λx.2×x\nisEven≜λx.x%2=0")
        .with_evidence_block("⟨δ≜0.95⟩")
        .build();

    let document = parse_test_document(&document_content)
        .expect("Failed to parse test document");

    let mut config = InvariantDiscoveryConfig::default();
    config.enable_z3_verification = true;
    config.verification_timeout = 10000;
    
    let mut discovery = InvariantDiscovery::new(config);
    
    let invariants = discovery.discover_invariants(&document)
        .expect("Failed to discover invariants");

    InvariantAssertion::new(invariants)
        .has_type_invariant()
        .has_functional_invariant()
        .all_verified();
}

#[test]
fn test_invariant_discovery_export_formats() {
    let document_content = DiscoveryTestDocumentBuilder::new()
        .with_meta_block("domain≜export-test")
        .with_types_block("Unit≜{unit}")
        .with_rules_block("∀x:Unit→Valid(x)")
        .with_functions_block("id≜λx.x")
        .with_evidence_block("⟨δ≜0.7⟩")
        .build();

    let document = parse_test_document(&document_content)
        .expect("Failed to parse test document");

    let config = InvariantDiscoveryConfig::default();
    let mut discovery = InvariantDiscovery::new(config);
    
    let invariants = discovery.discover_invariants(&document)
        .expect("Failed to discover invariants");

    // Test JSON export
    let json_export = discovery.export_json(&invariants);
    assert!(!json_export.is_empty(), "JSON export should not be empty");

    // Test SMT-LIB export
    let smt_export = discovery.export_smt_lib(&invariants);
    assert!(!smt_export.is_empty(), "SMT-LIB export should not be empty");

    // Test human-readable export
    let readable_export = discovery.export_human_readable(&invariants);
    assert!(!readable_export.is_empty(), "Human-readable export should not be empty");
}

#[test]
fn test_invariant_discovery_performance() {
    let document_content = DiscoveryTestDocumentBuilder::new()
        .with_meta_block("domain≜performance-test")
        .with_types_block("Counter≜ℕ\nList≜{Nil,Cons(ℕ,List)}")
        .with_rules_block("∀c:Counter→c≥0\n∀l:List→WellFormed(l)")
        .with_functions_block("inc≜λx.x+1\nlength≜λl.case[Nil⇒0,Cons(x,xs)⇒1+length(xs)]")
        .with_evidence_block("⟨δ≜0.8⟩")
        .build();

    let document = parse_test_document(&document_content)
        .expect("Failed to parse test document");

    let mut config = InvariantDiscoveryConfig::default();
    config.max_invariants = 5; // Limit for performance test
    config.verification_timeout = 1000; // Short timeout
    
    let mut discovery = InvariantDiscovery::new(config);
    
    let start_time = std::time::Instant::now();
    let invariants = discovery.discover_invariants(&document)
        .expect("Failed to discover invariants");
    let duration = start_time.elapsed();

    // Performance check - should complete quickly
    assert!(duration.as_secs() < 5, "Invariant discovery took too long: {:?}", duration);
    
    InvariantAssertion::new(invariants)
        .has_type_invariant()
        .all_verified();
}