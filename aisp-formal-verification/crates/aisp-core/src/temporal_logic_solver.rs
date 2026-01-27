//! Temporal logic formula solving and satisfiability checking
//!
//! This module provides capabilities for solving Linear Temporal Logic (LTL)
//! and Computation Tree Logic (CTL) formulas, including satisfiability checking,
//! formula evaluation, and constraint solving.

use crate::error::*;
use crate::ast::canonical::Span;
use crate::temporal_operator_analyzer::{TemporalOperator, OperatorInstance};
use crate::temporal_pattern_detector::{TemporalPattern, PatternType};
use std::collections::{HashMap, HashSet};

/// Logical value for temporal logic
#[derive(Debug, Clone, PartialEq)]
pub enum LogicalValue {
    True,
    False,
    Unknown,
}

/// Temporal logic formula solver
pub struct TemporalLogicSolver {
    /// Formula cache for performance
    formula_cache: HashMap<String, SolverResult>,
    /// State space cache
    state_cache: HashMap<String, StateSpace>,
    /// Solver configuration
    config: SolverConfig,
}

/// Solver configuration options
#[derive(Debug, Clone)]
pub struct SolverConfig {
    /// Maximum search depth for bounded model checking
    pub max_depth: usize,
    /// Timeout for solver operations (milliseconds)
    pub timeout_ms: u64,
    /// Enable formula simplification
    pub enable_simplification: bool,
    /// Use approximation algorithms for complex formulas
    pub use_approximation: bool,
}

/// Result of formula solving
#[derive(Debug, Clone)]
pub struct SolverResult {
    /// Formula identifier
    pub formula_id: String,
    /// Satisfiability status
    pub satisfiable: SatisfiabilityStatus,
    /// Witness trace (if satisfiable)
    pub witness: Option<ExecutionTrace>,
    /// Counterexample (if unsatisfiable)
    pub counterexample: Option<Counterexample>,
    /// Solver statistics
    pub statistics: SolverStatistics,
    /// Simplified formula (if simplification enabled)
    pub simplified_formula: Option<String>,
}

/// Satisfiability status of a formula
#[derive(Debug, Clone, PartialEq)]
pub enum SatisfiabilityStatus {
    /// Formula is satisfiable
    Satisfiable,
    /// Formula is unsatisfiable
    Unsatisfiable,
    /// Satisfiability unknown (timeout or complexity)
    Unknown,
    /// Formula is valid (always true)
    Valid,
    /// Formula is unsatisfiable (always false)
    Contradiction,
}

/// Execution trace for temporal formulas
#[derive(Debug, Clone)]
pub struct ExecutionTrace {
    /// Sequence of states
    pub states: Vec<StateSnapshot>,
    /// State transitions
    pub transitions: Vec<Transition>,
    /// Trace length
    pub length: usize,
    /// Whether trace represents infinite execution
    pub is_infinite: bool,
    /// Loop point for infinite traces
    pub loop_point: Option<usize>,
}

/// State snapshot in execution
#[derive(Debug, Clone)]
pub struct StateSnapshot {
    /// State identifier
    pub id: String,
    /// Variable assignments
    pub assignments: HashMap<String, LogicalValue>,
    /// State properties
    pub properties: HashSet<String>,
    /// Timestamp or step number
    pub step: usize,
}

/// State transition
#[derive(Debug, Clone)]
pub struct Transition {
    /// Source state
    pub from: String,
    /// Target state
    pub to: String,
    /// Transition guard
    pub guard: Option<String>,
    /// Transition action
    pub action: Option<String>,
}

/// Counterexample for unsatisfiable formulas
#[derive(Debug, Clone)]
pub struct Counterexample {
    /// Formula that was violated
    pub violated_formula: String,
    /// Execution trace showing violation
    pub trace: ExecutionTrace,
    /// Explanation of violation
    pub explanation: String,
    /// Step where violation occurs
    pub violation_step: usize,
}

