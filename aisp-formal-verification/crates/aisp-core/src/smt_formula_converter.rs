//! SMT Formula Conversion
//!
//! This module handles conversion of property formulas
//! to SMT-LIB format for Z3 verification.

use crate::error::*;
use crate::property_types::*;
use std::fmt::Write;

/// Converts property formulas to SMT-LIB format
pub struct SMTFormulaConverter {
    /// Variable counter for unique names
    var_counter: usize,
}

impl SMTFormulaConverter {
    /// Create new SMT formula converter
    pub fn new() -> Self {
        Self { var_counter: 0 }
    }

    /// Convert property formula to SMT-LIB format
    pub fn convert_formula_to_smt(&mut self, formula: &PropertyFormula) -> AispResult<String> {
        self.convert_structure_to_smt(&formula.structure)
    }

    /// Convert formula structure to SMT-LIB format
    pub fn convert_structure_to_smt(&mut self, structure: &FormulaStructure) -> AispResult<String> {
        match structure {
            FormulaStructure::Atomic(atomic) => {
                let terms: Result<Vec<String>, AispError> = atomic.terms.iter()
                    .map(|term| self.convert_term_to_smt(term))
                    .collect();
                let terms = terms?;
                if terms.is_empty() {
                    Ok(format!("({})", atomic.predicate))
                } else {
                    Ok(format!("({} {})", atomic.predicate, terms.join(" ")))
                }
            }
            FormulaStructure::Negation(inner) => {
                let inner_smt = self.convert_structure_to_smt(inner)?;
                Ok(format!("(not {})", inner_smt))
            }
            FormulaStructure::Conjunction(formulas) => {
                let smt_formulas: Result<Vec<String>, AispError> = formulas.iter()
                    .map(|f| self.convert_structure_to_smt(f))
                    .collect();
                let smt_formulas = smt_formulas?;
                Ok(format!("(and {})", smt_formulas.join(" ")))
            }
            FormulaStructure::Disjunction(formulas) => {
                let smt_formulas: Result<Vec<String>, AispError> = formulas.iter()
                    .map(|f| self.convert_structure_to_smt(f))
                    .collect();
                let smt_formulas = smt_formulas?;
                Ok(format!("(or {})", smt_formulas.join(" ")))
            }
            FormulaStructure::Implication(left, right) => {
                let left_smt = self.convert_structure_to_smt(left)?;
                let right_smt = self.convert_structure_to_smt(right)?;
                Ok(format!("(=> {} {})", left_smt, right_smt))
            }
            FormulaStructure::Biconditional(left, right) => {
                let left_smt = self.convert_structure_to_smt(left)?;
                let right_smt = self.convert_structure_to_smt(right)?;
                Ok(format!("(= {} {})", left_smt, right_smt))
            }
            FormulaStructure::Universal(quantifier, inner) => {
                let var_name = &quantifier.variable;
                let default_sort = "Int".to_string();
                let var_sort = quantifier.variable_type.as_ref().unwrap_or(&default_sort);
                let inner_smt = self.convert_structure_to_smt(inner)?;
                Ok(format!("(forall (({} {})) {})", var_name, var_sort, inner_smt))
            }
            FormulaStructure::Existential(quantifier, inner) => {
                let var_name = &quantifier.variable;
                let default_sort = "Int".to_string();
                let var_sort = quantifier.variable_type.as_ref().unwrap_or(&default_sort);
                let inner_smt = self.convert_structure_to_smt(inner)?;
                Ok(format!("(exists (({} {})) {})", var_name, var_sort, inner_smt))
            }
            FormulaStructure::TemporalAlways(inner) => {
                // Encode temporal always as universal quantification over time
                let inner_smt = self.convert_structure_to_smt(inner)?;
                let time_var = self.fresh_var("t");
                Ok(format!("(forall (({} Int)) (=> (>= {} 0) {}))", time_var, time_var, inner_smt))
            }
            FormulaStructure::TemporalEventually(inner) => {
                // Encode temporal eventually as existential quantification over time
                let inner_smt = self.convert_structure_to_smt(inner)?;
                let time_var = self.fresh_var("t");
                Ok(format!("(exists (({} Int)) (and (>= {} 0) {}))", time_var, time_var, inner_smt))
            }
            FormulaStructure::TemporalUntil(left, right) => {
                // Encode until as: ∃t≥0. right(t) ∧ ∀s∈[0,t). left(s)
                let left_smt = self.convert_structure_to_smt(left)?;
                let right_smt = self.convert_structure_to_smt(right)?;
                let t_var = self.fresh_var("t");
                let s_var = self.fresh_var("s");
                Ok(format!(
                    "(exists (({} Int)) (and (>= {} 0) {} (forall (({} Int)) (=> (and (>= {} 0) (< {} {})) {}))))",
                    t_var, t_var, right_smt, s_var, s_var, s_var, t_var, left_smt
                ))
            }
            FormulaStructure::ArithmeticEqual(left, right) => {
                let left_smt = self.convert_term_to_smt(left)?;
                let right_smt = self.convert_term_to_smt(right)?;
                Ok(format!("(= {} {})", left_smt, right_smt))
            }
            FormulaStructure::ArithmeticLessEqual(left, right) => {
                let left_smt = self.convert_term_to_smt(left)?;
                let right_smt = self.convert_term_to_smt(right)?;
                Ok(format!("(<= {} {})", left_smt, right_smt))
            }
            FormulaStructure::SetMembership(element, _set) => {
                let element_smt = self.convert_term_to_smt(element)?;
                // Simplified set membership encoding
                Ok(format!("(member {} set)", element_smt))
            }
            FormulaStructure::FunctionApplication(func_name, terms) => {
                let terms_smt: Result<Vec<String>, AispError> = terms.iter()
                    .map(|term| self.convert_term_to_smt(term))
                    .collect();
                let terms_smt = terms_smt?;
                Ok(format!("({} {})", func_name, terms_smt.join(" ")))
            }
        }
    }

