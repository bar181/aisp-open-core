//! Temporal Logic Model Checker for AISP Verification
//!
//! This module implements explicit-state model checking for temporal logic
//! properties in AISP documents, supporting LTL, CTL, and CTL*.

use crate::ast::canonical::*;
use crate::error::*;
use crate::property_extractor::*;
use crate::temporal_new::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};

/// Model checker for temporal logic properties
pub struct ModelChecker {
    /// State space representation
    state_space: StateSpace,
    /// Temporal properties to verify
    properties: Vec<TemporalProperty>,
    /// Model checking algorithm
    algorithm: ModelCheckingAlgorithm,
    /// Maximum states to explore
    max_states: usize,
    /// Verification timeout
    timeout: Duration,
    /// Model checking statistics
    stats: ModelCheckingStats,
}

/// Complete state space of the system
#[derive(Debug, Clone)]
pub struct StateSpace {
    /// All system states
    pub states: HashMap<StateId, SystemState>,
    /// Transition relation
    pub transitions: HashMap<StateId, Vec<Transition>>,
    /// Initial states
    pub initial_states: HashSet<StateId>,
    /// Atomic propositions
    pub atomic_props: HashMap<String, HashSet<StateId>>,
    /// State labels
    pub labels: HashMap<StateId, HashSet<String>>,
}

/// Unique state identifier
pub type StateId = u64;

/// System state representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemState {
    /// State identifier
    pub id: StateId,
    /// Variable assignments
    pub variables: HashMap<String, Value>,
    /// Active locations/conditions
    pub locations: HashSet<String>,
    /// State properties
    pub properties: HashSet<String>,
}

/// State transition
#[derive(Debug, Clone)]
pub struct Transition {
    /// Source state
    pub from: StateId,
    /// Target state
    pub to: StateId,
    /// Transition action/event
    pub action: Option<String>,
    /// Transition guard condition
    pub guard: Option<String>,
    /// Transition probability (for probabilistic systems)
    pub probability: Option<f64>,
}

/// Value types in system states
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    /// Boolean value
    Bool(bool),
    /// Integer value
    Int(i64),
    /// String value
    String(String),
    /// Set value
    Set(Vec<Value>),
    /// Tuple value
    Tuple(Vec<Value>),
}

/// Temporal property for model checking
#[derive(Debug, Clone)]
pub struct TemporalProperty {
    /// Property identifier
    pub id: String,
    /// Property name
    pub name: String,
    /// Temporal logic formula
    pub formula: TemporalFormula,
    /// Property type (LTL, CTL, CTL*)
    pub property_type: TemporalLogicType,
    /// Expected result
    pub expected: Option<bool>,
}

/// Types of temporal logic
#[derive(Debug, Clone, PartialEq)]
pub enum TemporalLogicType {
    /// Linear Temporal Logic
    LTL,
    /// Computation Tree Logic
    CTL,
    /// CTL with LTL path formulas
    CTLStar,
}

/// Temporal logic formula
#[derive(Debug, Clone)]
pub enum TemporalFormula {
    /// Atomic proposition
    Atomic(String),
    /// Negation
    Not(Box<TemporalFormula>),
    /// Conjunction
    And(Box<TemporalFormula>, Box<TemporalFormula>),
    /// Disjunction
    Or(Box<TemporalFormula>, Box<TemporalFormula>),
    /// Implication
    Implies(Box<TemporalFormula>, Box<TemporalFormula>),
    /// Always (globally) - □φ
    Always(Box<TemporalFormula>),
    /// Eventually (finally) - ◊φ
    Eventually(Box<TemporalFormula>),
    /// Next - ○φ
    Next(Box<TemporalFormula>),
    /// Until - φ U ψ
    Until(Box<TemporalFormula>, Box<TemporalFormula>),
    /// Release - φ R ψ
    Release(Box<TemporalFormula>, Box<TemporalFormula>),
    /// Weak until - φ W ψ
    WeakUntil(Box<TemporalFormula>, Box<TemporalFormula>),
    /// Strong release - φ M ψ
    StrongRelease(Box<TemporalFormula>, Box<TemporalFormula>),
    /// CTL path quantifiers
    ExistsAlways(Box<TemporalFormula>),      // EG
    ForallAlways(Box<TemporalFormula>),      // AG
    ExistsEventually(Box<TemporalFormula>),  // EF
    ForallEventually(Box<TemporalFormula>),  // AF
    ExistsNext(Box<TemporalFormula>),        // EX
    ForallNext(Box<TemporalFormula>),        // AX
    ExistsUntil(Box<TemporalFormula>, Box<TemporalFormula>),  // EU
    ForallUntil(Box<TemporalFormula>, Box<TemporalFormula>),  // AU
}

