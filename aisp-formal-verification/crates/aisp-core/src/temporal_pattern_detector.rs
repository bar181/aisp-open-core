//! Temporal pattern detection for AISP documents
//!
//! This module identifies and analyzes common temporal logic patterns
//! such as safety, liveness, response, persistence, and fairness patterns.

use crate::ast::*;
use crate::error::*;
use crate::temporal_operator_analyzer::{TemporalOperator, OperatorInstance};
use std::collections::HashMap;

/// Temporal pattern detector
pub struct TemporalPatternDetector {
    /// Pattern matching rules
    pattern_rules: Vec<PatternRule>,
    /// Detected patterns
    detected_patterns: Vec<TemporalPattern>,
}

/// Temporal pattern recognition result
#[derive(Debug, Clone)]
pub struct TemporalPattern {
    /// Type of pattern
    pub pattern_type: PatternType,
    /// Human-readable description
    pub description: String,
    /// Pattern instances found
    pub instances: Vec<PatternInstance>,
    /// Confidence level (0.0-1.0)
    pub confidence: f64,
    /// Pattern strength metrics
    pub strength: PatternStrength,
}

/// Types of temporal patterns
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternType {
    /// Safety pattern (□P) - "something bad never happens"
    Safety,
    /// Liveness pattern (◊P) - "something good eventually happens"
    Liveness,
    /// Response pattern (□(P → ◊Q)) - "if P then eventually Q"
    Response,
    /// Persistence pattern (◊□P) - "eventually P holds forever"
    Persistence,
    /// Recurrence pattern (□◊P) - "P happens infinitely often"
    Recurrence,
    /// Fairness pattern - "fair scheduling or resource access"
    Fairness,
    /// Precedence pattern (¬Q U P) - "Q cannot happen before P"
    Precedence,
    /// Absence pattern (□¬P) - "P never happens"
    Absence,
    /// Existence pattern (◊P) - "P happens at least once"
    Existence,
    /// Chain pattern (□(P → X Q)) - "P is always followed by Q in next step"
    Chain,
}

/// Pattern instance found in the document
#[derive(Debug, Clone)]
pub struct PatternInstance {
    /// Formula implementing this pattern
    pub formula: String,
    /// Variables involved in the pattern
    pub variables: Vec<String>,
    /// Location in the document
    pub location: Span,
    /// Pattern strength (0.0-1.0)
    pub strength: f64,
    /// Context where pattern appears
    pub context: String,
    /// Pattern quality assessment
    pub quality: PatternQuality,
}

/// Pattern strength metrics
#[derive(Debug, Clone)]
pub struct PatternStrength {
    /// Syntactic strength (how well it matches the pattern)
    pub syntactic: f64,
    /// Semantic strength (how meaningful the pattern is)
    pub semantic: f64,
    /// Coverage strength (how much of the system it covers)
    pub coverage: f64,
    /// Overall strength score
    pub overall: f64,
}

/// Pattern quality assessment
#[derive(Debug, Clone, PartialEq)]
pub enum PatternQuality {
    /// High quality - well-formed and meaningful
    High,
    /// Medium quality - acceptable with minor issues
    Medium,
    /// Low quality - problematic or unclear
    Low,
    /// Very low quality - likely incorrect
    VeryLow,
}

/// Pattern matching rule
#[derive(Debug, Clone)]
pub struct PatternRule {
    /// Pattern type this rule detects
    pub pattern_type: PatternType,
    /// Required operator sequence
    pub operator_sequence: Vec<TemporalOperator>,
    /// Minimum confidence threshold
    pub min_confidence: f64,
    /// Pattern description template
    pub description_template: String,
}

/// Pattern analysis result
#[derive(Debug, Clone)]
pub struct PatternAnalysisResult {
    /// All detected patterns
    pub patterns: Vec<TemporalPattern>,
    /// Pattern statistics
    pub statistics: PatternStatistics,
    /// Pattern quality assessment
    pub quality_summary: QualitySummary,
    /// Recommendations for improvements
    pub recommendations: Vec<PatternRecommendation>,
    /// Analysis warnings
    pub warnings: Vec<AispWarning>,
}

