//! Hebbian Learning Constraint Validation for AISP 5.1
//!
//! This module implements validation for Hebbian learning constraints, which enforce
//! the fundamental learning principles in AISP systems:
//!
//! **Key Constraint: 10:1 Failure Penalty Ratio**
//!
//! Where:
//! - Learning rate must be adjusted based on success/failure feedback
//! - Failed learning attempts carry 10x penalty compared to successful ones
//! - Synaptic weight updates must follow Hebbian plasticity rules
//! - Temporal learning patterns must maintain consistency
//!
//! Hebbian learning validation ensures that AISP systems learn efficiently
//! and maintain stable knowledge acquisition patterns.

use crate::{
    ast::canonical::{CanonicalAispDocument as AispDocument, CanonicalAispBlock as AispBlock, *},
    error::*,
    semantic::DeepVerificationResult,
};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Hebbian learning validation result
#[derive(Debug, Clone)]
pub struct HebbianValidationResult {
    /// Whether Hebbian learning constraints are satisfied
    pub valid: bool,
    /// Overall learning consistency score
    pub learning_score: f64,
    /// Constraint validation results
    pub constraints: HebbianConstraints,
    /// Learning pattern analysis
    pub patterns: HebbianPatterns,
    /// Validation statistics
    pub stats: HebbianStats,
    /// Analysis errors
    pub errors: Vec<String>,
    /// Analysis warnings
    pub warnings: Vec<String>,
}

/// Hebbian learning constraints
#[derive(Debug, Clone)]
pub struct HebbianConstraints {
    /// 10:1 failure penalty ratio validation
    pub penalty_ratio_valid: bool,
    /// Learning rate bounds validation
    pub learning_rate_valid: bool,
    /// Synaptic weight update validation
    pub weight_update_valid: bool,
    /// Temporal consistency validation
    pub temporal_consistency_valid: bool,
    /// Detailed constraint metrics
    pub constraint_metrics: ConstraintMetrics,
}

/// Detailed constraint metrics
#[derive(Debug, Clone)]
pub struct ConstraintMetrics {
    /// Measured penalty ratio (target: 10.0)
    pub measured_penalty_ratio: f64,
    /// Learning rate value (target: 0.001-0.1)
    pub learning_rate: f64,
    /// Weight update magnitude (target: < 1.0)
    pub weight_update_magnitude: f64,
    /// Temporal consistency score (target: > 0.8)
    pub temporal_consistency: f64,
}

/// Hebbian learning patterns
#[derive(Debug, Clone)]
pub struct HebbianPatterns {
    /// Detected learning episodes
    pub episodes: Vec<LearningEpisode>,
    /// Pattern statistics
    pub pattern_stats: PatternStats,
    /// Plasticity measures
    pub plasticity: PlasticityMeasures,
}

/// Individual learning episode
#[derive(Debug, Clone)]
pub struct LearningEpisode {
    /// Episode identifier
    pub id: String,
    /// Learning outcome (success/failure)
    pub outcome: LearningOutcome,
    /// Weight change magnitude
    pub weight_change: f64,
    /// Temporal position in sequence
    pub temporal_position: usize,
    /// Associated rule or function
    pub associated_element: String,
}

/// Learning outcome classification
#[derive(Debug, Clone, PartialEq)]
pub enum LearningOutcome {
    /// Successful learning episode
    Success,
    /// Failed learning episode (10x penalty)
    Failure,
    /// Neutral/no-change episode
    Neutral,
}

/// Pattern analysis statistics
#[derive(Debug, Clone)]
pub struct PatternStats {
    /// Total learning episodes detected
    pub total_episodes: usize,
    /// Successful episodes count
    pub success_count: usize,
    /// Failed episodes count
    pub failure_count: usize,
    /// Neutral episodes count
    pub neutral_count: usize,
    /// Success rate percentage
    pub success_rate: f64,
    /// Average weight change per episode
    pub average_weight_change: f64,
}