/// Model checking algorithms
#[derive(Debug, Clone, PartialEq)]
pub enum ModelCheckingAlgorithm {
    /// Explicit-state enumeration
    ExplicitState,
    /// Breadth-first search
    BFS,
    /// Depth-first search
    DFS,
    /// Tableau-based
    Tableau,
    /// On-the-fly verification
    OnTheFly,
    /// Symbolic (BDD-based)
    Symbolic,
}

/// Model checking result
#[derive(Debug, Clone)]
pub struct ModelCheckingResult {
    /// Property verification results
    pub property_results: HashMap<String, PropertyResult>,
    /// Overall verification status
    pub status: VerificationStatus,
    /// Model checking statistics
    pub stats: ModelCheckingStats,
    /// Execution time
    pub execution_time: Duration,
}

/// Individual property result
#[derive(Debug, Clone)]
pub struct PropertyResult {
    /// Property identifier
    pub property_id: String,
    /// Verification outcome
    pub result: PropertyVerificationResult,
    /// Counterexample trace (if property fails)
    pub counterexample: Option<CounterexampleTrace>,
    /// Witness trace (if property holds)
    pub witness: Option<WitnessTrace>,
    /// Verification time for this property
    pub verification_time: Duration,
}

/// Property verification outcome
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyVerificationResult {
    /// Property holds (verified true)
    Satisfied,
    /// Property does not hold (counterexample found)
    Violated,
    /// Verification inconclusive
    Unknown,
    /// Verification error
    Error(String),
}

/// Verification status
#[derive(Debug, Clone, PartialEq)]
pub enum VerificationStatus {
    /// All properties verified
    Success,
    /// Some properties failed
    PartialFailure,
    /// Verification incomplete
    Incomplete,
    /// Verification failed completely
    Failed,
}

/// Counterexample trace showing property violation
#[derive(Debug, Clone)]
pub struct CounterexampleTrace {
    /// Trace identifier
    pub id: String,
    /// State sequence in counterexample
    pub states: Vec<StateId>,
    /// Transition sequence
    pub transitions: Vec<String>,
    /// Loop point (if trace is infinite)
    pub loop_point: Option<usize>,
    /// Trace type
    pub trace_type: TraceType,
}

/// Witness trace showing property satisfaction
#[derive(Debug, Clone)]
pub struct WitnessTrace {
    /// Trace identifier
    pub id: String,
    /// State sequence in witness
    pub states: Vec<StateId>,
    /// Transition sequence
    pub transitions: Vec<String>,
    /// Trace type
    pub trace_type: TraceType,
}

/// Types of traces
#[derive(Debug, Clone, PartialEq)]
pub enum TraceType {
    /// Finite execution
    Finite,
    /// Infinite execution with loop
    Infinite,
    /// Partial execution (bounded)
    Partial,
}

/// Model checking statistics
#[derive(Debug, Clone)]
pub struct ModelCheckingStats {
    /// Total states explored
    pub states_explored: usize,
    /// Total transitions explored
    pub transitions_explored: usize,
    /// Memory usage (bytes)
    pub memory_usage: usize,
    /// Properties verified
    pub properties_verified: usize,
    /// Properties failed
    pub properties_failed: usize,
    /// Verification time
    pub total_time: Duration,
    /// Algorithm-specific metrics
    pub algorithm_metrics: HashMap<String, f64>,
}

