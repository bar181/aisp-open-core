//! Temporal model checking for AISP documents
//!
//! This module provides model checking capabilities for temporal logic properties,
//! including state space exploration, path analysis, and property verification.

use crate::ast::*;
use crate::error::*;
use crate::temporal_logic_solver::{StateSpace, StateNode, StateTransition, ExecutionTrace};
use std::collections::{HashMap, HashSet, VecDeque};

/// Model checking verification result
#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub property_name: String,
    pub status: VerificationStatus,
    pub execution_time: f64,
    pub memory_usage: usize,
}

/// Temporal model checker
pub struct TemporalModelChecker {
    /// State space cache
    state_spaces: HashMap<String, StateSpace>,
    /// Model checking configuration
    config: ModelCheckerConfig,
    /// Verification results cache
    results_cache: HashMap<String, VerificationResult>,
}

/// Model checker configuration
#[derive(Debug, Clone)]
pub struct ModelCheckerConfig {
    /// Maximum state space size
    pub max_states: usize,
    /// Maximum search depth
    pub max_depth: usize,
    /// Enable state space reduction techniques
    pub enable_reduction: bool,
    /// Use symbolic model checking
    pub use_symbolic: bool,
    /// Timeout for verification (milliseconds)
    pub timeout_ms: u64,
}

/// Model checking result for a document
#[derive(Debug, Clone)]
pub struct ModelCheckingResult {
    /// All verified properties
    pub verified_properties: Vec<PropertyVerification>,
    /// State space analysis
    pub state_space_analysis: StateSpaceAnalysis,
    /// Reachability analysis
    pub reachability_analysis: ReachabilityAnalysis,
    /// Liveness analysis
    pub liveness_analysis: LivenessAnalysis,
    /// Safety analysis
    pub safety_analysis: SafetyAnalysis,
    /// Model checking warnings
    pub warnings: Vec<AispWarning>,
    /// Performance metrics
    pub performance_metrics: ModelCheckerMetrics,
}

/// Property verification result
#[derive(Debug, Clone)]
pub struct PropertyVerification {
    /// Property identifier
    pub property_id: String,
    /// Property description
    pub description: String,
    /// Property type
    pub property_type: PropertyType,
    /// Verification status
    pub status: VerificationStatus,
    /// Witness trace (if property holds)
    pub witness: Option<ExecutionTrace>,
    /// Counterexample (if property violated)
    pub counterexample: Option<CounterexampleTrace>,
    /// Verification time
    pub verification_time_ms: u64,
}

/// Property types for verification
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyType {
    /// Safety property (something bad never happens)
    Safety,
    /// Liveness property (something good eventually happens)
    Liveness,
    /// Fairness property (fair scheduling/resource access)
    Fairness,
    /// Reachability property (state/condition is reachable)
    Reachability,
    /// Deadlock freedom
    DeadlockFreedom,
    /// Response property (request followed by response)
    Response,
}

/// Verification status
#[derive(Debug, Clone, PartialEq)]
pub enum VerificationStatus {
    /// Property is verified (holds)
    Verified,
    /// Property is violated
    Violated,
    /// Verification inconclusive
    Inconclusive,
    /// Verification timed out
    Timeout,
    /// Verification error
    Error(String),
}

/// State space analysis results
#[derive(Debug, Clone)]
pub struct StateSpaceAnalysis {
    /// Total number of states
    pub total_states: usize,
    /// Number of initial states
    pub initial_states: usize,
    /// Number of final states
    pub final_states: usize,
    /// Number of transitions
    pub total_transitions: usize,
    /// Average branching factor
    pub avg_branching_factor: f64,
    /// Maximum path length
    pub max_path_length: usize,
    /// Strongly connected components
    pub strongly_connected_components: Vec<Vec<String>>,
}

/// Reachability analysis results
#[derive(Debug, Clone)]
pub struct ReachabilityAnalysis {
    /// States reachable from initial states
    pub reachable_states: HashSet<String>,
    /// Unreachable states
    pub unreachable_states: HashSet<String>,
    /// Reachability graph
    pub reachability_graph: HashMap<String, HashSet<String>>,
    /// Shortest paths between states
    pub shortest_paths: HashMap<(String, String), Vec<String>>,
    /// Dead states (no outgoing transitions)
    pub dead_states: HashSet<String>,
}