/// Synaptic plasticity measures
#[derive(Debug, Clone)]
pub struct PlasticityMeasures {
    /// Long-term potentiation (LTP) strength
    pub ltp_strength: f64,
    /// Long-term depression (LTD) strength
    pub ltd_strength: f64,
    /// Plasticity balance ratio (LTP/LTD)
    pub balance_ratio: f64,
    /// Hebbian efficiency score
    pub efficiency_score: f64,
}

/// Hebbian learning analysis statistics
#[derive(Debug, Clone)]
pub struct HebbianStats {
    /// Total analysis time
    pub analysis_time: Duration,
    /// Number of constraints checked
    pub constraints_checked: usize,
    /// Number of learning patterns analyzed
    pub patterns_analyzed: usize,
    /// Plasticity calculations performed
    pub plasticity_calculations: usize,
}

/// Configuration for Hebbian learning validation
#[derive(Debug, Clone)]
pub struct HebbianConfig {
    /// Target penalty ratio (default: 10.0)
    pub target_penalty_ratio: f64,
    /// Acceptable ratio tolerance
    pub penalty_ratio_tolerance: f64,
    /// Minimum learning rate
    pub min_learning_rate: f64,
    /// Maximum learning rate
    pub max_learning_rate: f64,
    /// Maximum weight update magnitude
    pub max_weight_update: f64,
    /// Minimum temporal consistency score
    pub min_temporal_consistency: f64,
    /// Maximum analysis time allowed
    pub max_analysis_time: Duration,
    /// Enable plasticity analysis
    pub enable_plasticity_analysis: bool,
}

impl Default for HebbianConfig {
    fn default() -> Self {
        Self {
            target_penalty_ratio: 10.0,
            penalty_ratio_tolerance: 1.0,
            min_learning_rate: 0.001,
            max_learning_rate: 0.1,
            max_weight_update: 1.0,
            min_temporal_consistency: 0.8,
            max_analysis_time: Duration::from_secs(5),
            enable_plasticity_analysis: true,
        }
    }
}

/// Hebbian learning validator implementing 10:1 penalty ratio analysis
pub struct HebbianValidator {
    /// Validation configuration
    config: HebbianConfig,
    /// Learning episode cache
    episode_cache: HashMap<String, LearningEpisode>,
    /// Analysis statistics
    stats: HebbianStats,
}

impl HebbianValidator {
    /// Create new Hebbian learning validator
    pub fn new(config: HebbianConfig) -> Self {
        Self {
            config,
            episode_cache: HashMap::new(),
            stats: HebbianStats {
                analysis_time: Duration::from_secs(0),
                constraints_checked: 0,
                patterns_analyzed: 0,
                plasticity_calculations: 0,
            },
        }
    }

    /// Validate Hebbian learning constraints for AISP document
    pub fn validate_hebbian_learning(&mut self, document: &AispDocument, semantic_result: &DeepVerificationResult) -> AispResult<HebbianValidationResult> {
        let start_time = Instant::now();
        
        // Extract and analyze learning patterns
        let patterns = self.analyze_learning_patterns(document, semantic_result)?;
        
        // Validate Hebbian constraints
        let constraints = self.validate_constraints(document, &patterns)?;
        
        // Calculate overall learning score
        let learning_score = self.calculate_learning_score(&constraints, &patterns)?;
        
        // Update statistics
        self.stats.analysis_time = start_time.elapsed();
        self.stats.constraints_checked += 4; // penalty_ratio, learning_rate, weight_update, temporal
        self.stats.patterns_analyzed += patterns.episodes.len();
        
        Ok(HebbianValidationResult {
            valid: self.is_validation_successful(&constraints),
            learning_score,
            constraints: constraints.clone(),
            patterns,
            stats: self.stats.clone(),
            errors: vec![],
            warnings: self.generate_warnings(&constraints),
        })
    }

