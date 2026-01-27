//! Anti-Drift Protocol Verification for AISP 5.1
//!
//! This module implements validation for anti-drift protocols, which ensure
//! semantic stability and consistency over time in AISP systems:
//!
//! **Core Anti-Drift Principles:**
//! - Semantic stability: meanings should not drift without explicit updates
//! - Temporal consistency: behavior should be predictable across time
//! - Drift detection: automatic identification of semantic changes
//! - Correction protocols: mechanisms to restore semantic stability
//!
//! Anti-drift validation is essential for maintaining reliable AI systems
//! that preserve their intended behavior over extended periods.

use crate::{
    ast::canonical::{CanonicalAispDocument as AispDocument, CanonicalAispBlock as AispBlock, *},
    error::*,
    semantic::DeepVerificationResult,
};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Anti-drift validation result
#[derive(Debug, Clone)]
pub struct AntiDriftValidationResult {
    /// Whether anti-drift protocols are effective
    pub valid: bool,
    /// Overall drift resistance score
    pub drift_resistance_score: f64,
    /// Detected drift patterns
    pub drift_patterns: DriftPatterns,
    /// Stability measurements
    pub stability_metrics: StabilityMetrics,
    /// Correction protocols status
    pub correction_protocols: CorrectionProtocols,
    /// Validation statistics
    pub stats: AntiDriftStats,
    /// Analysis errors
    pub errors: Vec<String>,
    /// Analysis warnings
    pub warnings: Vec<String>,
}

/// Detected drift patterns
#[derive(Debug, Clone)]
pub struct DriftPatterns {
    /// Individual drift incidents
    pub incidents: Vec<DriftIncident>,
    /// Drift trend analysis
    pub trends: DriftTrends,
    /// Pattern classification
    pub classification: DriftClassification,
}

/// Individual drift incident
#[derive(Debug, Clone)]
pub struct DriftIncident {
    /// Incident identifier
    pub id: String,
    /// Drift type classification
    pub drift_type: DriftType,
    /// Severity level (0.0-1.0)
    pub severity: f64,
    /// Temporal position when detected
    pub temporal_position: f64,
    /// Affected semantic elements
    pub affected_elements: Vec<String>,
    /// Detected change magnitude
    pub change_magnitude: f64,
}

/// Types of semantic drift
#[derive(Debug, Clone, PartialEq)]
pub enum DriftType {
    /// Gradual semantic shift
    GradualShift,
    /// Sudden semantic change
    AbruptChange,
    /// Conceptual boundary drift
    BoundaryDrift,
    /// Systematic bias introduction
    BiasIntroduction,
    /// Context-dependent drift
    ContextualDrift,
    /// Increasing semantic complexity
    Complexification,
    /// Decreasing semantic precision
    Simplification,
}

/// Drift trend analysis
#[derive(Debug, Clone)]
pub struct DriftTrends {
    /// Overall drift velocity (change per time unit)
    pub drift_velocity: f64,
    /// Drift acceleration (change in velocity)
    pub drift_acceleration: f64,
    /// Periodicity in drift patterns
    pub periodicity: Option<f64>,
    /// Dominant drift direction
    pub dominant_direction: DriftDirection,
}

/// Drift direction classification
#[derive(Debug, Clone, PartialEq)]
pub enum DriftDirection {
    /// Increasing semantic complexity
    Complexification,
    /// Decreasing semantic precision
    Simplification,
    /// Lateral semantic shift
    LateralShift,
    /// No clear direction
    Random,
}

/// Drift pattern classification
#[derive(Debug, Clone)]
pub struct DriftClassification {
    /// Number of gradual shifts detected
    pub gradual_shifts: usize,
    /// Number of abrupt changes detected
    pub abrupt_changes: usize,
    /// Most severe drift incident
    pub max_severity: f64,
    /// Average drift severity
    pub average_severity: f64,
    /// Drift frequency (incidents per time unit)
    pub drift_frequency: f64,
}