/// Liveness analysis results
#[derive(Debug, Clone)]
pub struct LivenessAnalysis {
    /// Liveness properties verified
    pub verified_properties: Vec<LivenessProperty>,
    /// Fair execution paths
    pub fair_paths: Vec<ExecutionPath>,
    /// Progress measures
    pub progress_measures: Vec<ProgressMeasure>,
    /// Fairness constraints
    pub fairness_constraints: Vec<FairnessConstraint>,
}

/// Safety analysis results
#[derive(Debug, Clone)]
pub struct SafetyAnalysis {
    /// Safety properties verified
    pub verified_properties: Vec<SafetyProperty>,
    /// Invariant checking results
    pub invariants: Vec<InvariantCheck>,
    /// Bad states detected
    pub bad_states: HashSet<String>,
    /// Safety violations found
    pub violations: Vec<SafetyViolation>,
}

/// Counterexample trace
#[derive(Debug, Clone)]
pub struct CounterexampleTrace {
    /// Property that was violated
    pub violated_property: String,
    /// Execution trace showing violation
    pub execution_trace: ExecutionTrace,
    /// Step where violation occurs
    pub violation_step: usize,
    /// Explanation of violation
    pub explanation: String,
    /// Whether trace is minimal
    pub is_minimal: bool,
}

/// Execution path in model
#[derive(Debug, Clone)]
pub struct ExecutionPath {
    /// Path identifier
    pub id: String,
    /// Sequence of states
    pub states: Vec<String>,
    /// Path length
    pub length: usize,
    /// Whether path is infinite (has cycle)
    pub is_infinite: bool,
    /// Properties satisfied along path
    pub satisfied_properties: Vec<String>,
}

/// Liveness property
#[derive(Debug, Clone)]
pub struct LivenessProperty {
    /// Property name
    pub name: String,
    /// Property formula
    pub formula: String,
    /// Whether property holds
    pub holds: bool,
    /// Witness path (if property holds)
    pub witness_path: Option<ExecutionPath>,
}

/// Safety property
#[derive(Debug, Clone)]
pub struct SafetyProperty {
    /// Property name
    pub name: String,
    /// Property formula
    pub formula: String,
    /// Whether property holds
    pub holds: bool,
    /// Violation trace (if property violated)
    pub violation_trace: Option<CounterexampleTrace>,
}

/// Progress measure
#[derive(Debug, Clone)]
pub struct ProgressMeasure {
    /// Measure name
    pub name: String,
    /// Progress function
    pub function: String,
    /// Current progress value
    pub value: f64,
    /// Progress trend
    pub trend: ProgressTrend,
}

/// Progress trend types
#[derive(Debug, Clone, PartialEq)]
pub enum ProgressTrend {
    /// Progress is increasing
    Increasing,
    /// Progress is decreasing
    Decreasing,
    /// Progress is stable
    Stable,
    /// Progress is oscillating
    Oscillating,
}

/// Fairness constraint
#[derive(Debug, Clone)]
pub struct FairnessConstraint {
    /// Constraint identifier
    pub id: String,
    /// Constraint description
    pub description: String,
    /// Constraint formula
    pub formula: String,
    /// Whether constraint is satisfied
    pub satisfied: bool,
}

/// Invariant checking result
#[derive(Debug, Clone)]
pub struct InvariantCheck {
    /// Invariant identifier
    pub id: String,
    /// Invariant formula
    pub formula: String,
    /// Whether invariant holds
    pub holds: bool,
    /// States where invariant is violated
    pub violation_states: Vec<String>,
}

/// Safety violation
#[derive(Debug, Clone)]
pub struct SafetyViolation {
    /// Violation identifier
    pub id: String,
    /// Violated safety property
    pub property: String,
    /// State where violation occurs
    pub violation_state: String,
    /// Trace leading to violation
    pub trace: ExecutionTrace,
}

/// Model checker performance metrics
#[derive(Debug, Clone)]
pub struct ModelCheckerMetrics {
    /// Total verification time
    pub total_time_ms: u64,
    /// State space construction time
    pub construction_time_ms: u64,
    /// Property verification time
    pub verification_time_ms: u64,
    /// Peak memory usage
    pub peak_memory_bytes: usize,
    /// Number of states explored
    pub states_explored: usize,
    /// Number of transitions evaluated
    pub transitions_evaluated: usize,
}