    /// Analyze learning patterns in AISP document
    fn analyze_learning_patterns(&mut self, document: &AispDocument, semantic_result: &DeepVerificationResult) -> AispResult<HebbianPatterns> {
        let mut episodes = Vec::new();
        
        // Extract learning episodes from rules and functions
        episodes.extend(self.extract_episodes_from_rules(document)?);
        episodes.extend(self.extract_episodes_from_functions(document)?);
        episodes.extend(self.extract_episodes_from_evidence(document, semantic_result)?);
        
        // Calculate pattern statistics
        let pattern_stats = self.calculate_pattern_stats(&episodes);
        
        // Calculate plasticity measures if enabled
        let plasticity = if self.config.enable_plasticity_analysis {
            self.calculate_plasticity_measures(&episodes)?
        } else {
            PlasticityMeasures {
                ltp_strength: 0.5,
                ltd_strength: 0.5,
                balance_ratio: 1.0,
                efficiency_score: 0.5,
            }
        };
        
        Ok(HebbianPatterns {
            episodes,
            pattern_stats,
            plasticity,
        })
    }

    /// Extract learning episodes from rules block
    fn extract_episodes_from_rules(&mut self, document: &AispDocument) -> AispResult<Vec<LearningEpisode>> {
        let mut episodes = Vec::new();
        
        for block in &document.blocks {
            if let AispBlock::Rules(rules_block) = block {
                for (index, rule) in rules_block.rules.iter().enumerate() {
                    let episode = LearningEpisode {
                        id: format!("rule_{}", index),
                        outcome: self.classify_rule_outcome(rule)?,
                        weight_change: self.estimate_rule_weight_change(rule)?,
                        temporal_position: index,
                        associated_element: format!("rule_{}", index),
                    };
                    episodes.push(episode);
                }
            }
        }
        
        Ok(episodes)
    }

    /// Extract learning episodes from functions block
    fn extract_episodes_from_functions(&mut self, document: &AispDocument) -> AispResult<Vec<LearningEpisode>> {
        let mut episodes = Vec::new();
        
        for block in &document.blocks {
            if let AispBlock::Functions(functions_block) = block {
                for (index, function) in functions_block.functions.iter().enumerate() {
                    let episode = LearningEpisode {
                        id: format!("function_{}", function.name),
                        outcome: self.classify_function_outcome(function)?,
                        weight_change: self.estimate_function_weight_change(function)?,
                        temporal_position: index,
                        associated_element: function.name.clone(),
                    };
                    episodes.push(episode);
                }
            }
        }
        
        Ok(episodes)
    }

    /// Extract learning episodes from evidence block
    fn extract_episodes_from_evidence(&mut self, document: &AispDocument, semantic_result: &DeepVerificationResult) -> AispResult<Vec<LearningEpisode>> {
        let mut episodes = Vec::new();
        
        for block in &document.blocks {
            if let AispBlock::Evidence(evidence_block) = block {
                // Create episode based on overall evidence quality
                let episode = LearningEpisode {
                    id: "evidence_overall".to_string(),
                    outcome: self.classify_evidence_outcome(evidence_block, semantic_result)?,
                    weight_change: semantic_result.delta().min(1.0),
                    temporal_position: 0,
                    associated_element: "evidence".to_string(),
                };
                episodes.push(episode);
            }
        }
        
        Ok(episodes)
    }

    /// Classify learning outcome for a logical rule
    fn classify_rule_outcome(&self, rule: &LogicalRule) -> AispResult<LearningOutcome> {
        // Analyze rule complexity and structure to classify outcome
        let complexity = self.calculate_expression_complexity(&rule.expression);
        
        match complexity {
            0.0..=0.3 => Ok(LearningOutcome::Failure),   // Too simple, likely failed learning
            0.3..=0.7 => Ok(LearningOutcome::Success),   // Good complexity, successful learning
            _ => Ok(LearningOutcome::Neutral),           // Overly complex, neutral outcome
        }
    }

    /// Classify learning outcome for a function definition
    fn classify_function_outcome(&self, function: &FunctionDefinition) -> AispResult<LearningOutcome> {
        // Analyze function complexity
        let param_count = function.lambda.parameters.len() as f64;
        let body_complexity = self.calculate_expression_complexity(&function.lambda.body);
        
        let overall_complexity = (param_count / 10.0) + body_complexity;
        
        match overall_complexity {
            0.0..=0.2 => Ok(LearningOutcome::Failure),
            0.2..=0.8 => Ok(LearningOutcome::Success),
            _ => Ok(LearningOutcome::Neutral),
        }
    }