/// Stability measurements
#[derive(Debug, Clone)]
pub struct StabilityMetrics {
    /// Semantic consistency over time
    pub semantic_consistency: f64,
    /// Behavioral predictability score
    pub behavioral_predictability: f64,
    /// Meaning preservation ratio
    pub meaning_preservation: f64,
    /// Temporal stability coefficient
    pub temporal_stability: f64,
    /// Reference baseline deviation
    pub baseline_deviation: f64,
}

/// Correction protocols status
#[derive(Debug, Clone)]
pub struct CorrectionProtocols {
    /// Automatic correction capabilities
    pub auto_correction_enabled: bool,
    /// Manual correction protocols available
    pub manual_correction_available: bool,
    /// Correction success rate
    pub correction_success_rate: f64,
    /// Time to correction (average)
    pub average_correction_time: Duration,
    /// Available correction methods
    pub correction_methods: Vec<CorrectionMethod>,
}

/// Available correction methods
#[derive(Debug, Clone)]
pub struct CorrectionMethod {
    /// Method identifier
    pub id: String,
    /// Method type
    pub method_type: CorrectionType,
    /// Effectiveness score
    pub effectiveness: f64,
    /// Applicable drift types
    pub applicable_drift_types: Vec<DriftType>,
}

/// Types of drift correction
#[derive(Debug, Clone, PartialEq)]
pub enum CorrectionType {
    /// Revert to previous state
    StateReversion,
    /// Apply semantic constraints
    ConstraintEnforcement,
    /// Bias correction
    BiasCorrection,
    /// Context normalization
    ContextNormalization,
    /// Manual intervention
    ManualIntervention,
}

/// Anti-drift analysis statistics
#[derive(Debug, Clone)]
pub struct AntiDriftStats {
    /// Total analysis time
    pub analysis_time: Duration,
    /// Number of drift checks performed
    pub drift_checks_performed: usize,
    /// Stability calculations count
    pub stability_calculations: usize,
    /// Correction protocol evaluations
    pub correction_evaluations: usize,
    /// Temporal samples analyzed
    pub temporal_samples: usize,
}

/// Configuration for anti-drift validation
#[derive(Debug, Clone)]
pub struct AntiDriftConfig {
    /// Maximum acceptable drift velocity
    pub max_drift_velocity: f64,
    /// Drift severity threshold for alerts
    pub severity_threshold: f64,
    /// Minimum stability requirement
    pub min_stability_score: f64,
    /// Time window for drift analysis
    pub analysis_time_window: Duration,
    /// Maximum analysis time allowed
    pub max_analysis_time: Duration,
    /// Enable automatic drift correction
    pub enable_auto_correction: bool,
    /// Reference baseline for comparison
    pub reference_baseline: Option<String>,
}

impl Default for AntiDriftConfig {
    fn default() -> Self {
        Self {
            max_drift_velocity: 0.1,
            severity_threshold: 0.3,
            min_stability_score: 0.8,
            analysis_time_window: Duration::from_secs(3600), // 1 hour
            max_analysis_time: Duration::from_secs(10),
            enable_auto_correction: true,
            reference_baseline: None,
        }
    }
}

/// Anti-drift protocol validator implementing semantic stability analysis
pub struct AntiDriftValidator {
    /// Validation configuration
    config: AntiDriftConfig,
    /// Historical drift measurements
    drift_history: Vec<DriftMeasurement>,
    /// Correction protocol registry
    correction_registry: HashMap<String, CorrectionMethod>,
    /// Analysis statistics
    stats: AntiDriftStats,
}

/// Historical drift measurement
#[derive(Debug, Clone)]
pub struct DriftMeasurement {
    /// Timestamp of measurement
    pub timestamp: f64,
    /// Measured drift magnitude
    pub magnitude: f64,
    /// Semantic elements involved
    pub elements: Vec<String>,
}