/// Pattern statistics
#[derive(Debug, Clone)]
pub struct PatternStatistics {
    /// Total patterns found
    pub total_patterns: usize,
    /// Patterns by type
    pub patterns_by_type: HashMap<PatternType, usize>,
    /// Average pattern strength
    pub avg_strength: f64,
    /// Pattern density (patterns per 100 LOC)
    pub pattern_density: f64,
    /// Coverage metrics
    pub coverage: CoverageMetrics,
}

/// Coverage metrics for patterns
#[derive(Debug, Clone)]
pub struct CoverageMetrics {
    /// Safety coverage (% of system covered by safety patterns)
    pub safety_coverage: f64,
    /// Liveness coverage (% of system covered by liveness patterns)
    pub liveness_coverage: f64,
    /// Overall temporal coverage
    pub overall_coverage: f64,
}

/// Quality summary of patterns
#[derive(Debug, Clone)]
pub struct QualitySummary {
    /// High quality patterns count
    pub high_quality_count: usize,
    /// Medium quality patterns count
    pub medium_quality_count: usize,
    /// Low quality patterns count
    pub low_quality_count: usize,
    /// Overall quality score (0.0-1.0)
    pub overall_quality_score: f64,
}

/// Pattern improvement recommendation
#[derive(Debug, Clone)]
pub struct PatternRecommendation {
    /// Type of recommendation
    pub recommendation_type: RecommendationType,
    /// Descriptive message
    pub message: String,
    /// Priority level
    pub priority: RecommendationPriority,
    /// Affected pattern (if specific)
    pub affected_pattern: Option<String>,
}

/// Types of pattern recommendations
#[derive(Debug, Clone, PartialEq)]
pub enum RecommendationType {
    /// Add missing safety patterns
    AddSafety,
    /// Add missing liveness patterns
    AddLiveness,
    /// Simplify complex patterns
    Simplify,
    /// Improve pattern clarity
    ImproveClarity,
    /// Fix pattern conflicts
    ResolveConflicts,
    /// Enhance coverage
    EnhanceCoverage,
}

/// Recommendation priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecommendationPriority {
    /// Critical - should be addressed immediately
    Critical,
    /// High priority
    High,
    /// Medium priority
    Medium,
    /// Low priority - nice to have
    Low,
}

impl TemporalPatternDetector {
    /// Create a new temporal pattern detector
    pub fn new() -> Self {
        let pattern_rules = Self::create_pattern_rules();
        
        Self {
            pattern_rules,
            detected_patterns: Vec::new(),
        }
    }

    /// Detect temporal patterns from operator instances
    pub fn detect_patterns(
        &mut self,
        operators: &[OperatorInstance],
        document_size: usize,
    ) -> PatternAnalysisResult {
        self.detected_patterns.clear();

        let mut warnings = Vec::new();

        // Detect each pattern type
        for rule in &self.pattern_rules.clone() {
            let patterns = self.detect_pattern_type(operators, rule);
            self.detected_patterns.extend(patterns);
        }

        // Calculate statistics
        let statistics = self.calculate_statistics(document_size);

        // Assess pattern quality
        let quality_summary = self.assess_pattern_quality();

        // Generate recommendations
        let recommendations = self.generate_recommendations(&statistics, &quality_summary);

        // Generate warnings
        self.generate_warnings(&statistics, &mut warnings);

        PatternAnalysisResult {
            patterns: self.detected_patterns.clone(),
            statistics,
            quality_summary,
            recommendations,
            warnings,
        }
    }

