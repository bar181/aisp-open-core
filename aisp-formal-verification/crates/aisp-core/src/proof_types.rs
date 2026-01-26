//! Proof Types and Data Structures
//!
//! This module defines the core data structures for formal proofs,
//! proof results, and counterexamples used in theorem proving.

use crate::property_types::*;
use std::collections::HashMap;
use std::time::Duration;

/// Complete proof result
#[derive(Debug, Clone)]
pub struct ProofResult {
    /// Proof outcome
    pub outcome: ProofOutcome,
    /// Generated proof (if successful)
    pub proof: Option<FormalProof>,
    /// Counterexample (if disproven)
    pub counterexample: Option<Counterexample>,
    /// Proof search statistics
    pub stats: ProofStats,
    /// Proof search time
    pub search_time: Duration,
    /// Proof steps taken
    pub steps_explored: usize,
}

/// Proof outcome classification
#[derive(Debug, Clone, PartialEq)]
pub enum ProofOutcome {
    /// Property successfully proven
    Proven,
    /// Property disproven with counterexample
    Disproven,
    /// Proof search timed out
    Timeout,
    /// Unknown (no proof found within limits)
    Unknown,
    /// Error during proof search
    Error(String),
}

/// Formal proof representation
#[derive(Debug, Clone)]
pub struct FormalProof {
    /// Proven formula
    pub conclusion: PropertyFormula,
    /// Proof steps in natural deduction
    pub steps: Vec<ProofStep>,
    /// Axioms used in proof
    pub axioms_used: Vec<String>,
    /// Inference rules applied
    pub rules_applied: Vec<String>,
    /// Proof tree structure
    pub proof_tree: ProofTree,
    /// Proof validation result
    pub is_valid: bool,
    /// Proof complexity metrics
    pub complexity: ProofComplexity,
}

/// Single proof step in natural deduction
#[derive(Debug, Clone)]
pub struct ProofStep {
    /// Step number
    pub step_id: usize,
    /// Formula derived in this step
    pub formula: FormulaStructure,
    /// Justification for this step
    pub justification: StepJustification,
    /// Dependencies (previous step IDs)
    pub dependencies: Vec<usize>,
    /// Discharge level for assumptions
    pub discharge_level: usize,
    /// Annotations for proof checker
    pub annotations: HashMap<String, String>,
}

/// Step justification types
#[derive(Debug, Clone)]
pub enum StepJustification {
    /// Assumption introduction
    Assumption,
    /// Axiom application
    Axiom(String),
    /// Inference rule application
    InferenceRule(String, Vec<usize>),
    /// Definition expansion
    Definition(String),
    /// Lemma application
    Lemma(String),
    /// Contradiction derived
    Contradiction(Vec<usize>),
    /// Hypothesis discharge
    Discharge(usize),
}

/// Proof tree for structural representation
#[derive(Debug, Clone)]
pub struct ProofTree {
    /// Root formula (conclusion)
    pub root: FormulaStructure,
    /// Child subtrees
    pub children: Vec<ProofTree>,
    /// Rule used at this node
    pub rule: Option<String>,
    /// Node annotations
    pub annotations: HashMap<String, String>,
}

/// Proof complexity metrics
#[derive(Debug, Clone)]
pub struct ProofComplexity {
    /// Total number of steps
    pub step_count: usize,
    /// Maximum depth of proof
    pub max_depth: usize,
    /// Number of assumptions
    pub assumption_count: usize,
    /// Number of inference rules used
    pub rule_applications: usize,
    /// Proof branching factor
    pub branching_factor: f64,
    /// Complexity score (1-10)
    pub complexity_score: u8,
}

/// Counterexample for disproven properties
#[derive(Debug, Clone)]
pub struct Counterexample {
    /// Variable assignments
    pub assignments: HashMap<String, String>,
    /// Function interpretations
    pub functions: HashMap<String, FunctionInterpretation>,
    /// Evaluation trace
    pub trace: Vec<EvaluationStep>,
    /// Counterexample validity check
    pub is_valid: bool,
}

