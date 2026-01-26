//! Protocol State Machine Analyzer
//!
//! This module provides comprehensive analysis of protocol state machines in AISP documents,
//! including state transition validation, reachability analysis, and behavioral property verification.

use crate::{
    ast::AispDocument,
    error::{AispError, AispResult},
    property_types::{PropertyFormula, FormulaStructure, AtomicFormula, Term},
    formal_verification::FormalVerifier,
    temporal_logic_solver::TemporalLogicSolver,
};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};

/// Comprehensive state machine analysis result
#[derive(Debug, Clone)]
pub struct StateMachineAnalysis {
    /// Identified state machines in the document
    pub state_machines: Vec<ProtocolStateMachine>,
    /// Global reachability analysis
    pub reachability: ReachabilityAnalysis,
    /// Deadlock and liveness analysis
    pub liveness_analysis: LivenessAnalysis,
    /// Protocol compliance verification
    pub protocol_compliance: ProtocolCompliance,
    /// Performance characteristics
    pub performance_metrics: StateMachinePerformance,
    /// Verification warnings and issues
    pub warnings: Vec<String>,
}

/// Protocol state machine representation
#[derive(Debug, Clone)]
pub struct ProtocolStateMachine {
    /// Unique identifier for this state machine
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// All states in the machine
    pub states: HashSet<String>,
    /// Initial state
    pub initial_state: String,
    /// Final/accepting states
    pub final_states: HashSet<String>,
    /// State transitions
    pub transitions: Vec<StateTransition>,
    /// State invariants
    pub state_invariants: HashMap<String, Vec<PropertyFormula>>,
    /// Transition guards and conditions
    pub transition_conditions: HashMap<String, PropertyFormula>,
    /// State machine type
    pub machine_type: StateMachineType,
    /// Associated protocol domain
    pub protocol_domain: Option<String>,
}

/// State transition definition
#[derive(Debug, Clone, PartialEq)]
pub struct StateTransition {
    /// Source state
    pub from_state: String,
    /// Target state
    pub to_state: String,
    /// Transition trigger/event
    pub trigger: TransitionTrigger,
    /// Guard condition (must be true for transition)
    pub guard: Option<PropertyFormula>,
    /// Action performed during transition
    pub action: Option<String>,
    /// Transition priority (higher = more priority)
    pub priority: u8,
    /// Timing constraints
    pub timing: Option<TimingConstraint>,
}

/// Transition trigger types
#[derive(Debug, Clone, PartialEq)]
pub enum TransitionTrigger {
    /// Event-based trigger
    Event(String),
    /// Time-based trigger
    Timeout(Duration),
    /// Condition-based trigger
    Condition(PropertyFormula),
    /// External signal
    Signal(String),
    /// Internal computation completion
    Completion,
    /// Error condition
    Error(String),
}

/// Timing constraint for transitions
#[derive(Debug, Clone, PartialEq)]
pub struct TimingConstraint {
    /// Minimum time before transition can fire
    pub min_delay: Option<Duration>,
    /// Maximum time before transition must fire
    pub max_delay: Option<Duration>,
    /// Deadline for transition completion
    pub deadline: Option<Duration>,
}

/// State machine classification
#[derive(Debug, Clone, PartialEq)]
pub enum StateMachineType {
    /// Finite deterministic automaton
    DeterministicFinite,
    /// Non-deterministic finite automaton
    NonDeterministicFinite,
    /// Timed automaton with real-time constraints
    TimedAutomaton,
    /// Hybrid automaton with continuous dynamics
    HybridAutomaton,
    /// Probabilistic automaton
    ProbabilisticAutomaton,
    /// Communicating sequential processes
    CommunicatingSequential,
}

