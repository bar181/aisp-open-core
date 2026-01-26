//! Type checking for AISP documents
//!
//! This module provides type validation and inference for AISP type systems,
//! ensuring type safety across the document.

use crate::ast::*;
use crate::error::*;
use std::collections::{HashMap, HashSet};

/// Type checker for AISP documents
pub struct TypeChecker {
    /// Type definitions from the Types block
    type_definitions: HashMap<String, TypeExpression>,
    /// Error accumulator
    errors: Vec<AispError>,
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new() -> Self {
        let mut type_checker = Self {
            type_definitions: HashMap::new(),
            errors: Vec::new(),
        };
        
        // Add built-in mathematical types
        type_checker.add_builtin_types();
        type_checker
    }
    
    /// Add built-in AISP mathematical types
    fn add_builtin_types(&mut self) {
        // Vector space types commonly used in AISP
        self.type_definitions.insert(
            "VectorSpace768".to_string(),
            TypeExpression::Basic(BasicType::VectorSpace(768)),
        );
        self.type_definitions.insert(
            "VectorSpace512".to_string(),
            TypeExpression::Basic(BasicType::VectorSpace(512)),
        );
        self.type_definitions.insert(
            "VectorSpace256".to_string(),
            TypeExpression::Basic(BasicType::VectorSpace(256)),
        );
        
        // Mathematical structures
        self.type_definitions.insert(
            "RealVector".to_string(),
            TypeExpression::Basic(BasicType::RealVector),
        );
        self.type_definitions.insert(
            "DirectSum".to_string(),
            TypeExpression::Basic(BasicType::DirectSum),
        );
        self.type_definitions.insert(
            "Structure".to_string(),
            TypeExpression::Basic(BasicType::MathematicalStructure("Structure".to_string())),
        );
        self.type_definitions.insert(
            "Composite".to_string(),
            TypeExpression::Basic(BasicType::MathematicalStructure("Composite".to_string())),
        );
        
        // Dimension-specific real vector spaces (ℝⁿ notation)
        for n in [7, 8, 256, 512, 768] {
            self.type_definitions.insert(
                format!("ℝ{}", n),
                TypeExpression::Basic(BasicType::VectorSpace(n)),
            );
        }
        
        // Generic real vector space (ℝⁿ)
        self.type_definitions.insert(
            "ℝⁿ".to_string(),
            TypeExpression::Basic(BasicType::RealVector),
        );
    }

    /// Check types throughout the entire document
    pub fn check_document(&mut self, document: &AispDocument) -> TypeCheckResult {
        // Extract type definitions
        self.extract_type_definitions(document);
        
        // Validate type definitions are well-formed
        self.validate_type_definitions();
        
        // Check function signatures
        self.check_function_types(document);
        
        // Check meta value types
        self.check_meta_types(document);
        
        TypeCheckResult {
            errors: self.errors.clone(),
            type_graph: self.build_type_dependency_graph(),
            undefined_types: self.find_undefined_types(),
        }
    }

    /// Extract type definitions from Types block
    fn extract_type_definitions(&mut self, document: &AispDocument) {
        for block in &document.blocks {
            if let AispBlock::Types(types_block) = block {
                for (name, definition) in &types_block.definitions {
                    // Check for redefinition of user-defined types (not built-ins)
                    if self.is_user_defined_type(name) {
                        self.errors.push(AispError::TypeError {
                            message: format!("Type '{}' redefined, using first definition", name),
                        });
                    } else {
                        self.type_definitions.insert(name.clone(), definition.type_expr.clone());
                    }
                }
            }
        }
    }
    
    /// Check if a type is user-defined (not a built-in type)
    fn is_user_defined_type(&self, name: &str) -> bool {
        // Built-in mathematical types that should not be flagged as redefinitions
        let builtin_types = [
            "VectorSpace768", "VectorSpace512", "VectorSpace256",
            "RealVector", "DirectSum", "Structure", "Composite",
            "ℝ7", "ℝ8", "ℝ256", "ℝ512", "ℝ768", "ℝⁿ"
        ];
        
        !builtin_types.contains(&name) && self.type_definitions.contains_key(name)
    }

    /// Validate that type definitions are well-formed
    fn validate_type_definitions(&mut self) {
        for (name, type_expr) in &self.type_definitions.clone() {
            if let Err(error) = self.validate_type_expression(type_expr, &mut HashSet::new()) {
                self.errors.push(error);
            }
        }
    }