/// Function interpretation in counterexample
#[derive(Debug, Clone)]
pub struct FunctionInterpretation {
    /// Function name
    pub name: String,
    /// Input/output mappings
    pub mappings: Vec<(Vec<String>, String)>,
    /// Default value
    pub default: Option<String>,
}

/// Evaluation step in counterexample
#[derive(Debug, Clone)]
pub struct EvaluationStep {
    /// Formula being evaluated
    pub formula: FormulaStructure,
    /// Result of evaluation
    pub result: bool,
    /// Context at this step
    pub context: HashMap<String, String>,
}

/// Statistics about proof search
#[derive(Debug, Clone, Default)]
pub struct ProofStats {
    /// Steps explored during search
    pub steps_explored: usize,
    /// Maximum search depth reached
    pub max_depth_reached: usize,
    /// Number of backtrack operations
    pub backtrack_count: usize,
    /// Number of axioms applied
    pub axioms_applied: usize,
    /// Number of inference rules applied
    pub rules_applied: usize,
    /// Number of failed proof attempts
    pub failed_attempts: usize,
    /// Search time in milliseconds
    pub search_time_ms: u64,
    /// Memory usage during search
    pub memory_usage_mb: f64,
}

impl ProofResult {
    /// Create new proof result
    pub fn new(outcome: ProofOutcome) -> Self {
        Self {
            outcome,
            proof: None,
            counterexample: None,
            stats: ProofStats::default(),
            search_time: Duration::ZERO,
            steps_explored: 0,
        }
    }

    /// Check if proof was successful
    pub fn is_proven(&self) -> bool {
        matches!(self.outcome, ProofOutcome::Proven)
    }

    /// Check if property was disproven
    pub fn is_disproven(&self) -> bool {
        matches!(self.outcome, ProofOutcome::Disproven)
    }

    /// Check if result is conclusive
    pub fn is_conclusive(&self) -> bool {
        matches!(self.outcome, ProofOutcome::Proven | ProofOutcome::Disproven)
    }
}

impl FormalProof {
    /// Validate proof structure
    pub fn validate(&self) -> bool {
        // Basic validation - check step dependencies
        for step in &self.steps {
            for &dep in &step.dependencies {
                if dep >= step.step_id {
                    return false; // Invalid dependency
                }
            }
        }
        self.is_valid
    }

    /// Calculate proof metrics
    pub fn calculate_complexity(&mut self) {
        let step_count = self.steps.len();
        let max_depth = self.calculate_max_depth();
        let assumption_count = self.count_assumptions();
        let rule_applications = self.count_rule_applications();
        let branching_factor = self.calculate_branching_factor();
        
        // Calculate complexity score (1-10)
        let complexity_score = ((step_count.min(50) as f64 / 5.0) + 
                               (max_depth.min(20) as f64 / 2.0) +
                               (assumption_count.min(10) as f64)).min(10.0) as u8;

        self.complexity = ProofComplexity {
            step_count,
            max_depth,
            assumption_count,
            rule_applications,
            branching_factor,
            complexity_score,
        };
    }

    /// Calculate maximum proof depth
    fn calculate_max_depth(&self) -> usize {
        self.steps.iter()
            .map(|step| step.discharge_level)
            .max()
            .unwrap_or(0)
    }

    /// Count assumption steps
    fn count_assumptions(&self) -> usize {
        self.steps.iter()
            .filter(|step| matches!(step.justification, StepJustification::Assumption))
            .count()
    }

    /// Count rule applications
    fn count_rule_applications(&self) -> usize {
        self.steps.iter()
            .filter(|step| matches!(step.justification, StepJustification::InferenceRule(_, _)))
            .count()
    }

    /// Calculate average branching factor
    fn calculate_branching_factor(&self) -> f64 {
        if self.steps.is_empty() {
            return 0.0;
        }
        
        let total_deps: usize = self.steps.iter()
            .map(|step| step.dependencies.len())
            .sum();
        
        total_deps as f64 / self.steps.len() as f64
    }
}

