//! SMT-LIB Generator for AISP Formal Verification
//!
//! This module coordinates the generation of SMT-LIB formulas
//! from extracted properties using specialized converters.

use crate::error::*;
use crate::property_types::*;
use crate::smt_types::*;
use crate::smt_formula_converter::SMTFormulaConverter;
use std::collections::{HashMap, HashSet};
use std::fmt::Write;

/// SMT-LIB formula generator for Z3 solver
pub struct SMTGenerator {
    /// Type environment for sort declarations
    type_env: HashMap<String, String>,
    /// Function environment for function declarations
    function_env: HashMap<String, String>,
    /// Generated SMT-LIB declarations
    declarations: Vec<String>,
    /// Generated SMT-LIB assertions
    assertions: Vec<String>,
    /// Sort declarations
    sort_declarations: HashSet<String>,
    /// Formula converter
    converter: SMTFormulaConverter,
}

impl SMTGenerator {
    /// Create new SMT generator
    pub fn new() -> Self {
        Self {
            type_env: HashMap::new(),
            function_env: HashMap::new(),
            declarations: Vec::new(),
            assertions: Vec::new(),
            sort_declarations: HashSet::new(),
            converter: SMTFormulaConverter::new(),
        }
    }

    /// Generate SMT-LIB program from extracted properties
    pub fn generate_smt_program(&mut self, properties: &[ExtractedProperty]) -> AispResult<SMTGenerationResult> {
        let start_time = std::time::Instant::now();
        let mut warnings = Vec::new();
        
        // Clear previous state
        self.reset();

        // Generate basic SMT-LIB header
        let mut script = String::new();
        writeln!(script, "(set-info :source |AISP Formal Verification|)").unwrap();
        writeln!(script, "(set-info :category \"industrial\")").unwrap();
        writeln!(script, "(set-logic ALL)").unwrap();
        writeln!(script).unwrap();

        // Generate sort declarations
        self.generate_basic_sorts(&mut script)?;
        self.extract_sorts_from_properties(properties, &mut script)?;

        // Generate function declarations
        self.extract_functions_from_properties(properties, &mut script)?;

        // Generate assertions for each property
        let mut property_map = HashMap::new();
        let mut check_commands = Vec::new();
        let mut expected_results = Vec::new();

        for (i, property) in properties.iter().enumerate() {
            let assertion_name = format!("property_{}", i);
            
            match self.generate_property_assertion(property, &assertion_name, &mut script) {
                Ok(expected) => {
                    property_map.insert(property.id.clone(), assertion_name.clone());
                    check_commands.push(format!("(check-sat-assuming ({}))", assertion_name));
                    expected_results.push(expected);
                }
                Err(e) => {
                    warnings.push(format!("Failed to encode property {}: {}", property.id, e));
                }
            }
        }

        // Add final check-sat command
        writeln!(script, "(check-sat)").unwrap();
        writeln!(script, "(exit)").unwrap();

        let generation_time = start_time.elapsed();
        
        let program = SMTProgram {
            script: script.clone(),
            property_map,
            check_commands,
            expected_results,
        };

        let stats = SMTGenerationStats {
            properties_encoded: properties.len() - warnings.len(),
            sort_declarations: self.sort_declarations.len(),
            function_declarations: self.function_env.len(),
            assertions_generated: self.assertions.len(),
            script_size: script.len(),
            generation_time_ms: generation_time.as_millis() as u64,
        };

        Ok(SMTGenerationResult {
            program,
            stats,
            warnings,
        })
    }

    /// Reset generator state
    fn reset(&mut self) {
        self.type_env.clear();
        self.function_env.clear();
        self.declarations.clear();
        self.assertions.clear();
        self.sort_declarations.clear();
        self.converter.reset();
    }