/// Solver performance statistics
#[derive(Debug, Clone)]
pub struct SolverStatistics {
    /// Solving time in milliseconds
    pub solving_time_ms: u64,
    /// Number of states explored
    pub states_explored: usize,
    /// Number of transitions considered
    pub transitions_evaluated: usize,
    /// Peak memory usage (bytes)
    pub peak_memory_bytes: usize,
    /// Number of solver iterations
    pub iterations: usize,
}

/// State space representation
#[derive(Debug, Clone)]
pub struct StateSpace {
    /// All states in the space
    pub states: HashMap<String, StateNode>,
    /// Initial states
    pub initial_states: HashSet<String>,
    /// Final/accepting states
    pub final_states: HashSet<String>,
    /// Transitions between states
    pub transitions: Vec<StateTransition>,
}

/// State node in state space
#[derive(Debug, Clone)]
pub struct StateNode {
    /// State identifier
    pub id: String,
    /// Variable values in this state
    pub valuations: HashMap<String, LogicalValue>,
    /// Atomic propositions true in this state
    pub atomic_props: HashSet<String>,
    /// Whether this is an initial state
    pub is_initial: bool,
    /// Whether this is a final state
    pub is_final: bool,
}

/// State transition in state space
#[derive(Debug, Clone)]
pub struct StateTransition {
    /// Source state
    pub from: String,
    /// Target state
    pub to: String,
    /// Transition condition
    pub condition: Option<String>,
    /// Transition probability (for probabilistic systems)
    pub probability: Option<f64>,
}

/// Formula analysis result
#[derive(Debug, Clone)]
pub struct FormulaAnalysisResult {
    /// All analyzed formulas
    pub formulas: Vec<AnalyzedFormula>,
    /// Overall satisfiability status
    pub overall_status: SatisfiabilityStatus,
    /// Formula dependencies
    pub dependencies: HashMap<String, Vec<String>>,
    /// Solver warnings
    pub warnings: Vec<AispWarning>,
    /// Performance summary
    pub performance_summary: PerformanceSummary,
}

/// Analyzed temporal formula
#[derive(Debug, Clone)]
pub struct AnalyzedFormula {
    /// Formula identifier
    pub id: String,
    /// Original formula text
    pub formula: String,
    /// Formula type (LTL or CTL)
    pub formula_type: FormulaType,
    /// Complexity metrics
    pub complexity: FormulaComplexity,
    /// Solver result
    pub result: SolverResult,
}

/// Type of temporal formula
#[derive(Debug, Clone, PartialEq)]
pub enum FormulaType {
    /// Linear Temporal Logic
    LTL,
    /// Computation Tree Logic
    CTL,
    /// Mixed or unknown type
    Mixed,
}

/// Formula complexity metrics
#[derive(Debug, Clone)]
pub struct FormulaComplexity {
    /// Number of temporal operators
    pub temporal_operators: usize,
    /// Number of boolean operators
    pub boolean_operators: usize,
    /// Maximum nesting depth
    pub nesting_depth: usize,
    /// Number of atomic propositions
    pub atomic_propositions: usize,
    /// Complexity score (0.0-1.0)
    pub complexity_score: f64,
}

/// Performance summary
#[derive(Debug, Clone)]
pub struct PerformanceSummary {
    /// Total solving time
    pub total_time_ms: u64,
    /// Average time per formula
    pub avg_time_per_formula_ms: f64,
    /// Maximum memory usage
    pub max_memory_bytes: usize,
    /// Number of timeouts
    pub timeouts: usize,
}

impl TemporalLogicSolver {
    /// Create a new temporal logic solver
    pub fn new() -> Self {
        Self {
            formula_cache: HashMap::new(),
            state_cache: HashMap::new(),
            config: SolverConfig::default(),
        }
    }

    /// Create solver with custom configuration
    pub fn with_config(config: SolverConfig) -> Self {
        Self {
            formula_cache: HashMap::new(),
            state_cache: HashMap::new(),
            config,
        }
    }