impl TemporalModelChecker {
    /// Create new temporal model checker
    pub fn new() -> Self {
        Self {
            state_spaces: HashMap::new(),
            config: ModelCheckerConfig::default(),
            results_cache: HashMap::new(),
        }
    }

    /// Create model checker with configuration
    pub fn with_config(config: ModelCheckerConfig) -> Self {
        Self {
            state_spaces: HashMap::new(),
            config,
            results_cache: HashMap::new(),
        }
    }

    /// Perform model checking on AISP document
    pub fn check_model(&mut self, document: &AispDocument) -> ModelCheckingResult {
        let start_time = std::time::Instant::now();
        let construction_start = start_time;

        // Build state space from document
        let state_space = self.build_state_space_from_document(document);
        let construction_time = construction_start.elapsed().as_millis() as u64;

        // Analyze state space
        let state_space_analysis = self.analyze_state_space(&state_space);

        // Perform reachability analysis
        let reachability_analysis = self.perform_reachability_analysis(&state_space);

        // Verify properties
        let verification_start = std::time::Instant::now();
        let verified_properties = self.verify_properties(document, &state_space);
        let verification_time = verification_start.elapsed().as_millis() as u64;

        // Perform liveness analysis
        let liveness_analysis = self.perform_liveness_analysis(&state_space);

        // Perform safety analysis
        let safety_analysis = self.perform_safety_analysis(&state_space);

        // Collect warnings
        let warnings = self.generate_warnings(&state_space_analysis, &reachability_analysis);

        let total_time = start_time.elapsed().as_millis() as u64;

        let performance_metrics = ModelCheckerMetrics {
            total_time_ms: total_time,
            construction_time_ms: construction_time,
            verification_time_ms: verification_time,
            peak_memory_bytes: self.estimate_memory_usage(),
            states_explored: state_space.states.len(),
            transitions_evaluated: state_space.transitions.len(),
        };

        ModelCheckingResult {
            verified_properties,
            state_space_analysis,
            reachability_analysis,
            liveness_analysis,
            safety_analysis,
            warnings,
            performance_metrics,
        }
    }

    /// Build state space from AISP document
    fn build_state_space_from_document(&mut self, document: &AispDocument) -> StateSpace {
        let mut states = HashMap::new();
        let mut transitions = Vec::new();
        let mut initial_states = HashSet::new();
        let final_states = HashSet::new();

        // Create initial state
        let initial_state = StateNode {
            id: "init".to_string(),
            valuations: HashMap::new(),
            atomic_props: HashSet::new(),
            is_initial: true,
            is_final: false,
        };
        states.insert("init".to_string(), initial_state);
        initial_states.insert("init".to_string());

        // Extract states from rules and functions
        for block in &document.blocks {
            match block {
                AispBlock::Rules(rules) => {
                    self.extract_states_from_rules(rules, &mut states, &mut transitions);
                }
                AispBlock::Functions(functions) => {
                    self.extract_states_from_functions(functions, &mut states, &mut transitions);
                }
                _ => {}
            }
        }

        // If no states were extracted, create a minimal model
        if states.len() == 1 {
            let final_state = StateNode {
                id: "final".to_string(),
                valuations: HashMap::new(),
                atomic_props: HashSet::new(),
                is_initial: false,
                is_final: true,
            };
            states.insert("final".to_string(), final_state);
            
            transitions.push(StateTransition {
                from: "init".to_string(),
                to: "final".to_string(),
                condition: None,
                probability: Some(1.0),
            });
        }

        StateSpace {
            states,
            initial_states,
            final_states,
            transitions,
        }
    }

    /// Extract states from rules block
    fn extract_states_from_rules(
        &self,
        _rules: &RulesBlock,
        states: &mut HashMap<String, StateNode>,
        transitions: &mut Vec<StateTransition>,
    ) {
        // TODO: Implement state extraction from logical rules
        // For now, create a simple linear progression
        let state_count = 3;
        for i in 1..=state_count {
            let state_id = format!("s{}", i);
            let state = StateNode {
                id: state_id.clone(),
                valuations: HashMap::new(),
                atomic_props: HashSet::new(),
                is_initial: i == 1,
                is_final: i == state_count,
            };
            states.insert(state_id.clone(), state);

            if i > 1 {
                transitions.push(StateTransition {
                    from: format!("s{}", i - 1),
                    to: state_id,
                    condition: None,
                    probability: Some(1.0),
                });
            }
        }
    }