impl AntiDriftValidator {
    /// Create new anti-drift validator
    pub fn new(config: AntiDriftConfig) -> Self {
        let mut correction_registry = HashMap::new();
        
        // Initialize standard correction methods
        correction_registry.insert("state_reversion".to_string(), CorrectionMethod {
            id: "state_reversion".to_string(),
            method_type: CorrectionType::StateReversion,
            effectiveness: 0.9,
            applicable_drift_types: vec![DriftType::AbruptChange, DriftType::BiasIntroduction],
        });
        
        correction_registry.insert("constraint_enforcement".to_string(), CorrectionMethod {
            id: "constraint_enforcement".to_string(),
            method_type: CorrectionType::ConstraintEnforcement,
            effectiveness: 0.85,
            applicable_drift_types: vec![DriftType::GradualShift, DriftType::BoundaryDrift],
        });
        
        correction_registry.insert("bias_correction".to_string(), CorrectionMethod {
            id: "bias_correction".to_string(),
            method_type: CorrectionType::BiasCorrection,
            effectiveness: 0.8,
            applicable_drift_types: vec![DriftType::BiasIntroduction],
        });
        
        Self {
            config,
            drift_history: Vec::new(),
            correction_registry,
            stats: AntiDriftStats {
                analysis_time: Duration::from_secs(0),
                drift_checks_performed: 0,
                stability_calculations: 0,
                correction_evaluations: 0,
                temporal_samples: 0,
            },
        }
    }

    /// Validate anti-drift protocols for AISP document
    pub fn validate_anti_drift(&mut self, document: &AispDocument, semantic_result: &DeepVerificationResult) -> AispResult<AntiDriftValidationResult> {
        let start_time = Instant::now();
        
        // Analyze drift patterns
        let drift_patterns = self.analyze_drift_patterns(document, semantic_result)?;
        
        // Calculate stability metrics
        let stability_metrics = self.calculate_stability_metrics(document, semantic_result, &drift_patterns)?;
        
        // Evaluate correction protocols
        let correction_protocols = self.evaluate_correction_protocols(&drift_patterns)?;
        
        // Calculate overall drift resistance score
        let drift_resistance_score = self.calculate_drift_resistance_score(&drift_patterns, &stability_metrics, &correction_protocols)?;
        
        // Update statistics
        self.stats.analysis_time = start_time.elapsed();
        self.stats.drift_checks_performed += drift_patterns.incidents.len();
        self.stats.stability_calculations += 5; // semantic, behavioral, meaning, temporal, baseline
        
        Ok(AntiDriftValidationResult {
            valid: self.is_validation_successful(&stability_metrics, &drift_patterns),
            drift_resistance_score,
            drift_patterns: drift_patterns.clone(),
            stability_metrics: stability_metrics.clone(),
            correction_protocols,
            stats: self.stats.clone(),
            errors: vec![],
            warnings: self.generate_warnings(&drift_patterns, &stability_metrics),
        })
    }

    /// Analyze drift patterns in AISP document
    fn analyze_drift_patterns(&mut self, document: &AispDocument, semantic_result: &DeepVerificationResult) -> AispResult<DriftPatterns> {
        let mut incidents = Vec::new();
        
        // Detect drift from semantic analysis
        incidents.extend(self.detect_semantic_drift(document, semantic_result)?);
        
        // Detect structural drift
        incidents.extend(self.detect_structural_drift(document)?);
        
        // Detect behavioral drift
        incidents.extend(self.detect_behavioral_drift(document)?);
        
        // Analyze drift trends
        let trends = self.analyze_drift_trends(&incidents);
        
        // Classify drift patterns
        let classification = self.classify_drift_patterns(&incidents);
        
        Ok(DriftPatterns {
            incidents,
            trends,
            classification,
        })
    }