    /// Solve temporal logic formulas from document
    pub fn solve_formulas(
        &mut self,
        operators: &[OperatorInstance],
        patterns: &[TemporalPattern],
        document_size: usize,
    ) -> FormulaAnalysisResult {
        let start_time = std::time::Instant::now();
        let mut analyzed_formulas = Vec::new();
        let mut warnings = Vec::new();
        let mut dependencies = HashMap::new();
        let mut timeouts = 0;

        // Extract formulas from operators and patterns
        let formulas = self.extract_formulas(operators, patterns);

        // Analyze each formula
        for formula in formulas {
            let formula_start = std::time::Instant::now();
            
            // Check cache first
            if let Some(cached_result) = self.formula_cache.get(&formula.id) {
                analyzed_formulas.push(AnalyzedFormula {
                    id: formula.id.clone(),
                    formula: formula.formula.clone(),
                    formula_type: formula.formula_type.clone(),
                    complexity: formula.complexity.clone(),
                    result: cached_result.clone(),
                });
                continue;
            }

            // Solve the formula
            let result = match self.solve_single_formula(&formula) {
                Ok(res) => res,
                Err(_) => {
                    timeouts += 1;
                    SolverResult {
                        formula_id: formula.id.clone(),
                        satisfiable: SatisfiabilityStatus::Unknown,
                        witness: None,
                        counterexample: None,
                        statistics: SolverStatistics {
                            solving_time_ms: self.config.timeout_ms,
                            states_explored: 0,
                            transitions_evaluated: 0,
                            peak_memory_bytes: 0,
                            iterations: 0,
                        },
                        simplified_formula: None,
                    }
                }
            };

            // Cache the result
            self.formula_cache.insert(formula.id.clone(), result.clone());

            // Check for warnings
            if result.statistics.solving_time_ms > self.config.timeout_ms / 2 {
                warnings.push(AispWarning::warning(format!(
                    "Formula {} took {}ms to solve (approaching timeout)",
                    formula.id, result.statistics.solving_time_ms
                )));
            }

            analyzed_formulas.push(AnalyzedFormula {
                id: formula.id.clone(),
                formula: formula.formula.clone(),
                formula_type: formula.formula_type.clone(),
                complexity: formula.complexity.clone(),
                result,
            });
        }

        // Determine overall status
        let overall_status = self.determine_overall_status(&analyzed_formulas);

        // Extract dependencies
        dependencies = self.extract_formula_dependencies(&analyzed_formulas);

        let total_time = start_time.elapsed().as_millis() as u64;
        let performance_summary = PerformanceSummary {
            total_time_ms: total_time,
            avg_time_per_formula_ms: if analyzed_formulas.is_empty() {
                0.0
            } else {
                total_time as f64 / analyzed_formulas.len() as f64
            },
            max_memory_bytes: self.estimate_memory_usage(),
            timeouts,
        };

        FormulaAnalysisResult {
            formulas: analyzed_formulas,
            overall_status,
            dependencies,
            warnings,
            performance_summary,
        }
    }

    /// Extract formulas from operators and patterns
    fn extract_formulas(&self, operators: &[OperatorInstance], patterns: &[TemporalPattern]) -> Vec<AnalyzedFormula> {
        let mut formulas = Vec::new();
        let mut formula_id_counter = 0;

        // Extract formulas from operator sequences
        for window in operators.windows(2) {
            if window.len() == 2 {
                let formula_text = format!("{} {}", window[0].operator, window[1].operator);
                let complexity = self.calculate_complexity(&formula_text);
                let formula_type = self.determine_formula_type(&[window[0].operator.clone(), window[1].operator.clone()]);

                formulas.push(AnalyzedFormula {
                    id: format!("formula_{}", formula_id_counter),
                    formula: formula_text,
                    formula_type,
                    complexity,
                    result: SolverResult::empty(), // Will be filled during solving
                });
                formula_id_counter += 1;
            }
        }

        // Extract formulas from patterns
        for pattern in patterns {
            for instance in &pattern.instances {
                let complexity = self.calculate_complexity(&instance.formula);
                let formula_type = self.determine_pattern_formula_type(&pattern.pattern_type);

                formulas.push(AnalyzedFormula {
                    id: format!("pattern_{}", formula_id_counter),
                    formula: instance.formula.clone(),
                    formula_type,
                    complexity,
                    result: SolverResult::empty(),
                });
                formula_id_counter += 1;
            }
        }

        formulas
    }