    /// Classify learning outcome for evidence block
    fn classify_evidence_outcome(&self, _evidence_block: &EvidenceBlock, semantic_result: &DeepVerificationResult) -> AispResult<LearningOutcome> {
        // Base classification on semantic analysis results
        if semantic_result.delta() >= 0.7 && semantic_result.ambiguity() < 0.02 {
            Ok(LearningOutcome::Success)
        } else if semantic_result.delta() < 0.3 || semantic_result.ambiguity() > 0.1 {
            Ok(LearningOutcome::Failure)
        } else {
            Ok(LearningOutcome::Neutral)
        }
    }

    /// Calculate expression complexity for outcome classification
    fn calculate_expression_complexity(&self, expr: &LogicalExpression) -> f64 {
        match expr {
            LogicalExpression::Variable(_) | LogicalExpression::Constant(_) => 0.1,
            LogicalExpression::Binary { left, right, .. } => {
                0.3 + (self.calculate_expression_complexity(left) + self.calculate_expression_complexity(right)) / 2.0
            }
            LogicalExpression::Unary { operand, .. } => {
                0.2 + self.calculate_expression_complexity(operand)
            }
            LogicalExpression::Application { arguments, .. } => {
                let arg_complexity: f64 = arguments.iter()
                    .map(|arg| self.calculate_expression_complexity(arg))
                    .sum::<f64>() / arguments.len() as f64;
                0.4 + arg_complexity
            }
            LogicalExpression::Membership { element, set } => {
                0.3 + (self.calculate_expression_complexity(element) + self.calculate_expression_complexity(set)) / 2.0
            }
            LogicalExpression::Temporal { operand, .. } => {
                0.5 + self.calculate_expression_complexity(operand)
            }
            LogicalExpression::Raw(_) => {
                0.2 // Simple complexity for raw expressions
            }
        }
    }

    /// Estimate weight change magnitude for a rule
    fn estimate_rule_weight_change(&self, rule: &LogicalRule) -> AispResult<f64> {
        let complexity = self.calculate_expression_complexity(&rule.expression);
        let quantifier_weight = if rule.quantifier.is_some() { 0.3 } else { 0.1 };
        
        Ok((complexity + quantifier_weight).min(2.0))
    }

    /// Estimate weight change magnitude for a function
    fn estimate_function_weight_change(&self, function: &FunctionDefinition) -> AispResult<f64> {
        let param_weight = function.lambda.parameters.len() as f64 * 0.1;
        let body_complexity = self.calculate_expression_complexity(&function.lambda.body);
        
        Ok((param_weight + body_complexity).min(2.0))
    }

    /// Calculate pattern statistics
    fn calculate_pattern_stats(&self, episodes: &[LearningEpisode]) -> PatternStats {
        let total_episodes = episodes.len();
        let success_count = episodes.iter().filter(|e| e.outcome == LearningOutcome::Success).count();
        let failure_count = episodes.iter().filter(|e| e.outcome == LearningOutcome::Failure).count();
        let neutral_count = episodes.iter().filter(|e| e.outcome == LearningOutcome::Neutral).count();
        
        let success_rate = if total_episodes > 0 {
            success_count as f64 / total_episodes as f64
        } else {
            0.0
        };
        
        let average_weight_change = if total_episodes > 0 {
            episodes.iter().map(|e| e.weight_change).sum::<f64>() / total_episodes as f64
        } else {
            0.0
        };
        
        PatternStats {
            total_episodes,
            success_count,
            failure_count,
            neutral_count,
            success_rate,
            average_weight_change,
        }
    }