    /// Generate basic SMT sorts
    fn generate_basic_sorts(&mut self, script: &mut String) -> AispResult<()> {
        // AISP basic types
        let basic_sorts = [
            ("AispBool", "Bool"),
            ("AispInt", "Int"), 
            ("AispNat", "Int"),
            ("AispReal", "Real"),
            ("AispString", "String"),
        ];

        for (aisp_sort, smt_sort) in &basic_sorts {
            if matches!(*smt_sort, "Int" | "Bool" | "Real" | "String") {
                // Built-in sorts don't need declaration
                self.type_env.insert(aisp_sort.to_string(), smt_sort.to_string());
            } else {
                writeln!(script, "(declare-sort {} 0)", aisp_sort)?;
                self.sort_declarations.insert(aisp_sort.to_string());
                self.type_env.insert(aisp_sort.to_string(), aisp_sort.to_string());
            }
        }

        // Basic predicates for type safety
        writeln!(script, "(declare-fun hasType (Int AispInt) Bool)")?;
        writeln!(script, "(declare-fun wellFormed (Int) Bool)")?;
        writeln!(script, "(declare-fun typeSafe (Int) Bool)")?;
        writeln!(script, "(declare-fun structurallyValid (Int) Bool)")?;
        writeln!(script, "(declare-fun total (Int) Bool)")?;
        writeln!(script)?;

        Ok(())
    }

    /// Extract and declare sorts from properties
    fn extract_sorts_from_properties(&mut self, properties: &[ExtractedProperty], script: &mut String) -> AispResult<()> {
        let mut custom_sorts = HashSet::new();

        for property in properties {
            // Extract sorts from property context
            for (type_name, _) in &property.context.type_definitions {
                if !self.type_env.contains_key(type_name) {
                    custom_sorts.insert(type_name.clone());
                }
            }

            // Extract sorts from formula
            self.extract_sorts_from_formula(&property.formula, &mut custom_sorts)?;
        }

        // Declare custom sorts
        for sort_name in custom_sorts {
            if !self.sort_declarations.contains(&sort_name) {
                writeln!(script, "(declare-sort {} 0)", sort_name)?;
                self.sort_declarations.insert(sort_name.clone());
                self.type_env.insert(sort_name.clone(), sort_name.clone());
            }
        }

        writeln!(script)?;
        Ok(())
    }

    /// Extract sorts from formula structure
    fn extract_sorts_from_formula(&self, formula: &PropertyFormula, custom_sorts: &mut HashSet<String>) -> AispResult<()> {
        // Extract from quantifiers
        for quantifier in &formula.quantifiers {
            if let Some(var_type) = &quantifier.variable_type {
                if !self.type_env.contains_key(var_type) {
                    custom_sorts.insert(var_type.clone());
                }
            }
        }

        Ok(())
    }

    /// Extract and declare functions from properties
    fn extract_functions_from_properties(&mut self, properties: &[ExtractedProperty], script: &mut String) -> AispResult<()> {
        let mut functions = HashSet::new();

        // Collect all function symbols from properties
        for property in properties {
            functions.extend(property.formula.functions.iter().cloned());
            functions.extend(property.formula.predicates.iter().cloned());
        }

        // Declare functions
        for func_name in functions {
            if !self.function_env.contains_key(&func_name) {
                // Generate simple uninterpreted function declaration
                let declaration = format!("(declare-fun {} (Int) Bool)", func_name);
                writeln!(script, "{}", declaration)?;
                self.function_env.insert(func_name, declaration);
            }
        }

        writeln!(script)?;
        Ok(())
    }