    /// Convert term to SMT-LIB format
    pub fn convert_term_to_smt(&mut self, term: &Term) -> AispResult<String> {
        match term {
            Term::Variable(name, _) => Ok(name.clone()),
            Term::Constant(value, const_type) => {
                match const_type.as_str() {
                    "Int" | "ℤ" | "ℕ" => Ok(value.clone()),
                    "Bool" | "𝔹" => Ok(if value == "true" { "true".to_string() } else { "false".to_string() }),
                    "Real" | "ℝ" => Ok(value.clone()),
                    "String" | "𝕊" => Ok(format!("\"{}\"", value)),
                    _ => Ok(value.clone()), // Default to value as-is
                }
            }
            Term::Function(name, args) => {
                let args_smt: Result<Vec<String>, AispError> = args.iter()
                    .map(|arg| self.convert_term_to_smt(arg))
                    .collect();
                let args_smt = args_smt?;
                Ok(format!("({} {})", name, args_smt.join(" ")))
            }
            Term::Arithmetic(op, left, right) => {
                let left_smt = self.convert_term_to_smt(left)?;
                let right_smt = self.convert_term_to_smt(right)?;
                let op_smt = match op {
                    ArithmeticOp::Add => "+",
                    ArithmeticOp::Subtract => "-",
                    ArithmeticOp::Multiply => "*",
                    ArithmeticOp::Divide => "div",
                    ArithmeticOp::Modulo => "mod",
                    ArithmeticOp::Power => "^", // May need custom encoding
                };
                Ok(format!("({} {} {})", op_smt, left_smt, right_smt))
            }
            Term::Set(elements) => {
                let elements_smt: Result<Vec<String>, AispError> = elements.iter()
                    .map(|elem| self.convert_term_to_smt(elem))
                    .collect();
                let elements_smt = elements_smt?;
                Ok(format!("(set {})", elements_smt.join(" ")))
            }
            Term::ArrayAccess(array, index) => {
                let array_smt = self.convert_term_to_smt(array)?;
                let index_smt = self.convert_term_to_smt(index)?;
                Ok(format!("(select {} {})", array_smt, index_smt))
            }
        }
    }

    /// Generate fresh variable name
    pub fn fresh_var(&mut self, prefix: &str) -> String {
        let var = format!("{}_{}", prefix, self.var_counter);
        self.var_counter += 1;
        var
    }

    /// Reset converter state
    pub fn reset(&mut self) {
        self.var_counter = 0;
    }
}

