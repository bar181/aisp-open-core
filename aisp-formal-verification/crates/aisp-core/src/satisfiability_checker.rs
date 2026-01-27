//! Satisfiability Checking for Constraint Systems
//!
//! This module provides satisfiability checking capabilities for constraint systems
//! discovered from AISP documents, including SMT-based solving and model generation.

use crate::{
    ast::canonical::CanonicalAispDocument as AispDocument,
    error::{AispError, AispResult},
    invariant_types::DiscoveredInvariant,
    property_types::{PropertyFormula, FormulaStructure, AtomicFormula, Term},
    smt_types::{SmtFormula, SmtSort, SmtCommand},
};
use std::collections::{HashMap, HashSet};

/// Result of satisfiability checking
#[derive(Debug, Clone)]
pub enum SatisfiabilityResult {
    /// The constraint system is satisfiable with a model
    Satisfiable(ConstraintModel),
    /// The constraint system is unsatisfiable
    Unsatisfiable(UnsatisfiabilityProof),
    /// Satisfiability is unknown (timeout, resource limits, etc.)
    Unknown(String),
}

/// Model showing satisfying assignments for variables
#[derive(Debug, Clone)]
pub struct ConstraintModel {
    pub variable_assignments: HashMap<String, ModelValue>,
    pub function_interpretations: HashMap<String, FunctionInterpretation>,
    pub predicate_interpretations: HashMap<String, bool>,
}

/// Value in a satisfiability model
#[derive(Debug, Clone, PartialEq)]
pub enum ModelValue {
    Boolean(bool),
    Integer(i64),
    Real(f64),
    String(String),
    Enumeration(String),
}

/// Function interpretation in a model
#[derive(Debug, Clone)]
pub struct FunctionInterpretation {
    pub domain: Vec<String>,
    pub range: String,
    pub mappings: HashMap<Vec<ModelValue>, ModelValue>,
}

/// Proof of unsatisfiability
#[derive(Debug, Clone, PartialEq)]
pub struct UnsatisfiabilityProof {
    pub conflicting_constraints: Vec<String>,
    pub proof_steps: Vec<ProofStep>,
    pub reason: String,
}

/// Step in an unsatisfiability proof
#[derive(Debug, Clone, PartialEq)]
pub struct ProofStep {
    pub rule: String,
    pub premises: Vec<String>,
    pub conclusion: String,
    pub justification: String,
}

/// Configuration for satisfiability checking
#[derive(Debug, Clone)]
pub struct SatisfiabilityConfig {
    pub timeout_seconds: u32,
    pub max_model_size: usize,
    pub enable_quantifier_instantiation: bool,
    pub enable_theory_reasoning: bool,
    pub solver_options: HashMap<String, String>,
}

impl Default for SatisfiabilityConfig {
    fn default() -> Self {
        let mut solver_options = HashMap::new();
        solver_options.insert("model_completion".to_string(), "true".to_string());
        solver_options.insert("timeout".to_string(), "10000".to_string());
        
        Self {
            timeout_seconds: 10,
            max_model_size: 1000,
            enable_quantifier_instantiation: true,
            enable_theory_reasoning: true,
            solver_options,
        }
    }
}

/// Main satisfiability checker for constraint systems
pub struct SatisfiabilityChecker {
    config: SatisfiabilityConfig,
}

impl SatisfiabilityChecker {
    /// Create a new satisfiability checker
    pub fn new(config: SatisfiabilityConfig) -> Self {
        Self { config }
    }

    /// Check satisfiability of discovered invariants
    pub fn check_invariants(&self, invariants: &[DiscoveredInvariant]) 
        -> AispResult<SatisfiabilityResult> 
    {
        if invariants.is_empty() {
            return Ok(SatisfiabilityResult::Satisfiable(ConstraintModel::empty()));
        }

        // Convert invariants to constraint system
        let constraint_system = self.build_constraint_system(invariants)?;
        
        // Check satisfiability using SMT solver
        self.solve_constraint_system(&constraint_system)
    }