    /// Extract states from functions block
    fn extract_states_from_functions(
        &self,
        _functions: &FunctionsBlock,
        _states: &mut HashMap<String, StateNode>,
        _transitions: &mut Vec<StateTransition>,
    ) {
        // TODO: Implement state extraction from function definitions
    }

    /// Analyze state space characteristics
    fn analyze_state_space(&self, state_space: &StateSpace) -> StateSpaceAnalysis {
        let total_states = state_space.states.len();
        let initial_states = state_space.initial_states.len();
        let final_states = state_space.final_states.len();
        let total_transitions = state_space.transitions.len();

        // Calculate average branching factor
        let mut total_outgoing = 0;
        let mut states_with_outgoing = 0;
        for state in state_space.states.keys() {
            let outgoing_count = state_space.transitions.iter()
                .filter(|t| t.from == *state)
                .count();
            if outgoing_count > 0 {
                total_outgoing += outgoing_count;
                states_with_outgoing += 1;
            }
        }

        let avg_branching_factor = if states_with_outgoing > 0 {
            total_outgoing as f64 / states_with_outgoing as f64
        } else {
            0.0
        };

        // Find strongly connected components (simplified)
        let strongly_connected_components = self.find_sccs(state_space);

        // Calculate maximum path length (simplified BFS)
        let max_path_length = self.calculate_max_path_length(state_space);

        StateSpaceAnalysis {
            total_states,
            initial_states,
            final_states,
            total_transitions,
            avg_branching_factor,
            max_path_length,
            strongly_connected_components,
        }
    }

    /// Find strongly connected components
    fn find_sccs(&self, state_space: &StateSpace) -> Vec<Vec<String>> {
        // TODO: Implement proper SCC algorithm (Tarjan's or Kosaraju's)
        // For now, return each state as its own SCC
        state_space.states.keys().map(|s| vec![s.clone()]).collect()
    }

    /// Calculate maximum path length
    fn calculate_max_path_length(&self, state_space: &StateSpace) -> usize {
        // Simplified BFS to find longest path
        let mut max_length = 0;
        
        for initial_state in &state_space.initial_states {
            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();
            queue.push_back((initial_state.clone(), 0));

            while let Some((state, depth)) = queue.pop_front() {
                if visited.contains(&state) {
                    continue;
                }
                visited.insert(state.clone());
                max_length = max_length.max(depth);

                for transition in &state_space.transitions {
                    if transition.from == state && !visited.contains(&transition.to) {
                        queue.push_back((transition.to.clone(), depth + 1));
                    }
                }
            }
        }

        max_length
    }

    /// Perform reachability analysis
    fn perform_reachability_analysis(&self, state_space: &StateSpace) -> ReachabilityAnalysis {
        let mut reachable_states = HashSet::new();
        let mut reachability_graph = HashMap::new();

        // BFS from initial states to find reachable states
        let mut queue = VecDeque::new();
        for initial_state in &state_space.initial_states {
            queue.push_back(initial_state.clone());
            reachable_states.insert(initial_state.clone());
        }

        while let Some(state) = queue.pop_front() {
            let mut reachable_from_state = HashSet::new();
            
            for transition in &state_space.transitions {
                if transition.from == state {
                    reachable_from_state.insert(transition.to.clone());
                    if !reachable_states.contains(&transition.to) {
                        reachable_states.insert(transition.to.clone());
                        queue.push_back(transition.to.clone());
                    }
                }
            }
            
            reachability_graph.insert(state, reachable_from_state);
        }

        let unreachable_states = state_space.states.keys()
            .filter(|s| !reachable_states.contains(*s))
            .cloned()
            .collect();

        // Find dead states
        let dead_states = state_space.states.keys()
            .filter(|state| {
                !state_space.transitions.iter().any(|t| t.from == **state)
            })
            .cloned()
            .collect();

        ReachabilityAnalysis {
            reachable_states,
            unreachable_states,
            reachability_graph,
            shortest_paths: HashMap::new(), // TODO: Implement shortest path calculation
            dead_states,
        }
    }

