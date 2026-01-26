//! Proof Search Algorithms and Strategies
//!
//! This module implements various proof search algorithms including
//! natural deduction, resolution, and forward/backward chaining.

use crate::error::*;
use crate::property_types::*;
use crate::proof_types::*;
use crate::axiom_system::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};

/// Proof search strategies
#[derive(Debug, Clone, PartialEq)]
pub enum ProofSearchStrategy {
    /// Forward chaining from axioms
    ForwardChaining,
    /// Backward chaining from goal
    BackwardChaining,
    /// Bidirectional search
    Bidirectional,
    /// Resolution-based
    Resolution,
    /// Tableau method
    Tableau,
    /// Natural deduction
    NaturalDeduction,
}

/// Proof search engine that coordinates different search strategies
pub struct ProofSearchEngine {
    /// Available axioms
    axioms: Vec<Axiom>,
    /// Available inference rules
    inference_rules: Vec<InferenceRule>,
    /// Search configuration
    config: SearchConfig,
    /// Search statistics
    stats: ProofSearchStats,
}

/// Configuration for proof search
#[derive(Debug, Clone)]
pub struct SearchConfig {
    /// Maximum search depth
    pub max_depth: usize,
    /// Search timeout
    pub timeout: Duration,
    /// Maximum steps to explore
    pub max_steps: usize,
    /// Enable caching
    pub enable_caching: bool,
    /// Heuristic weights
    pub heuristic_weights: HeuristicWeights,
}

/// Weights for proof search heuristics
#[derive(Debug, Clone)]
pub struct HeuristicWeights {
    /// Weight for formula complexity
    pub complexity_weight: f64,
    /// Weight for axiom priority
    pub axiom_priority_weight: f64,
    /// Weight for rule priority
    pub rule_priority_weight: f64,
    /// Weight for goal distance
    pub goal_distance_weight: f64,
}

/// Statistics for proof search
#[derive(Debug, Clone, Default)]
pub struct ProofSearchStats {
    /// Search time
    pub search_time: Duration,
    /// Total steps explored
    pub steps_explored: usize,
    /// Rules applied
    pub rules_applied: usize,
    /// Backtracking operations
    pub backtrack_count: usize,
    /// Cache hits
    pub cache_hits: usize,
    /// Cache misses
    pub cache_misses: usize,
    /// Memory usage estimate
    pub memory_usage_mb: f64,
}

/// Search context for maintaining state during proof search
#[derive(Debug, Clone)]
pub struct SearchContext {
    /// Current proof state
    pub current_state: ProofState,
    /// Goal formula
    pub goal: FormulaStructure,
    /// Search depth
    pub depth: usize,
    /// Visited states (for cycle detection)
    pub visited: HashSet<String>,
    /// Proof steps so far
    pub proof_steps: Vec<ProofStep>,
}

/// State during proof search
#[derive(Debug, Clone)]
pub struct ProofState {
    /// Available formulas (hypotheses)
    pub hypotheses: Vec<FormulaStructure>,
    /// Current subgoals
    pub subgoals: Vec<FormulaStructure>,
    /// Variable bindings
    pub bindings: HashMap<String, Term>,
    /// Discharge stack for assumptions
    pub discharge_stack: Vec<usize>,
}

impl ProofSearchEngine {
    /// Create new proof search engine
    pub fn new(axioms: Vec<Axiom>, inference_rules: Vec<InferenceRule>) -> Self {
        Self {
            axioms,
            inference_rules,
            config: SearchConfig::default(),
            stats: ProofSearchStats::default(),
        }
    }

    /// Set search configuration
    pub fn with_config(mut self, config: SearchConfig) -> Self {
        self.config = config;
        self
    }