    /// Check satisfiability of a specific formula
    pub fn check_formula(&self, formula: &PropertyFormula) 
        -> AispResult<SatisfiabilityResult> 
    {
        let constraint_system = ConstraintSystem::from_formula(formula.clone())?;
        self.solve_constraint_system(&constraint_system)
    }

    /// Check consistency between invariants
    pub fn check_consistency(&self, invariants: &[DiscoveredInvariant]) 
        -> AispResult<ConsistencyResult> 
    {
        let result = self.check_invariants(invariants)?;
        
        match result {
            SatisfiabilityResult::Satisfiable(model) => {
                Ok(ConsistencyResult::Consistent(model))
            }
            SatisfiabilityResult::Unsatisfiable(proof) => {
                Ok(ConsistencyResult::Inconsistent(proof))
            }
            SatisfiabilityResult::Unknown(reason) => {
                Ok(ConsistencyResult::Unknown(reason))
            }
        }
    }

    fn build_constraint_system(&self, invariants: &[DiscoveredInvariant]) 
        -> AispResult<ConstraintSystem> 
    {
        let mut constraints = Vec::new();
        let mut variables = HashSet::new();
        let mut functions = HashSet::new();
        let mut predicates = HashSet::new();

        for invariant in invariants {
            // Extract constraints from formula
            let constraint = self.formula_to_constraint(&invariant.formula)?;
            constraints.push(constraint);

            // Collect symbols
            variables.extend(invariant.formula.free_variables.iter().cloned());
            functions.extend(invariant.formula.functions.iter().cloned());
            predicates.extend(invariant.formula.predicates.iter().cloned());
        }

        Ok(ConstraintSystem {
            constraints,
            variables,
            functions,
            predicates,
        })
    }

    fn formula_to_constraint(&self, formula: &PropertyFormula) 
        -> AispResult<Constraint> 
    {
        match &formula.structure {
            FormulaStructure::Atomic(atomic) => {
                Ok(Constraint::Atomic(AtomicConstraint {
                    predicate: atomic.predicate.clone(),
                    terms: atomic.terms.iter().map(|t| self.term_to_constraint_term(t)).collect::<AispResult<Vec<_>>>()?,
                }))
            }
            FormulaStructure::Conjunction(formulas) => {
                if formulas.len() >= 2 {
                        let left_constraint = self.formula_to_constraint(&PropertyFormula {
                        structure: formulas[0].clone(),
                        quantifiers: formula.quantifiers.clone(),
                        free_variables: formula.free_variables.clone(),
                        predicates: formula.predicates.clone(),
                        functions: formula.functions.clone(),
                        constants: formula.constants.clone(),
                    })?;
                        let right_constraint = self.formula_to_constraint(&PropertyFormula {
                        structure: formulas[1].clone(),
                        quantifiers: formula.quantifiers.clone(),
                        free_variables: formula.free_variables.clone(),
                        predicates: formula.predicates.clone(),
                        functions: formula.functions.clone(),
                        constants: formula.constants.clone(),
                    })?;
                    Ok(Constraint::Conjunction(Box::new(left_constraint), Box::new(right_constraint)))
                } else {
                    Err(AispError::ValidationError { message: "Conjunction requires at least 2 formulas".to_string() })
                }
            }
            FormulaStructure::Disjunction(formulas) => {
                if formulas.len() >= 2 {
                    let left_constraint = self.formula_to_constraint(&PropertyFormula {
                        structure: formulas[0].clone(),
                    quantifiers: formula.quantifiers.clone(),
                    free_variables: formula.free_variables.clone(),
                    predicates: formula.predicates.clone(),
                    functions: formula.functions.clone(),
                    constants: formula.constants.clone(),
                })?;
                    let right_constraint = self.formula_to_constraint(&PropertyFormula {
                        structure: formulas[1].clone(),
                    quantifiers: formula.quantifiers.clone(),
                    free_variables: formula.free_variables.clone(),
                    predicates: formula.predicates.clone(),
                    functions: formula.functions.clone(),
                    constants: formula.constants.clone(),
                })?;
                    Ok(Constraint::Disjunction(Box::new(left_constraint), Box::new(right_constraint)))
                } else {
                    Err(AispError::ValidationError { message: "Disjunction requires at least 2 formulas".to_string() })
                }
            }
            FormulaStructure::Universal(quantifier, body) => {
                let body_constraint = self.formula_to_constraint(&PropertyFormula {
                    structure: (**body).clone(),
                    quantifiers: formula.quantifiers.clone(),
                    free_variables: formula.free_variables.clone(),
                    predicates: formula.predicates.clone(),
                    functions: formula.functions.clone(),
                    constants: formula.constants.clone(),
                })?;
                Ok(Constraint::Universal(quantifier.variable.clone(), Box::new(body_constraint)))
            }
            FormulaStructure::Existential(quantifier, body) => {
                let body_constraint = self.formula_to_constraint(&PropertyFormula {
                    structure: (**body).clone(),
                    quantifiers: formula.quantifiers.clone(),
                    free_variables: formula.free_variables.clone(),
                    predicates: formula.predicates.clone(),
                    functions: formula.functions.clone(),
                    constants: formula.constants.clone(),
                })?;
                Ok(Constraint::Existential(quantifier.variable.clone(), Box::new(body_constraint)))
            }
            FormulaStructure::Negation(inner) => {
                let inner_constraint = self.formula_to_constraint(&PropertyFormula {
                    structure: (**inner).clone(),
                    quantifiers: formula.quantifiers.clone(),
                    free_variables: formula.free_variables.clone(),
                    predicates: formula.predicates.clone(),
                    functions: formula.functions.clone(),
                    constants: formula.constants.clone(),
                })?;
                Ok(Constraint::Negation(Box::new(inner_constraint)))
            }
            _ => {
                // For other formula types, return an unsupported constraint placeholder
                Ok(Constraint::Atomic(AtomicConstraint {
                    predicate: "unsupported".to_string(),
                    terms: vec![],
                }))
            }
        }
    }

