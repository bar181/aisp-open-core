//! Property Extractor for AISP Formal Verification
//!
//! This module coordinates the extraction of formal properties from
//! AISP documents using the various specialized extractors.

use crate::ast::*;
use crate::error::*;
use crate::property_types::*;
use crate::formula_converter::FormulaConverter;
use crate::property_factory::PropertyFactory;
use std::collections::HashMap;

/// Advanced property extractor for AISP documents
pub struct PropertyExtractor {
    /// Type environment from document
    type_env: HashMap<String, TypeExpression>,
    /// Function environment from document
    function_env: HashMap<String, LambdaExpression>,
    /// Extracted properties
    properties: Vec<ExtractedProperty>,
    /// Property factory for creating properties
    factory: PropertyFactory,
}

impl PropertyExtractor {
    /// Create new property extractor
    pub fn new() -> Self {
        Self {
            type_env: HashMap::new(),
            function_env: HashMap::new(),
            properties: Vec::new(),
            factory: PropertyFactory::new(),
        }
    }

    /// Extract all formal properties from an AISP document
    pub fn extract_properties(&mut self, doc: &AispDocument) -> AispResult<Vec<ExtractedProperty>> {
        self.properties.clear();
        self.factory.reset();

        // Build type and function environments
        self.build_environments(doc)?;

        // Extract properties from each block type
        for block in &doc.blocks {
            match block {
                AispBlock::Types(types_block) => {
                    self.extract_type_properties(types_block)?;
                }
                AispBlock::Rules(rules_block) => {
                    self.extract_rule_properties(rules_block)?;
                }
                AispBlock::Functions(funcs_block) => {
                    self.extract_function_properties(funcs_block)?;
                }
                AispBlock::Meta(meta_block) => {
                    self.extract_meta_properties(meta_block)?;
                }
                _ => {
                    // Evidence block doesn't contain extractable properties
                }
            }
        }

        Ok(self.properties.clone())
    }

    /// Build type and function environments for property extraction
    fn build_environments(&mut self, doc: &AispDocument) -> AispResult<()> {
        // Extract type definitions
        for block in &doc.blocks {
            if let AispBlock::Types(types_block) = block {
                for (name, type_def) in &types_block.definitions {
                    self.type_env.insert(name.clone(), type_def.type_expr.clone());
                }
            }
        }

        // Extract function definitions
        for block in &doc.blocks {
            if let AispBlock::Functions(funcs_block) = block {
                for (name, func_def) in &funcs_block.functions {
                    self.function_env.insert(name.clone(), func_def.lambda.clone());
                }
            }
        }

        Ok(())
    }

    /// Extract type safety and structural properties from type definitions
    fn extract_type_properties(&mut self, types_block: &TypesBlock) -> AispResult<()> {
        for (type_name, type_def) in &types_block.definitions {
            // Generate type safety property
            let type_safety = ExtractedProperty {
                id: self.factory.next_property_id(),
                name: format!("{}_type_safety", type_name),
                property_type: PropertyType::TypeSafety,
                formula: self.factory.create_type_safety_formula(type_name, &type_def.type_expr)?,
                context: self.create_context(),
                source_location: SourceLocation {
                    block_type: "Types".to_string(),
                    line: None,
                    column: None,
                    source_text: Some(format!("{}≜{:?}", type_name, type_def.type_expr)),
                },
                complexity: PropertyComplexity {
                    quantifier_depth: 1,
                    logical_connectives: 1,
                    function_applications: 2,
                    variable_count: 1,
                    difficulty_score: 3,
                },
            };
            self.properties.push(type_safety);

            // Generate structural invariant if applicable
            if let TypeExpression::Tuple(fields) = &type_def.type_expr {
                let structural_invariant = self.factory.create_tuple_structural_invariant(type_name, fields)?;
                self.properties.push(structural_invariant);
            }

            // Generate set membership properties for enumerations
            if let TypeExpression::Enumeration(values) = &type_def.type_expr {
                let membership_property = self.factory.create_enumeration_property(type_name, values)?;
                self.properties.push(membership_property);
            }
        }

        Ok(())
    }

    /// Extract logical assertions and constraints from rules
    fn extract_rule_properties(&mut self, rules_block: &RulesBlock) -> AispResult<()> {
        for (i, rule) in rules_block.rules.iter().enumerate() {
            let property = ExtractedProperty {
                id: self.factory.next_property_id(),
                name: format!("rule_{}", i),
                property_type: FormulaConverter::classify_rule_property(&rule.expression),
                formula: FormulaConverter::convert_rule_to_formula(&rule.expression)?,
                context: self.create_context(),
                source_location: SourceLocation {
                    block_type: "Rules".to_string(),
                    line: None,
                    column: None,
                    source_text: Some(format!("{:?}", rule.expression)),
                },
                complexity: FormulaConverter::analyze_rule_complexity(&rule.expression),
            };
            self.properties.push(property);
        }

        Ok(())
    }