    /// Verify properties in the document
    fn verify_properties(&mut self, _document: &AispDocument, state_space: &StateSpace) -> Vec<PropertyVerification> {
        let mut verifications = Vec::new();

        // Verify basic properties
        verifications.push(self.verify_deadlock_freedom(state_space));
        verifications.push(self.verify_reachability(state_space));

        verifications
    }

    /// Verify deadlock freedom property
    fn verify_deadlock_freedom(&self, state_space: &StateSpace) -> PropertyVerification {
        let start_time = std::time::Instant::now();
        
        // Check if any non-final state has no outgoing transitions
        let has_deadlock = state_space.states.iter().any(|(state_id, state)| {
            !state.is_final && 
            !state_space.transitions.iter().any(|t| t.from == *state_id)
        });

        let status = if has_deadlock {
            VerificationStatus::Violated
        } else {
            VerificationStatus::Verified
        };

        let verification_time = start_time.elapsed().as_millis() as u64;

        PropertyVerification {
            property_id: "deadlock_freedom".to_string(),
            description: "System is free from deadlocks".to_string(),
            property_type: PropertyType::DeadlockFreedom,
            status,
            witness: None,
            counterexample: None,
            verification_time_ms: verification_time,
        }
    }

    /// Verify reachability property
    fn verify_reachability(&self, state_space: &StateSpace) -> PropertyVerification {
        let start_time = std::time::Instant::now();
        
        // Check if final states are reachable from initial states
        let final_states_reachable = !state_space.final_states.is_empty() && 
            self.are_final_states_reachable(state_space);

        let status = if final_states_reachable {
            VerificationStatus::Verified
        } else {
            VerificationStatus::Violated
        };

        let verification_time = start_time.elapsed().as_millis() as u64;

        PropertyVerification {
            property_id: "final_state_reachability".to_string(),
            description: "Final states are reachable from initial states".to_string(),
            property_type: PropertyType::Reachability,
            status,
            witness: None,
            counterexample: None,
            verification_time_ms: verification_time,
        }
    }

    /// Check if final states are reachable
    fn are_final_states_reachable(&self, state_space: &StateSpace) -> bool {
        // BFS to check reachability
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        for initial_state in &state_space.initial_states {
            queue.push_back(initial_state.clone());
        }

        while let Some(state) = queue.pop_front() {
            if visited.contains(&state) {
                continue;
            }
            visited.insert(state.clone());

            if state_space.final_states.contains(&state) {
                return true;
            }

            for transition in &state_space.transitions {
                if transition.from == state && !visited.contains(&transition.to) {
                    queue.push_back(transition.to.clone());
                }
            }
        }

        false
    }

    /// Perform liveness analysis
    fn perform_liveness_analysis(&self, _state_space: &StateSpace) -> LivenessAnalysis {
        // TODO: Implement liveness analysis
        LivenessAnalysis {
            verified_properties: Vec::new(),
            fair_paths: Vec::new(),
            progress_measures: Vec::new(),
            fairness_constraints: Vec::new(),
        }
    }

    /// Perform safety analysis
    fn perform_safety_analysis(&self, _state_space: &StateSpace) -> SafetyAnalysis {
        // TODO: Implement safety analysis
        SafetyAnalysis {
            verified_properties: Vec::new(),
            invariants: Vec::new(),
            bad_states: HashSet::new(),
            violations: Vec::new(),
        }
    }

    /// Generate warnings based on analysis results
    fn generate_warnings(
        &self,
        state_space_analysis: &StateSpaceAnalysis,
        reachability_analysis: &ReachabilityAnalysis,
    ) -> Vec<AispWarning> {
        let mut warnings = Vec::new();

        // Warn about unreachable states
        if !reachability_analysis.unreachable_states.is_empty() {
            warnings.push(AispWarning::warning(format!(
                "{} unreachable states detected: {}",
                reachability_analysis.unreachable_states.len(),
                reachability_analysis.unreachable_states.iter().take(3).cloned().collect::<Vec<_>>().join(", ")
            )));
        }

        // Warn about dead states
        if !reachability_analysis.dead_states.is_empty() {
            warnings.push(AispWarning::warning(format!(
                "{} dead states detected (no outgoing transitions)",
                reachability_analysis.dead_states.len()
            )));
        }

        // Warn about high branching factor
        if state_space_analysis.avg_branching_factor > 5.0 {
            warnings.push(AispWarning::warning(format!(
                "High average branching factor: {:.2}",
                state_space_analysis.avg_branching_factor
            )));
        }

        // Warn about large state space
        if state_space_analysis.total_states > self.config.max_states / 2 {
            warnings.push(AispWarning::warning(format!(
                "Large state space: {} states (approaching limit of {})",
                state_space_analysis.total_states,
                self.config.max_states
            )));
        }

        warnings
    }

