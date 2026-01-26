//! AISP Core - High-performance parsing and validation for AI Symbolic Protocol
//!
//! This crate provides the foundational components for parsing and validating
//! AISP 5.1 documents with zero-copy parsing and strong type safety guarantees.

pub mod ast;
pub mod parser;
pub mod relational;
pub mod semantic;
pub mod temporal;
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
pub mod parser_new;

// New modular semantic analysis components
pub mod type_checker;
pub mod symbol_analyzer;
pub mod quality_analyzer;

// New modular relational analysis components
pub mod constraint_solver;
pub mod set_analyzer;
pub mod dependency_analyzer;
pub mod type_graph;
pub mod conflict_detector;
pub mod relational_new;

// New modular temporal analysis components
pub mod temporal_operator_analyzer;
pub mod temporal_pattern_detector;
pub mod temporal_logic_solver;
pub mod temporal_model_checker;
pub mod temporal_new;

// Advanced formal verification components
pub mod property_types;
pub mod formula_converter;
pub mod property_factory;
pub mod property_extractor_main;
pub mod property_extractor;
pub mod smt_types;
pub mod smt_formula_converter;
pub mod smt_generator_main;
pub mod smt_generator;
pub mod proof_types;
pub mod axiom_system;
pub mod proof_search;
pub mod theorem_prover_main;
pub mod theorem_prover;
pub mod model_checker;

// Test fixtures and utilities
#[cfg(test)]
pub mod test_fixtures;

pub use ast::*;
pub use parser::*;
pub use relational::*;
pub use semantic::*;
pub use temporal::*;
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