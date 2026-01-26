//! Property Factory for Creating Formal Properties
//!
//! This module handles the creation of specific types of formal properties
//! from AISP constructs like type definitions and function signatures.

use crate::ast::*;
use crate::error::*;
use crate::property_types::{PropertyFormula, FormulaStructure, AtomicFormula, Term, PropertyComplexity, ExtractedProperty, PropertyType, PropertyContext, SourceLocation};
use crate::property_types::Quantifier as PropertyQuantifier;
use std::collections::{HashMap, HashSet};

/// Factory for creating formal properties from AISP constructs
pub struct PropertyFactory {
    /// Property ID counter
    next_id: usize,
}

impl PropertyFactory {
    /// Create new property factory
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    /// Generate next property ID
    pub fn next_property_id(&mut self) -> String {
        let id = format!("prop_{}", self.next_id);
        self.next_id += 1;
        id
    }

    /// Create type safety formula: ∀x. hasType(x, T) → wellFormed(x)
    pub fn create_type_safety_formula(&self, type_name: &str, _type_expr: &TypeExpression) -> AispResult<PropertyFormula> {
        let quantifier = PropertyQuantifier {
            variable: "x".to_string(),
            variable_type: Some(type_name.to_string()),
            domain: None,
        };

        let has_type = FormulaStructure::FunctionApplication(
            "hasType".to_string(),
            vec![
                Term::Variable("x".to_string(), Some(type_name.to_string())),
                Term::Constant(type_name.to_string(), "Type".to_string()),
            ],
        );

        let well_formed = FormulaStructure::FunctionApplication(
            "wellFormed".to_string(),
            vec![Term::Variable("x".to_string(), Some(type_name.to_string()))],
        );

        let structure = FormulaStructure::Universal(
            quantifier.clone(),
            Box::new(FormulaStructure::Implication(
                Box::new(has_type),
                Box::new(well_formed),
            )),
        );

        Ok(PropertyFormula {
            structure,
            quantifiers: vec![quantifier],
            free_variables: HashSet::new(),
            predicates: [&"hasType", &"wellFormed"].iter().map(|s| s.to_string()).collect(),
            functions: HashSet::new(),
            constants: [type_name].iter().map(|s| s.to_string()).collect(),
        })
    }

    /// Create structural invariant for struct types
    pub fn create_structural_invariant(&mut self, type_name: &str, _fields: &[(String, TypeExpression)]) -> AispResult<ExtractedProperty> {
        let quantifier = PropertyQuantifier {
            variable: "x".to_string(),
            variable_type: Some(type_name.to_string()),
            domain: None,
        };

        let structure = FormulaStructure::Universal(
            quantifier.clone(),
            Box::new(FormulaStructure::FunctionApplication(
                "structurallyValid".to_string(),
                vec![Term::Variable("x".to_string(), Some(type_name.to_string()))],
            )),
        );

        Ok(ExtractedProperty {
            id: self.next_property_id(),
            name: format!("{}_structural_invariant", type_name),
            property_type: PropertyType::StructuralInvariant,
            formula: PropertyFormula {
                structure,
                quantifiers: vec![quantifier],
                free_variables: HashSet::new(),
                predicates: [&"structurallyValid"].iter().map(|s| s.to_string()).collect(),
                functions: HashSet::new(),
                constants: [type_name].iter().map(|s| s.to_string()).collect(),
            },
            context: Self::create_empty_context(),
            source_location: SourceLocation {
                block_type: "Types".to_string(),
                line: None,
                column: None,
                source_text: Some(format!("{} struct invariant", type_name)),
            },
            complexity: PropertyComplexity {
                quantifier_depth: 1,
                logical_connectives: 0,
                function_applications: 1,
                variable_count: 1,
                difficulty_score: 2,
            },
        })
    }

    /// Create enumeration membership property
    pub fn create_enumeration_property(&mut self, type_name: &str, values: &[String]) -> AispResult<ExtractedProperty> {
        let quantifier = PropertyQuantifier {
            variable: "x".to_string(),
            variable_type: Some(type_name.to_string()),
            domain: None,
        };

        // Create disjunction of all possible values
        let value_checks: Vec<FormulaStructure> = values
            .iter()
            .map(|value| {
                FormulaStructure::ArithmeticEqual(
                    Term::Variable("x".to_string(), Some(type_name.to_string())),
                    Term::Constant(value.clone(), type_name.to_string()),
                )
            })
            .collect();

        let structure = FormulaStructure::Universal(
            quantifier.clone(),
            Box::new(FormulaStructure::Disjunction(value_checks)),
        );

        Ok(ExtractedProperty {
            id: self.next_property_id(),
            name: format!("{}_membership", type_name),
            property_type: PropertyType::SetMembership,
            formula: PropertyFormula {
                structure,
                quantifiers: vec![quantifier],
                free_variables: HashSet::new(),
                predicates: HashSet::new(),
                functions: HashSet::new(),
                constants: values.iter().cloned().collect(),
            },
            context: Self::create_empty_context(),
            source_location: SourceLocation {
                block_type: "Types".to_string(),
                line: None,
                column: None,
                source_text: Some(format!("{} ∈ {{{}}}", type_name, values.join(", "))),
            },
            complexity: PropertyComplexity {
                quantifier_depth: 1,
                logical_connectives: values.len().saturating_sub(1),
                function_applications: 0,
                variable_count: 1,
                difficulty_score: 2,
            },
        })
    }