    /// Estimate memory usage
    fn estimate_memory_usage(&self) -> usize {
        // Rough estimation
        let state_spaces_size = self.state_spaces.len() * 4096;
        let results_cache_size = self.results_cache.len() * 1024;
        state_spaces_size + results_cache_size + 8192 // Base overhead
    }
}

impl Default for ModelCheckerConfig {
    fn default() -> Self {
        Self {
            max_states: 10000,
            max_depth: 1000,
            enable_reduction: true,
            use_symbolic: false,
            timeout_ms: 30000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_checker_creation() {
        let checker = TemporalModelChecker::new();
        assert!(checker.state_spaces.is_empty());
        assert!(checker.results_cache.is_empty());
    }

    #[test]
    fn test_config_defaults() {
        let config = ModelCheckerConfig::default();
        assert_eq!(config.max_states, 10000);
        assert_eq!(config.max_depth, 1000);
        assert!(config.enable_reduction);
        assert!(!config.use_symbolic);
    }

    #[test]
    fn test_deadlock_freedom_verification() {
        let checker = TemporalModelChecker::new();
        
        // Create simple state space without deadlocks
        let mut states = HashMap::new();
        states.insert("s1".to_string(), StateNode {
            id: "s1".to_string(),
            valuations: HashMap::new(),
            atomic_props: HashSet::new(),
            is_initial: true,
            is_final: false,
        });
        states.insert("s2".to_string(), StateNode {
            id: "s2".to_string(),
            valuations: HashMap::new(),
            atomic_props: HashSet::new(),
            is_initial: false,
            is_final: true,
        });

        let state_space = StateSpace {
            states,
            initial_states: HashSet::from_iter(vec!["s1".to_string()]),
            final_states: HashSet::from_iter(vec!["s2".to_string()]),
            transitions: vec![StateTransition {
                from: "s1".to_string(),
                to: "s2".to_string(),
                condition: None,
                probability: Some(1.0),
            }],
        };

        let result = checker.verify_deadlock_freedom(&state_space);
        assert_eq!(result.status, VerificationStatus::Verified);
    }

    #[test]
    fn test_max_path_length_calculation() {
        let checker = TemporalModelChecker::new();
        
        let mut states = HashMap::new();
        states.insert("s1".to_string(), StateNode {
            id: "s1".to_string(),
            valuations: HashMap::new(),
            atomic_props: HashSet::new(),
            is_initial: true,
            is_final: false,
        });
        states.insert("s2".to_string(), StateNode {
            id: "s2".to_string(),
            valuations: HashMap::new(),
            atomic_props: HashSet::new(),
            is_initial: false,
            is_final: false,
        });
        states.insert("s3".to_string(), StateNode {
            id: "s3".to_string(),
            valuations: HashMap::new(),
            atomic_props: HashSet::new(),
            is_initial: false,
            is_final: true,
        });

        let state_space = StateSpace {
            states,
            initial_states: HashSet::from_iter(vec!["s1".to_string()]),
            final_states: HashSet::from_iter(vec!["s3".to_string()]),
            transitions: vec![
                StateTransition {
                    from: "s1".to_string(),
                    to: "s2".to_string(),
                    condition: None,
                    probability: Some(1.0),
                },
                StateTransition {
                    from: "s2".to_string(),
                    to: "s3".to_string(),
                    condition: None,
                    probability: Some(1.0),
                },
            ],
        };

        let max_length = checker.calculate_max_path_length(&state_space);
        assert_eq!(max_length, 2); // s1 -> s2 -> s3 (3 states, 2 transitions)
    }
}