    /// Calculate plasticity measures
    fn calculate_plasticity_measures(&mut self, episodes: &[LearningEpisode]) -> AispResult<PlasticityMeasures> {
        self.stats.plasticity_calculations += 1;
        
        let successful_changes: f64 = episodes.iter()
            .filter(|e| e.outcome == LearningOutcome::Success)
            .map(|e| e.weight_change)
            .sum();
        
        let failed_changes: f64 = episodes.iter()
            .filter(|e| e.outcome == LearningOutcome::Failure)
            .map(|e| e.weight_change)
            .sum();
        
        // LTP (Long-Term Potentiation) from successful learning
        let ltp_strength = if episodes.iter().any(|e| e.outcome == LearningOutcome::Success) {
            successful_changes / episodes.len() as f64
        } else {
            0.0
        };
        
        // LTD (Long-Term Depression) from failed learning (with 10x penalty)
        let ltd_strength = if episodes.iter().any(|e| e.outcome == LearningOutcome::Failure) {
            (failed_changes * self.config.target_penalty_ratio) / episodes.len() as f64
        } else {
            0.0
        };
        
        // Balance ratio (should be close to target penalty ratio)
        let balance_ratio = if ltp_strength > 0.0 {
            ltd_strength / ltp_strength
        } else {
            0.0
        };
        
        // Efficiency score based on how close we are to optimal balance
        let efficiency_score = if balance_ratio > 0.0 {
            let ratio_diff = (balance_ratio - self.config.target_penalty_ratio).abs();
            (1.0 - (ratio_diff / self.config.target_penalty_ratio)).max(0.0)
        } else {
            0.0
        };
        
        Ok(PlasticityMeasures {
            ltp_strength,
            ltd_strength,
            balance_ratio,
            efficiency_score,
        })
    }

    /// Validate Hebbian learning constraints
    fn validate_constraints(&mut self, _document: &AispDocument, patterns: &HebbianPatterns) -> AispResult<HebbianConstraints> {
        // Validate 10:1 penalty ratio
        let measured_penalty_ratio = patterns.plasticity.balance_ratio;
        let penalty_ratio_valid = (measured_penalty_ratio - self.config.target_penalty_ratio).abs() <= self.config.penalty_ratio_tolerance;
        
        // Validate learning rate (estimated from average weight change)
        let estimated_learning_rate = patterns.pattern_stats.average_weight_change;
        let learning_rate_valid = estimated_learning_rate >= self.config.min_learning_rate && 
                                 estimated_learning_rate <= self.config.max_learning_rate;
        
        // Validate weight update magnitude
        let max_weight_change = patterns.episodes.iter()
            .map(|e| e.weight_change)
            .fold(0.0, f64::max);
        let weight_update_valid = max_weight_change <= self.config.max_weight_update;
        
        // Validate temporal consistency
        let temporal_consistency = self.calculate_temporal_consistency(&patterns.episodes);
        let temporal_consistency_valid = temporal_consistency >= self.config.min_temporal_consistency;
        
        Ok(HebbianConstraints {
            penalty_ratio_valid,
            learning_rate_valid,
            weight_update_valid,
            temporal_consistency_valid,
            constraint_metrics: ConstraintMetrics {
                measured_penalty_ratio,
                learning_rate: estimated_learning_rate,
                weight_update_magnitude: max_weight_change,
                temporal_consistency,
            },
        })
    }

    /// Calculate temporal consistency of learning episodes
    fn calculate_temporal_consistency(&self, episodes: &[LearningEpisode]) -> f64 {
        if episodes.len() < 2 {
            return 1.0; // Perfect consistency for single or no episodes
        }
        
        // Calculate consistency based on temporal ordering and outcome patterns
        let mut consistency_sum = 0.0;
        let mut comparisons = 0;
        
        for window in episodes.windows(2) {
            let episode1 = &window[0];
            let episode2 = &window[1];
            
            // Check temporal ordering
            let temporal_consistency = if episode2.temporal_position > episode1.temporal_position {
                1.0
            } else {
                0.0
            };
            
            // Check outcome consistency (similar outcomes should have similar weight changes)
            let outcome_consistency = if episode1.outcome == episode2.outcome {
                let weight_diff = (episode1.weight_change - episode2.weight_change).abs();
                1.0 - weight_diff.min(1.0)
            } else {
                0.5 // Neutral for different outcomes
            };
            
            consistency_sum += (temporal_consistency + outcome_consistency) / 2.0;
            comparisons += 1;
        }
        
        if comparisons > 0 {
            consistency_sum / comparisons as f64
        } else {
            1.0
        }
    }