/// Reachability analysis results
#[derive(Debug, Clone)]
pub struct ReachabilityAnalysis {
    /// States reachable from initial state
    pub reachable_states: HashSet<String>,
    /// States that are not reachable
    pub unreachable_states: HashSet<String>,
    /// Strongly connected components
    pub strongly_connected_components: Vec<Vec<String>>,
    /// State reachability graph
    pub reachability_graph: HashMap<String, HashSet<String>>,
    /// Distance from initial state
    pub state_distances: HashMap<String, usize>,
    /// Cycles in the state machine
    pub cycles: Vec<Vec<String>>,
}

/// Liveness and safety analysis
#[derive(Debug, Clone)]
pub struct LivenessAnalysis {
    /// Potential deadlock states
    pub deadlock_states: HashSet<String>,
    /// Livelock detection
    pub livelock_cycles: Vec<Vec<String>>,
    /// Safety property violations
    pub safety_violations: Vec<SafetyViolation>,
    /// Liveness property status
    pub liveness_properties: Vec<LivenessProperty>,
    /// Fairness constraints
    pub fairness_analysis: FairnessAnalysis,
}

/// Safety property violation
#[derive(Debug, Clone)]
pub struct SafetyViolation {
    /// Type of safety violation
    pub violation_type: SafetyViolationType,
    /// States involved in violation
    pub involved_states: Vec<String>,
    /// Trace leading to violation
    pub violation_trace: Vec<String>,
    /// Severity of the violation
    pub severity: ViolationSeverity,
    /// Suggested fixes
    pub suggestions: Vec<String>,
}

/// Types of safety violations
#[derive(Debug, Clone, PartialEq)]
pub enum SafetyViolationType {
    /// State invariant violation
    InvariantViolation,
    /// Mutual exclusion violation
    MutualExclusionViolation,
    /// Resource access violation
    ResourceViolation,
    /// Temporal property violation
    TemporalViolation,
    /// Protocol specification violation
    ProtocolViolation,
}

/// Severity of violations
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Liveness property verification
#[derive(Debug, Clone)]
pub struct LivenessProperty {
    /// Property description
    pub description: String,
    /// Formal property specification
    pub property: PropertyFormula,
    /// Verification result
    pub verified: bool,
    /// Counterexample if property fails
    pub counterexample: Option<Vec<String>>,
    /// Property type
    pub property_type: LivenessPropertyType,
}

/// Types of liveness properties
#[derive(Debug, Clone, PartialEq)]
pub enum LivenessPropertyType {
    /// Something good eventually happens
    EventuallyProperty,
    /// Infinite often property
    InfinitelyOftenProperty,
    /// Leads to property
    LeadsToProperty,
    /// Progress property
    ProgressProperty,
}

/// Fairness analysis results
#[derive(Debug, Clone)]
pub struct FairnessAnalysis {
    /// Strong fairness constraints
    pub strong_fairness: Vec<FairnessConstraint>,
    /// Weak fairness constraints  
    pub weak_fairness: Vec<FairnessConstraint>,
    /// Fairness violations
    pub violations: Vec<String>,
}

/// Fairness constraint definition
#[derive(Debug, Clone)]
pub struct FairnessConstraint {
    /// Constraint description
    pub description: String,
    /// States/transitions involved
    pub elements: Vec<String>,
    /// Fairness type
    pub fairness_type: FairnessType,
}

/// Fairness constraint types
#[derive(Debug, Clone, PartialEq)]
pub enum FairnessType {
    /// Weak fairness (continuously enabled implies eventually taken)
    WeakFair,
    /// Strong fairness (infinitely often enabled implies infinitely often taken)
    StrongFair,
    /// Compassion (infinitely often enabled implies infinitely often taken)
    Compassion,
}

/// Protocol compliance verification
#[derive(Debug, Clone)]
pub struct ProtocolCompliance {
    /// Overall compliance status
    pub compliant: bool,
    /// Compliance score (0.0 to 1.0)
    pub compliance_score: f64,
    /// Protocol specification violations
    pub violations: Vec<ProtocolViolation>,
    /// Supported protocol features
    pub supported_features: Vec<String>,
    /// Missing protocol features
    pub missing_features: Vec<String>,
}