    /// Create function well-defined formula
    pub fn create_function_well_defined_formula(&self, _func_name: &str, _lambda: &LambdaExpression) -> AispResult<PropertyFormula> {
        // Simplified implementation
        Ok(PropertyFormula {
            structure: FormulaStructure::Atomic(AtomicFormula {
                predicate: "wellDefined".to_string(),
                terms: vec![],
                type_signature: None,
            }),
            quantifiers: vec![],
            free_variables: HashSet::new(),
            predicates: [&"wellDefined"].iter().map(|s| s.to_string()).collect(),
            functions: HashSet::new(),
            constants: HashSet::new(),
        })
    }

    /// Create totality property for functions
    pub fn create_totality_property(&mut self, func_name: &str, lambda: &LambdaExpression) -> AispResult<ExtractedProperty> {
        Ok(ExtractedProperty {
            id: self.next_property_id(),
            name: format!("{}_totality", func_name),
            property_type: PropertyType::FunctionalCorrectness,
            formula: PropertyFormula {
                structure: FormulaStructure::Atomic(AtomicFormula {
                    predicate: "total".to_string(),
                    terms: vec![Term::Variable(func_name.to_string(), None)],
                    type_signature: None,
                }),
                quantifiers: vec![],
                free_variables: HashSet::new(),
                predicates: [&"total"].iter().map(|s| s.to_string()).collect(),
                functions: HashSet::new(),
                constants: [func_name].iter().map(|s| s.to_string()).collect(),
            },
            context: Self::create_empty_context(),
            source_location: SourceLocation {
                block_type: "Functions".to_string(),
                line: None,
                column: None,
                source_text: Some(format!("{} totality", func_name)),
            },
            complexity: PropertyComplexity {
                quantifier_depth: lambda.parameters.len(),
                logical_connectives: 1,
                function_applications: 1,
                variable_count: lambda.parameters.len(),
                difficulty_score: 6,
            },
        })
    }

    /// Create structural invariant for tuple types
    pub fn create_tuple_structural_invariant(&mut self, type_name: &str, fields: &Vec<TypeExpression>) -> AispResult<ExtractedProperty> {
        let quantifier = PropertyQuantifier {
            variable: "x".to_string(),
            variable_type: Some(type_name.to_string()),
            domain: None,
        };
        let structure = FormulaStructure::Universal(
            quantifier.clone(),
            Box::new(FormulaStructure::FunctionApplication(
                "structurallyValid".to_string(),
                vec![Term::Variable("x".to_string(), Some(type_name.to_string()))],
            )),
        );
        Ok(ExtractedProperty {
            id: self.next_property_id(),
            name: format!("{}_structural_invariant", type_name),
            property_type: PropertyType::StructuralInvariant,
            formula: PropertyFormula {
                structure,
                quantifiers: vec![quantifier],
                free_variables: HashSet::new(),
                predicates: [&"structurallyValid"].iter().map(|s| s.to_string()).collect(),
                functions: HashSet::new(),
                constants: [type_name].iter().map(|s| s.to_string()).collect(),
            },
            context: Self::create_empty_context(),
            source_location: SourceLocation {
                block_type: "Types".to_string(),
                line: None,
                column: None,
                source_text: Some(format!("{} structural invariant", type_name)),
            },
            complexity: PropertyComplexity {
                quantifier_depth: 1,
                logical_connectives: 1,
                function_applications: 1,
                variable_count: fields.len(),
                difficulty_score: 2 + (fields.len().min(5) as u8),
            },
        })
    }

    /// Create empty context
    pub fn create_empty_context() -> PropertyContext {
        PropertyContext {
            type_definitions: HashMap::new(),
            function_definitions: HashMap::new(),
            constants: HashMap::new(),
            dependencies: vec![],
        }
    }

    /// Should generate totality property for function
    pub fn should_generate_totality_property(&self, _lambda: &LambdaExpression) -> bool {
        true // Generate for all functions in this implementation
    }

    /// Reset factory state
    pub fn reset(&mut self) {
        self.next_id = 0;
    }
}