    /// Execute natural deduction proof search
    pub fn natural_deduction_search(&mut self, goal: &FormulaStructure) -> AispResult<ProofOutcome> {
        let start_time = Instant::now();
        let mut context = SearchContext::new(goal.clone());
        
        self.stats = ProofSearchStats::default();
        
        let result = self.natural_deduction_recursive(&mut context, 0);
        
        self.stats.search_time = start_time.elapsed();
        
        match result {
            Ok(proof_steps) => {
                // Construct formal proof
                if !proof_steps.is_empty() {
                    Ok(ProofOutcome::Proven)
                } else {
                    Ok(ProofOutcome::Unknown)
                }
            }
            Err(_) => Ok(ProofOutcome::Unknown),
        }
    }

    /// Recursive natural deduction search
    fn natural_deduction_recursive(&mut self, context: &mut SearchContext, depth: usize) -> AispResult<Vec<ProofStep>> {
        // Check termination conditions
        if depth >= self.config.max_depth {
            return Err(AispError::validation_error("Maximum depth reached".to_string()));
        }

        if self.stats.steps_explored >= self.config.max_steps {
            return Err(AispError::validation_error("Maximum steps reached".to_string()));
        }

        self.stats.steps_explored += 1;

        // Check if goal is already in hypotheses
        if context.current_state.hypotheses.contains(&context.goal) {
            return Ok(vec![]);
        }

        // Try to apply inference rules
        for rule in &self.inference_rules.clone() {
            if let Ok(step) = self.try_apply_rule(rule, context, depth) {
                context.proof_steps.push(step.clone());
                
                // Continue search with new state
                if let Ok(mut remaining_steps) = self.natural_deduction_recursive(context, depth + 1) {
                    let mut all_steps = vec![step];
                    all_steps.append(&mut remaining_steps);
                    return Ok(all_steps);
                }
                
                // Backtrack
                context.proof_steps.pop();
                self.stats.backtrack_count += 1;
            }
        }

        // Try to apply axioms
        for axiom in &self.axioms.clone() {
            if self.axiom_applies_to_goal(axiom, &context.goal) {
                let step = self.create_axiom_step(axiom, context, depth);
                context.proof_steps.push(step.clone());
                
                if let Ok(mut remaining_steps) = self.natural_deduction_recursive(context, depth + 1) {
                    let mut all_steps = vec![step];
                    all_steps.append(&mut remaining_steps);
                    return Ok(all_steps);
                }
                
                // Backtrack
                context.proof_steps.pop();
                self.stats.backtrack_count += 1;
            }
        }

        Err(AispError::validation_error("No applicable rules found".to_string()))
    }

    /// Execute backward chaining proof search
    pub fn backward_chaining_search(&mut self, goal: &FormulaStructure) -> AispResult<ProofOutcome> {
        let start_time = Instant::now();
        let mut context = SearchContext::new(goal.clone());
        
        self.stats = ProofSearchStats::default();
        
        let result = self.backward_chaining_recursive(&mut context, 0);
        
        self.stats.search_time = start_time.elapsed();
        
        match result {
            Ok(_) => Ok(ProofOutcome::Proven),
            Err(_) => Ok(ProofOutcome::Unknown),
        }
    }

    /// Recursive backward chaining search
    fn backward_chaining_recursive(&mut self, context: &mut SearchContext, depth: usize) -> AispResult<()> {
        // Check termination conditions
        if depth >= self.config.max_depth {
            return Err(AispError::validation_error("Maximum depth reached".to_string()));
        }

        self.stats.steps_explored += 1;

        // Check if goal is an axiom
        for axiom in &self.axioms {
            if self.formula_matches(&axiom.formula, &context.goal) {
                return Ok(());
            }
        }

        // Try to decompose goal using inference rules
        let inference_rules = self.inference_rules.clone();
        for rule in &inference_rules {
            if self.rule_concludes(&rule.conclusion, &context.goal) {
                // Create subgoals for rule premises
                let mut subgoals = Vec::new();
                for premise in &rule.premises {
                    if let Some(subgoal) = self.instantiate_pattern(&premise.pattern, &context.goal) {
                        subgoals.push(subgoal);
                    }
                }

                // Try to prove all subgoals
                let mut all_proved = true;
                for subgoal in subgoals {
                    let mut subcontext = context.clone();
                    subcontext.goal = subgoal;
                    
                    if self.backward_chaining_recursive(&mut subcontext, depth + 1).is_err() {
                        all_proved = false;
                        break;
                    }
                }

                if all_proved {
                    self.stats.rules_applied += 1;
                    return Ok(());
                }
            }
        }

        Err(AispError::validation_error("Goal cannot be proven".to_string()))
    }