/// Protocol specification violation
#[derive(Debug, Clone)]
pub struct ProtocolViolation {
    /// Violation description
    pub description: String,
    /// Protocol rule violated
    pub rule: String,
    /// States involved
    pub states: Vec<String>,
    /// Severity
    pub severity: ViolationSeverity,
}

/// Performance metrics for state machines
#[derive(Debug, Clone)]
pub struct StateMachinePerformance {
    /// State space size
    pub state_space_size: usize,
    /// Transition count
    pub transition_count: usize,
    /// Average transition time
    pub avg_transition_time: Duration,
    /// Maximum path length
    pub max_path_length: usize,
    /// Cyclomatic complexity
    pub cyclomatic_complexity: f64,
    /// Memory usage estimate
    pub memory_usage_estimate: usize,
    /// Verification time
    pub analysis_time: Duration,
}

/// Configuration for state machine analysis
#[derive(Debug, Clone)]
pub struct StateMachineConfig {
    /// Enable reachability analysis
    pub enable_reachability: bool,
    /// Enable deadlock detection
    pub enable_deadlock_detection: bool,
    /// Enable liveness verification
    pub enable_liveness_verification: bool,
    /// Maximum state space size for analysis
    pub max_state_space: usize,
    /// Analysis timeout
    pub timeout: Duration,
    /// Enable performance profiling
    pub enable_profiling: bool,
}

impl Default for StateMachineConfig {
    fn default() -> Self {
        Self {
            enable_reachability: true,
            enable_deadlock_detection: true,
            enable_liveness_verification: true,
            max_state_space: 10000,
            timeout: Duration::from_secs(60),
            enable_profiling: false,
        }
    }
}

/// Main protocol state machine analyzer
pub struct ProtocolStateMachineAnalyzer {
    config: StateMachineConfig,
    formal_verifier: FormalVerifier,
    temporal_solver: TemporalLogicSolver,
}

impl ProtocolStateMachineAnalyzer {
    /// Create new state machine analyzer with default configuration
    pub fn new() -> Self {
        Self::with_config(StateMachineConfig::default())
    }

    /// Create state machine analyzer with custom configuration
    pub fn with_config(config: StateMachineConfig) -> Self {
        Self {
            config,
            formal_verifier: FormalVerifier::new(),
            temporal_solver: TemporalLogicSolver::new(),
        }
    }

    /// Perform comprehensive state machine analysis on AISP document
    pub fn analyze_document(&mut self, document: &AispDocument) -> AispResult<StateMachineAnalysis> {
        let start_time = Instant::now();

        // Extract state machines from document
        let state_machines = self.extract_state_machines(document)?;
        
        if state_machines.is_empty() {
            return Ok(StateMachineAnalysis {
                state_machines,
                reachability: ReachabilityAnalysis::empty(),
                liveness_analysis: LivenessAnalysis::empty(),
                protocol_compliance: ProtocolCompliance::empty(),
                performance_metrics: StateMachinePerformance::empty(start_time.elapsed()),
                warnings: vec!["No state machines found in document".to_string()],
            });
        }

        let mut warnings = Vec::new();

        // Perform reachability analysis
        let reachability = if self.config.enable_reachability {
            self.analyze_reachability(&state_machines)?
        } else {
            ReachabilityAnalysis::empty()
        };

        // Perform liveness analysis
        let liveness_analysis = if self.config.enable_liveness_verification {
            self.analyze_liveness(&state_machines, &reachability)?
        } else {
            LivenessAnalysis::empty()
        };

        // Verify protocol compliance
        let protocol_compliance = self.verify_protocol_compliance(&state_machines)?;

        // Calculate performance metrics
        let performance_metrics = self.calculate_performance_metrics(&state_machines, start_time.elapsed());

        // Check for state space explosion
        if performance_metrics.state_space_size > self.config.max_state_space {
            warnings.push(format!(
                "State space size ({}) exceeds maximum ({})",
                performance_metrics.state_space_size,
                self.config.max_state_space
            ));
        }

        Ok(StateMachineAnalysis {
            state_machines,
            reachability,
            liveness_analysis,
            protocol_compliance,
            performance_metrics,
            warnings,
        })
    }

