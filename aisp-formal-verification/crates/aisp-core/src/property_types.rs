//! Property Types and Structures for Formal Verification
//!
//! This module defines the core data structures for representing
//! extracted formal properties from AISP documents.

use std::collections::{HashMap, HashSet};

/// Extracted formal property ready for verification
#[derive(Debug, Clone)]
pub struct ExtractedProperty {
    /// Unique property identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Property type classification
    pub property_type: PropertyType,
    /// Mathematical formula representation
    pub formula: PropertyFormula,
    /// Context information for verification
    pub context: PropertyContext,
    /// Source location in AISP document
    pub source_location: SourceLocation,
    /// Complexity metrics
    pub complexity: PropertyComplexity,
}

/// Types of extractable formal properties
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyType {
    /// Type safety invariant (∀x:T. P(x))
    TypeSafety,
    /// Function correctness property (pre/post conditions)
    FunctionalCorrectness,
    /// Temporal safety property (□P)
    TemporalSafety,
    /// Temporal liveness property (◊P)
    TemporalLiveness,
    /// Relational constraint (∀x,y. R(x,y) → P(x,y))
    RelationalConstraint,
    /// Set membership property (x ∈ S → P(x))
    SetMembership,
    /// Arithmetic constraint (x + y = z → P(x,y,z))
    ArithmeticConstraint,
    /// Logical assertion from rules (∀x. P(x) ⇒ Q(x))
    LogicalAssertion,
    /// Structural invariant (well-formedness)
    StructuralInvariant,
    /// Dependency constraint (A depends on B)
    DependencyConstraint,
}

/// Mathematical formula representation
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyFormula {
    /// Logical structure of the formula
    pub structure: FormulaStructure,
    /// Quantified variables
    pub quantifiers: Vec<Quantifier>,
    /// Free variables
    pub free_variables: HashSet<String>,
    /// Predicate symbols
    pub predicates: HashSet<String>,
    /// Function symbols
    pub functions: HashSet<String>,
    /// Constants
    pub constants: HashSet<String>,
}

/// Logical structure of formulas
#[derive(Debug, Clone, PartialEq)]
pub enum FormulaStructure {
    /// Atomic proposition (P(x))
    Atomic(AtomicFormula),
    /// Negation (¬P)
    Negation(Box<FormulaStructure>),
    /// Conjunction (P ∧ Q)
    Conjunction(Vec<FormulaStructure>),
    /// Disjunction (P ∨ Q)
    Disjunction(Vec<FormulaStructure>),
    /// Implication (P → Q)
    Implication(Box<FormulaStructure>, Box<FormulaStructure>),
    /// Biconditional (P ↔ Q)
    Biconditional(Box<FormulaStructure>, Box<FormulaStructure>),
    /// Universal quantification (∀x. P(x))
    Universal(Quantifier, Box<FormulaStructure>),
    /// Existential quantification (∃x. P(x))
    Existential(Quantifier, Box<FormulaStructure>),
    /// Temporal always (□P)
    TemporalAlways(Box<FormulaStructure>),
    /// Temporal eventually (◊P)
    TemporalEventually(Box<FormulaStructure>),
    /// Temporal until (P U Q)
    TemporalUntil(Box<FormulaStructure>, Box<FormulaStructure>),
    /// Arithmetic equality (x = y)
    ArithmeticEqual(Term, Term),
    /// Arithmetic inequality (x ≤ y)
    ArithmeticLessEqual(Term, Term),
    /// Set membership (x ∈ S)
    SetMembership(Term, Term),
    /// Function application (f(x))
    FunctionApplication(String, Vec<Term>),
}

/// Atomic formula (predicate application)
#[derive(Debug, Clone, PartialEq)]
pub struct AtomicFormula {
    pub predicate: String,
    pub terms: Vec<Term>,
    pub type_signature: Option<TypeSignature>,
}

/// Mathematical terms
#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    /// Variable (x)
    Variable(String, Option<String>), // name, type
    /// Constant (42, "hello")
    Constant(String, String), // value, type
    /// Function application (f(x,y))
    Function(String, Vec<Term>),
    /// Arithmetic expression (x + y)
    Arithmetic(ArithmeticOp, Box<Term>, Box<Term>),
    /// Set constructor ({a, b, c})
    Set(Vec<Term>),
    /// Array access (arr[i])
    ArrayAccess(Box<Term>, Box<Term>),
}

/// Arithmetic operations
#[derive(Debug, Clone, PartialEq)]
pub enum ArithmeticOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
}

/// Quantifier information
#[derive(Debug, Clone, PartialEq)]
pub struct Quantifier {
    /// Variable being quantified
    pub variable: String,
    /// Type of the variable
    pub variable_type: Option<String>,
    /// Domain restriction (optional)
    pub domain: Option<Term>,
}

/// Type signature for predicates and functions
#[derive(Debug, Clone, PartialEq)]
pub struct TypeSignature {
    /// Input types
    pub inputs: Vec<String>,
    /// Output type
    pub output: String,
}

/// Context information for property verification
#[derive(Debug, Clone)]
pub struct PropertyContext {
    /// Type definitions available in scope
    pub type_definitions: HashMap<String, String>,
    /// Function definitions available in scope
    pub function_definitions: HashMap<String, String>,
    /// Constants available in scope
    pub constants: HashMap<String, String>,
    /// Dependencies on other properties
    pub dependencies: Vec<String>,
}