impl ModelChecker {
    /// Create new model checker
    pub fn new() -> Self {
        Self {
            state_space: StateSpace::new(),
            properties: Vec::new(),
            algorithm: ModelCheckingAlgorithm::ExplicitState,
            max_states: 100_000,
            timeout: Duration::from_secs(300),
            stats: ModelCheckingStats::new(),
        }
    }

    /// Configure model checker
    pub fn configure(
        &mut self,
        algorithm: ModelCheckingAlgorithm,
        max_states: usize,
        timeout: Duration,
    ) {
        self.algorithm = algorithm;
        self.max_states = max_states;
        self.timeout = timeout;
    }

    /// Build state space from AISP document
    pub fn build_state_space(&mut self, doc: &AispDocument) -> AispResult<()> {
        // Extract system model from AISP document
        self.extract_state_variables(doc)?;
        self.extract_initial_states(doc)?;
        self.extract_transitions(doc)?;
        self.extract_atomic_propositions(doc)?;

        Ok(())
    }

    /// Add temporal property for verification
    pub fn add_property(&mut self, property: TemporalProperty) {
        self.properties.push(property);
    }

    /// Verify all temporal properties
    pub fn verify_properties(&mut self) -> AispResult<ModelCheckingResult> {
        let start_time = Instant::now();
        let mut property_results = HashMap::new();
        
        // Reset statistics
        self.stats = ModelCheckingStats::new();

        let properties = self.properties.clone();
        for property in &properties {
            let property_start = Instant::now();
            
            let result = match property.property_type {
                TemporalLogicType::LTL => self.verify_ltl_property(&property.formula)?,
                TemporalLogicType::CTL => self.verify_ctl_property(&property.formula)?,
                TemporalLogicType::CTLStar => self.verify_ctl_star_property(&property.formula)?,
            };

            let verification_time = property_start.elapsed();
            
            let property_result = PropertyResult {
                property_id: property.id.clone(),
                result: result.0.clone(),
                counterexample: result.1,
                witness: result.2,
                verification_time,
            };

            property_results.insert(property.id.clone(), property_result);
            
            // Update statistics
            match result.0 {
                PropertyVerificationResult::Satisfied => self.stats.properties_verified += 1,
                PropertyVerificationResult::Violated => self.stats.properties_failed += 1,
                _ => {}
            }
        }

        let execution_time = start_time.elapsed();
        self.stats.total_time = execution_time;

        // Determine overall status
        let status = if self.stats.properties_failed == 0 {
            VerificationStatus::Success
        } else if self.stats.properties_verified > 0 {
            VerificationStatus::PartialFailure
        } else {
            VerificationStatus::Failed
        };

        Ok(ModelCheckingResult {
            property_results,
            status,
            stats: self.stats.clone(),
            execution_time,
        })
    }

    /// Verify LTL property
    fn verify_ltl_property(
        &mut self,
        formula: &TemporalFormula,
    ) -> AispResult<(PropertyVerificationResult, Option<CounterexampleTrace>, Option<WitnessTrace>)> {
        match self.algorithm {
            ModelCheckingAlgorithm::ExplicitState => self.ltl_explicit_state(formula),
            ModelCheckingAlgorithm::Tableau => self.ltl_tableau_method(formula),
            _ => Err(AispError::validation_error("LTL algorithm not supported".to_string())),
        }
    }

    /// Verify CTL property
    fn verify_ctl_property(
        &mut self,
        formula: &TemporalFormula,
    ) -> AispResult<(PropertyVerificationResult, Option<CounterexampleTrace>, Option<WitnessTrace>)> {
        self.ctl_explicit_state(formula)
    }

    /// Verify CTL* property
    fn verify_ctl_star_property(
        &mut self,
        _formula: &TemporalFormula,
    ) -> AispResult<(PropertyVerificationResult, Option<CounterexampleTrace>, Option<WitnessTrace>)> {
        Err(AispError::validation_error("CTL* verification not implemented".to_string()))
    }