    /// Extract function correctness properties
    fn extract_function_properties(&mut self, funcs_block: &FunctionsBlock) -> AispResult<()> {
        for (func_name, func_def) in &funcs_block.functions {
            // Generate function well-definedness property
            let well_defined = ExtractedProperty {
                id: self.factory.next_property_id(),
                name: format!("{}_well_defined", func_name),
                property_type: PropertyType::FunctionalCorrectness,
                formula: self.factory.create_function_well_defined_formula(func_name, &func_def.lambda)?,
                context: self.create_context(),
                source_location: SourceLocation {
                    block_type: "Functions".to_string(),
                    line: None,
                    column: None,
                    source_text: Some(format!("{}≜{:?}", func_name, func_def.lambda)),
                },
                complexity: PropertyComplexity {
                    quantifier_depth: 1,
                    logical_connectives: 2,
                    function_applications: 1,
                    variable_count: func_def.lambda.parameters.len(),
                    difficulty_score: 4,
                },
            };
            self.properties.push(well_defined);

            // Generate totality property (function is defined for all inputs)
            if self.factory.should_generate_totality_property(&func_def.lambda) {
                let totality = self.factory.create_totality_property(func_name, &func_def.lambda)?;
                self.properties.push(totality);
            }
        }

        Ok(())
    }

    /// Extract meta-level assertions and global constraints
    fn extract_meta_properties(&mut self, meta_block: &MetaBlock) -> AispResult<()> {
        for (key, _entry) in &meta_block.entries {
            // Simplified: assume logical assertion
            let meta_property = ExtractedProperty {
                id: self.factory.next_property_id(),
                name: format!("meta_{}", key),
                property_type: PropertyType::LogicalAssertion,
                formula: PropertyFormula {
                    structure: FormulaStructure::Atomic(AtomicFormula {
                        predicate: key.clone(),
                        terms: vec![],
                        type_signature: None,
                    }),
                    quantifiers: vec![],
                    free_variables: std::collections::HashSet::new(),
                    predicates: [key.as_str()].iter().map(|s| s.to_string()).collect(),
                    functions: std::collections::HashSet::new(),
                    constants: std::collections::HashSet::new(),
                },
                context: self.create_context(),
                source_location: SourceLocation {
                    block_type: "Meta".to_string(),
                    line: None,
                    column: None,
                    source_text: Some(format!("meta_{}", key)),
                },
                complexity: PropertyComplexity {
                    quantifier_depth: 0,
                    logical_connectives: 0,
                    function_applications: 0,
                    variable_count: 0,
                    difficulty_score: 1,
                },
            };
            self.properties.push(meta_property);
        }

        Ok(())
    }

    /// Create property context from current environments
    fn create_context(&self) -> PropertyContext {
        PropertyContext {
            type_definitions: self.type_env.iter().map(|(k, v)| (k.clone(), format!("{:?}", v))).collect(),
            function_definitions: self.function_env.iter().map(|(k, v)| (k.clone(), format!("{:?}", v))).collect(),
            constants: HashMap::new(),
            dependencies: vec![],
        }
    }

    /// Get extracted properties
    pub fn get_properties(&self) -> &[ExtractedProperty] {
        &self.properties
    }

    /// Get property statistics
    pub fn get_statistics(&self) -> PropertyExtractionStats {
        let type_count = self.properties.iter().filter(|p| p.property_type == PropertyType::TypeSafety).count();
        let function_count = self.properties.iter().filter(|p| p.property_type == PropertyType::FunctionalCorrectness).count();
        let temporal_count = self.properties.iter().filter(|p| 
            p.property_type == PropertyType::TemporalSafety || p.property_type == PropertyType::TemporalLiveness
        ).count();
        let relational_count = self.properties.iter().filter(|p| p.property_type == PropertyType::RelationalConstraint).count();

        PropertyExtractionStats {
            total_properties: self.properties.len(),
            type_properties: type_count,
            function_properties: function_count,
            temporal_properties: temporal_count,
            relational_properties: relational_count,
            average_complexity: if !self.properties.is_empty() {
                self.properties.iter().map(|p| p.complexity.difficulty_score as f64).sum::<f64>() / self.properties.len() as f64
            } else {
                0.0
            },
        }
    }
}