    /// Generate SMT assertion for a property
    fn generate_property_assertion(&mut self, property: &ExtractedProperty, assertion_name: &str, script: &mut String) -> AispResult<SMTExpectedResult> {
        let smt_formula = self.converter.convert_formula_to_smt(&property.formula)?;
        
        // Determine expected result based on property type
        let expected = match property.property_type {
            PropertyType::TypeSafety | PropertyType::FunctionalCorrectness | PropertyType::TemporalSafety => {
                // These should be valid (unsat when negated)
                SMTExpectedResult::Unsat
            }
            PropertyType::TemporalLiveness => {
                // Liveness properties should be satisfiable
                SMTExpectedResult::Sat
            }
            _ => SMTExpectedResult::Unknown
        };

        // Generate assertion
        writeln!(script, "; Property: {}", property.name)?;
        writeln!(script, "; Type: {:?}", property.property_type)?;
        writeln!(script, "(declare-const {} Bool)", assertion_name)?;
        
        if expected == SMTExpectedResult::Unsat {
            // For validity checking, assert the negation
            writeln!(script, "(assert (= {} (not {})))", assertion_name, smt_formula)?;
        } else {
            // For satisfiability checking, assert the formula
            writeln!(script, "(assert (= {} {}))", assertion_name, smt_formula)?;
        }
        
        writeln!(script)?;
        
        Ok(expected)
    }

    /// Get generation statistics
    pub fn get_stats(&self) -> SMTGenerationStats {
        SMTGenerationStats {
            properties_encoded: self.assertions.len(),
            sort_declarations: self.sort_declarations.len(),
            function_declarations: self.function_env.len(),
            assertions_generated: self.assertions.len(),
            script_size: 0, // Will be computed during generation
            generation_time_ms: 0, // Will be computed during generation
        }
    }
}

impl Default for SMTGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smt_generator_creation() {
        let generator = SMTGenerator::new();
        assert!(generator.type_env.is_empty());
        assert!(generator.function_env.is_empty());
        assert!(generator.sort_declarations.is_empty());
    }

    #[test]
    fn test_empty_properties_generation() -> AispResult<()> {
        let mut generator = SMTGenerator::new();
        let result = generator.generate_smt_program(&[])?;
        
        assert!(result.program.script.contains("(set-logic ALL)"));
        assert!(result.program.property_map.is_empty());
        assert_eq!(result.stats.properties_encoded, 0);
        
        Ok(())
    }

    #[test]
    fn test_basic_sort_generation() -> AispResult<()> {
        let mut generator = SMTGenerator::new();
        let mut script = String::new();
        
        generator.generate_basic_sorts(&mut script)?;
        
        assert!(script.contains("declare-fun hasType"));
        assert!(script.contains("declare-fun wellFormed"));
        assert!(generator.type_env.contains_key("AispInt"));
        
        Ok(())
    }

    #[test]
    fn test_property_assertion_generation() -> AispResult<()> {
        let mut generator = SMTGenerator::new();
        
        let property = ExtractedProperty {
            id: "test_prop".to_string(),
            name: "test_property".to_string(),
            property_type: PropertyType::TypeSafety,
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
            context: PropertyContext {
                type_definitions: HashMap::new(),
                function_definitions: HashMap::new(),
                constants: HashMap::new(),
                dependencies: vec![],
            },
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
                difficulty_score: 3,
            },
        };

        let mut script = String::new();
        let result = generator.generate_property_assertion(&property, "test_assertion", &mut script)?;
        
        assert_eq!(result, SMTExpectedResult::Unsat);
        assert!(script.contains("test_property"));
        assert!(script.contains("test_assertion"));
        
        Ok(())
    }

    #[test]
    fn test_reset_functionality() {
        let mut generator = SMTGenerator::new();
        
        // Add some data
        generator.type_env.insert("Test".to_string(), "TestSort".to_string());
        generator.function_env.insert("f".to_string(), "test_func".to_string());
        generator.sort_declarations.insert("CustomSort".to_string());
        
        assert!(!generator.type_env.is_empty());
        assert!(!generator.function_env.is_empty());
        assert!(!generator.sort_declarations.is_empty());
        
        // Reset
        generator.reset();
        
        assert!(generator.type_env.is_empty());
        assert!(generator.function_env.is_empty());
        assert!(generator.sort_declarations.is_empty());
    }
}