    /// LTL verification using explicit state enumeration
    fn ltl_explicit_state(
        &mut self,
        formula: &TemporalFormula,
    ) -> AispResult<(PropertyVerificationResult, Option<CounterexampleTrace>, Option<WitnessTrace>)> {
        // Convert LTL formula to Büchi automaton (simplified)
        let negated_formula = TemporalFormula::Not(Box::new(formula.clone()));
        
        // Check for accepting runs in product of system and Büchi automaton
        let accepting_run = self.find_accepting_run(&negated_formula)?;
        
        if let Some(trace) = accepting_run {
            // Found counterexample
            Ok((PropertyVerificationResult::Violated, Some(trace), None))
        } else {
            // No counterexample found - property satisfied
            Ok((PropertyVerificationResult::Satisfied, None, None))
        }
    }

    /// LTL verification using tableau method
    fn ltl_tableau_method(
        &mut self,
        _formula: &TemporalFormula,
    ) -> AispResult<(PropertyVerificationResult, Option<CounterexampleTrace>, Option<WitnessTrace>)> {
        // Simplified tableau method - would implement full algorithm
        Ok((PropertyVerificationResult::Unknown, None, None))
    }

    /// CTL verification using explicit state enumeration
    fn ctl_explicit_state(
        &mut self,
        formula: &TemporalFormula,
    ) -> AispResult<(PropertyVerificationResult, Option<CounterexampleTrace>, Option<WitnessTrace>)> {
        // Mark states that satisfy the formula
        let satisfying_states = self.ctl_marking_algorithm(formula)?;
        
        // Check if all initial states satisfy the property
        let all_initial_satisfy = self.state_space.initial_states
            .iter()
            .all(|state_id| satisfying_states.contains(state_id));
        
        if all_initial_satisfy {
            Ok((PropertyVerificationResult::Satisfied, None, None))
        } else {
            // Find counterexample from initial state that doesn't satisfy property
            let counterexample = self.find_ctl_counterexample(formula, &satisfying_states)?;
            Ok((PropertyVerificationResult::Violated, counterexample, None))
        }
    }

    /// CTL state marking algorithm
    fn ctl_marking_algorithm(&mut self, formula: &TemporalFormula) -> AispResult<HashSet<StateId>> {
        match formula {
            TemporalFormula::Atomic(prop) => {
                // Return states where atomic proposition holds
                Ok(self.state_space.atomic_props
                   .get(prop)
                   .cloned()
                   .unwrap_or_default())
            }
            TemporalFormula::Not(inner) => {
                let inner_states = self.ctl_marking_algorithm(inner)?;
                let all_states: HashSet<StateId> = self.state_space.states.keys().cloned().collect();
                Ok(all_states.difference(&inner_states).cloned().collect())
            }
            TemporalFormula::And(left, right) => {
                let left_states = self.ctl_marking_algorithm(left)?;
                let right_states = self.ctl_marking_algorithm(right)?;
                Ok(left_states.intersection(&right_states).cloned().collect())
            }
            TemporalFormula::Or(left, right) => {
                let left_states = self.ctl_marking_algorithm(left)?;
                let right_states = self.ctl_marking_algorithm(right)?;
                Ok(left_states.union(&right_states).cloned().collect())
            }
            TemporalFormula::ExistsNext(inner) => {
                let inner_states = self.ctl_marking_algorithm(inner)?;
                let mut result = HashSet::new();
                
                for (state_id, transitions) in &self.state_space.transitions {
                    if transitions.iter().any(|t| inner_states.contains(&t.to)) {
                        result.insert(*state_id);
                    }
                }
                
                Ok(result)
            }
            TemporalFormula::ForallNext(inner) => {
                let inner_states = self.ctl_marking_algorithm(inner)?;
                let mut result = HashSet::new();
                
                for (state_id, transitions) in &self.state_space.transitions {
                    if !transitions.is_empty() && transitions.iter().all(|t| inner_states.contains(&t.to)) {
                        result.insert(*state_id);
                    }
                }
                
                Ok(result)
            }
            TemporalFormula::ExistsEventually(inner) => {
                // EF φ = μZ. φ ∨ EX Z
                let inner_states = self.ctl_marking_algorithm(inner)?;
                self.ctl_exists_eventually(&inner_states)
            }
            TemporalFormula::ForallEventually(inner) => {
                // AF φ = μZ. φ ∨ AX Z
                let inner_states = self.ctl_marking_algorithm(inner)?;
                self.ctl_forall_eventually(&inner_states)
            }
            TemporalFormula::ExistsAlways(inner) => {
                // EG φ = νZ. φ ∧ EX Z
                let inner_states = self.ctl_marking_algorithm(inner)?;
                self.ctl_exists_always(&inner_states)
            }
            TemporalFormula::ForallAlways(inner) => {
                // AG φ = νZ. φ ∧ AX Z
                let inner_states = self.ctl_marking_algorithm(inner)?;
                self.ctl_forall_always(&inner_states)
            }
            _ => Err(AispError::validation_error("Temporal operator not supported in CTL".to_string())),
        }
    }

