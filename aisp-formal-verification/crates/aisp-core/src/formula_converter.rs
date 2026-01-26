//! Formula Conversion for AISP Expressions
//!
//! This module handles conversion of AISP logical expressions
//! to formal mathematical formulas for verification.

use crate::ast::*;
use crate::error::*;
use crate::property_types::{FormulaStructure, PropertyFormula, Term, ArithmeticOp, AtomicFormula, PropertyComplexity, PropertyType};
use crate::property_types::Quantifier as PropertyQuantifier;
use std::collections::HashSet;

/// Converts AISP logical expressions to formal mathematical formulas
pub struct FormulaConverter;

impl FormulaConverter {
    /// Convert logical expression to formula structure
    pub fn convert_logical_expression_structure(expr: &LogicalExpression) -> AispResult<FormulaStructure> {
        match expr {
            LogicalExpression::Temporal { op, operand } => {
                let operand_structure = Self::convert_logical_expression_structure(operand)?;
                match op {
                    TemporalOperator::Always => Ok(FormulaStructure::TemporalAlways(Box::new(operand_structure))),
                    TemporalOperator::Eventually => Ok(FormulaStructure::TemporalEventually(Box::new(operand_structure))),
                    _ => Ok(operand_structure), // Simplified for other temporal operators
                }
            }
            LogicalExpression::Binary { op, left, right } => {
                let left_structure = Self::convert_logical_expression_structure(left)?;
                let right_structure = Self::convert_logical_expression_structure(right)?;
                
                match op {
                    BinaryOperator::And => Ok(FormulaStructure::Conjunction(vec![left_structure, right_structure])),
                    BinaryOperator::Or => Ok(FormulaStructure::Disjunction(vec![left_structure, right_structure])),
                    BinaryOperator::Implication => Ok(FormulaStructure::Implication(Box::new(left_structure), Box::new(right_structure))),
                    BinaryOperator::Equals => {
                        Ok(FormulaStructure::ArithmeticEqual(
                            Term::Variable("left".to_string(), None),
                            Term::Variable("right".to_string(), None),
                        ))
                    }
                    _ => {
                        // Default handling for other operators
                        Ok(FormulaStructure::Atomic(AtomicFormula {
                            predicate: format!("{:?}", op),
                            terms: vec![
                                Term::Variable("left".to_string(), None),
                                Term::Variable("right".to_string(), None),
                            ],
                            type_signature: None,
                        }))
                    }
                }
            }
            LogicalExpression::Application { function, arguments } => {
                let terms = arguments.iter().map(|arg| {
                    Term::Variable(format!("{:?}", arg), None)
                }).collect();
                
                Ok(FormulaStructure::FunctionApplication(
                    function.clone(),
                    terms,
                ))
            }
            LogicalExpression::Variable(name) => {
                Ok(FormulaStructure::Atomic(AtomicFormula {
                    predicate: name.clone(),
                    terms: vec![],
                    type_signature: None,
                }))
            }
            LogicalExpression::Constant(value) => {
                Ok(FormulaStructure::Atomic(AtomicFormula {
                    predicate: format!("{:?}", value),
                    terms: vec![],
                    type_signature: None,
                }))
            }
            _ => {
                // Fallback for other expression types
                Ok(FormulaStructure::Atomic(AtomicFormula {
                    predicate: "unknown".to_string(),
                    terms: vec![],
                    type_signature: None,
                }))
            }
        }
    }

    /// Convert rule expression to mathematical formula
    pub fn convert_rule_to_formula(expr: &LogicalExpression) -> AispResult<PropertyFormula> {
        let structure = Self::convert_logical_expression_structure(expr)?;
        
        Ok(PropertyFormula {
            structure,
            quantifiers: Self::extract_quantifiers(expr),
            free_variables: Self::extract_free_variables(expr),
            predicates: Self::extract_predicates(expr),
            functions: Self::extract_functions(expr),
            constants: Self::extract_constants(expr),
        })
    }