    /// Detect semantic drift from analysis results
    fn detect_semantic_drift(&mut self, _document: &AispDocument, semantic_result: &DeepVerificationResult) -> AispResult<Vec<DriftIncident>> {
        let mut incidents = Vec::new();
        self.stats.temporal_samples += 1;
        
        // Create drift measurement
        let measurement = DriftMeasurement {
            timestamp: 1.0, // Normalized timestamp
            magnitude: semantic_result.ambiguity(),
            elements: vec!["semantic_analysis".to_string()],
        };
        
        self.drift_history.push(measurement);
        
        // Detect drift based on ambiguity levels
        if semantic_result.ambiguity() > 0.02 {
            let incident = DriftIncident {
                id: "semantic_ambiguity_drift".to_string(),
                drift_type: if semantic_result.ambiguity() > 0.1 {
                    DriftType::AbruptChange
                } else {
                    DriftType::GradualShift
                },
                severity: semantic_result.ambiguity().min(1.0),
                temporal_position: 1.0,
                affected_elements: vec!["semantic_content".to_string()],
                change_magnitude: semantic_result.ambiguity(),
            };
            incidents.push(incident);
        }
        
        // Detect drift based on delta changes
        if semantic_result.delta() < 0.5 {
            let incident = DriftIncident {
                id: "semantic_density_drift".to_string(),
                drift_type: DriftType::Simplification,
                severity: 1.0 - semantic_result.delta(),
                temporal_position: 1.0,
                affected_elements: vec!["semantic_density".to_string()],
                change_magnitude: 1.0 - semantic_result.delta(),
            };
            incidents.push(incident);
        }
        
        Ok(incidents)
    }

    /// Detect structural drift in document organization
    fn detect_structural_drift(&mut self, document: &AispDocument) -> AispResult<Vec<DriftIncident>> {
        let mut incidents = Vec::new();
        
        // Analyze block structure stability
        let expected_blocks = 5; // Meta, Types, Rules, Functions, Evidence
        let actual_blocks = document.blocks.len();
        
        if actual_blocks < expected_blocks {
            let incident = DriftIncident {
                id: "structural_completeness_drift".to_string(),
                drift_type: DriftType::Simplification,
                severity: (expected_blocks - actual_blocks) as f64 / expected_blocks as f64,
                temporal_position: 1.0,
                affected_elements: vec!["document_structure".to_string()],
                change_magnitude: (expected_blocks - actual_blocks) as f64,
            };
            incidents.push(incident);
        }
        
        // Check for block type diversity
        let mut block_types = std::collections::HashSet::new();
        for block in &document.blocks {
            block_types.insert(block.block_type());
        }
        
        if block_types.len() < 3 {
            let incident = DriftIncident {
                id: "structural_diversity_drift".to_string(),
                drift_type: DriftType::Simplification,
                severity: (5 - block_types.len()) as f64 / 5.0,
                temporal_position: 1.0,
                affected_elements: vec!["block_diversity".to_string()],
                change_magnitude: (5 - block_types.len()) as f64,
            };
            incidents.push(incident);
        }
        
        Ok(incidents)
    }

    /// Detect behavioral drift in functions and rules
    fn detect_behavioral_drift(&mut self, document: &AispDocument) -> AispResult<Vec<DriftIncident>> {
        let mut incidents = Vec::new();
        
        // Analyze function complexity drift
        for block in &document.blocks {
            match block {
                AispBlock::Functions(functions_block) => {
                    let mut complexity_sum = 0.0;
                    let mut function_count = 0;
                    
                    for (i, function) in functions_block.functions.iter().enumerate() {
                        let complexity = self.calculate_function_complexity(function);
                        complexity_sum += complexity;
                        function_count += 1;
                        
                        // Detect overly complex or overly simple functions
                        if complexity > 0.8 {
                            let incident = DriftIncident {
                                id: format!("function_complexity_drift_{}", i),
                                drift_type: DriftType::Complexification,
                                severity: complexity - 0.8,
                                temporal_position: 1.0,
                                affected_elements: vec![function.name.clone()],
                                change_magnitude: complexity,
                            };
                            incidents.push(incident);
                        } else if complexity < 0.2 {
                            let incident = DriftIncident {
                                id: format!("function_simplicity_drift_{}", i),
                                drift_type: DriftType::Simplification,
                                severity: 0.2 - complexity,
                                temporal_position: 1.0,
                                affected_elements: vec![function.name.clone()],
                                change_magnitude: complexity,
                            };
                            incidents.push(incident);
                        }
                    }
                }
                AispBlock::Rules(rules_block) => {
                    // Analyze rule complexity distribution
                    if rules_block.rules.is_empty() {
                        let incident = DriftIncident {
                            id: "rules_absence_drift".to_string(),
                            drift_type: DriftType::Simplification,
                            severity: 0.7,
                            temporal_position: 1.0,
                            affected_elements: vec!["logical_rules".to_string()],
                            change_magnitude: 1.0,
                        };
                        incidents.push(incident);
                    }
                }
                _ => continue,
            }
        }
        
        Ok(incidents)
    }

