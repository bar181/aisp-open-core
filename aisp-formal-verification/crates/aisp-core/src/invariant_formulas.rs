//! Invariant Formula Construction
//!
//! This module handles the construction of mathematical formulas
//! representing discovered invariants.

use crate::{
    error::AispResult,
    property_types::{
        PropertyFormula, FormulaStructure, AtomicFormula, Term, 
        Quantifier as PropQuantifier,
    },
};
use std::collections::HashSet;

/// Create a non-negativity formula for natural number types
/// Formula: ∀x:TypeName → x ≥ 0
pub fn create_non_negativity_formula(type_name: &str) -> AispResult<PropertyFormula> {
    Ok(PropertyFormula {
        structure: FormulaStructure::Universal(
            PropQuantifier {
                variable: "x".to_string(),
                variable_type: Some(type_name.to_string()),
                domain: None,
            },
            Box::new(FormulaStructure::Atomic(AtomicFormula {
                predicate: "≥".to_string(),
                terms: vec![
                    Term::Variable("x".to_string(), Some(type_name.to_string())),
                    Term::Constant("0".to_string(), "ℕ".to_string()),
                ],
                type_signature: None,
            }))
        ),
        quantifiers: vec![PropQuantifier {
            variable: "x".to_string(),
            variable_type: Some(type_name.to_string()),
            domain: None,
        }],
        free_variables: HashSet::new(),
        predicates: {
            let mut set = HashSet::new();
            set.insert("≥".to_string());
            set
        },
        functions: HashSet::new(),
        constants: {
            let mut set = HashSet::new();
            set.insert("0".to_string());
            set
        },
    })
}

/// Create a membership formula for enumeration types
/// Formula: ∀x:TypeName → x ∈ {variant1, variant2, ...}
pub fn create_membership_formula(type_name: &str, variants: &[String]) -> AispResult<PropertyFormula> {
    Ok(PropertyFormula {
        structure: FormulaStructure::Universal(
            PropQuantifier {
                variable: "x".to_string(),
                variable_type: Some(type_name.to_string()),
                domain: None,
            },
            Box::new(FormulaStructure::Atomic(AtomicFormula {
                predicate: "∈".to_string(),
                terms: vec![
                    Term::Variable("x".to_string(), Some(type_name.to_string())),
                    Term::Constant(format!("{{{}}}", variants.join(",")), type_name.to_string()),
                ],
                type_signature: None,
            }))
        ),
        quantifiers: vec![PropQuantifier {
            variable: "x".to_string(),
            variable_type: Some(type_name.to_string()),
            domain: None,
        }],
        free_variables: HashSet::new(),
        predicates: {
            let mut set = HashSet::new();
            set.insert("∈".to_string());
            set
        },
        functions: HashSet::new(),
        constants: {
            let mut set = HashSet::new();
            set.insert(format!("{{{}}}", variants.join(",")));
            set
        },
    })
}

/// Create a well-formed formula for generic types
/// Formula: ∀x:TypeName → WellFormed(x)
pub fn create_well_formed_formula(type_name: &str) -> AispResult<PropertyFormula> {
    Ok(PropertyFormula {
        structure: FormulaStructure::Universal(
            PropQuantifier {
                variable: "x".to_string(),
                variable_type: Some(type_name.to_string()),
                domain: None,
            },
            Box::new(FormulaStructure::Atomic(AtomicFormula {
                predicate: "WellFormed".to_string(),
                terms: vec![
                    Term::Variable("x".to_string(), Some(type_name.to_string())),
                ],
                type_signature: None,
            }))
        ),
        quantifiers: vec![PropQuantifier {
            variable: "x".to_string(),
            variable_type: Some(type_name.to_string()),
            domain: None,
        }],
        free_variables: HashSet::new(),
        predicates: {
            let mut set = HashSet::new();
            set.insert("WellFormed".to_string());
            set
        },
        functions: HashSet::new(),
        constants: HashSet::new(),
    })
}