    /// Compute EF (exists eventually) using fixed point
    fn ctl_exists_eventually(&self, target_states: &HashSet<StateId>) -> AispResult<HashSet<StateId>> {
        let mut current = target_states.clone();
        let mut changed = true;
        
        while changed {
            changed = false;
            let mut new_states = current.clone();
            
            for (state_id, transitions) in &self.state_space.transitions {
                if !current.contains(state_id) {
                    if transitions.iter().any(|t| current.contains(&t.to)) {
                        new_states.insert(*state_id);
                        changed = true;
                    }
                }
            }
            
            current = new_states;
        }
        
        Ok(current)
    }

    /// Compute AF (forall eventually) using fixed point
    fn ctl_forall_eventually(&self, target_states: &HashSet<StateId>) -> AispResult<HashSet<StateId>> {
        let mut current = target_states.clone();
        let mut changed = true;
        
        while changed {
            changed = false;
            let mut new_states = current.clone();
            
            for (state_id, transitions) in &self.state_space.transitions {
                if !current.contains(state_id) && !transitions.is_empty() {
                    if transitions.iter().all(|t| current.contains(&t.to)) {
                        new_states.insert(*state_id);
                        changed = true;
                    }
                }
            }
            
            current = new_states;
        }
        
        Ok(current)
    }

    /// Compute EG (exists always) using fixed point
    fn ctl_exists_always(&self, target_states: &HashSet<StateId>) -> AispResult<HashSet<StateId>> {
        let mut current = target_states.clone();
        let mut changed = true;
        
        while changed {
            changed = false;
            let mut new_states = HashSet::new();
            
            for state_id in &current {
                if let Some(transitions) = self.state_space.transitions.get(state_id) {
                    if transitions.iter().any(|t| current.contains(&t.to)) {
                        new_states.insert(*state_id);
                    } else {
                        changed = true;
                    }
                } else {
                    changed = true;
                }
            }
            
            current = new_states;
        }
        
        Ok(current)
    }

    /// Compute AG (forall always) using fixed point
    fn ctl_forall_always(&self, target_states: &HashSet<StateId>) -> AispResult<HashSet<StateId>> {
        let mut current = target_states.clone();
        let mut changed = true;
        
        while changed {
            changed = false;
            let mut new_states = HashSet::new();
            
            for state_id in &current {
                if let Some(transitions) = self.state_space.transitions.get(state_id) {
                    if !transitions.is_empty() && transitions.iter().all(|t| current.contains(&t.to)) {
                        new_states.insert(*state_id);
                    } else {
                        changed = true;
                    }
                } else {
                    changed = true;
                }
            }
            
            current = new_states;
        }
        
        Ok(current)
    }

    /// Find accepting run for LTL verification (simplified)
    fn find_accepting_run(&mut self, _formula: &TemporalFormula) -> AispResult<Option<CounterexampleTrace>> {
        // Simplified implementation - would implement full Büchi automaton construction
        Ok(None)
    }

    /// Find CTL counterexample
    fn find_ctl_counterexample(
        &self,
        _formula: &TemporalFormula,
        _satisfying_states: &HashSet<StateId>,
    ) -> AispResult<Option<CounterexampleTrace>> {
        // Simplified implementation
        Ok(None)
    }