    /// Calculate overall learning score
    fn calculate_learning_score(&self, constraints: &HebbianConstraints, patterns: &HebbianPatterns) -> AispResult<f64> {
        // Weight different aspects of learning validation
        let constraint_score = self.calculate_constraint_score(constraints);
        let pattern_score = patterns.pattern_stats.success_rate;
        let plasticity_score = patterns.plasticity.efficiency_score;
        
        // Weighted average
        let learning_score = (constraint_score * 0.5) + (pattern_score * 0.3) + (plasticity_score * 0.2);
        
        Ok(learning_score.min(1.0))
    }

    /// Calculate constraint validation score
    fn calculate_constraint_score(&self, constraints: &HebbianConstraints) -> f64 {
        let mut score = 0.0;
        let mut total_constraints = 0;
        
        if constraints.penalty_ratio_valid {
            score += 1.0;
        }
        total_constraints += 1;
        
        if constraints.learning_rate_valid {
            score += 1.0;
        }
        total_constraints += 1;
        
        if constraints.weight_update_valid {
            score += 1.0;
        }
        total_constraints += 1;
        
        if constraints.temporal_consistency_valid {
            score += 1.0;
        }
        total_constraints += 1;
        
        if total_constraints > 0 {
            score / total_constraints as f64
        } else {
            0.0
        }
    }

    /// Check if validation is successful
    fn is_validation_successful(&self, constraints: &HebbianConstraints) -> bool {
        constraints.penalty_ratio_valid && 
        constraints.learning_rate_valid &&
        constraints.weight_update_valid &&
        constraints.temporal_consistency_valid
    }

    /// Generate analysis warnings
    fn generate_warnings(&self, constraints: &HebbianConstraints) -> Vec<String> {
        let mut warnings = Vec::new();
        
        if !constraints.penalty_ratio_valid {
            warnings.push(format!(
                "Penalty ratio {:.2} deviates from target {:.2} beyond tolerance {:.2}",
                constraints.constraint_metrics.measured_penalty_ratio,
                self.config.target_penalty_ratio,
                self.config.penalty_ratio_tolerance
            ));
        }
        
        if !constraints.learning_rate_valid {
            warnings.push(format!(
                "Learning rate {:.4} is outside acceptable range [{:.4}, {:.4}]",
                constraints.constraint_metrics.learning_rate,
                self.config.min_learning_rate,
                self.config.max_learning_rate
            ));
        }
        
        if !constraints.weight_update_valid {
            warnings.push(format!(
                "Maximum weight update {:.3} exceeds limit {:.3}",
                constraints.constraint_metrics.weight_update_magnitude,
                self.config.max_weight_update
            ));
        }
        
        if !constraints.temporal_consistency_valid {
            warnings.push(format!(
                "Temporal consistency {:.3} is below minimum {:.3}",
                constraints.constraint_metrics.temporal_consistency,
                self.config.min_temporal_consistency
            ));
        }
        
        if self.stats.analysis_time > self.config.max_analysis_time {
            warnings.push("Hebbian learning analysis exceeded maximum time limit".to_string());
        }
        
        warnings
    }