impl Default for SMTFormulaConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smt_formula_converter_creation() {
        let converter = SMTFormulaConverter::new();
        assert_eq!(converter.var_counter, 0);
    }

    #[test]
    fn test_fresh_variable_generation() {
        let mut converter = SMTFormulaConverter::new();
        let var1 = converter.fresh_var("x");
        let var2 = converter.fresh_var("x");
        let var3 = converter.fresh_var("y");
        
        assert_eq!(var1, "x_0");
        assert_eq!(var2, "x_1");
        assert_eq!(var3, "y_2");
    }

    #[test]
    fn test_basic_term_conversion() -> AispResult<()> {
        let mut converter = SMTFormulaConverter::new();
        
        let int_term = Term::Constant("42".to_string(), "Int".to_string());
        let bool_term = Term::Constant("true".to_string(), "Bool".to_string());
        let var_term = Term::Variable("x".to_string(), Some("Int".to_string()));
        
        assert_eq!(converter.convert_term_to_smt(&int_term)?, "42");
        assert_eq!(converter.convert_term_to_smt(&bool_term)?, "true");
        assert_eq!(converter.convert_term_to_smt(&var_term)?, "x");
        
        Ok(())
    }

    #[test]
    fn test_arithmetic_term_conversion() -> AispResult<()> {
        let mut converter = SMTFormulaConverter::new();
        
        let left = Term::Variable("x".to_string(), Some("Int".to_string()));
        let right = Term::Constant("5".to_string(), "Int".to_string());
        let add_term = Term::Arithmetic(ArithmeticOp::Add, Box::new(left), Box::new(right));
        
        assert_eq!(converter.convert_term_to_smt(&add_term)?, "(+ x 5)");
        
        Ok(())
    }

    #[test]
    fn test_atomic_formula_conversion() -> AispResult<()> {
        let mut converter = SMTFormulaConverter::new();
        
        let atomic = FormulaStructure::Atomic(AtomicFormula {
            predicate: "P".to_string(),
            terms: vec![
                Term::Variable("x".to_string(), Some("Int".to_string())),
                Term::Constant("0".to_string(), "Int".to_string()),
            ],
            type_signature: None,
        });
        
        assert_eq!(converter.convert_structure_to_smt(&atomic)?, "(P x 0)");
        
        Ok(())
    }

    #[test]
    fn test_quantified_formula_conversion() -> AispResult<()> {
        let mut converter = SMTFormulaConverter::new();
        
        let quantifier = Quantifier {
            variable: "x".to_string(),
            variable_type: Some("Int".to_string()),
            domain: None,
        };
        
        let inner = FormulaStructure::Atomic(AtomicFormula {
            predicate: "P".to_string(),
            terms: vec![Term::Variable("x".to_string(), Some("Int".to_string()))],
            type_signature: None,
        });
        
        let universal = FormulaStructure::Universal(quantifier, Box::new(inner));
        
        assert_eq!(converter.convert_structure_to_smt(&universal)?, "(forall ((x Int)) (P x))");
        
        Ok(())
    }

    #[test]
    fn test_temporal_formula_conversion() -> AispResult<()> {
        let mut converter = SMTFormulaConverter::new();
        
        let inner = FormulaStructure::Atomic(AtomicFormula {
            predicate: "P".to_string(),
            terms: vec![],
            type_signature: None,
        });
        
        let always = FormulaStructure::TemporalAlways(Box::new(inner));
        let result = converter.convert_structure_to_smt(&always)?;
        
        assert!(result.contains("forall"));
        assert!(result.contains(">="));
        assert!(result.contains("(P)"));
        
        Ok(())
    }

    #[test]
    fn test_boolean_connectives() -> AispResult<()> {
        let mut converter = SMTFormulaConverter::new();
        
        let p = FormulaStructure::Atomic(AtomicFormula {
            predicate: "P".to_string(),
            terms: vec![],
            type_signature: None,
        });
        
        let q = FormulaStructure::Atomic(AtomicFormula {
            predicate: "Q".to_string(),
            terms: vec![],
            type_signature: None,
        });
        
        // Test conjunction
        let conjunction = FormulaStructure::Conjunction(vec![p.clone(), q.clone()]);
        assert_eq!(converter.convert_structure_to_smt(&conjunction)?, "(and (P) (Q))");
        
        // Test disjunction
        let disjunction = FormulaStructure::Disjunction(vec![p.clone(), q.clone()]);
        assert_eq!(converter.convert_structure_to_smt(&disjunction)?, "(or (P) (Q))");
        
        // Test implication
        let implication = FormulaStructure::Implication(Box::new(p), Box::new(q));
        assert_eq!(converter.convert_structure_to_smt(&implication)?, "(=> (P) (Q))");
        
        Ok(())
    }

    #[test]
    fn test_converter_reset() {
        let mut converter = SMTFormulaConverter::new();
        
        // Generate some variables
        converter.fresh_var("x");
        converter.fresh_var("y");
        assert_eq!(converter.var_counter, 2);
        
        // Reset
        converter.reset();
        assert_eq!(converter.var_counter, 0);
        
        // Should start from 0 again
        let var = converter.fresh_var("z");
        assert_eq!(var, "z_0");
    }

    #[test]
    fn test_atomic_formula_whitespace_formatting() -> AispResult<()> {
        let mut converter = SMTFormulaConverter::new();
        
        // Test atomic formula with no terms (should have no extra space)
        let atomic_no_terms = FormulaStructure::Atomic(AtomicFormula {
            predicate: "P".to_string(),
            terms: vec![],
            type_signature: None,
        });
        assert_eq!(converter.convert_structure_to_smt(&atomic_no_terms)?, "(P)");
        
        // Test atomic formula with terms (should have space before terms)
        let atomic_with_terms = FormulaStructure::Atomic(AtomicFormula {
            predicate: "Equal".to_string(),
            terms: vec![
                Term::Variable("x".to_string(), Some("Int".to_string())),
                Term::Variable("y".to_string(), Some("Int".to_string()))
            ],
            type_signature: None,
        });
        let result = converter.convert_structure_to_smt(&atomic_with_terms)?;
        assert!(result.starts_with("(Equal "));
        assert!(result.contains(" x "));
        assert!(result.contains(" y"));
        assert!(result.ends_with(")"));
        
        Ok(())
    }
}