    /// Create built-in pattern matching rules
    fn create_pattern_rules() -> Vec<PatternRule> {
        vec![
            // Safety pattern: □P
            PatternRule {
                pattern_type: PatternType::Safety,
                operator_sequence: vec![TemporalOperator::Always],
                min_confidence: 0.8,
                description_template: "Safety property: {formula} must always hold".to_string(),
            },
            
            // Liveness pattern: ◊P
            PatternRule {
                pattern_type: PatternType::Liveness,
                operator_sequence: vec![TemporalOperator::Eventually],
                min_confidence: 0.8,
                description_template: "Liveness property: {formula} must eventually occur".to_string(),
            },
            
            // Response pattern: □(P → ◊Q) - approximated as □...◊
            PatternRule {
                pattern_type: PatternType::Response,
                operator_sequence: vec![TemporalOperator::Always, TemporalOperator::Eventually],
                min_confidence: 0.7,
                description_template: "Response property: if {p} then eventually {q}".to_string(),
            },
            
            // Persistence pattern: ◊□P
            PatternRule {
                pattern_type: PatternType::Persistence,
                operator_sequence: vec![TemporalOperator::Eventually, TemporalOperator::Always],
                min_confidence: 0.7,
                description_template: "Persistence property: {formula} eventually holds forever".to_string(),
            },
            
            // Recurrence pattern: □◊P  
            PatternRule {
                pattern_type: PatternType::Recurrence,
                operator_sequence: vec![TemporalOperator::Always, TemporalOperator::Eventually],
                min_confidence: 0.6,
                description_template: "Recurrence property: {formula} occurs infinitely often".to_string(),
            },
            
            // Chain pattern: □(P → XQ) - approximated as □...X
            PatternRule {
                pattern_type: PatternType::Chain,
                operator_sequence: vec![TemporalOperator::Always, TemporalOperator::Next],
                min_confidence: 0.7,
                description_template: "Chain property: {p} is always followed by {q}".to_string(),
            },
        ]
    }

    /// Detect patterns of a specific type
    fn detect_pattern_type(
        &self,
        operators: &[OperatorInstance],
        rule: &PatternRule,
    ) -> Vec<TemporalPattern> {
        let mut patterns = Vec::new();
        let seq_len = rule.operator_sequence.len();
        
        if seq_len == 0 || operators.len() < seq_len {
            return patterns;
        }

        // Look for operator sequences matching the pattern
        for i in 0..=operators.len().saturating_sub(seq_len) {
            let window = &operators[i..i + seq_len];
            
            if self.sequence_matches(window, &rule.operator_sequence) {
                let pattern = self.create_pattern_from_sequence(window, rule);
                if pattern.confidence >= rule.min_confidence {
                    patterns.push(pattern);
                }
            }
        }

        patterns
    }

    /// Check if operator sequence matches pattern rule
    fn sequence_matches(
        &self,
        operators: &[OperatorInstance],
        expected: &[TemporalOperator],
    ) -> bool {
        if operators.len() != expected.len() {
            return false;
        }

        for (op_instance, expected_op) in operators.iter().zip(expected.iter()) {
            if op_instance.operator != *expected_op {
                return false;
            }
        }

        true
    }

    /// Create pattern from matched operator sequence
    fn create_pattern_from_sequence(
        &self,
        operators: &[OperatorInstance],
        rule: &PatternRule,
    ) -> TemporalPattern {
        let mut variables = Vec::new();
        let mut formula_parts = Vec::new();

        for op in operators {
            variables.extend(op.operands.clone());
            formula_parts.push(format!("{}", op.operator));
        }

        // Remove duplicates from variables
        variables.sort();
        variables.dedup();

        let formula = formula_parts.join(" ");
        let strength = self.calculate_pattern_strength(operators);
        let quality = self.assess_instance_quality(&strength, &variables);

        let instance = PatternInstance {
            formula: formula.clone(),
            variables: variables.clone(),
            location: operators[0].location.clone(),
            strength: strength.overall,
            context: format!("{:?}", operators[0].context),
            quality,
        };

        let description = rule.description_template
            .replace("{formula}", &formula)
            .replace("{p}", variables.get(0).unwrap_or(&"P".to_string()))
            .replace("{q}", variables.get(1).unwrap_or(&"Q".to_string()));

        TemporalPattern {
            pattern_type: rule.pattern_type.clone(),
            description,
            instances: vec![instance],
            confidence: strength.overall,
            strength,
        }
    }