    /// Get validation statistics
    pub fn get_stats(&self) -> &HebbianStats {
        &self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{DocumentHeader, DocumentMetadata, Span, Position};
    use crate::semantic::QualityTier;

    fn create_test_document() -> AispDocument {
        AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "hebbian_test".to_string(),
                date: "2026-01-26".to_string(),
                metadata: None,
            },
            metadata: DocumentMetadata {
                domain: Some("ai".to_string()),
                protocol: Some("aisp".to_string()),
            },
            blocks: vec![],
            span: Span {
                start: Position { line: 1, column: 1, offset: 0 },
                end: Position { line: 1, column: 1, offset: 0 },
            },
        }
    }

    fn create_test_semantic_result() -> DeepVerificationResult {
        DeepVerificationResult {
            delta: 0.8,
            ambiguity: 0.01,
            completeness: 0.9,
            tier: QualityTier::Gold,
            quality_score: 0.85,
            validation_errors: vec![],
            warnings: vec![],
        }
    }

    #[test]
    fn test_hebbian_validator_creation() {
        let config = HebbianConfig::default();
        let validator = HebbianValidator::new(config);
        
        assert_eq!(validator.stats.constraints_checked, 0);
        assert_eq!(validator.stats.patterns_analyzed, 0);
    }

    #[test]
    fn test_learning_outcome_classification() {
        let config = HebbianConfig::default();
        let validator = HebbianValidator::new(config);
        
        let simple_expr = LogicalExpression::Variable("x".to_string());
        let complexity = validator.calculate_expression_complexity(&simple_expr);
        assert!(complexity <= 0.3);
    }

    #[test]
    fn test_pattern_stats_calculation() {
        let config = HebbianConfig::default();
        let validator = HebbianValidator::new(config);
        
        let episodes = vec![
            LearningEpisode {
                id: "ep1".to_string(),
                outcome: LearningOutcome::Success,
                weight_change: 0.5,
                temporal_position: 1,
                associated_element: "rule1".to_string(),
            },
            LearningEpisode {
                id: "ep2".to_string(),
                outcome: LearningOutcome::Failure,
                weight_change: 0.3,
                temporal_position: 2,
                associated_element: "rule2".to_string(),
            },
        ];
        
        let stats = validator.calculate_pattern_stats(&episodes);
        
        assert_eq!(stats.total_episodes, 2);
        assert_eq!(stats.success_count, 1);
        assert_eq!(stats.failure_count, 1);
        assert_eq!(stats.success_rate, 0.5);
        assert_eq!(stats.average_weight_change, 0.4);
    }

    #[test]
    fn test_temporal_consistency_calculation() {
        let config = HebbianConfig::default();
        let validator = HebbianValidator::new(config);
        
        let ordered_episodes = vec![
            LearningEpisode {
                id: "ep1".to_string(),
                outcome: LearningOutcome::Success,
                weight_change: 0.5,
                temporal_position: 1,
                associated_element: "rule1".to_string(),
            },
            LearningEpisode {
                id: "ep2".to_string(),
                outcome: LearningOutcome::Success,
                weight_change: 0.6,
                temporal_position: 2,
                associated_element: "rule2".to_string(),
            },
        ];
        
        let consistency = validator.calculate_temporal_consistency(&ordered_episodes);
        assert!(consistency > 0.8); // Should be high for ordered, similar episodes
    }

    #[test]
    fn test_penalty_ratio_validation() {
        let config = HebbianConfig {
            target_penalty_ratio: 10.0,
            penalty_ratio_tolerance: 1.0,
            ..Default::default()
        };
        let mut validator = HebbianValidator::new(config);
        
        let episodes = vec![
            LearningEpisode {
                id: "success".to_string(),
                outcome: LearningOutcome::Success,
                weight_change: 0.1,
                temporal_position: 1,
                associated_element: "rule1".to_string(),
            },
            LearningEpisode {
                id: "failure".to_string(),
                outcome: LearningOutcome::Failure,
                weight_change: 0.1,
                temporal_position: 2,
                associated_element: "rule2".to_string(),
            },
        ];
        
        let plasticity = validator.calculate_plasticity_measures(&episodes).unwrap();
        
        // LTD should be approximately 10x LTP due to penalty ratio
        assert!(plasticity.balance_ratio > 5.0); // Should be close to target 10.0
    }

    #[test]
    fn test_constraint_validation() {
        let constraints = HebbianConstraints {
            penalty_ratio_valid: true,
            learning_rate_valid: true,
            weight_update_valid: false, // One constraint fails
            temporal_consistency_valid: true,
            constraint_metrics: ConstraintMetrics {
                measured_penalty_ratio: 10.0,
                learning_rate: 0.01,
                weight_update_magnitude: 1.5, // Exceeds limit
                temporal_consistency: 0.9,
            },
        };
        
        let config = HebbianConfig::default();
        let validator = HebbianValidator::new(config);
        
        assert!(!validator.is_validation_successful(&constraints));
        
        let score = validator.calculate_constraint_score(&constraints);
        assert_eq!(score, 0.75); // 3 out of 4 constraints pass
    }
}