    /// Solve a single temporal logic formula
    fn solve_single_formula(&mut self, formula: &AnalyzedFormula) -> AispResult<SolverResult> {
        let start_time = std::time::Instant::now();
        let mut states_explored = 0;
        let mut transitions_evaluated = 0;

        // Build state space for the formula
        let state_space = self.build_state_space(&formula.formula)?;
        states_explored = state_space.states.len();

        // Perform satisfiability checking based on formula type
        let (satisfiable, witness, counterexample) = match formula.formula_type {
            FormulaType::LTL => self.check_ltl_satisfiability(&formula.formula, &state_space)?,
            FormulaType::CTL => self.check_ctl_satisfiability(&formula.formula, &state_space)?,
            FormulaType::Mixed => {
                // Try LTL first, then CTL
                match self.check_ltl_satisfiability(&formula.formula, &state_space) {
                    Ok(result) => result,
                    Err(_) => self.check_ctl_satisfiability(&formula.formula, &state_space)?,
                }
            }
        };

        transitions_evaluated = state_space.transitions.len();
        let solving_time = start_time.elapsed().as_millis() as u64;

        // Simplify formula if enabled
        let simplified_formula = if self.config.enable_simplification {
            Some(self.simplify_formula(&formula.formula))
        } else {
            None
        };

        let statistics = SolverStatistics {
            solving_time_ms: solving_time,
            states_explored,
            transitions_evaluated,
            peak_memory_bytes: self.estimate_memory_usage(),
            iterations: 1,
        };

        Ok(SolverResult {
            formula_id: formula.id.clone(),
            satisfiable,
            witness,
            counterexample,
            statistics,
            simplified_formula,
        })
    }

    /// Check LTL formula satisfiability
    fn check_ltl_satisfiability(
        &self,
        _formula: &str,
        _state_space: &StateSpace,
    ) -> AispResult<(SatisfiabilityStatus, Option<ExecutionTrace>, Option<Counterexample>)> {
        // TODO: Implement proper LTL satisfiability checking using tableau method or automata construction
        // For now, return satisfiable as a placeholder
        Ok((SatisfiabilityStatus::Satisfiable, None, None))
    }

    /// Check CTL formula satisfiability
    fn check_ctl_satisfiability(
        &self,
        _formula: &str,
        _state_space: &StateSpace,
    ) -> AispResult<(SatisfiabilityStatus, Option<ExecutionTrace>, Option<Counterexample>)> {
        // TODO: Implement CTL model checking algorithm
        // For now, return satisfiable as a placeholder
        Ok((SatisfiabilityStatus::Satisfiable, None, None))
    }

    /// Build state space for formula analysis
    fn build_state_space(&mut self, formula: &str) -> AispResult<StateSpace> {
        // Check cache first
        if let Some(cached) = self.state_cache.get(formula) {
            return Ok(cached.clone());
        }

        // Build minimal state space for the formula
        let mut states = HashMap::new();
        let mut transitions = Vec::new();
        let initial_states = HashSet::from_iter(vec!["s0".to_string()]);
        let final_states = HashSet::from_iter(vec!["sf".to_string()]);

        // Create initial state
        states.insert("s0".to_string(), StateNode {
            id: "s0".to_string(),
            valuations: HashMap::new(),
            atomic_props: HashSet::new(),
            is_initial: true,
            is_final: false,
        });

        // Create final state
        states.insert("sf".to_string(), StateNode {
            id: "sf".to_string(),
            valuations: HashMap::new(),
            atomic_props: HashSet::new(),
            is_initial: false,
            is_final: true,
        });

        // Add transition
        transitions.push(StateTransition {
            from: "s0".to_string(),
            to: "sf".to_string(),
            condition: None,
            probability: Some(1.0),
        });

        let state_space = StateSpace {
            states,
            initial_states,
            final_states,
            transitions,
        };

        // Cache the result
        self.state_cache.insert(formula.to_string(), state_space.clone());
        Ok(state_space)
    }

