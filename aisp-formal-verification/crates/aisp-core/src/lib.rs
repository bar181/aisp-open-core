//! AISP Core - High-performance parsing and validation for AI Symbolic Protocol
//!
//! This crate provides the foundational components for parsing and validating
//! AISP 5.1 documents with zero-copy parsing and strong type safety guarantees.

pub mod ast;
pub mod parser_new;
pub mod relational_new;
pub mod semantic;
pub mod temporal_new;
pub mod validator;
pub mod z3_integration;
pub mod error;
pub mod symbols;
pub mod conflict_types;

// New modular parser components
pub mod lexer;
pub mod token_parser;
pub mod header_parser;
pub mod meta_parser;
pub mod types_parser;
pub mod evidence_parser;
pub mod logic_parser;

// New modular semantic analysis components
pub mod type_checker;

// New modular relational analysis components
pub mod constraint_solver;
pub mod conflict_detector;

// New modular temporal analysis components
pub mod temporal_operator_analyzer;
pub mod temporal_pattern_detector;
pub mod temporal_logic_solver;
pub mod temporal_model_checker;

// Advanced formal verification components
pub mod property_types;
pub mod formula_converter;
pub mod property_factory;
pub mod property_extractor;
pub mod smt_types;
pub mod smt_formula_converter;
pub mod smt_generator;
pub mod proof_types;
pub mod axiom_system;
pub mod proof_search;
pub mod theorem_prover;
pub mod model_checker;

// Invariant discovery system components
pub mod invariant_types;
pub mod invariant_formulas;
pub mod invariant_analyzer;
pub mod invariant_exporters;
pub mod invariant_discovery_main;
pub mod invariant_discovery;

// Satisfiability checking components
pub mod satisfiability_checker;

// Formal verification framework
pub mod formal_verification;

// Advanced behavioral verification components
pub mod protocol_state_machine;
pub mod concurrent_behavior_verifier;
pub mod resource_utilization_analyzer;
pub mod performance_constraint_verifier;

// Formal methods and remediation components  
pub mod formal_semantics;
pub mod soundness_proofs;
pub mod completeness_analysis;
pub mod semantic_preservation;

// Rigorous mathematical foundations
pub mod mathematical_semantics;
pub mod mechanized_proofs;

// Test fixtures and utilities are now handled inline in each module

pub use ast::*;
pub use parser_new::*;
pub use relational_new::*;
pub use semantic::*;
pub use temporal_new::*;
pub use validator::*;
pub use z3_integration::*;
pub use error::*;

/// AISP version supported by this implementation
pub const AISP_VERSION: &str = "5.1";

/// Maximum supported document size (1MB)
pub const MAX_DOCUMENT_SIZE: usize = 1024 * 1024;

/// Quality tier thresholds
pub mod tier_thresholds {
    pub const PLATINUM: f64 = 0.75;
    pub const GOLD: f64 = 0.60;
    pub const SILVER: f64 = 0.40;
    pub const BRONZE: f64 = 0.20;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(AISP_VERSION, "5.1");
    }
}