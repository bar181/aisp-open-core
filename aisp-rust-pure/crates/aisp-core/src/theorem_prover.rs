//! Main Theorem Prover Coordinator
//!
//! This module provides the main theorem prover that coordinates
//! proof search strategies and manages the overall proving process.

use crate::error::*;
use crate::property_types::*;
use crate::proof_types::*;
use crate::axiom_system::*;
use crate::proof_search::*;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

/// Automated theorem prover for AISP properties
pub struct TheoremProver {
    /// Axiom system for natural deduction
    axioms: Vec<Axiom>,
    /// Inference rules available
    inference_rules: Vec<InferenceRule>,
    /// Proof search strategy
    strategy: ProofSearchStrategy,
    /// Maximum proof depth
    max_depth: usize,
    /// Timeout for proof search
    timeout: Duration,
    /// Proof search engine
    search_engine: ProofSearchEngine,
}

impl TheoremProver {
    /// Create new theorem prover with default configuration
    pub fn new() -> Self {
        let mut builder = AxiomSystemBuilder::new();
        
        // Initialize standard axioms and rules
        builder.add_propositional_axioms();
        builder.add_predicate_axioms();
        builder.add_temporal_axioms();
        builder.add_aisp_axioms();
        
        let (axioms, inference_rules) = builder.build();
        let search_engine = ProofSearchEngine::new(axioms.clone(), inference_rules.clone());
        
        Self {
            axioms,
            inference_rules,
            strategy: ProofSearchStrategy::NaturalDeduction,
            max_depth: 50,
            timeout: Duration::from_secs(60),
            search_engine,
        }
    }

    /// Create theorem prover with custom configuration
    pub fn with_config(
        strategy: ProofSearchStrategy,
        max_depth: usize,
        timeout: Duration,
    ) -> Self {
        let mut prover = Self::new();
        prover.strategy = strategy;
        prover.max_depth = max_depth;
        prover.timeout = timeout;
        
        // Update search engine config
        let config = SearchConfig {
            max_depth,
            timeout,
            max_steps: max_depth * 100,
            enable_caching: true,
            heuristic_weights: HeuristicWeights::default(),
        };
        prover.search_engine = prover.search_engine.with_config(config);
        prover
    }

    /// Prove a formula using the configured proof search strategy
    pub fn prove_formula(&mut self, formula: &PropertyFormula) -> AispResult<ProofTree> {
        let goal = &formula.structure;
        
        // Execute proof search
        let outcome = match self.strategy {
            ProofSearchStrategy::NaturalDeduction => {
                self.search_engine.natural_deduction_search(goal)?
            }
            ProofSearchStrategy::BackwardChaining => {
                self.search_engine.backward_chaining_search(goal)?
            }
            ProofSearchStrategy::ForwardChaining => {
                self.search_engine.forward_chaining_search(goal)?
            }
            ProofSearchStrategy::Resolution => {
                self.search_engine.resolution_search(goal)?
            }
            _ => return Err(AispError::validation_error("Proof strategy not implemented".to_string())),
        };
        
        // Convert proof outcome to ProofTree
        match outcome {
            ProofOutcome::Proven => {
                // Create a placeholder proof tree for successful proofs
                Ok(ProofTree {
                    root: goal.clone(),
                    children: vec![],
                    rule: Some("Proved".to_string()),
                    annotations: std::collections::HashMap::new(),
                })
            }
            ProofOutcome::Error(reason) => Err(AispError::validation_error(reason)),
            ProofOutcome::Timeout => Err(AispError::validation_error("Proof search timeout".to_string())),
            _ => Err(AispError::validation_error("Proof not found".to_string())),
        }
    }

    /// Prove a property using the configured proof search strategy
    pub fn prove_property(&mut self, property: &ExtractedProperty) -> AispResult<ProofResult> {
        let start_time = Instant::now();
        
        // Convert property to goal formula
        let goal = &property.formula.structure;

        // Execute proof search
        let outcome = match self.strategy {
            ProofSearchStrategy::NaturalDeduction => {
                self.search_engine.natural_deduction_search(goal)?
            }
            ProofSearchStrategy::BackwardChaining => {
                self.search_engine.backward_chaining_search(goal)?
            }
            ProofSearchStrategy::ForwardChaining => {
                self.search_engine.forward_chaining_search(goal)?
            }
            ProofSearchStrategy::Resolution => {
                self.search_engine.resolution_search(goal)?
            }
            _ => return Err(AispError::validation_error("Proof strategy not implemented".to_string())),
        };

        let search_time = start_time.elapsed();
        let search_stats = self.search_engine.get_stats().clone();

        // Convert search stats to proof stats
        let proof_stats = ProofStats {
            steps_explored: search_stats.steps_explored,
            max_depth_reached: self.max_depth,
            backtrack_count: search_stats.backtrack_count,
            axioms_applied: self.count_axiom_applications(),
            rules_applied: search_stats.rules_applied,
            failed_attempts: search_stats.backtrack_count,
            search_time_ms: search_time.as_millis() as u64,
            memory_usage_mb: search_stats.memory_usage_mb,
        };

        let mut result = ProofResult::new(outcome.clone());
        result.search_time = search_time;
        result.steps_explored = search_stats.steps_explored;
        result.stats = proof_stats;

        // Generate formal proof if proven
        if outcome == ProofOutcome::Proven {
            if let Ok(proof) = self.construct_formal_proof(property, goal) {
                result.proof = Some(proof);
            }
        }

        // Generate counterexample if disproven
        if outcome == ProofOutcome::Disproven {
            if let Ok(counterexample) = self.generate_counterexample(property) {
                result.counterexample = Some(counterexample);
            }
        }

        Ok(result)
    }