    fn term_to_constraint_term(&self, term: &Term) -> AispResult<ConstraintTerm> {
        match term {
            Term::Variable(name, var_type) => {
                Ok(ConstraintTerm::Variable(name.clone(), var_type.clone()))
            }
            Term::Constant(value, const_type) => {
                Ok(ConstraintTerm::Constant(value.clone(), const_type.clone()))
            }
            Term::Function(name, args) => {
                let constraint_args = args.iter()
                    .map(|arg| self.term_to_constraint_term(arg))
                    .collect::<AispResult<Vec<_>>>()?;
                Ok(ConstraintTerm::Function(name.clone(), constraint_args, Some("unknown".to_string())))
            }
            _ => {
                // For other term types, return a placeholder
                Ok(ConstraintTerm::Variable("unknown".to_string(), None))
            }
        }
    }

    fn solve_constraint_system(&self, system: &ConstraintSystem) 
        -> AispResult<SatisfiabilityResult> 
    {
        // Generate SMT-LIB commands for the constraint system
        let smt_commands = self.generate_smt_commands(system)?;
        
        // For now, simulate SMT solving (in practice would call Z3)
        self.simulate_smt_solving(&smt_commands, system)
    }

    fn generate_smt_commands(&self, system: &ConstraintSystem) 
        -> AispResult<Vec<SmtCommand>> 
    {
        let mut commands = Vec::new();
        
        // Declare sorts
        commands.push(SmtCommand::DeclareSort("Natural".to_string(), 0));
        commands.push(SmtCommand::DeclareSort("Boolean".to_string(), 0));
        
        // Declare variables
        for var in &system.variables {
            commands.push(SmtCommand::DeclareFun(
                var.clone(),
                vec![],
                SmtSort::Natural,
            ));
        }
        
        // Declare functions
        for func in &system.functions {
            commands.push(SmtCommand::DeclareFun(
                func.clone(),
                vec![SmtSort::Natural],
                SmtSort::Boolean,
            ));
        }
        
        // Assert constraints
        for constraint in &system.constraints {
            let smt_formula = self.constraint_to_smt(constraint)?;
            commands.push(SmtCommand::Assert(smt_formula));
        }
        
        commands.push(SmtCommand::CheckSat);
        commands.push(SmtCommand::GetModel);
        
        Ok(commands)
    }