impl Default for PropertyFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;

    fn create_test_span() -> Span {
        Span::new(1, 1, 1, 10)
    }

    #[test]
    fn test_property_factory_creation() {
        let factory = PropertyFactory::new();
        assert_eq!(factory.next_id, 0);
    }

    #[test]
    fn test_property_id_generation() {
        let mut factory = PropertyFactory::new();
        let id1 = factory.next_property_id();
        let id2 = factory.next_property_id();
        
        assert_eq!(id1, "prop_0");
        assert_eq!(id2, "prop_1");
    }

    #[test]
    fn test_type_safety_formula_creation() -> AispResult<()> {
        let factory = PropertyFactory::new();
        let formula = factory.create_type_safety_formula("Int", &TypeExpression::Basic(BasicType::Integer))?;
        
        match formula.structure {
            FormulaStructure::Universal(quantifier, _) => {
                assert_eq!(quantifier.variable, "x");
                assert_eq!(quantifier.variable_type, Some("Int".to_string()));
            }
            _ => panic!("Expected universal quantification"),
        }
        
        assert!(formula.predicates.contains("hasType"));
        assert!(formula.predicates.contains("wellFormed"));
        assert!(formula.constants.contains("Int"));
        
        Ok(())
    }

    #[test]
    fn test_structural_invariant_creation() -> AispResult<()> {
        let mut factory = PropertyFactory::new();
        let fields = vec![
            ("x".to_string(), TypeExpression::Basic(BasicType::Integer)),
            ("y".to_string(), TypeExpression::Basic(BasicType::Integer)),
        ];
        
        let property = factory.create_structural_invariant("Point", &fields)?;
        
        assert_eq!(property.name, "Point_structural_invariant");
        assert_eq!(property.property_type, PropertyType::StructuralInvariant);
        assert_eq!(property.complexity.quantifier_depth, 1);
        assert_eq!(property.complexity.difficulty_score, 2);
        
        Ok(())
    }

    #[test]
    fn test_enumeration_property_creation() -> AispResult<()> {
        let mut factory = PropertyFactory::new();
        let values = vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()];
        
        let property = factory.create_enumeration_property("Color", &values)?;
        
        assert_eq!(property.name, "Color_membership");
        assert_eq!(property.property_type, PropertyType::SetMembership);
        
        match property.formula.structure {
            FormulaStructure::Universal(_, inner) => {
                match inner.as_ref() {
                    FormulaStructure::Disjunction(disjuncts) => {
                        assert_eq!(disjuncts.len(), 3);
                    }
                    _ => panic!("Expected disjunction inside universal"),
                }
            }
            _ => panic!("Expected universal quantification"),
        }
        
        assert!(property.formula.constants.contains("Red"));
        assert!(property.formula.constants.contains("Green"));
        assert!(property.formula.constants.contains("Blue"));
        
        Ok(())
    }

    #[test]
    fn test_function_well_defined_formula() -> AispResult<()> {
        let factory = PropertyFactory::new();
        let lambda = LambdaExpression {
            parameters: vec!["x".to_string()],
            body: LogicalExpression::Variable("x".to_string()),
            span: create_test_span(),
        };
        
        let formula = factory.create_function_well_defined_formula("f", &lambda)?;
        
        match formula.structure {
            FormulaStructure::Atomic(atomic) => {
                assert_eq!(atomic.predicate, "wellDefined");
            }
            _ => panic!("Expected atomic formula"),
        }
        
        assert!(formula.predicates.contains("wellDefined"));
        
        Ok(())
    }

    #[test]
    fn test_totality_property_creation() -> AispResult<()> {
        let mut factory = PropertyFactory::new();
        let lambda = LambdaExpression {
            parameters: vec!["x".to_string(), "y".to_string()],
            body: LogicalExpression::Variable("result".to_string()),
            span: create_test_span(),
        };
        
        let property = factory.create_totality_property("add", &lambda)?;
        
        assert_eq!(property.name, "add_totality");
        assert_eq!(property.property_type, PropertyType::FunctionalCorrectness);
        assert_eq!(property.complexity.variable_count, 2); // Two parameters
        
        Ok(())
    }

    #[test]
    fn test_should_generate_totality() {
        let factory = PropertyFactory::new();
        let lambda = LambdaExpression {
            parameters: vec!["x".to_string()],
            body: LogicalExpression::Variable("x".to_string()),
            span: create_test_span(),
        };
        
        assert!(factory.should_generate_totality_property(&lambda));
    }

    #[test]
    fn test_factory_reset() {
        let mut factory = PropertyFactory::new();
        
        // Generate some IDs
        factory.next_property_id();
        factory.next_property_id();
        assert_eq!(factory.next_id, 2);
        
        // Reset and verify
        factory.reset();
        assert_eq!(factory.next_id, 0);
        
        let id = factory.next_property_id();
        assert_eq!(id, "prop_0");
    }

    #[test]
    fn test_empty_context_creation() {
        let context = PropertyFactory::create_empty_context();
        
        assert!(context.type_definitions.is_empty());
        assert!(context.function_definitions.is_empty());
        assert!(context.constants.is_empty());
        assert!(context.dependencies.is_empty());
    }
}