    /// Validate a type expression recursively
    fn validate_type_expression(
        &self, 
        type_expr: &TypeExpression, 
        visited: &mut HashSet<String>
    ) -> AispResult<()> {
        match type_expr {
            TypeExpression::Basic(_) => Ok(()),
            TypeExpression::Reference(name) => {
                if visited.contains(name) {
                    return Err(AispError::TypeError {
                        message: format!("Circular type reference detected: {}", name),
                    });
                }
                
                if !self.type_definitions.contains_key(name) {
                    return Err(AispError::UndefinedSymbol {
                        symbol: name.clone(),
                    });
                }
                
                visited.insert(name.clone());
                let referenced_type = &self.type_definitions[name];
                self.validate_type_expression(referenced_type, visited)?;
                visited.remove(name);
                
                Ok(())
            }
            TypeExpression::Array { element_type, size: _ } => {
                self.validate_type_expression(element_type, visited)
            }
            TypeExpression::Function { input, output } => {
                self.validate_type_expression(input, visited)?;
                self.validate_type_expression(output, visited)
            }
            TypeExpression::Enumeration(values) => {
                if values.is_empty() {
                    return Err(AispError::TypeError {
                        message: "Enumeration cannot be empty".to_string(),
                    });
                }
                
                let mut unique_values = HashSet::new();
                for value in values {
                    if !unique_values.insert(value) {
                        return Err(AispError::TypeError {
                            message: format!("Duplicate enumeration value: {}", value),
                        });
                    }
                }
                
                Ok(())
            }
            TypeExpression::Tuple(elements) => {
                for element in elements {
                    self.validate_type_expression(element, visited)?;
                }
                Ok(())
            }
            TypeExpression::Generic { .. } => {
                // TODO: Validate generic type constraints
                Ok(())
            }
        }
    }

    /// Check function type signatures
    fn check_function_types(&mut self, document: &AispDocument) {
        for block in &document.blocks {
            if let AispBlock::Functions(functions_block) = block {
                for (name, function) in &functions_block.functions {
                    if let Err(error) = self.validate_function_signature(function) {
                        self.errors.push(error);
                    }
                }
            }
        }
    }

    /// Validate a function signature
    fn validate_function_signature(&self, function: &FunctionDefinition) -> AispResult<()> {
        // For now, just check that the function has parameters and a body
        if function.lambda.parameters.is_empty() {
            return Err(AispError::TypeError {
                message: format!("Function {} has no parameters", function.name),
            });
        }
        
        // TODO: More sophisticated type checking of lambda expressions
        Ok(())
    }

    /// Check meta value types
    fn check_meta_types(&mut self, document: &AispDocument) {
        for block in &document.blocks {
            if let AispBlock::Meta(meta_block) = block {
                for (key, entry) in &meta_block.entries {
                    if let Err(error) = self.validate_meta_entry(entry) {
                        self.errors.push(error);
                    }
                }
            }
        }
    }

    /// Validate a meta entry type
    fn validate_meta_entry(&self, entry: &MetaEntry) -> AispResult<()> {
        match &entry.value {
            MetaValue::Number(n) => {
                if n.is_nan() || n.is_infinite() {
                    return Err(AispError::TypeError {
                        message: format!("Invalid number in meta entry {}", entry.key),
                    });
                }
            }
            MetaValue::String(s) => {
                if s.is_empty() {
                    return Err(AispError::TypeError {
                        message: format!("Empty string in meta entry {}", entry.key),
                    });
                }
            }
            MetaValue::Boolean(_) => {
                // Booleans are always valid
            }
            MetaValue::Constraint(_) => {
                // TODO: Validate logical constraints
            }
        }
        
        Ok(())
    }

    /// Build dependency graph between types
    fn build_type_dependency_graph(&self) -> HashMap<String, Vec<String>> {
        let mut graph = HashMap::new();
        
        for (name, type_expr) in &self.type_definitions {
            let mut dependencies = Vec::new();
            self.collect_type_dependencies(type_expr, &mut dependencies);
            graph.insert(name.clone(), dependencies);
        }
        
        graph
    }