    /// Calculate pattern strength metrics
    fn calculate_pattern_strength(&self, operators: &[OperatorInstance]) -> PatternStrength {
        // Syntactic strength - how well-formed the pattern is
        let syntactic = if operators.len() <= 2 { 0.9 } else { 0.7 };

        // Semantic strength - how meaningful the variables are
        let total_operands: usize = operators.iter().map(|op| op.operands.len()).sum();
        let semantic = if total_operands > 0 { 0.8 } else { 0.5 };

        // Coverage strength - nesting and complexity
        let max_nesting = operators.iter().map(|op| op.nesting_level).max().unwrap_or(0);
        let coverage = if max_nesting <= 2 { 0.9 } else { 0.6 };

        let overall = (syntactic + semantic + coverage) / 3.0;

        PatternStrength {
            syntactic,
            semantic,
            coverage,
            overall,
        }
    }

    /// Assess quality of a pattern instance
    fn assess_instance_quality(&self, strength: &PatternStrength, variables: &[String]) -> PatternQuality {
        let score = strength.overall;
        let has_meaningful_vars = !variables.is_empty() && 
            variables.iter().any(|v| v.len() > 1 && !v.chars().all(|c| c.is_ascii_punctuation()));

        match (score, has_meaningful_vars) {
            (s, true) if s >= 0.8 => PatternQuality::High,
            (s, _) if s >= 0.7 => PatternQuality::Medium,
            (s, _) if s >= 0.5 => PatternQuality::Low,
            _ => PatternQuality::VeryLow,
        }
    }

    /// Calculate pattern statistics
    fn calculate_statistics(&self, document_size: usize) -> PatternStatistics {
        let total_patterns = self.detected_patterns.len();
        
        let mut patterns_by_type = HashMap::new();
        let mut total_strength = 0.0;

        for pattern in &self.detected_patterns {
            *patterns_by_type.entry(pattern.pattern_type.clone()).or_insert(0) += 1;
            total_strength += pattern.strength.overall;
        }

        let avg_strength = if total_patterns > 0 {
            total_strength / total_patterns as f64
        } else {
            0.0
        };

        let pattern_density = if document_size > 0 {
            (total_patterns as f64 / document_size as f64) * 100.0
        } else {
            0.0
        };

        let coverage = self.calculate_coverage_metrics();

        PatternStatistics {
            total_patterns,
            patterns_by_type,
            avg_strength,
            pattern_density,
            coverage,
        }
    }

    /// Calculate coverage metrics
    fn calculate_coverage_metrics(&self) -> CoverageMetrics {
        let total_patterns = self.detected_patterns.len() as f64;
        
        if total_patterns == 0.0 {
            return CoverageMetrics {
                safety_coverage: 0.0,
                liveness_coverage: 0.0,
                overall_coverage: 0.0,
            };
        }

        let safety_count = self.detected_patterns
            .iter()
            .filter(|p| matches!(p.pattern_type, PatternType::Safety | PatternType::Absence))
            .count() as f64;

        let liveness_count = self.detected_patterns
            .iter()
            .filter(|p| matches!(p.pattern_type, PatternType::Liveness | PatternType::Response | PatternType::Persistence))
            .count() as f64;

        CoverageMetrics {
            safety_coverage: (safety_count / total_patterns) * 100.0,
            liveness_coverage: (liveness_count / total_patterns) * 100.0,
            overall_coverage: ((safety_count + liveness_count) / total_patterns) * 100.0,
        }
    }