    /// Calculate formula complexity metrics
    fn calculate_complexity(&self, formula: &str) -> FormulaComplexity {
        let temporal_ops = formula.chars().filter(|c| "□◊XUR".contains(*c)).count();
        let boolean_ops = formula.chars().filter(|c| "∧∨¬→↔".contains(*c)).count();
        let nesting_depth = self.calculate_nesting_depth(formula);
        let atomic_props = self.count_atomic_propositions(formula);

        let complexity_score = ((temporal_ops + boolean_ops) as f64 / 10.0 + nesting_depth as f64 / 5.0).min(1.0);

        FormulaComplexity {
            temporal_operators: temporal_ops,
            boolean_operators: boolean_ops,
            nesting_depth,
            atomic_propositions: atomic_props,
            complexity_score,
        }
    }

    /// Calculate nesting depth of formula
    fn calculate_nesting_depth(&self, formula: &str) -> usize {
        let mut depth: usize = 0;
        let mut max_depth: usize = 0;

        for ch in formula.chars() {
            match ch {
                '(' | '[' | '{' => {
                    depth += 1;
                    max_depth = max_depth.max(depth);
                }
                ')' | ']' | '}' => {
                    depth = depth.saturating_sub(1);
                }
                _ => {}
            }
        }

        max_depth
    }

    /// Count atomic propositions in formula
    fn count_atomic_propositions(&self, formula: &str) -> usize {
        // Simplified: count alphabetic sequences as atomic propositions
        let mut count = 0;
        let mut in_prop = false;

        for ch in formula.chars() {
            if ch.is_alphabetic() {
                if !in_prop {
                    count += 1;
                    in_prop = true;
                }
            } else {
                in_prop = false;
            }
        }

        count
    }

    /// Determine formula type from operators
    fn determine_formula_type(&self, operators: &[TemporalOperator]) -> FormulaType {
        // Simplified heuristic: if only basic temporal operators, assume LTL
        for op in operators {
            match op {
                TemporalOperator::Always | TemporalOperator::Eventually | 
                TemporalOperator::Next | TemporalOperator::Until => continue,
                _ => return FormulaType::Mixed,
            }
        }
        FormulaType::LTL
    }

    /// Determine formula type from pattern type
    fn determine_pattern_formula_type(&self, pattern_type: &PatternType) -> FormulaType {
        match pattern_type {
            PatternType::Safety | PatternType::Liveness => FormulaType::LTL,
            _ => FormulaType::Mixed,
        }
    }