    /// Calculate function complexity for drift analysis
    fn calculate_function_complexity(&self, function: &FunctionDefinition) -> f64 {
        let param_complexity = function.lambda.parameters.len() as f64 * 0.1;
        let body_complexity = self.calculate_expression_complexity(&function.lambda.body);
        
        (param_complexity + body_complexity).min(1.0)
    }

    /// Calculate expression complexity
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
                0.25 + (self.calculate_expression_complexity(element) + self.calculate_expression_complexity(set)) / 2.0
            }
            LogicalExpression::Temporal { operand, .. } => {
                0.5 + self.calculate_expression_complexity(operand)
            }
            LogicalExpression::Raw(_) => {
                0.2 // Simple complexity for raw expressions
            }
        }
    }

    /// Analyze drift trends from detected incidents
    fn analyze_drift_trends(&self, incidents: &[DriftIncident]) -> DriftTrends {
        if incidents.is_empty() {
            return DriftTrends {
                drift_velocity: 0.0,
                drift_acceleration: 0.0,
                periodicity: None,
                dominant_direction: DriftDirection::Random,
            };
        }
        
        // Calculate drift velocity (change over time)
        let total_magnitude: f64 = incidents.iter().map(|i| i.change_magnitude).sum();
        let time_span = incidents.iter().map(|i| i.temporal_position).fold(0.0, f64::max);
        let drift_velocity = if time_span > 0.0 { total_magnitude / time_span } else { 0.0 };
        
        // Approximate drift acceleration (simplified)
        let drift_acceleration = if incidents.len() > 1 {
            let recent_magnitude: f64 = incidents.iter().rev().take(incidents.len() / 2).map(|i| i.change_magnitude).sum();
            let earlier_magnitude: f64 = incidents.iter().take(incidents.len() / 2).map(|i| i.change_magnitude).sum();
            recent_magnitude - earlier_magnitude
        } else {
            0.0
        };
        
        // Determine dominant direction
        let complexification_count = incidents.iter().filter(|i| i.drift_type == DriftType::GradualShift || i.drift_type == DriftType::AbruptChange).count();
        let simplification_count = incidents.iter().filter(|i| i.drift_type == DriftType::Simplification).count();
        
        let dominant_direction = if complexification_count > simplification_count {
            DriftDirection::Complexification
        } else if simplification_count > complexification_count {
            DriftDirection::Simplification
        } else {
            DriftDirection::Random
        };
        
        DriftTrends {
            drift_velocity,
            drift_acceleration,
            periodicity: None, // Would require more temporal data to detect
            dominant_direction,
        }
    }

    /// Classify drift patterns
    fn classify_drift_patterns(&self, incidents: &[DriftIncident]) -> DriftClassification {
        let gradual_shifts = incidents.iter().filter(|i| i.drift_type == DriftType::GradualShift).count();
        let abrupt_changes = incidents.iter().filter(|i| i.drift_type == DriftType::AbruptChange).count();
        
        let max_severity = incidents.iter().map(|i| i.severity).fold(0.0, f64::max);
        let average_severity = if incidents.is_empty() {
            0.0
        } else {
            incidents.iter().map(|i| i.severity).sum::<f64>() / incidents.len() as f64
        };
        
        let drift_frequency = incidents.len() as f64; // Simplified frequency calculation
        
        DriftClassification {
            gradual_shifts,
            abrupt_changes,
            max_severity,
            average_severity,
            drift_frequency,
        }
    }

    /// Calculate stability metrics
    fn calculate_stability_metrics(&mut self, _document: &AispDocument, semantic_result: &DeepVerificationResult, drift_patterns: &DriftPatterns) -> AispResult<StabilityMetrics> {
        self.stats.stability_calculations += 1;
        
        // Semantic consistency (inverse of ambiguity)
        let semantic_consistency = 1.0 - semantic_result.ambiguity().min(1.0);
        
        // Behavioral predictability (inverse of drift frequency)
        let behavioral_predictability = if drift_patterns.classification.drift_frequency > 0.0 {
            1.0 / (1.0 + drift_patterns.classification.drift_frequency)
        } else {
            1.0
        };
        
        // Meaning preservation (based on delta)
        let meaning_preservation = semantic_result.delta().min(1.0);
        
        // Temporal stability (inverse of drift velocity)
        let temporal_stability = if drift_patterns.trends.drift_velocity > 0.0 {
            1.0 / (1.0 + drift_patterns.trends.drift_velocity)
        } else {
            1.0
        };
        
        // Baseline deviation (simplified calculation)
        let baseline_deviation = drift_patterns.classification.average_severity;
        
        Ok(StabilityMetrics {
            semantic_consistency,
            behavioral_predictability,
            meaning_preservation,
            temporal_stability,
            baseline_deviation,
        })
    }

    /// Evaluate correction protocols
    fn evaluate_correction_protocols(&mut self, drift_patterns: &DriftPatterns) -> AispResult<CorrectionProtocols> {
        self.stats.correction_evaluations += 1;
        
        // Determine available correction methods
        let mut applicable_methods = Vec::new();
        let mut total_effectiveness = 0.0;
        let mut method_count = 0;
        
        for incident in &drift_patterns.incidents {
            for method in self.correction_registry.values() {
                if method.applicable_drift_types.contains(&incident.drift_type) {
                    applicable_methods.push(method.clone());
                    total_effectiveness += method.effectiveness;
                    method_count += 1;
                }
            }
        }
        
        // Calculate correction success rate
        let correction_success_rate = if method_count > 0 {
            total_effectiveness / method_count as f64
        } else {
            0.0
        };
        
        // Remove duplicates from applicable methods
        applicable_methods.sort_by(|a, b| a.id.cmp(&b.id));
        applicable_methods.dedup_by(|a, b| a.id == b.id);
        
        Ok(CorrectionProtocols {
            auto_correction_enabled: self.config.enable_auto_correction,
            manual_correction_available: true, // Always available
            correction_success_rate,
            average_correction_time: Duration::from_secs(30), // Estimated
            correction_methods: applicable_methods,
        })
    }

    /// Calculate overall drift resistance score
    fn calculate_drift_resistance_score(&self, drift_patterns: &DriftPatterns, stability_metrics: &StabilityMetrics, correction_protocols: &CorrectionProtocols) -> AispResult<f64> {
        // Weight different aspects
        let stability_weight = 0.5;
        let drift_resistance_weight = 0.3;
        let correction_weight = 0.2;
        
        // Calculate stability score
        let stability_score = (stability_metrics.semantic_consistency + 
                              stability_metrics.behavioral_predictability + 
                              stability_metrics.meaning_preservation + 
                              stability_metrics.temporal_stability) / 4.0;
        
        // Calculate drift resistance (inverse of severity)
        let drift_resistance = if drift_patterns.classification.max_severity > 0.0 {
            1.0 - drift_patterns.classification.average_severity
        } else {
            1.0
        };
        
        // Use correction success rate
        let correction_score = correction_protocols.correction_success_rate;
        
        // Weighted combination
        let overall_score = (stability_score * stability_weight) +
                           (drift_resistance * drift_resistance_weight) +
                           (correction_score * correction_weight);
        
        Ok(overall_score.min(1.0))
    }

    /// Check if validation is successful
    fn is_validation_successful(&self, stability_metrics: &StabilityMetrics, drift_patterns: &DriftPatterns) -> bool {
        let stability_score = (stability_metrics.semantic_consistency + 
                              stability_metrics.behavioral_predictability + 
                              stability_metrics.meaning_preservation + 
                              stability_metrics.temporal_stability) / 4.0;
        
        stability_score >= self.config.min_stability_score &&
        drift_patterns.trends.drift_velocity <= self.config.max_drift_velocity &&
        drift_patterns.classification.max_severity <= self.config.severity_threshold
    }

    /// Generate analysis warnings
    fn generate_warnings(&self, drift_patterns: &DriftPatterns, stability_metrics: &StabilityMetrics) -> Vec<String> {
        let mut warnings = Vec::new();
        
        if drift_patterns.trends.drift_velocity > self.config.max_drift_velocity {
            warnings.push(format!(
                "Drift velocity {:.4} exceeds maximum threshold {:.4}",
                drift_patterns.trends.drift_velocity,
                self.config.max_drift_velocity
            ));
        }
        
        if drift_patterns.classification.max_severity > self.config.severity_threshold {
            warnings.push(format!(
                "Maximum drift severity {:.3} exceeds threshold {:.3}",
                drift_patterns.classification.max_severity,
                self.config.severity_threshold
            ));
        }
        
        if stability_metrics.semantic_consistency < self.config.min_stability_score {
            warnings.push(format!(
                "Semantic consistency {:.3} is below minimum stability score {:.3}",
                stability_metrics.semantic_consistency,
                self.config.min_stability_score
            ));
        }
        
        if drift_patterns.classification.abrupt_changes > 0 {
            warnings.push(format!(
                "{} abrupt semantic changes detected - may indicate instability",
                drift_patterns.classification.abrupt_changes
            ));
        }
        
        if self.stats.analysis_time > self.config.max_analysis_time {
            warnings.push("Anti-drift analysis exceeded maximum time limit".to_string());
        }
        
        warnings
    }

    /// Get validation statistics
    pub fn get_stats(&self) -> &AntiDriftStats {
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
                name: "anti_drift_test".to_string(),
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

    fn create_stable_semantic_result() -> DeepVerificationResult {
        DeepVerificationResult {
            delta: 0.9,
            ambiguity: 0.01,
            completeness: 0.95,
            tier: QualityTier::Platinum,
            quality_score: 0.95,
            validation_errors: vec![],
            warnings: vec![],
        }
    }

    fn create_drifted_semantic_result() -> DeepVerificationResult {
        DeepVerificationResult {
            delta: 0.3,
            ambiguity: 0.15,
            completeness: 0.6,
            tier: QualityTier::Bronze,
            quality_score: 0.4,
            validation_errors: vec!["semantic_drift_detected".to_string()],
            warnings: vec!["high_ambiguity".to_string()],
        }
    }

    #[test]
    fn test_anti_drift_validator_creation() {
        let config = AntiDriftConfig::default();
        let validator = AntiDriftValidator::new(config);
        
        assert_eq!(validator.stats.drift_checks_performed, 0);
        assert_eq!(validator.stats.stability_calculations, 0);
        assert_eq!(validator.correction_registry.len(), 3); // 3 default correction methods
    }

    #[test]
    fn test_stable_document_validation() {
        let config = AntiDriftConfig::default();
        let mut validator = AntiDriftValidator::new(config);
        let document = create_test_document();
        let semantic_result = create_stable_semantic_result();
        
        let result = validator.validate_anti_drift(&document, &semantic_result).unwrap();
        
        assert!(result.valid);
        assert!(result.drift_resistance_score > 0.8);
        assert!(result.stability_metrics.semantic_consistency > 0.9);
    }

    #[test]
    fn test_drifted_document_validation() {
        let config = AntiDriftConfig::default();
        let mut validator = AntiDriftValidator::new(config);
        let document = create_test_document();
        let semantic_result = create_drifted_semantic_result();
        
        let result = validator.validate_anti_drift(&document, &semantic_result).unwrap();
        
        assert!(!result.valid); // Should fail validation due to drift
        assert!(!result.warnings.is_empty()); // Should have warnings
        assert!(result.drift_patterns.incidents.len() > 0); // Should detect drift incidents
    }

    #[test]
    fn test_drift_pattern_classification() {
        let config = AntiDriftConfig::default();
        let validator = AntiDriftValidator::new(config);
        
        let incidents = vec![
            DriftIncident {
                id: "drift1".to_string(),
                drift_type: DriftType::GradualShift,
                severity: 0.3,
                temporal_position: 1.0,
                affected_elements: vec!["element1".to_string()],
                change_magnitude: 0.3,
            },
            DriftIncident {
                id: "drift2".to_string(),
                drift_type: DriftType::AbruptChange,
                severity: 0.8,
                temporal_position: 2.0,
                affected_elements: vec!["element2".to_string()],
                change_magnitude: 0.8,
            },
        ];
        
        let classification = validator.classify_drift_patterns(&incidents);
        
        assert_eq!(classification.gradual_shifts, 1);
        assert_eq!(classification.abrupt_changes, 1);
        assert_eq!(classification.max_severity, 0.8);
        assert!((classification.average_severity - 0.55).abs() < 0.01);
    }

    #[test]
    fn test_drift_trends_analysis() {
        let config = AntiDriftConfig::default();
        let validator = AntiDriftValidator::new(config);
        
        let incidents = vec![
            DriftIncident {
                id: "drift1".to_string(),
                drift_type: DriftType::Complexification,
                severity: 0.2,
                temporal_position: 1.0,
                affected_elements: vec!["element1".to_string()],
                change_magnitude: 0.2,
            },
            DriftIncident {
                id: "drift2".to_string(),
                drift_type: DriftType::Complexification,
                severity: 0.4,
                temporal_position: 2.0,
                affected_elements: vec!["element2".to_string()],
                change_magnitude: 0.4,
            },
        ];
        
        let trends = validator.analyze_drift_trends(&incidents);
        
        assert_eq!(trends.dominant_direction, DriftDirection::Complexification);
        assert!(trends.drift_velocity > 0.0);
    }

    #[test]
    fn test_stability_metrics_calculation() {
        let config = AntiDriftConfig::default();
        let mut validator = AntiDriftValidator::new(config);
        let document = create_test_document();
        let semantic_result = create_stable_semantic_result();
        
        let drift_patterns = DriftPatterns {
            incidents: vec![],
            trends: DriftTrends {
                drift_velocity: 0.0,
                drift_acceleration: 0.0,
                periodicity: None,
                dominant_direction: DriftDirection::Random,
            },
            classification: DriftClassification {
                gradual_shifts: 0,
                abrupt_changes: 0,
                max_severity: 0.0,
                average_severity: 0.0,
                drift_frequency: 0.0,
            },
        };
        
        let metrics = validator.calculate_stability_metrics(&document, &semantic_result, &drift_patterns).unwrap();
        
        assert!(metrics.semantic_consistency > 0.9);
        assert_eq!(metrics.behavioral_predictability, 1.0); // No drift = perfect predictability
        assert!(metrics.meaning_preservation > 0.8);
        assert_eq!(metrics.temporal_stability, 1.0); // No drift velocity = perfect stability
    }

    #[test]
    fn test_correction_protocol_evaluation() {
        let config = AntiDriftConfig {
            enable_auto_correction: true,
            ..Default::default()
        };
        let mut validator = AntiDriftValidator::new(config);
        
        let drift_patterns = DriftPatterns {
            incidents: vec![
                DriftIncident {
                    id: "bias_drift".to_string(),
                    drift_type: DriftType::BiasIntroduction,
                    severity: 0.5,
                    temporal_position: 1.0,
                    affected_elements: vec!["bias_element".to_string()],
                    change_magnitude: 0.5,
                },
            ],
            trends: DriftTrends {
                drift_velocity: 0.1,
                drift_acceleration: 0.0,
                periodicity: None,
                dominant_direction: DriftDirection::Random,
            },
            classification: DriftClassification {
                gradual_shifts: 0,
                abrupt_changes: 0,
                max_severity: 0.5,
                average_severity: 0.5,
                drift_frequency: 1.0,
            },
        };
        
        let protocols = validator.evaluate_correction_protocols(&drift_patterns).unwrap();
        
        assert!(protocols.auto_correction_enabled);
        assert!(protocols.manual_correction_available);
        assert!(protocols.correction_success_rate > 0.0);
        assert!(!protocols.correction_methods.is_empty());
    }
}