    /// Convert logical expression to formula for meta properties
    pub fn convert_logical_expression_to_formula(expr: &LogicalExpression) -> AispResult<PropertyFormula> {
        // Use the same conversion as for rules
        Self::convert_rule_to_formula(expr)
    }

    /// Extract quantifiers from logical expression
    pub fn extract_quantifiers(_expr: &LogicalExpression) -> Vec<PropertyQuantifier> {
        // Simplified implementation - would implement full quantifier extraction
        vec![]
    }

    /// Extract free variables from logical expression
    pub fn extract_free_variables(_expr: &LogicalExpression) -> HashSet<String> {
        // Simplified implementation - would implement full variable extraction
        HashSet::new()
    }

    /// Extract predicate symbols from logical expression
    pub fn extract_predicates(_expr: &LogicalExpression) -> HashSet<String> {
        // Simplified implementation - would implement full predicate extraction
        HashSet::new()
    }

    /// Extract function symbols from logical expression
    pub fn extract_functions(_expr: &LogicalExpression) -> HashSet<String> {
        // Simplified implementation - would implement full function extraction
        HashSet::new()
    }

    /// Extract constants from logical expression
    pub fn extract_constants(_expr: &LogicalExpression) -> HashSet<String> {
        // Simplified implementation - would implement full constant extraction
        HashSet::new()
    }

    /// Analyze complexity of logical expression
    pub fn analyze_logical_expression_complexity(_expr: &LogicalExpression) -> PropertyComplexity {
        // Simplified implementation
        PropertyComplexity {
            quantifier_depth: 1,
            logical_connectives: 2,
            function_applications: 1,
            variable_count: 2,
            difficulty_score: 7,
        }
    }

    /// Analyze rule complexity
    pub fn analyze_rule_complexity(_expr: &LogicalExpression) -> PropertyComplexity {
        // Simplified implementation
        PropertyComplexity {
            quantifier_depth: 1,
            logical_connectives: 1,
            function_applications: 1,
            variable_count: 1,
            difficulty_score: 5,
        }
    }

    /// Check if expression contains relational predicates
    pub fn contains_relational_predicates(_expr: &LogicalExpression) -> bool {
        // Simplified implementation - would analyze expression structure
        false
    }

    /// Check if expression contains arithmetic operations
    pub fn contains_arithmetic_operations(_expr: &LogicalExpression) -> bool {
        // Simplified implementation - would analyze expression structure
        false
    }