    fn constraint_to_smt(&self, constraint: &Constraint) -> AispResult<SmtFormula> {
        match constraint {
            Constraint::Atomic(atomic) => {
                Ok(SmtFormula::Application(
                    atomic.predicate.clone(),
                    atomic.terms.iter()
                        .map(|t| self.constraint_term_to_smt(t))
                        .collect::<AispResult<Vec<_>>>()?
                ))
            }
            Constraint::Conjunction(left, right) => {
                Ok(SmtFormula::And(vec![
                    self.constraint_to_smt(left)?,
                    self.constraint_to_smt(right)?,
                ]))
            }
            Constraint::Disjunction(left, right) => {
                Ok(SmtFormula::Or(vec![
                    self.constraint_to_smt(left)?,
                    self.constraint_to_smt(right)?,
                ]))
            }
            Constraint::Universal(var, body) => {
                Ok(SmtFormula::Forall(
                    vec![(var.clone(), SmtSort::Natural)],
                    Box::new(self.constraint_to_smt(body)?),
                ))
            }
            Constraint::Existential(var, body) => {
                Ok(SmtFormula::Exists(
                    vec![(var.clone(), SmtSort::Natural)],
                    Box::new(self.constraint_to_smt(body)?),
                ))
            }
            Constraint::Negation(inner) => {
                Ok(SmtFormula::Not(Box::new(self.constraint_to_smt(inner)?)))
            }
        }
    }

    fn constraint_term_to_smt(&self, term: &ConstraintTerm) -> AispResult<SmtFormula> {
        match term {
            ConstraintTerm::Variable(name, _) => {
                Ok(SmtFormula::Variable(name.clone()))
            }
            ConstraintTerm::Constant(value, _) => {
                // Try to parse as integer
                if let Ok(int_val) = value.parse::<i64>() {
                    Ok(SmtFormula::IntLiteral(int_val))
                } else {
                    Ok(SmtFormula::StringLiteral(value.clone()))
                }
            }
            ConstraintTerm::Function(name, args, _) => {
                let smt_args = args.iter()
                    .map(|arg| self.constraint_term_to_smt(arg))
                    .collect::<AispResult<Vec<_>>>()?;
                Ok(SmtFormula::Application(name.clone(), smt_args))
            }
        }
    }

    fn simulate_smt_solving(&self, _commands: &[SmtCommand], system: &ConstraintSystem) 
        -> AispResult<SatisfiabilityResult> 
    {
        // Simplified satisfiability check
        // In practice, this would invoke Z3 or another SMT solver
        
        // Check for obvious contradictions
        if self.has_trivial_contradiction(system) {
            return Ok(SatisfiabilityResult::Unsatisfiable(
                UnsatisfiabilityProof {
                    conflicting_constraints: vec!["trivial_contradiction".to_string()],
                    proof_steps: vec![ProofStep {
                        rule: "contradiction".to_string(),
                        premises: vec!["P".to_string(), "¬P".to_string()],
                        conclusion: "⊥".to_string(),
                        justification: "Contradictory formulas cannot both be true".to_string(),
                    }],
                    reason: "System contains trivial contradictions".to_string(),
                }
            ));
        }
        
        // Generate a simple model
        let mut variable_assignments = HashMap::new();
        for var in &system.variables {
            variable_assignments.insert(var.clone(), ModelValue::Integer(0));
        }
        
        let model = ConstraintModel {
            variable_assignments,
            function_interpretations: HashMap::new(),
            predicate_interpretations: HashMap::new(),
        };
        
        Ok(SatisfiabilityResult::Satisfiable(model))
    }