    /// Assess overall pattern quality
    fn assess_pattern_quality(&self) -> QualitySummary {
        let mut high_quality_count = 0;
        let mut medium_quality_count = 0;
        let mut low_quality_count = 0;

        for pattern in &self.detected_patterns {
            for instance in &pattern.instances {
                match instance.quality {
                    PatternQuality::High => high_quality_count += 1,
                    PatternQuality::Medium => medium_quality_count += 1,
                    PatternQuality::Low | PatternQuality::VeryLow => low_quality_count += 1,
                }
            }
        }

        let total = high_quality_count + medium_quality_count + low_quality_count;
        let overall_quality_score = if total > 0 {
            (high_quality_count as f64 * 1.0 + medium_quality_count as f64 * 0.6) / total as f64
        } else {
            0.0
        };

        QualitySummary {
            high_quality_count,
            medium_quality_count,
            low_quality_count,
            overall_quality_score,
        }
    }

    /// Generate pattern recommendations
    fn generate_recommendations(
        &self,
        statistics: &PatternStatistics,
        quality: &QualitySummary,
    ) -> Vec<PatternRecommendation> {
        let mut recommendations = Vec::new();

        // Check for missing safety patterns
        let safety_count = statistics.patterns_by_type.get(&PatternType::Safety).unwrap_or(&0);
        if *safety_count == 0 && statistics.total_patterns > 0 {
            recommendations.push(PatternRecommendation {
                recommendation_type: RecommendationType::AddSafety,
                message: "Consider adding safety patterns (□P) to specify invariant properties".to_string(),
                priority: RecommendationPriority::High,
                affected_pattern: None,
            });
        }

        // Check for missing liveness patterns
        let liveness_count = statistics.patterns_by_type.get(&PatternType::Liveness).unwrap_or(&0);
        if *liveness_count == 0 && statistics.total_patterns > 0 {
            recommendations.push(PatternRecommendation {
                recommendation_type: RecommendationType::AddLiveness,
                message: "Consider adding liveness patterns (◊P) to specify progress properties".to_string(),
                priority: RecommendationPriority::High,
                affected_pattern: None,
            });
        }

        // Check pattern quality
        if quality.overall_quality_score < 0.6 {
            recommendations.push(PatternRecommendation {
                recommendation_type: RecommendationType::ImproveClarity,
                message: "Consider improving pattern clarity with more meaningful variable names".to_string(),
                priority: RecommendationPriority::Medium,
                affected_pattern: None,
            });
        }

        // Check coverage
        if statistics.coverage.overall_coverage < 50.0 {
            recommendations.push(PatternRecommendation {
                recommendation_type: RecommendationType::EnhanceCoverage,
                message: "Consider adding more temporal patterns to improve system coverage".to_string(),
                priority: RecommendationPriority::Medium,
                affected_pattern: None,
            });
        }

        recommendations
    }