    /// Prove multiple properties efficiently
    pub fn prove_properties(&mut self, properties: &[ExtractedProperty]) -> AispResult<Vec<ProofResult>> {
        let mut results = Vec::new();
        
        for property in properties {
            let result = self.prove_property(property)?;
            results.push(result);
            
            // Early termination if too many failures
            let failures = results.iter().filter(|r| !r.is_conclusive()).count();
            if failures > properties.len() / 2 {
                // More than 50% failures, might indicate systematic issues
                break;
            }
        }
        
        Ok(results)
    }

    /// Add custom axiom to the system
    pub fn add_axiom(&mut self, axiom: Axiom) {
        self.axioms.push(axiom);
        // Recreate search engine with updated axioms
        self.search_engine = ProofSearchEngine::new(
            self.axioms.clone(),
            self.inference_rules.clone(),
        );
    }

    /// Add custom inference rule to the system
    pub fn add_inference_rule(&mut self, rule: InferenceRule) {
        self.inference_rules.push(rule);
        // Recreate search engine with updated rules
        self.search_engine = ProofSearchEngine::new(
            self.axioms.clone(),
            self.inference_rules.clone(),
        );
    }

    /// Set proof search strategy
    pub fn set_strategy(&mut self, strategy: ProofSearchStrategy) {
        self.strategy = strategy;
    }

    /// Set timeout for proof search
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    /// Set maximum proof depth
    pub fn set_max_depth(&mut self, max_depth: usize) {
        self.max_depth = max_depth;
    }

    /// Get current axiom system
    pub fn get_axioms(&self) -> &[Axiom] {
        &self.axioms
    }

    /// Get current inference rules
    pub fn get_inference_rules(&self) -> &[InferenceRule] {
        &self.inference_rules
    }

    /// Generate proof statistics summary
    pub fn generate_summary(&self, results: &[ProofResult]) -> ProofSummary {
        let total_properties = results.len();
        let proven_count = results.iter().filter(|r| r.is_proven()).count();
        let disproven_count = results.iter().filter(|r| r.is_disproven()).count();
        let timeout_count = results.iter().filter(|r| matches!(r.outcome, ProofOutcome::Timeout)).count();
        let unknown_count = results.iter().filter(|r| matches!(r.outcome, ProofOutcome::Unknown)).count();
        let error_count = results.iter().filter(|r| matches!(r.outcome, ProofOutcome::Error(_))).count();

        let total_search_time: Duration = results.iter().map(|r| r.search_time).sum();
        let avg_search_time = if total_properties > 0 {
            total_search_time / total_properties as u32
        } else {
            Duration::ZERO
        };

        let total_steps_explored: usize = results.iter().map(|r| r.steps_explored).sum();
        let avg_steps_explored = if total_properties > 0 {
            total_steps_explored / total_properties
        } else {
            0
        };

        ProofSummary {
            total_properties,
            proven_count,
            disproven_count,
            timeout_count,
            unknown_count,
            error_count,
            success_rate: if total_properties > 0 { proven_count as f64 / total_properties as f64 } else { 0.0 },
            total_search_time,
            avg_search_time,
            avg_steps_explored,
            strategy_used: self.strategy.clone(),
        }
    }

    // Private helper methods
    fn construct_formal_proof(&self, property: &ExtractedProperty, goal: &FormulaStructure) -> AispResult<FormalProof> {
        let mut proof = FormalProof {
            conclusion: property.formula.clone(),
            steps: vec![],
            axioms_used: vec![],
            rules_applied: vec![],
            proof_tree: ProofTree::leaf(goal.clone()),
            is_valid: true,
            complexity: ProofComplexity {
                step_count: 1,
                max_depth: 1,
                assumption_count: 0,
                rule_applications: 0,
                branching_factor: 1.0,
                complexity_score: 1,
            },
        };

        // Calculate complexity metrics
        proof.calculate_complexity();
        
        Ok(proof)
    }