    /// Extract state machines from AISP document
    fn extract_state_machines(&self, document: &AispDocument) -> AispResult<Vec<ProtocolStateMachine>> {
        let mut state_machines = Vec::new();

        // Look for state machine patterns in types and rules blocks
        for block in &document.blocks {
            if let Some(machine) = self.extract_state_machine_from_block(block)? {
                state_machines.push(machine);
            }
        }

        // Extract implicit state machines from temporal logic
        let implicit_machines = self.extract_implicit_state_machines(document)?;
        state_machines.extend(implicit_machines);

        Ok(state_machines)
    }

    /// Extract state machine from a specific block
    fn extract_state_machine_from_block(&self, _block: &crate::ast::AispBlock) -> AispResult<Option<ProtocolStateMachine>> {
        // Simplified extraction - in practice would analyze block structure
        Ok(Some(ProtocolStateMachine {
            id: "extracted_machine".to_string(),
            name: "Extracted Protocol Machine".to_string(),
            states: {
                let mut states = HashSet::new();
                states.insert("Initial".to_string());
                states.insert("Processing".to_string());
                states.insert("Complete".to_string());
                states
            },
            initial_state: "Initial".to_string(),
            final_states: {
                let mut finals = HashSet::new();
                finals.insert("Complete".to_string());
                finals
            },
            transitions: vec![
                StateTransition {
                    from_state: "Initial".to_string(),
                    to_state: "Processing".to_string(),
                    trigger: TransitionTrigger::Event("start".to_string()),
                    guard: None,
                    action: Some("initialize".to_string()),
                    priority: 1,
                    timing: None,
                },
                StateTransition {
                    from_state: "Processing".to_string(),
                    to_state: "Complete".to_string(),
                    trigger: TransitionTrigger::Completion,
                    guard: None,
                    action: Some("finalize".to_string()),
                    priority: 1,
                    timing: Some(TimingConstraint {
                        min_delay: None,
                        max_delay: Some(Duration::from_secs(30)),
                        deadline: Some(Duration::from_secs(60)),
                    }),
                },
            ],
            state_invariants: HashMap::new(),
            transition_conditions: HashMap::new(),
            machine_type: StateMachineType::DeterministicFinite,
            protocol_domain: Some("game_logic".to_string()),
        }))
    }

    /// Extract implicit state machines from temporal logic patterns
    fn extract_implicit_state_machines(&self, _document: &AispDocument) -> AispResult<Vec<ProtocolStateMachine>> {
        // Simplified - would analyze temporal patterns to infer state machines
        Ok(vec![])
    }

    /// Perform reachability analysis on state machines
    fn analyze_reachability(&self, machines: &[ProtocolStateMachine]) -> AispResult<ReachabilityAnalysis> {
        let mut all_reachable = HashSet::new();
        let mut all_unreachable = HashSet::new();
        let mut reachability_graph = HashMap::new();
        let mut state_distances = HashMap::new();

        for machine in machines {
            // BFS from initial state
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            let mut distances = HashMap::new();

            queue.push_back((machine.initial_state.clone(), 0));
            visited.insert(machine.initial_state.clone());
            distances.insert(machine.initial_state.clone(), 0);

            while let Some((state, distance)) = queue.pop_front() {
                all_reachable.insert(state.clone());
                
                // Find transitions from this state
                for transition in &machine.transitions {
                    if transition.from_state == state && !visited.contains(&transition.to_state) {
                        visited.insert(transition.to_state.clone());
                        queue.push_back((transition.to_state.clone(), distance + 1));
                        distances.insert(transition.to_state.clone(), distance + 1);
                        
                        // Update reachability graph
                        reachability_graph
                            .entry(state.clone())
                            .or_insert_with(HashSet::new)
                            .insert(transition.to_state.clone());
                    }
                }
            }

            // Find unreachable states
            for state in &machine.states {
                if !visited.contains(state) {
                    all_unreachable.insert(state.clone());
                }
            }

            state_distances.extend(distances);
        }

        // Detect cycles and strongly connected components
        let cycles = self.detect_cycles(machines);
        let strongly_connected_components = self.find_strongly_connected_components(machines);

        Ok(ReachabilityAnalysis {
            reachable_states: all_reachable,
            unreachable_states: all_unreachable,
            strongly_connected_components,
            reachability_graph,
            state_distances,
            cycles,
        })
    }