    /// Generate analysis warnings
    fn generate_warnings(&self, statistics: &PatternStatistics, warnings: &mut Vec<AispWarning>) {
        // Warn about pattern imbalance
        let safety_count = statistics.patterns_by_type.get(&PatternType::Safety).unwrap_or(&0);
        let liveness_count = statistics.patterns_by_type.get(&PatternType::Liveness).unwrap_or(&0);

        if *safety_count > 0 && *liveness_count == 0 {
            warnings.push(AispWarning::warning(
                "Only safety patterns detected - consider adding liveness properties".to_string(),
            ));
        }

        if *liveness_count > 0 && *safety_count == 0 {
            warnings.push(AispWarning::warning(
                "Only liveness patterns detected - consider adding safety properties".to_string(),
            ));
        }

        // Warn about low pattern density
        if statistics.pattern_density < 0.5 {
            warnings.push(AispWarning::warning(
                "Low temporal pattern density - document may lack temporal specifications".to_string(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::temporal_operator_analyzer::OperatorContext;

    fn create_test_operator(op: TemporalOperator, operands: Vec<String>) -> OperatorInstance {
        OperatorInstance {
            operator: op,
            location: Span::new(1, 1, 1, 10),
            context: OperatorContext::Rule("test".to_string()),
            operands,
            nesting_level: 1,
        }
    }

    #[test]
    fn test_pattern_detector_creation() {
        let detector = TemporalPatternDetector::new();
        assert!(!detector.pattern_rules.is_empty());
        assert!(detector.pattern_rules.iter().any(|r| r.pattern_type == PatternType::Safety));
        assert!(detector.pattern_rules.iter().any(|r| r.pattern_type == PatternType::Liveness));
    }

    #[test]
    fn test_safety_pattern_detection() {
        let mut detector = TemporalPatternDetector::new();
        
        let operators = vec![
            create_test_operator(TemporalOperator::Always, vec!["P".to_string()]),
        ];

        let result = detector.detect_patterns(&operators, 100);
        assert!(result.patterns.iter().any(|p| p.pattern_type == PatternType::Safety));
    }

    #[test]
    fn test_liveness_pattern_detection() {
        let mut detector = TemporalPatternDetector::new();
        
        let operators = vec![
            create_test_operator(TemporalOperator::Eventually, vec!["Q".to_string()]),
        ];

        let result = detector.detect_patterns(&operators, 100);
        assert!(result.patterns.iter().any(|p| p.pattern_type == PatternType::Liveness));
    }

    #[test]
    fn test_response_pattern_detection() {
        let mut detector = TemporalPatternDetector::new();
        
        let operators = vec![
            create_test_operator(TemporalOperator::Always, vec!["P".to_string()]),
            create_test_operator(TemporalOperator::Eventually, vec!["Q".to_string()]),
        ];

        let result = detector.detect_patterns(&operators, 100);
        assert!(result.patterns.iter().any(|p| matches!(p.pattern_type, PatternType::Response | PatternType::Recurrence)));
    }

    #[test]
    fn test_pattern_statistics() {
        let mut detector = TemporalPatternDetector::new();
        
        let operators = vec![
            create_test_operator(TemporalOperator::Always, vec!["P".to_string()]),
            create_test_operator(TemporalOperator::Eventually, vec!["Q".to_string()]),
        ];

        let result = detector.detect_patterns(&operators, 100);
        assert!(result.statistics.total_patterns > 0);
        assert!(result.statistics.pattern_density > 0.0);
    }

    #[test]
    fn test_pattern_quality_assessment() {
        let detector = TemporalPatternDetector::new();
        
        let high_strength = PatternStrength {
            syntactic: 0.9,
            semantic: 0.9,
            coverage: 0.9,
            overall: 0.9,
        };
        
        let meaningful_vars = vec!["process_ready".to_string(), "task_complete".to_string()];
        let quality = detector.assess_instance_quality(&high_strength, &meaningful_vars);
        assert_eq!(quality, PatternQuality::High);
        
        let low_strength = PatternStrength {
            syntactic: 0.3,
            semantic: 0.3,
            coverage: 0.3,
            overall: 0.3,
        };
        
        let poor_vars = vec!["p".to_string()];
        let quality = detector.assess_instance_quality(&low_strength, &poor_vars);
        assert_eq!(quality, PatternQuality::VeryLow);
    }

    #[test]
    fn test_recommendation_generation() {
        let mut detector = TemporalPatternDetector::new();
        
        // Only safety patterns
        let operators = vec![
            create_test_operator(TemporalOperator::Always, vec!["P".to_string()]),
        ];

        let result = detector.detect_patterns(&operators, 100);
        assert!(result.recommendations.iter().any(|r| r.recommendation_type == RecommendationType::AddLiveness));
    }

    #[test]
    fn test_coverage_metrics() {
        let mut detector = TemporalPatternDetector::new();
        
        let operators = vec![
            create_test_operator(TemporalOperator::Always, vec!["P".to_string()]),
            create_test_operator(TemporalOperator::Eventually, vec!["Q".to_string()]),
        ];

        let result = detector.detect_patterns(&operators, 100);
        assert!(result.statistics.coverage.safety_coverage > 0.0);
        assert!(result.statistics.coverage.liveness_coverage > 0.0);
        assert!(result.statistics.coverage.overall_coverage > 0.0);
    }
}