/// Create a range constraint formula
/// Formula: ∀x:TypeName → min ≤ x ≤ max
pub fn create_range_formula(type_name: &str, min: i64, max: i64) -> AispResult<PropertyFormula> {
    // Create conjunction of two comparisons
    let min_constraint = FormulaStructure::Atomic(AtomicFormula {
        predicate: "≥".to_string(),
        terms: vec![
            Term::Variable("x".to_string(), Some(type_name.to_string())),
            Term::Constant(min.to_string(), "ℤ".to_string()),
        ],
        type_signature: None,
    });

    let max_constraint = FormulaStructure::Atomic(AtomicFormula {
        predicate: "≤".to_string(),
        terms: vec![
            Term::Variable("x".to_string(), Some(type_name.to_string())),
            Term::Constant(max.to_string(), "ℤ".to_string()),
        ],
        type_signature: None,
    });

    Ok(PropertyFormula {
        structure: FormulaStructure::Universal(
            PropQuantifier {
                variable: "x".to_string(),
                variable_type: Some(type_name.to_string()),
                domain: None,
            },
            Box::new(FormulaStructure::Conjunction(vec![min_constraint, max_constraint]))
        ),
        quantifiers: vec![PropQuantifier {
            variable: "x".to_string(),
            variable_type: Some(type_name.to_string()),
            domain: None,
        }],
        free_variables: HashSet::new(),
        predicates: {
            let mut set = HashSet::new();
            set.insert("≥".to_string());
            set.insert("≤".to_string());
            set
        },
        functions: HashSet::new(),
        constants: {
            let mut set = HashSet::new();
            set.insert(min.to_string());
            set.insert(max.to_string());
            set
        },
    })
}