impl Default for PropertyExtractor {
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
    fn test_property_extractor_creation() {
        let extractor = PropertyExtractor::new();
        assert_eq!(extractor.properties.len(), 0);
        assert!(extractor.type_env.is_empty());
        assert!(extractor.function_env.is_empty());
    }

    #[test]
    fn test_empty_document_extraction() -> AispResult<()> {
        let mut extractor = PropertyExtractor::new();
        let doc = AispDocument {
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
            blocks: vec![],
            span: create_test_span(),
        };
        
        let properties = extractor.extract_properties(&doc)?;
        assert!(properties.is_empty());
        
        Ok(())
    }

    #[test]
    fn test_type_block_extraction() -> AispResult<()> {
        let mut extractor = PropertyExtractor::new();
        let mut type_definitions = HashMap::new();
        type_definitions.insert("Integer".to_string(), TypeDefinition {
            name: "Integer".to_string(),
            type_expr: TypeExpression::Basic(BasicType::Integer),
            span: create_test_span(),
        });
        
        let types_block = TypesBlock {
            definitions: type_definitions,
            span: create_test_span(),
        };
        
        extractor.extract_type_properties(&types_block)?;
        assert!(!extractor.properties.is_empty());
        
        // Should have type safety property
        assert!(extractor.properties.iter().any(|p| p.name.contains("type_safety")));
        
        Ok(())
    }

    #[test]
    fn test_enumeration_type_extraction() -> AispResult<()> {
        let mut extractor = PropertyExtractor::new();
        let mut type_definitions = HashMap::new();
        type_definitions.insert("Color".to_string(), TypeDefinition {
            name: "Color".to_string(),
            type_expr: TypeExpression::Enumeration(vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()]),
            span: create_test_span(),
        });
        
        let types_block = TypesBlock {
            definitions: type_definitions,
            span: create_test_span(),
        };
        
        extractor.extract_type_properties(&types_block)?;
        
        // Should have type safety and membership properties
        assert!(extractor.properties.iter().any(|p| p.name.contains("type_safety")));
        assert!(extractor.properties.iter().any(|p| p.name.contains("membership")));
        
        Ok(())
    }

    #[test]
    fn test_function_extraction() -> AispResult<()> {
        let mut extractor = PropertyExtractor::new();
        let mut functions = HashMap::new();
        functions.insert("add".to_string(), FunctionDefinition {
            name: "add".to_string(),
            lambda: LambdaExpression {
                parameters: vec!["x".to_string(), "y".to_string()],
                body: LogicalExpression::Variable("result".to_string()),
                span: create_test_span(),
            },
            span: create_test_span(),
        });
        
        let funcs_block = FunctionsBlock {
            functions,
            span: create_test_span(),
        };
        
        extractor.extract_function_properties(&funcs_block)?;
        
        // Should have well-defined and totality properties
        assert!(extractor.properties.iter().any(|p| p.name.contains("well_defined")));
        assert!(extractor.properties.iter().any(|p| p.name.contains("totality")));
        
        Ok(())
    }

    #[test]
    fn test_statistics_calculation() -> AispResult<()> {
        let mut extractor = PropertyExtractor::new();
        
        // Add some test properties
        extractor.properties.push(ExtractedProperty {
            id: "prop_1".to_string(),
            name: "test_type_safety".to_string(),
            property_type: PropertyType::TypeSafety,
            formula: PropertyFormula {
                structure: FormulaStructure::Atomic(AtomicFormula {
                    predicate: "P".to_string(),
                    terms: vec![],
                    type_signature: None,
                }),
                quantifiers: vec![],
                free_variables: std::collections::HashSet::new(),
                predicates: std::collections::HashSet::new(),
                functions: std::collections::HashSet::new(),
                constants: std::collections::HashSet::new(),
            },
            context: PropertyFactory::create_empty_context(),
            source_location: SourceLocation {
                block_type: "Test".to_string(),
                line: None,
                column: None,
                source_text: None,
            },
            complexity: PropertyComplexity {
                quantifier_depth: 1,
                logical_connectives: 1,
                function_applications: 1,
                variable_count: 1,
                difficulty_score: 5,
            },
        });
        
        let stats = extractor.get_statistics();
        assert_eq!(stats.total_properties, 1);
        assert_eq!(stats.type_properties, 1);
        assert_eq!(stats.average_complexity, 5.0);
        
        Ok(())
    }
}