    /// Collect type dependencies recursively
    fn collect_type_dependencies(&self, type_expr: &TypeExpression, dependencies: &mut Vec<String>) {
        match type_expr {
            TypeExpression::Reference(name) => {
                if !dependencies.contains(name) {
                    dependencies.push(name.clone());
                }
            }
            TypeExpression::Array { element_type, .. } => {
                self.collect_type_dependencies(element_type, dependencies);
            }
            TypeExpression::Function { input, output } => {
                self.collect_type_dependencies(input, dependencies);
                self.collect_type_dependencies(output, dependencies);
            }
            _ => {} // Basic types and enumerations have no dependencies
        }
    }

    /// Find undefined type references
    fn find_undefined_types(&self) -> HashSet<String> {
        let undefined = HashSet::new();
        
        // TODO: Implement comprehensive undefined type detection
        // This would scan all type references and check if they're defined
        
        undefined
    }
}

/// Result of type checking
#[derive(Debug)]
pub struct TypeCheckResult {
    /// Type errors found
    pub errors: Vec<AispError>,
    /// Dependency graph between types
    pub type_graph: HashMap<String, Vec<String>>,
    /// Set of undefined type references
    pub undefined_types: HashSet<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_types_block() -> TypesBlock {
        let mut definitions = HashMap::new();
        
        definitions.insert("State".to_string(), TypeDefinition {
            name: "State".to_string(),
            type_expr: TypeExpression::Enumeration(vec!["A".to_string(), "B".to_string()]),
            span: Span::new(1, 1, 1, 10),
        });
        
        definitions.insert("Count".to_string(), TypeDefinition {
            name: "Count".to_string(),
            type_expr: TypeExpression::Basic(BasicType::Natural),
            span: Span::new(2, 1, 2, 10),
        });
        
        TypesBlock {
            definitions,
            span: Span::new(1, 1, 3, 1),
        }
    }

    #[test]
    fn test_extract_type_definitions() {
        let mut checker = TypeChecker::new();
        let document = AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "test".to_string(),
                date: "2026-01-25".to_string(),
                metadata: None,
            },
            metadata: DocumentMetadata {
                domain: None,
                protocol: None,
            },
            blocks: vec![AispBlock::Types(create_test_types_block())],
            span: Span::new(1, 1, 10, 1),
        };

        checker.extract_type_definitions(&document);
        
        assert!(checker.type_definitions.contains_key("State"));
        assert!(checker.type_definitions.contains_key("Count"));
    }

    #[test]
    fn test_validate_enumeration() {
        let checker = TypeChecker::new();
        let mut visited = HashSet::new();
        
        // Valid enumeration
        let valid_enum = TypeExpression::Enumeration(vec!["A".to_string(), "B".to_string()]);
        assert!(checker.validate_type_expression(&valid_enum, &mut visited).is_ok());
        
        // Empty enumeration (invalid)
        let empty_enum = TypeExpression::Enumeration(vec![]);
        assert!(checker.validate_type_expression(&empty_enum, &mut visited).is_err());
        
        // Duplicate values (invalid)
        let dup_enum = TypeExpression::Enumeration(vec!["A".to_string(), "A".to_string()]);
        assert!(checker.validate_type_expression(&dup_enum, &mut visited).is_err());
    }

    #[test]
    fn test_basic_type_validation() {
        let checker = TypeChecker::new();
        let mut visited = HashSet::new();
        
        let basic_type = TypeExpression::Basic(BasicType::Natural);
        assert!(checker.validate_type_expression(&basic_type, &mut visited).is_ok());
    }

    #[test]
    fn test_function_type_validation() {
        let mut checker = TypeChecker::new();
        
        let function = FunctionDefinition {
            name: "test".to_string(),
            lambda: LambdaExpression {
                parameters: vec!["x".to_string()],
                body: LogicalExpression::Variable("x".to_string()),
                span: Span::new(1, 1, 1, 10),
            },
            span: Span::new(1, 1, 1, 15),
        };
        
        assert!(checker.validate_function_signature(&function).is_ok());
        
        // Function with no parameters (invalid)
        let invalid_function = FunctionDefinition {
            name: "invalid".to_string(),
            lambda: LambdaExpression {
                parameters: vec![],
                body: LogicalExpression::Variable("x".to_string()),
                span: Span::new(1, 1, 1, 10),
            },
            span: Span::new(1, 1, 1, 15),
        };
        
        assert!(checker.validate_function_signature(&invalid_function).is_err());
    }
}