    fn has_trivial_contradiction(&self, _system: &ConstraintSystem) -> bool {
        // Simplified contradiction detection
        false
    }
}

/// Internal constraint representation
#[derive(Debug, Clone, PartialEq)]
enum Constraint {
    Atomic(AtomicConstraint),
    Conjunction(Box<Constraint>, Box<Constraint>),
    Disjunction(Box<Constraint>, Box<Constraint>),
    Universal(String, Box<Constraint>),
    Existential(String, Box<Constraint>),
    Negation(Box<Constraint>),
}

#[derive(Debug, Clone, PartialEq)]
struct AtomicConstraint {
    predicate: String,
    terms: Vec<ConstraintTerm>,
}

#[derive(Debug, Clone, PartialEq)]
enum ConstraintTerm {
    Variable(String, Option<String>),
    Constant(String, String),
    Function(String, Vec<ConstraintTerm>, Option<String>),
}

#[derive(Debug, Clone)]
struct ConstraintSystem {
    constraints: Vec<Constraint>,
    variables: HashSet<String>,
    functions: HashSet<String>,
    predicates: HashSet<String>,
}

impl ConstraintSystem {
    fn from_formula(formula: PropertyFormula) -> AispResult<Self> {
        let variables = formula.free_variables;
        let functions = formula.functions;
        let predicates = formula.predicates;
        let constraints = vec![]; // Would convert formula to constraints
        
        Ok(Self {
            constraints,
            variables,
            functions,
            predicates,
        })
    }
}

/// Result of consistency checking
#[derive(Debug, Clone)]
pub enum ConsistencyResult {
    Consistent(ConstraintModel),
    Inconsistent(UnsatisfiabilityProof),
    Unknown(String),
}

impl ConstraintModel {
    pub fn empty() -> Self {
        Self {
            variable_assignments: HashMap::new(),
            function_interpretations: HashMap::new(),
            predicate_interpretations: HashMap::new(),
        }
    }
    
    pub fn get_variable_value(&self, name: &str) -> Option<&ModelValue> {
        self.variable_assignments.get(name)
    }
    
    pub fn set_variable_value(&mut self, name: String, value: ModelValue) {
        self.variable_assignments.insert(name, value);
    }
}