    /// Simplify temporal logic formula
    fn simplify_formula(&self, formula: &str) -> String {
        // TODO: Implement formula simplification rules
        // For now, just remove extra whitespace
        formula.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    /// Determine overall satisfiability status
    fn determine_overall_status(&self, formulas: &[AnalyzedFormula]) -> SatisfiabilityStatus {
        if formulas.is_empty() {
            return SatisfiabilityStatus::Valid;
        }

        let mut has_unsatisfiable = false;
        let mut has_unknown = false;

        for formula in formulas {
            match formula.result.satisfiable {
                SatisfiabilityStatus::Unsatisfiable | SatisfiabilityStatus::Contradiction => {
                    has_unsatisfiable = true;
                }
                SatisfiabilityStatus::Unknown => {
                    has_unknown = true;
                }
                _ => {}
            }
        }

        if has_unsatisfiable {
            SatisfiabilityStatus::Unsatisfiable
        } else if has_unknown {
            SatisfiabilityStatus::Unknown
        } else {
            SatisfiabilityStatus::Satisfiable
        }
    }

    /// Extract formula dependencies
    fn extract_formula_dependencies(&self, formulas: &[AnalyzedFormula]) -> HashMap<String, Vec<String>> {
        let mut dependencies = HashMap::new();
        
        // TODO: Implement actual dependency analysis
        // For now, return empty dependencies
        for formula in formulas {
            dependencies.insert(formula.id.clone(), Vec::new());
        }
        
        dependencies
    }

    /// Estimate current memory usage
    fn estimate_memory_usage(&self) -> usize {
        // Rough estimation based on cache sizes
        let formula_cache_size = self.formula_cache.len() * 1024; // Rough estimate per entry
        let state_cache_size = self.state_cache.len() * 2048; // Larger estimate for state spaces
        formula_cache_size + state_cache_size
    }
}

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            max_depth: 100,
            timeout_ms: 5000,
            enable_simplification: true,
            use_approximation: false,
        }
    }
}

impl SolverResult {
    /// Create empty solver result
    fn empty() -> Self {
        Self {
            formula_id: String::new(),
            satisfiable: SatisfiabilityStatus::Unknown,
            witness: None,
            counterexample: None,
            statistics: SolverStatistics {
                solving_time_ms: 0,
                states_explored: 0,
                transitions_evaluated: 0,
                peak_memory_bytes: 0,
                iterations: 0,
            },
            simplified_formula: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::temporal_operator_analyzer::OperatorContext;

    fn create_test_operator(op: TemporalOperator) -> OperatorInstance {
        OperatorInstance {
            operator: op,
            location: Span::new(1, 1, 1, 10),
            context: OperatorContext::Rule("test".to_string()),
            operands: vec!["p".to_string()],
            nesting_level: 1,
        }
    }

    #[test]
    fn test_solver_creation() {
        let solver = TemporalLogicSolver::new();
        assert!(solver.formula_cache.is_empty());
        assert!(solver.state_cache.is_empty());
    }

    #[test]
    fn test_complexity_calculation() {
        let solver = TemporalLogicSolver::new();
        let formula = "□(p → ◊q)";
        let complexity = solver.calculate_complexity(formula);
        
        assert_eq!(complexity.temporal_operators, 2); // □ and ◊
        assert!(complexity.nesting_depth > 0);
        assert!(complexity.complexity_score > 0.0);
    }

    #[test]
    fn test_nesting_depth_calculation() {
        let solver = TemporalLogicSolver::new();
        assert_eq!(solver.calculate_nesting_depth("p"), 0);
        assert_eq!(solver.calculate_nesting_depth("(p)"), 1);
        assert_eq!(solver.calculate_nesting_depth("((p))"), 2);
        assert_eq!(solver.calculate_nesting_depth("□(p → ◊(q))"), 2);
    }

    #[test]
    fn test_formula_type_determination() {
        let solver = TemporalLogicSolver::new();
        let ops = vec![TemporalOperator::Always, TemporalOperator::Eventually];
        assert_eq!(solver.determine_formula_type(&ops), FormulaType::LTL);
    }

    #[test]
    fn test_empty_formula_analysis() {
        let mut solver = TemporalLogicSolver::new();
        let operators = vec![];
        let patterns = vec![];
        
        let result = solver.solve_formulas(&operators, &patterns, 0);
        assert!(result.formulas.is_empty());
        assert_eq!(result.overall_status, SatisfiabilityStatus::Valid);
    }

    #[test]
    fn test_solver_config() {
        let config = SolverConfig {
            max_depth: 50,
            timeout_ms: 1000,
            enable_simplification: false,
            use_approximation: true,
        };
        
        let solver = TemporalLogicSolver::with_config(config.clone());
        assert_eq!(solver.config.max_depth, 50);
        assert_eq!(solver.config.timeout_ms, 1000);
        assert!(!solver.config.enable_simplification);
        assert!(solver.config.use_approximation);
    }
}