    /// Execute forward chaining proof search
    pub fn forward_chaining_search(&mut self, goal: &FormulaStructure) -> AispResult<ProofOutcome> {
        let start_time = Instant::now();
        let mut derived = HashSet::new();
        let mut queue = VecDeque::new();
        
        self.stats = ProofSearchStats::default();
        
        // Initialize with axioms
        for axiom in &self.axioms {
            let formula_str = format!("{:?}", axiom.formula);
            if derived.insert(formula_str.clone()) {
                queue.push_back(axiom.formula.clone());
            }
        }

        // Forward chaining loop
        while let Some(current_formula) = queue.pop_front() {
            self.stats.steps_explored += 1;
            
            // Check if we reached the goal
            if self.formula_matches(&current_formula, goal) {
                self.stats.search_time = start_time.elapsed();
                return Ok(ProofOutcome::Proven);
            }

            // Check timeout
            if start_time.elapsed() > self.config.timeout {
                break;
            }

            // Apply inference rules to derive new formulas
            for rule in &self.inference_rules {
                if let Some(new_formula) = self.apply_forward_rule(rule, &current_formula, &derived) {
                    let formula_str = format!("{:?}", new_formula);
                    if derived.insert(formula_str) {
                        queue.push_back(new_formula);
                        self.stats.rules_applied += 1;
                    }
                }
            }
        }

        self.stats.search_time = start_time.elapsed();
        Ok(ProofOutcome::Unknown)
    }

    /// Execute resolution-based proof search
    pub fn resolution_search(&mut self, goal: &FormulaStructure) -> AispResult<ProofOutcome> {
        let start_time = Instant::now();
        
        // Convert to CNF and negate goal
        let negated_goal = FormulaStructure::Negation(Box::new(goal.clone()));
        
        // Initialize clause set with axioms and negated goal
        let mut clauses = Vec::new();
        
        // Convert axioms to clauses (simplified)
        for axiom in &self.axioms {
            clauses.push(self.to_clause(&axiom.formula));
        }
        clauses.push(self.to_clause(&negated_goal));

        // Resolution loop
        let mut iteration = 0;
        while iteration < self.config.max_steps {
            self.stats.steps_explored += 1;
            
            let mut new_clauses = Vec::new();
            
            // Try to resolve each pair of clauses
            for i in 0..clauses.len() {
                for j in i + 1..clauses.len() {
                    if let Some(resolvent) = self.resolve_clauses(&clauses[i], &clauses[j]) {
                        // Check for empty clause (contradiction)
                        if resolvent.is_empty() {
                            self.stats.search_time = start_time.elapsed();
                            return Ok(ProofOutcome::Proven);
                        }
                        
                        if !clauses.contains(&resolvent) && !new_clauses.contains(&resolvent) {
                            new_clauses.push(resolvent);
                            self.stats.rules_applied += 1;
                        }
                    }
                }
            }
            
            // Add new clauses
            if new_clauses.is_empty() {
                break; // No new clauses derived
            }
            clauses.extend(new_clauses);
            iteration += 1;
        }

        self.stats.search_time = start_time.elapsed();
        Ok(ProofOutcome::Unknown)
    }

    // Helper methods
    fn try_apply_rule(&self, rule: &InferenceRule, context: &SearchContext, depth: usize) -> AispResult<ProofStep> {
        // Simplified rule application
        if rule.premises.len() <= context.current_state.hypotheses.len() {
            Ok(ProofStep {
                step_id: depth,
                formula: context.goal.clone(),
                justification: StepJustification::InferenceRule(rule.name.clone(), vec![]),
                dependencies: vec![],
                discharge_level: depth,
                annotations: HashMap::new(),
            })
        } else {
            Err(AispError::validation_error("Rule not applicable".to_string()))
        }
    }