/// Source location in AISP document
#[derive(Debug, Clone)]
pub struct SourceLocation {
    /// Block type where property was found
    pub block_type: String,
    /// Line number (if available)
    pub line: Option<usize>,
    /// Column number (if available)
    pub column: Option<usize>,
    /// Source text snippet
    pub source_text: Option<String>,
}

/// Property complexity metrics
#[derive(Debug, Clone)]
pub struct PropertyComplexity {
    /// Number of quantifiers
    pub quantifier_depth: usize,
    /// Number of logical connectives
    pub logical_connectives: usize,
    /// Number of function applications
    pub function_applications: usize,
    /// Number of variables
    pub variable_count: usize,
    /// Estimated verification difficulty (1-10)
    pub difficulty_score: u8,
}

impl Default for PropertyComplexity {
    fn default() -> Self {
        Self {
            quantifier_depth: 0,
            logical_connectives: 0,
            function_applications: 0,
            variable_count: 0,
            difficulty_score: 1,
        }
    }
}

/// Property extraction statistics
#[derive(Debug, Clone)]
pub struct PropertyExtractionStats {
    pub total_properties: usize,
    pub type_properties: usize,
    pub function_properties: usize,
    pub temporal_properties: usize,
    pub relational_properties: usize,
    pub average_complexity: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_type_classification() {
        let type_safety = PropertyType::TypeSafety;
        let temporal_safety = PropertyType::TemporalSafety;
        assert_ne!(type_safety, temporal_safety);
    }

    #[test]
    fn test_property_complexity_calculation() {
        let complexity = PropertyComplexity {
            quantifier_depth: 2,
            logical_connectives: 3,
            function_applications: 1,
            variable_count: 4,
            difficulty_score: 6,
        };
        assert_eq!(complexity.difficulty_score, 6);
        assert_eq!(complexity.quantifier_depth, 2);
    }

    #[test]
    fn test_formula_structure_creation() {
        let atomic = FormulaStructure::Atomic(AtomicFormula {
            predicate: "test".to_string(),
            terms: vec![],
            type_signature: None,
        });
        
        matches!(atomic, FormulaStructure::Atomic(_));
    }

    #[test]
    fn test_quantifier_creation() {
        let quantifier = Quantifier {
            variable: "x".to_string(),
            variable_type: Some("Int".to_string()),
            domain: None,
        };
        
        assert_eq!(quantifier.variable, "x");
        assert_eq!(quantifier.variable_type, Some("Int".to_string()));
    }

    #[test]
    fn test_term_variable_creation() {
        let var_term = Term::Variable("x".to_string(), Some("Int".to_string()));
        match var_term {
            Term::Variable(name, type_opt) => {
                assert_eq!(name, "x");
                assert_eq!(type_opt, Some("Int".to_string()));
            }
            _ => panic!("Expected Variable term"),
        }
    }

    #[test]
    fn test_arithmetic_operations() {
        assert_eq!(ArithmeticOp::Add, ArithmeticOp::Add);
        assert_ne!(ArithmeticOp::Add, ArithmeticOp::Subtract);
    }

    #[test]
    fn test_property_context_creation() {
        let mut context = PropertyContext {
            type_definitions: HashMap::new(),
            function_definitions: HashMap::new(),
            constants: HashMap::new(),
            dependencies: Vec::new(),
        };
        
        context.type_definitions.insert("Int".to_string(), "Integer".to_string());
        assert_eq!(context.type_definitions.len(), 1);
    }

    #[test]
    fn test_source_location() {
        let loc = SourceLocation {
            block_type: "Types".to_string(),
            line: Some(42),
            column: Some(10),
            source_text: Some("test code".to_string()),
        };
        
        assert_eq!(loc.block_type, "Types");
        assert_eq!(loc.line, Some(42));
    }

    #[test]
    fn test_extracted_property_creation() {
        // Inline test utility - replaced test_fixtures
        use std::collections::HashMap;
        
        let property = ExtractedProperty {
            id: "prop_1".to_string(),
            name: "test_property".to_string(),
            formula: PropertyFormula {
                structure: FormulaStructure::Atomic(AtomicFormula {
                    predicate: "P".to_string(),
                    terms: vec![],
                    type_signature: None,
                }),
                quantifiers: vec![],
                free_variables: HashSet::new(),
                predicates: HashSet::new(),
                functions: HashSet::new(),
                constants: HashSet::new(),
            },
            property_type: PropertyType::TypeSafety,
            context: PropertyContext {
                type_definitions: HashMap::new(),
                function_definitions: HashMap::new(),
                constants: HashMap::new(),
                dependencies: vec![],
            },
            complexity: PropertyComplexity::default(),
            source_location: SourceLocation {
                block_type: "test".to_string(),
                line: Some(1),
                column: Some(1),
                source_text: Some("test property".to_string()),
            },
        };
        
        assert_eq!(property.id, "prop_1");
        assert_eq!(property.property_type, PropertyType::TypeSafety);
        
        // Verify formula structure
        match &property.formula.structure {
            FormulaStructure::Atomic(atomic) => {
                assert_eq!(atomic.predicate, "P");
            }
            _ => panic!("Expected atomic formula"),
        }
    }
}