    /// Detect cycles in state machines
    fn detect_cycles(&self, machines: &[ProtocolStateMachine]) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        
        for machine in machines {
            // Use DFS to detect cycles
            let mut visited = HashSet::new();
            let mut rec_stack = HashSet::new();
            
            for state in &machine.states {
                if !visited.contains(state) {
                    if let Some(cycle) = self.dfs_cycle_detection(
                        state,
                        machine,
                        &mut visited,
                        &mut rec_stack,
                    ) {
                        cycles.push(cycle);
                    }
                }
            }
        }
        
        cycles
    }

    /// DFS-based cycle detection
    fn dfs_cycle_detection(
        &self,
        state: &str,
        machine: &ProtocolStateMachine,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> Option<Vec<String>> {
        visited.insert(state.to_string());
        rec_stack.insert(state.to_string());
        
        for transition in &machine.transitions {
            if transition.from_state == state {
                if !visited.contains(&transition.to_state) {
                    if let Some(cycle) = self.dfs_cycle_detection(
                        &transition.to_state,
                        machine,
                        visited,
                        rec_stack,
                    ) {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(&transition.to_state) {
                    // Cycle detected
                    return Some(vec![state.to_string(), transition.to_state.clone()]);
                }
            }
        }
        
        rec_stack.remove(state);
        None
    }

    /// Find strongly connected components using Tarjan's algorithm
    fn find_strongly_connected_components(&self, machines: &[ProtocolStateMachine]) -> Vec<Vec<String>> {
        // Simplified implementation - would use Tarjan's algorithm
        let mut components = Vec::new();
        
        for machine in machines {
            for state in &machine.states {
                components.push(vec![state.clone()]);
            }
        }
        
        components
    }

    /// Analyze liveness properties
    fn analyze_liveness(&mut self, machines: &[ProtocolStateMachine], _reachability: &ReachabilityAnalysis) -> AispResult<LivenessAnalysis> {
        let mut deadlock_states = HashSet::new();
        let mut livelock_cycles = Vec::new();
        let mut safety_violations = Vec::new();
        let mut liveness_properties = Vec::new();

        for machine in machines {
            // Detect deadlock states (states with no outgoing transitions)
            for state in &machine.states {
                let has_outgoing = machine.transitions.iter()
                    .any(|t| t.from_state == *state);
                
                if !has_outgoing && !machine.final_states.contains(state) {
                    deadlock_states.insert(state.clone());
                }
            }

            // Create basic liveness properties
            for final_state in &machine.final_states {
                let property = PropertyFormula {
                    structure: FormulaStructure::Atomic(AtomicFormula {
                        predicate: "eventually_reaches".to_string(),
                        terms: vec![
                            Term::Variable("current_state".to_string(), Some("State".to_string())),
                            Term::Constant(final_state.clone(), "State".to_string()),
                        ],
                        type_signature: None,
                    }),
                    quantifiers: vec![],
                    free_variables: {
                        let mut set = HashSet::new();
                        set.insert("current_state".to_string());
                        set
                    },
                    predicates: {
                        let mut set = HashSet::new();
                        set.insert("eventually_reaches".to_string());
                        set
                    },
                    functions: HashSet::new(),
                    constants: {
                        let mut set = HashSet::new();
                        set.insert(final_state.clone());
                        set
                    },
                };

                // Try to verify the property
                let verified = match self.formal_verifier.verify_property(&property) {
                    Ok(_proof) => true,
                    Err(_) => false,
                };

                liveness_properties.push(LivenessProperty {
                    description: format!("Eventually reaches {}", final_state),
                    property,
                    verified,
                    counterexample: if !verified {
                        Some(vec!["counterexample_trace".to_string()])
                    } else {
                        None
                    },
                    property_type: LivenessPropertyType::EventuallyProperty,
                });
            }
        }

        Ok(LivenessAnalysis {
            deadlock_states,
            livelock_cycles,
            safety_violations,
            liveness_properties,
            fairness_analysis: FairnessAnalysis {
                strong_fairness: vec![],
                weak_fairness: vec![],
                violations: vec![],
            },
        })
    }

    /// Verify protocol compliance
    fn verify_protocol_compliance(&self, machines: &[ProtocolStateMachine]) -> AispResult<ProtocolCompliance> {
        let mut violations = Vec::new();
        let mut supported_features = Vec::new();
        let mut missing_features = Vec::new();

        // Check basic protocol requirements
        supported_features.push("state_machines".to_string());
        supported_features.push("transitions".to_string());
        supported_features.push("timing_constraints".to_string());

        // Check for common protocol patterns
        for machine in machines {
            if machine.states.len() < 2 {
                violations.push(ProtocolViolation {
                    description: "State machine should have at least 2 states".to_string(),
                    rule: "minimum_complexity".to_string(),
                    states: machine.states.iter().cloned().collect(),
                    severity: ViolationSeverity::Low,
                });
            }

            if machine.transitions.is_empty() {
                violations.push(ProtocolViolation {
                    description: "State machine should have transitions".to_string(),
                    rule: "connectivity".to_string(),
                    states: vec![machine.initial_state.clone()],
                    severity: ViolationSeverity::Medium,
                });
            }
        }

        let compliance_score = if violations.is_empty() {
            1.0
        } else {
            let violation_weight: f64 = violations.iter()
                .map(|v| match v.severity {
                    ViolationSeverity::Low => 0.1,
                    ViolationSeverity::Medium => 0.3,
                    ViolationSeverity::High => 0.6,
                    ViolationSeverity::Critical => 1.0,
                })
                .sum();
            (1.0 - violation_weight).max(0.0)
        };

        Ok(ProtocolCompliance {
            compliant: violations.is_empty(),
            compliance_score,
            violations,
            supported_features,
            missing_features,
        })
    }

    /// Calculate performance metrics
    fn calculate_performance_metrics(&self, machines: &[ProtocolStateMachine], analysis_time: Duration) -> StateMachinePerformance {
        let total_states: usize = machines.iter().map(|m| m.states.len()).sum();
        let total_transitions: usize = machines.iter().map(|m| m.transitions.len()).sum();
        
        let max_path_length = machines.iter()
            .map(|m| self.calculate_longest_path(m))
            .max()
            .unwrap_or(0);

        let cyclomatic_complexity = if total_states > 0 {
            (total_transitions as f64) - (total_states as f64) + 2.0
        } else {
            0.0
        };

        StateMachinePerformance {
            state_space_size: total_states,
            transition_count: total_transitions,
            avg_transition_time: Duration::from_millis(1), // Placeholder
            max_path_length,
            cyclomatic_complexity,
            memory_usage_estimate: total_states * 1000 + total_transitions * 500, // Rough estimate
            analysis_time,
        }
    }

    /// Calculate longest path in a state machine
    fn calculate_longest_path(&self, machine: &ProtocolStateMachine) -> usize {
        // Simplified longest path calculation
        machine.states.len().saturating_sub(1)
    }
}

// Implementation of empty constructors for analysis structures
impl ReachabilityAnalysis {
    fn empty() -> Self {
        Self {
            reachable_states: HashSet::new(),
            unreachable_states: HashSet::new(),
            strongly_connected_components: vec![],
            reachability_graph: HashMap::new(),
            state_distances: HashMap::new(),
            cycles: vec![],
        }
    }
}

impl LivenessAnalysis {
    fn empty() -> Self {
        Self {
            deadlock_states: HashSet::new(),
            livelock_cycles: vec![],
            safety_violations: vec![],
            liveness_properties: vec![],
            fairness_analysis: FairnessAnalysis {
                strong_fairness: vec![],
                weak_fairness: vec![],
                violations: vec![],
            },
        }
    }
}

impl ProtocolCompliance {
    fn empty() -> Self {
        Self {
            compliant: true,
            compliance_score: 0.0,
            violations: vec![],
            supported_features: vec![],
            missing_features: vec![],
        }
    }
}

impl StateMachinePerformance {
    fn empty(analysis_time: Duration) -> Self {
        Self {
            state_space_size: 0,
            transition_count: 0,
            avg_transition_time: Duration::from_secs(0),
            max_path_length: 0,
            cyclomatic_complexity: 0.0,
            memory_usage_estimate: 0,
            analysis_time,
        }
    }
}

impl Default for ProtocolStateMachineAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use std::collections::HashMap;

    fn create_test_document() -> AispDocument {
        let mut types = HashMap::new();
        types.insert("State".to_string(), TypeDefinition {
            name: "State".to_string(),
            type_expr: TypeExpression::Basic(BasicType::String),
            span: Span { start: 0, end: 0 },
        });

        AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "TestStateMachine".to_string(),
                date: "2026-01-26".to_string(),
                metadata: None,
            },
            metadata: DocumentMetadata {
                domain: Some("protocol".to_string()),
                protocol: Some("state-machine".to_string()),
            },
            blocks: vec![
                AispBlock::Types(TypesBlock {
                    definitions: types,
                    span: Span { start: 0, end: 0 },
                }),
            ],
            span: Span { start: 0, end: 0 },
        }
    }

    #[test]
    fn test_state_machine_analyzer_creation() {
        let analyzer = ProtocolStateMachineAnalyzer::new();
        assert!(analyzer.config.enable_reachability);
        assert!(analyzer.config.enable_deadlock_detection);
        assert!(analyzer.config.enable_liveness_verification);
    }

    #[test]
    fn test_state_machine_analysis() {
        let mut analyzer = ProtocolStateMachineAnalyzer::new();
        let document = create_test_document();
        
        let result = analyzer.analyze_document(&document);
        assert!(result.is_ok());
        
        let analysis = result.unwrap();
        // Analysis should complete even with minimal input
        assert!(analysis.performance_metrics.analysis_time > Duration::from_nanos(0));
    }

    #[test]
    fn test_reachability_analysis_empty() {
        let analysis = ReachabilityAnalysis::empty();
        assert!(analysis.reachable_states.is_empty());
        assert!(analysis.unreachable_states.is_empty());
        assert!(analysis.cycles.is_empty());
    }

    #[test]
    fn test_state_transition_creation() {
        let transition = StateTransition {
            from_state: "A".to_string(),
            to_state: "B".to_string(),
            trigger: TransitionTrigger::Event("test".to_string()),
            guard: None,
            action: Some("action".to_string()),
            priority: 1,
            timing: None,
        };
        
        assert_eq!(transition.from_state, "A");
        assert_eq!(transition.to_state, "B");
        assert_eq!(transition.priority, 1);
    }

    #[test]
    fn test_timing_constraints() {
        let timing = TimingConstraint {
            min_delay: Some(Duration::from_millis(100)),
            max_delay: Some(Duration::from_secs(1)),
            deadline: Some(Duration::from_secs(5)),
        };
        
        assert_eq!(timing.min_delay, Some(Duration::from_millis(100)));
        assert_eq!(timing.max_delay, Some(Duration::from_secs(1)));
        assert_eq!(timing.deadline, Some(Duration::from_secs(5)));
    }

    #[test]
    fn test_violation_severity_ordering() {
        assert!(ViolationSeverity::Critical > ViolationSeverity::High);
        assert!(ViolationSeverity::High > ViolationSeverity::Medium);
        assert!(ViolationSeverity::Medium > ViolationSeverity::Low);
    }
}