    fn axiom_applies_to_goal(&self, axiom: &Axiom, goal: &FormulaStructure) -> bool {
        self.formula_matches(&axiom.formula, goal)
    }

    fn create_axiom_step(&self, axiom: &Axiom, _context: &SearchContext, depth: usize) -> ProofStep {
        ProofStep {
            step_id: depth,
            formula: axiom.formula.clone(),
            justification: StepJustification::Axiom(axiom.name.clone()),
            dependencies: vec![],
            discharge_level: depth,
            annotations: HashMap::new(),
        }
    }

    fn formula_matches(&self, formula1: &FormulaStructure, formula2: &FormulaStructure) -> bool {
        // Simplified matching - exact equality
        format!("{:?}", formula1) == format!("{:?}", formula2)
    }

    fn rule_concludes(&self, pattern: &FormulaPattern, goal: &FormulaStructure) -> bool {
        // Simplified pattern matching
        matches!(pattern.pattern, PatternStructure::Variable(_))
    }

    fn instantiate_pattern(&self, _pattern: &PatternStructure, goal: &FormulaStructure) -> Option<FormulaStructure> {
        // Simplified instantiation
        Some(goal.clone())
    }

    fn apply_forward_rule(&self, _rule: &InferenceRule, _formula: &FormulaStructure, _derived: &HashSet<String>) -> Option<FormulaStructure> {
        // Simplified forward rule application
        None
    }

    fn to_clause(&self, _formula: &FormulaStructure) -> Vec<String> {
        // Simplified clause conversion
        vec!["P".to_string()]
    }

    fn resolve_clauses(&self, _clause1: &[String], _clause2: &[String]) -> Option<Vec<String>> {
        // Simplified resolution
        None
    }

    /// Get current search statistics
    pub fn get_stats(&self) -> &ProofSearchStats {
        &self.stats
    }
}

impl SearchContext {
    /// Create new search context
    pub fn new(goal: FormulaStructure) -> Self {
        Self {
            current_state: ProofState {
                hypotheses: Vec::new(),
                subgoals: vec![goal.clone()],
                bindings: HashMap::new(),
                discharge_stack: Vec::new(),
            },
            goal,
            depth: 0,
            visited: HashSet::new(),
            proof_steps: Vec::new(),
        }
    }
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            max_depth: 50,
            timeout: Duration::from_secs(60),
            max_steps: 10000,
            enable_caching: true,
            heuristic_weights: HeuristicWeights::default(),
        }
    }
}

impl Default for HeuristicWeights {
    fn default() -> Self {
        Self {
            complexity_weight: 1.0,
            axiom_priority_weight: 1.5,
            rule_priority_weight: 1.2,
            goal_distance_weight: 2.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof_search_engine_creation() {
        let engine = ProofSearchEngine::new(vec![], vec![]);
        assert_eq!(engine.axioms.len(), 0);
        assert_eq!(engine.inference_rules.len(), 0);
    }

    #[test]
    fn test_search_config_default() {
        let config = SearchConfig::default();
        assert_eq!(config.max_depth, 50);
        assert_eq!(config.max_steps, 10000);
        assert!(config.enable_caching);
    }

    #[test]
    fn test_search_context_creation() {
        let goal = FormulaStructure::Atomic(AtomicFormula {
            predicate: "P".to_string(),
            terms: vec![],
            type_signature: None,
        });
        let context = SearchContext::new(goal.clone());
        assert_eq!(context.goal, goal);
        assert_eq!(context.depth, 0);
    }

    #[test]
    fn test_proof_search_stats_default() {
        let stats = ProofSearchStats::default();
        assert_eq!(stats.steps_explored, 0);
        assert_eq!(stats.rules_applied, 0);
    }
}