impl ProofTree {
    /// Create leaf node
    pub fn leaf(formula: FormulaStructure) -> Self {
        Self {
            root: formula,
            children: Vec::new(),
            rule: None,
            annotations: HashMap::new(),
        }
    }

    /// Create node with children
    pub fn node(formula: FormulaStructure, rule: String, children: Vec<ProofTree>) -> Self {
        Self {
            root: formula,
            children,
            rule: Some(rule),
            annotations: HashMap::new(),
        }
    }

    /// Calculate tree depth
    pub fn depth(&self) -> usize {
        if self.children.is_empty() {
            1
        } else {
            1 + self.children.iter().map(|child| child.depth()).max().unwrap_or(0)
        }
    }
}

impl Counterexample {
    /// Create new counterexample
    pub fn new() -> Self {
        Self {
            assignments: HashMap::new(),
            functions: HashMap::new(),
            trace: Vec::new(),
            is_valid: false,
        }
    }

    /// Add variable assignment
    pub fn assign_variable(&mut self, var: String, value: String) {
        self.assignments.insert(var, value);
    }

    /// Add function interpretation
    pub fn add_function(&mut self, func: FunctionInterpretation) {
        self.functions.insert(func.name.clone(), func);
    }

    /// Validate counterexample
    pub fn validate(&mut self) -> bool {
        // Basic validation - check if assignments are consistent
        self.is_valid = !self.assignments.is_empty() || !self.functions.is_empty();
        self.is_valid
    }
}

impl Default for Counterexample {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof_result_creation() {
        let result = ProofResult::new(ProofOutcome::Proven);
        assert!(result.is_proven());
        assert!(!result.is_disproven());
        assert!(result.is_conclusive());
    }

    #[test]
    fn test_proof_outcome_equality() {
        assert_eq!(ProofOutcome::Proven, ProofOutcome::Proven);
        assert_ne!(ProofOutcome::Proven, ProofOutcome::Disproven);
        assert_eq!(ProofOutcome::Error("test".to_string()), ProofOutcome::Error("test".to_string()));
    }

    #[test]
    fn test_formal_proof_validation() {
        let proof = FormalProof {
            conclusion: PropertyFormula {
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
            steps: vec![],
            axioms_used: vec![],
            rules_applied: vec![],
            proof_tree: ProofTree::leaf(FormulaStructure::Atomic(AtomicFormula {
                predicate: "P".to_string(),
                terms: vec![],
                type_signature: None,
            })),
            is_valid: true,
            complexity: ProofComplexity {
                step_count: 0,
                max_depth: 0,
                assumption_count: 0,
                rule_applications: 0,
                branching_factor: 0.0,
                complexity_score: 1,
            },
        };

        assert!(proof.validate());
    }

    #[test]
    fn test_proof_tree_depth() {
        let leaf = ProofTree::leaf(FormulaStructure::Atomic(AtomicFormula {
            predicate: "P".to_string(),
            terms: vec![],
            type_signature: None,
        }));
        assert_eq!(leaf.depth(), 1);

        let tree = ProofTree::node(
            FormulaStructure::Atomic(AtomicFormula {
                predicate: "Q".to_string(),
                terms: vec![],
                type_signature: None,
            }),
            "modus_ponens".to_string(),
            vec![leaf],
        );
        assert_eq!(tree.depth(), 2);
    }

    #[test]
    fn test_counterexample_creation() {
        let mut counterexample = Counterexample::new();
        counterexample.assign_variable("x".to_string(), "42".to_string());
        
        assert!(counterexample.validate());
        assert!(counterexample.is_valid);
    }

    #[test]
    fn test_proof_stats_default() {
        let stats = ProofStats::default();
        assert_eq!(stats.steps_explored, 0);
        assert_eq!(stats.search_time_ms, 0);
    }
}