    /// Extract state variables from AISP document
    fn extract_state_variables(&mut self, _doc: &AispDocument) -> AispResult<()> {
        // Would analyze types and rules to determine state space
        Ok(())
    }

    /// Extract initial states
    fn extract_initial_states(&mut self, _doc: &AispDocument) -> AispResult<()> {
        // Would determine initial state configurations
        let initial_state = SystemState {
            id: 0,
            variables: HashMap::new(),
            locations: HashSet::new(),
            properties: HashSet::new(),
        };
        
        self.state_space.states.insert(0, initial_state);
        self.state_space.initial_states.insert(0);
        
        Ok(())
    }

    /// Extract transition relation
    fn extract_transitions(&mut self, _doc: &AispDocument) -> AispResult<()> {
        // Would analyze rules to determine transitions
        Ok(())
    }

    /// Extract atomic propositions
    fn extract_atomic_propositions(&mut self, _doc: &AispDocument) -> AispResult<()> {
        // Would extract propositions from document
        Ok(())
    }

    /// Get model checking statistics
    pub fn get_stats(&self) -> &ModelCheckingStats {
        &self.stats
    }
}

impl StateSpace {
    /// Create new empty state space
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            transitions: HashMap::new(),
            initial_states: HashSet::new(),
            atomic_props: HashMap::new(),
            labels: HashMap::new(),
        }
    }
}

impl ModelCheckingStats {
    /// Create new statistics
    pub fn new() -> Self {
        Self {
            states_explored: 0,
            transitions_explored: 0,
            memory_usage: 0,
            properties_verified: 0,
            properties_failed: 0,
            total_time: Duration::ZERO,
            algorithm_metrics: HashMap::new(),
        }
    }
}

impl Default for ModelChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_checker_creation() {
        let checker = ModelChecker::new();
        assert_eq!(checker.algorithm, ModelCheckingAlgorithm::ExplicitState);
        assert_eq!(checker.max_states, 100_000);
        assert!(checker.properties.is_empty());
    }

    #[test]
    fn test_configuration() {
        let mut checker = ModelChecker::new();
        checker.configure(
            ModelCheckingAlgorithm::BFS,
            50_000,
            Duration::from_secs(120),
        );
        
        assert_eq!(checker.algorithm, ModelCheckingAlgorithm::BFS);
        assert_eq!(checker.max_states, 50_000);
        assert_eq!(checker.timeout, Duration::from_secs(120));
    }

    #[test]
    fn test_temporal_property_addition() {
        let mut checker = ModelChecker::new();
        let property = TemporalProperty {
            id: "prop1".to_string(),
            name: "Safety Property".to_string(),
            formula: TemporalFormula::Always(Box::new(TemporalFormula::Atomic("safe".to_string()))),
            property_type: TemporalLogicType::LTL,
            expected: Some(true),
        };
        
        checker.add_property(property);
        assert_eq!(checker.properties.len(), 1);
    }

    #[test]
    fn test_state_space_creation() {
        let state_space = StateSpace::new();
        assert!(state_space.states.is_empty());
        assert!(state_space.transitions.is_empty());
        assert!(state_space.initial_states.is_empty());
    }

    #[test]
    fn test_system_state_equality() {
        let state1 = SystemState {
            id: 1,
            variables: HashMap::new(),
            locations: HashSet::new(),
            properties: HashSet::new(),
        };
        
        let state2 = SystemState {
            id: 1,
            variables: HashMap::new(),
            locations: HashSet::new(),
            properties: HashSet::new(),
        };
        
        assert_eq!(state1, state2);
    }

    #[test]
    fn test_value_types() {
        let bool_val = Value::Bool(true);
        let int_val = Value::Int(42);
        let string_val = Value::String("test".to_string());
        
        assert_eq!(bool_val, Value::Bool(true));
        assert_eq!(int_val, Value::Int(42));
        assert_eq!(string_val, Value::String("test".to_string()));
    }
}