/// Create an identity function formula
/// Formula: ∀x → f(x) = x
pub fn create_identity_formula(function_name: &str, type_name: &str) -> AispResult<PropertyFormula> {
    Ok(PropertyFormula {
        structure: FormulaStructure::Universal(
            PropQuantifier {
                variable: "x".to_string(),
                variable_type: Some(type_name.to_string()),
                domain: None,
            },
            Box::new(FormulaStructure::Atomic(AtomicFormula {
                predicate: "=".to_string(),
                terms: vec![
                    Term::Function(function_name.to_string(), vec![
                        Term::Variable("x".to_string(), Some(type_name.to_string()))
                    ]),
                    Term::Variable("x".to_string(), Some(type_name.to_string())),
                ],
                type_signature: None,
            }))
        ),
        quantifiers: vec![PropQuantifier {
            variable: "x".to_string(),
            variable_type: Some(type_name.to_string()),
            domain: None,
        }],
        free_variables: HashSet::new(),
        predicates: {
            let mut set = HashSet::new();
            set.insert("=".to_string());
            set
        },
        functions: {
            let mut set = HashSet::new();
            set.insert(function_name.to_string());
            set
        },
        constants: HashSet::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_non_negativity_formula() {
        let formula = create_non_negativity_formula("Natural").unwrap();
        
        // Check that it's a universal quantification
        match &formula.structure {
            FormulaStructure::Universal(quantifier, body) => {
                assert_eq!(quantifier.variable, "x");
                assert_eq!(quantifier.variable_type, Some("Natural".to_string()));
                
                // Check the body is an atomic formula with ≥ predicate
                match body.as_ref() {
                    FormulaStructure::Atomic(atomic) => {
                        assert_eq!(atomic.predicate, "≥");
                        assert_eq!(atomic.terms.len(), 2);
                    }
                    _ => panic!("Expected atomic formula"),
                }
            }
            _ => panic!("Expected universal quantification"),
        }
        
        // Check predicates and constants
        assert!(formula.predicates.contains("≥"));
        assert!(formula.constants.contains("0"));
    }

    #[test]
    fn test_create_membership_formula() {
        let variants = vec!["Active".to_string(), "Inactive".to_string()];
        let formula = create_membership_formula("Status", &variants).unwrap();
        
        // Check that it's a universal quantification
        match &formula.structure {
            FormulaStructure::Universal(quantifier, body) => {
                assert_eq!(quantifier.variable, "x");
                assert_eq!(quantifier.variable_type, Some("Status".to_string()));
                
                // Check the body is an atomic formula with ∈ predicate
                match body.as_ref() {
                    FormulaStructure::Atomic(atomic) => {
                        assert_eq!(atomic.predicate, "∈");
                        assert_eq!(atomic.terms.len(), 2);
                    }
                    _ => panic!("Expected atomic formula"),
                }
            }
            _ => panic!("Expected universal quantification"),
        }
        
        // Check predicates and constants
        assert!(formula.predicates.contains("∈"));
        assert!(formula.constants.contains("{Active,Inactive}"));
    }

    #[test]
    fn test_create_well_formed_formula() {
        let formula = create_well_formed_formula("CustomType").unwrap();
        
        match &formula.structure {
            FormulaStructure::Universal(quantifier, body) => {
                assert_eq!(quantifier.variable, "x");
                assert_eq!(quantifier.variable_type, Some("CustomType".to_string()));
                
                match body.as_ref() {
                    FormulaStructure::Atomic(atomic) => {
                        assert_eq!(atomic.predicate, "WellFormed");
                        assert_eq!(atomic.terms.len(), 1);
                    }
                    _ => panic!("Expected atomic formula"),
                }
            }
            _ => panic!("Expected universal quantification"),
        }
        
        assert!(formula.predicates.contains("WellFormed"));
    }

    #[test]
    fn test_create_range_formula() {
        let formula = create_range_formula("Counter", 0, 100).unwrap();
        
        match &formula.structure {
            FormulaStructure::Universal(quantifier, body) => {
                assert_eq!(quantifier.variable, "x");
                assert_eq!(quantifier.variable_type, Some("Counter".to_string()));
                
                // Should be a conjunction of two constraints
                match body.as_ref() {
                    FormulaStructure::Conjunction(constraints) => {
                        assert_eq!(constraints.len(), 2);
                    }
                    _ => panic!("Expected conjunction"),
                }
            }
            _ => panic!("Expected universal quantification"),
        }
        
        // Check predicates and constants
        assert!(formula.predicates.contains("≥"));
        assert!(formula.predicates.contains("≤"));
        assert!(formula.constants.contains("0"));
        assert!(formula.constants.contains("100"));
    }

    #[test]
    fn test_create_identity_formula() {
        let formula = create_identity_formula("identity", "Any").unwrap();
        
        match &formula.structure {
            FormulaStructure::Universal(quantifier, body) => {
                assert_eq!(quantifier.variable, "x");
                assert_eq!(quantifier.variable_type, Some("Any".to_string()));
                
                match body.as_ref() {
                    FormulaStructure::Atomic(atomic) => {
                        assert_eq!(atomic.predicate, "=");
                        assert_eq!(atomic.terms.len(), 2);
                        
                        // First term should be function application
                        match &atomic.terms[0] {
                            Term::Function(name, args) => {
                                assert_eq!(name, "identity");
                                assert_eq!(args.len(), 1);
                            }
                            _ => panic!("Expected function term"),
                        }
                    }
                    _ => panic!("Expected atomic formula"),
                }
            }
            _ => panic!("Expected universal quantification"),
        }
        
        assert!(formula.predicates.contains("="));
        assert!(formula.functions.contains("identity"));
    }

    #[test]
    fn test_formula_quantifiers_consistency() {
        let formula = create_non_negativity_formula("Test").unwrap();
        
        // Quantifiers field should match the structure
        assert_eq!(formula.quantifiers.len(), 1);
        assert_eq!(formula.quantifiers[0].variable, "x");
        assert_eq!(formula.quantifiers[0].variable_type, Some("Test".to_string()));
        
        // Free variables should be empty for universally quantified formulas
        assert!(formula.free_variables.is_empty());
    }
}