    /// Classify the type of property represented by a rule expression
    pub fn classify_rule_property(expr: &LogicalExpression) -> PropertyType {
        match expr {
            LogicalExpression::Temporal { op, .. } => {
                match op {
                    TemporalOperator::Always => PropertyType::TemporalSafety,
                    TemporalOperator::Eventually => PropertyType::TemporalLiveness,
                    _ => PropertyType::LogicalAssertion,
                }
            }
            LogicalExpression::Binary { left, right, .. } => {
                if Self::contains_relational_predicates(left) || Self::contains_relational_predicates(right) {
                    PropertyType::RelationalConstraint
                } else if Self::contains_arithmetic_operations(left) || Self::contains_arithmetic_operations(right) {
                    PropertyType::ArithmeticConstraint
                } else {
                    PropertyType::LogicalAssertion
                }
            }
            _ => PropertyType::LogicalAssertion,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_expression_conversion() {
        let expr = LogicalExpression::Variable("x".to_string());
        let result = FormulaConverter::convert_logical_expression_structure(&expr);
        
        assert!(result.is_ok());
        match result.unwrap() {
            FormulaStructure::Atomic(atomic) => {
                assert_eq!(atomic.predicate, "x");
            }
            _ => panic!("Expected atomic formula"),
        }
    }

    #[test]
    fn test_constant_expression_conversion() {
        let expr = LogicalExpression::Constant(ConstantValue::Number(42.0));
        let result = FormulaConverter::convert_logical_expression_structure(&expr);
        
        assert!(result.is_ok());
        match result.unwrap() {
            FormulaStructure::Atomic(atomic) => {
                assert!(atomic.predicate.contains("42"));
            }
            _ => panic!("Expected atomic formula"),
        }
    }

    #[test]
    fn test_binary_and_expression_conversion() {
        let left = Box::new(LogicalExpression::Variable("P".to_string()));
        let right = Box::new(LogicalExpression::Variable("Q".to_string()));
        let expr = LogicalExpression::Binary {
            op: BinaryOperator::And,
            left,
            right,
        };
        
        let result = FormulaConverter::convert_logical_expression_structure(&expr);
        assert!(result.is_ok());
        
        match result.unwrap() {
            FormulaStructure::Conjunction(formulas) => {
                assert_eq!(formulas.len(), 2);
            }
            _ => panic!("Expected conjunction"),
        }
    }

    #[test]
    fn test_temporal_always_conversion() {
        let operand = Box::new(LogicalExpression::Variable("P".to_string()));
        let expr = LogicalExpression::Temporal {
            op: TemporalOperator::Always,
            operand,
        };
        
        let result = FormulaConverter::convert_logical_expression_structure(&expr);
        assert!(result.is_ok());
        
        match result.unwrap() {
            FormulaStructure::TemporalAlways(_) => {
                // Success
            }
            _ => panic!("Expected temporal always"),
        }
    }

    #[test]
    fn test_rule_property_classification() {
        let temporal_expr = LogicalExpression::Temporal {
            op: TemporalOperator::Always,
            operand: Box::new(LogicalExpression::Variable("P".to_string())),
        };
        
        let property_type = FormulaConverter::classify_rule_property(&temporal_expr);
        assert_eq!(property_type, PropertyType::TemporalSafety);
        
        let liveness_expr = LogicalExpression::Temporal {
            op: TemporalOperator::Eventually,
            operand: Box::new(LogicalExpression::Variable("P".to_string())),
        };
        
        let liveness_type = FormulaConverter::classify_rule_property(&liveness_expr);
        assert_eq!(liveness_type, PropertyType::TemporalLiveness);
    }

    #[test]
    fn test_rule_to_formula_conversion() {
        let expr = LogicalExpression::Variable("P".to_string());
        let result = FormulaConverter::convert_rule_to_formula(&expr);
        
        assert!(result.is_ok());
        let formula = result.unwrap();
        
        match formula.structure {
            FormulaStructure::Atomic(atomic) => {
                assert_eq!(atomic.predicate, "P");
            }
            _ => panic!("Expected atomic formula"),
        }
    }

    #[test]
    fn test_function_application_conversion() {
        let expr = LogicalExpression::Application {
            function: "f".to_string(),
            arguments: vec![LogicalExpression::Variable("x".to_string())],
        };
        
        let result = FormulaConverter::convert_logical_expression_structure(&expr);
        assert!(result.is_ok());
        
        match result.unwrap() {
            FormulaStructure::FunctionApplication(name, terms) => {
                assert_eq!(name, "f");
                assert_eq!(terms.len(), 1);
            }
            _ => panic!("Expected function application"),
        }
    }

    #[test]
    fn test_complexity_analysis() {
        let expr = LogicalExpression::Variable("P".to_string());
        let complexity = FormulaConverter::analyze_rule_complexity(&expr);
        
        assert!(complexity.difficulty_score > 0);
        assert!(complexity.difficulty_score <= 10);
    }

    #[test]
    fn test_extract_operations() {
        let expr = LogicalExpression::Variable("P".to_string());
        
        let quantifiers = FormulaConverter::extract_quantifiers(&expr);
        let variables = FormulaConverter::extract_free_variables(&expr);
        let predicates = FormulaConverter::extract_predicates(&expr);
        
        // These are simplified implementations that return empty collections
        assert_eq!(quantifiers.len(), 0);
        assert_eq!(variables.len(), 0);
        assert_eq!(predicates.len(), 0);
    }
}