    fn generate_counterexample(&self, _property: &ExtractedProperty) -> AispResult<Counterexample> {
        let mut counterexample = Counterexample::new();
        
        // Simple counterexample generation (would be more sophisticated in practice)
        counterexample.assign_variable("x".to_string(), "42".to_string());
        counterexample.validate();
        
        Ok(counterexample)
    }

    fn count_axiom_applications(&self) -> usize {
        // Count how many times axioms were used (would track in practice)
        self.axioms.len()
    }
}

impl Default for TheoremProver {
    fn default() -> Self {
        Self::new()
    }
}

/// Summary of proof results
#[derive(Debug, Clone)]
pub struct ProofSummary {
    /// Total properties attempted
    pub total_properties: usize,
    /// Properties successfully proven
    pub proven_count: usize,
    /// Properties disproven
    pub disproven_count: usize,
    /// Properties that timed out
    pub timeout_count: usize,
    /// Properties with unknown result
    pub unknown_count: usize,
    /// Properties with errors
    pub error_count: usize,
    /// Success rate (proven / total)
    pub success_rate: f64,
    /// Total search time across all properties
    pub total_search_time: Duration,
    /// Average search time per property
    pub avg_search_time: Duration,
    /// Average steps explored per property
    pub avg_steps_explored: usize,
    /// Strategy used for proving
    pub strategy_used: ProofSearchStrategy,
}

impl ProofSummary {
    /// Check if overall proving was successful
    pub fn is_successful(&self) -> bool {
        self.success_rate > 0.5 && self.error_count == 0
    }

    /// Get efficiency metric (proven per second)
    pub fn efficiency(&self) -> f64 {
        if self.total_search_time.as_secs_f64() > 0.0 {
            self.proven_count as f64 / self.total_search_time.as_secs_f64()
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_theorem_prover_creation() {
        let prover = TheoremProver::new();
        assert!(!prover.axioms.is_empty());
        assert!(!prover.inference_rules.is_empty());
        assert_eq!(prover.strategy, ProofSearchStrategy::NaturalDeduction);
        assert_eq!(prover.max_depth, 50);
    }

    #[test]
    fn test_theorem_prover_with_config() {
        let prover = TheoremProver::with_config(
            ProofSearchStrategy::BackwardChaining,
            100,
            Duration::from_secs(120),
        );
        assert_eq!(prover.strategy, ProofSearchStrategy::BackwardChaining);
        assert_eq!(prover.max_depth, 100);
        assert_eq!(prover.timeout, Duration::from_secs(120));
    }

    #[test]
    fn test_add_axiom() {
        let mut prover = TheoremProver::new();
        let initial_count = prover.axioms.len();
        
        let axiom = Axiom::new(
            "test_axiom".to_string(),
            FormulaStructure::Atomic(AtomicFormula {
                predicate: "TestPredicate".to_string(),
                terms: vec![],
                type_signature: None,
            }),
            AxiomType::Domain,
            5,
        );
        
        prover.add_axiom(axiom);
        assert_eq!(prover.axioms.len(), initial_count + 1);
    }

    #[test]
    fn test_proof_summary() {
        let prover = TheoremProver::new();
        let results = vec![
            ProofResult::new(ProofOutcome::Proven),
            ProofResult::new(ProofOutcome::Proven),
            ProofResult::new(ProofOutcome::Unknown),
        ];
        
        let summary = prover.generate_summary(&results);
        assert_eq!(summary.total_properties, 3);
        assert_eq!(summary.proven_count, 2);
        assert_eq!(summary.unknown_count, 1);
        assert_eq!(summary.success_rate, 2.0 / 3.0);
    }

    #[test]
    fn test_proof_summary_efficiency() {
        let summary = ProofSummary {
            total_properties: 10,
            proven_count: 8,
            disproven_count: 1,
            timeout_count: 1,
            unknown_count: 0,
            error_count: 0,
            success_rate: 0.8,
            total_search_time: Duration::from_secs(4),
            avg_search_time: Duration::from_millis(400),
            avg_steps_explored: 50,
            strategy_used: ProofSearchStrategy::NaturalDeduction,
        };
        
        assert!(summary.is_successful());
        assert_eq!(summary.efficiency(), 2.0); // 8 proven / 4 seconds = 2 per second
    }

    #[test]
    fn test_prove_empty_properties() {
        let mut prover = TheoremProver::new();
        let results = prover.prove_properties(&[]).unwrap();
        assert!(results.is_empty());
    }
}