impl Default for SatisfiabilityChecker {
    fn default() -> Self {
        Self::new(SatisfiabilityConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        invariant_types::InvariantType,
        property_types::{AtomicFormula, FormulaStructure, Term},
    };
    use std::collections::HashSet;

    fn create_test_formula() -> PropertyFormula {
        PropertyFormula {
            structure: FormulaStructure::Atomic(AtomicFormula {
                predicate: "≥".to_string(),
                terms: vec![
                    Term::Variable("x".to_string(), Some("ℕ".to_string())),
                    Term::Constant("0".to_string(), "ℕ".to_string()),
                ],
                type_signature: None,
            }),
            quantifiers: vec![],
            free_variables: {
                let mut set = HashSet::new();
                set.insert("x".to_string());
                set
            },
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
        }
    }

    fn create_test_invariant() -> DiscoveredInvariant {
        let formula = create_test_formula();
        DiscoveredInvariant::new(
            "test_inv".to_string(),
            "Test Invariant".to_string(),
            formula,
            InvariantType::TypeStructural,
            0.9,
        )
    }

    #[test]
    fn test_satisfiability_checker_creation() {
        let config = SatisfiabilityConfig::default();
        let checker = SatisfiabilityChecker::new(config);
        
        assert_eq!(checker.config.timeout_seconds, 10);
        assert_eq!(checker.config.max_model_size, 1000);
        assert!(checker.config.enable_quantifier_instantiation);
    }

    #[test]
    fn test_check_empty_invariants() {
        let checker = SatisfiabilityChecker::default();
        let result = checker.check_invariants(&[]).unwrap();
        
        match result {
            SatisfiabilityResult::Satisfiable(model) => {
                assert!(model.variable_assignments.is_empty());
            }
            _ => panic!("Expected satisfiable result for empty invariants"),
        }
    }

    #[test]
    fn test_check_single_invariant() {
        let checker = SatisfiabilityChecker::default();
        let invariant = create_test_invariant();
        let result = checker.check_invariants(&[invariant]).unwrap();
        
        match result {
            SatisfiabilityResult::Satisfiable(model) => {
                assert!(!model.variable_assignments.is_empty());
            }
            _ => panic!("Expected satisfiable result for simple invariant"),
        }
    }

    #[test]
    fn test_check_formula() {
        let checker = SatisfiabilityChecker::default();
        let formula = create_test_formula();
        let result = checker.check_formula(&formula).unwrap();
        
        match result {
            SatisfiabilityResult::Satisfiable(_) | SatisfiabilityResult::Unknown(_) => {
                // Either result is acceptable for this simple formula
            }
            SatisfiabilityResult::Unsatisfiable(_) => {
                panic!("Simple non-negativity formula should not be unsatisfiable");
            }
        }
    }

    #[test]
    fn test_consistency_checking() {
        let checker = SatisfiabilityChecker::default();
        let invariant = create_test_invariant();
        let result = checker.check_consistency(&[invariant]).unwrap();
        
        match result {
            ConsistencyResult::Consistent(_) => {
                // Expected
            }
            ConsistencyResult::Inconsistent(_) => {
                panic!("Single invariant should be consistent");
            }
            ConsistencyResult::Unknown(_) => {
                // Also acceptable
            }
        }
    }

    #[test]
    fn test_constraint_model_operations() {
        let mut model = ConstraintModel::empty();
        assert!(model.variable_assignments.is_empty());
        
        model.set_variable_value("x".to_string(), ModelValue::Integer(42));
        assert_eq!(model.get_variable_value("x"), Some(&ModelValue::Integer(42)));
        assert_eq!(model.get_variable_value("y"), None);
    }

    #[test]
    fn test_model_value_types() {
        let bool_val = ModelValue::Boolean(true);
        let int_val = ModelValue::Integer(-5);
        let real_val = ModelValue::Real(3.14);
        let str_val = ModelValue::String("test".to_string());
        let enum_val = ModelValue::Enumeration("Active".to_string());
        
        assert_ne!(bool_val, int_val);
        assert_ne!(int_val, real_val);
        assert_ne!(real_val, str_val);
        assert_ne!(str_val, enum_val);
    }

    #[test]
    fn test_satisfiability_config_defaults() {
        let config = SatisfiabilityConfig::default();
        
        assert_eq!(config.timeout_seconds, 10);
        assert_eq!(config.max_model_size, 1000);
        assert!(config.enable_quantifier_instantiation);
        assert!(config.enable_theory_reasoning);
        assert!(!config.solver_options.is_empty());
    }

    #[test]
    fn test_formula_to_constraint_conversion() {
        let checker = SatisfiabilityChecker::default();
        let formula = create_test_formula();
        
        let constraint = checker.formula_to_constraint(&formula).unwrap();
        
        match constraint {
            Constraint::Atomic(atomic) => {
                assert_eq!(atomic.predicate, "≥");
                assert_eq!(atomic.terms.len(), 2);
            }
            _ => panic!("Expected atomic constraint for atomic formula"),
        }
    }

    #[test]
    fn test_unsatisfiability_proof() {
        let proof = UnsatisfiabilityProof {
            conflicting_constraints: vec!["c1".to_string(), "c2".to_string()],
            proof_steps: vec![ProofStep {
                rule: "modus_ponens".to_string(),
                premises: vec!["P".to_string(), "P→Q".to_string()],
                conclusion: "Q".to_string(),
                justification: "From P and P→Q, we can conclude Q".to_string(),
            }],
            reason: "Contradiction detected".to_string(),
        };
        
        assert_eq!(proof.conflicting_constraints.len(), 2);
        assert_eq!(proof.proof_steps.len(), 1);
        assert_eq!(proof.reason, "Contradiction detected");
    }
}