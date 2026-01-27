//! Ghost Intent Search Validation for AISP 5.1
//!
//! This module implements validation for ghost intent searches, which represent
//! the difference between intended system behavior (ψ_*) and current observed
//! behavior (ψ_have), formally defined as:
//!
//! **ψ_g ≜ ψ_* ⊖ ψ_have**
//!
//! Where:
//! - ψ_g: Ghost intent (missing capabilities or behaviors)
//! - ψ_*: Intended complete system behavior  
//! - ψ_have: Current observed system behavior
//! - ⊖: Set difference operation
//!
//! This validation is crucial for identifying gaps in AI system capabilities
//! and ensuring comprehensive behavior coverage.

use crate::{
    ast::canonical::{CanonicalAispDocument as AispDocument, CanonicalAispBlock as AispBlock, *},
    error::*,
    z3_verification::*,
};
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

/// Ghost intent validation result
#[derive(Debug, Clone)]
pub struct GhostIntentValidationResult {
    /// Whether ghost intent analysis is valid
    pub valid: bool,
    /// Detected ghost intents (missing behaviors)
    pub ghost_intents: Vec<GhostIntent>,
    /// Validation statistics
    pub stats: GhostIntentStats,
    /// Analysis errors
    pub errors: Vec<String>,
    /// Analysis warnings  
    pub warnings: Vec<String>,
}

/// Represents a detected ghost intent
#[derive(Debug, Clone)]
pub struct GhostIntent {
    /// Unique identifier for this ghost intent
    pub id: String,
    /// Description of the missing behavior
    pub description: String,
    /// Intended behavior specification
    pub intended_behavior: BehaviorSpec,
    /// Current system behavior
    pub current_behavior: Option<BehaviorSpec>,
    /// Confidence score for this ghost intent detection
    pub confidence: f64,
    /// Formal proof of the behavioral gap
    pub gap_proof: Option<BehaviorGapProof>,
}

/// Specification of system behavior
#[derive(Debug, Clone)]
pub struct BehaviorSpec {
    /// Behavior identifier
    pub id: String,
    /// Preconditions for this behavior
    pub preconditions: Vec<String>,
    /// Postconditions after behavior execution
    pub postconditions: Vec<String>,
    /// Invariants maintained during execution
    pub invariants: Vec<String>,
    /// Temporal constraints
    pub temporal_constraints: Vec<String>,
}

/// Formal proof of a behavioral gap
#[derive(Debug, Clone)]
pub struct BehaviorGapProof {
    /// Proof identifier
    pub id: String,
    /// SMT-LIB formula representing the gap
    pub gap_formula: String,
    /// Z3 verification result
    pub verification_result: Option<PropertyResult>,
    /// Proof certificate
    pub certificate: Option<String>,
}

/// Ghost intent analysis statistics
#[derive(Debug, Clone)]
pub struct GhostIntentStats {
    /// Total ghost intents detected
    pub total_ghost_intents: usize,
    /// High-confidence detections
    pub high_confidence_count: usize,
    /// Medium-confidence detections  
    pub medium_confidence_count: usize,
    /// Low-confidence detections
    pub low_confidence_count: usize,
    /// Analysis time
    pub analysis_time: Duration,
    /// SMT solver queries performed
    pub smt_queries: usize,
}

/// Configuration for ghost intent validation
#[derive(Debug, Clone)]
pub struct GhostIntentConfig {
    /// Minimum confidence threshold for reporting
    pub min_confidence_threshold: f64,
    /// Maximum analysis time allowed
    pub max_analysis_time: Duration,
    /// Enable formal verification of gaps
    pub enable_formal_verification: bool,
    /// Z3 timeout for individual queries
    pub z3_timeout_ms: u32,
}

impl Default for GhostIntentConfig {
    fn default() -> Self {
        Self {
            min_confidence_threshold: 0.6,
            max_analysis_time: Duration::from_secs(30),
            enable_formal_verification: true,
            z3_timeout_ms: 5000,
        }
    }
}

/// Ghost intent validator implementing ψ_g ≜ ψ_* ⊖ ψ_have analysis
pub struct GhostIntentValidator {
    /// Validation configuration
    config: GhostIntentConfig,
    /// Z3 verifier for formal analysis
    z3_verifier: Option<EnhancedZ3Verifier>,
    /// Analysis statistics
    stats: GhostIntentStats,
}

impl GhostIntentValidator {
    /// Create new ghost intent validator
    pub fn new(config: GhostIntentConfig) -> Self {
        let z3_verifier = if config.enable_formal_verification {
            EnhancedZ3Verifier::new().ok()
        } else {
            None
        };

        Self {
            config,
            z3_verifier,
            stats: GhostIntentStats {
                total_ghost_intents: 0,
                high_confidence_count: 0,
                medium_confidence_count: 0,
                low_confidence_count: 0,
                analysis_time: Duration::from_secs(0),
                smt_queries: 0,
            },
        }
    }

    /// Validate ghost intents in AISP document
    pub fn validate_ghost_intents(&mut self, document: &AispDocument) -> AispResult<GhostIntentValidationResult> {
        let start_time = Instant::now();
        
        // Extract behavior specifications from document
        let intended_behaviors = self.extract_intended_behaviors(document)?;
        let current_behaviors = self.extract_current_behaviors(document)?;
        
        // Perform ghost intent analysis: ψ_g ≜ ψ_* ⊖ ψ_have
        let ghost_intents = self.analyze_behavioral_gaps(&intended_behaviors, &current_behaviors)?;
        
        // Validate detected ghost intents
        let validated_ghost_intents = self.validate_ghost_intent_detections(ghost_intents)?;
        
        // Update statistics
        self.stats.analysis_time = start_time.elapsed();
        self.stats.total_ghost_intents = validated_ghost_intents.len();
        self.update_confidence_stats(&validated_ghost_intents);
        
        Ok(GhostIntentValidationResult {
            valid: self.is_validation_successful(&validated_ghost_intents),
            ghost_intents: validated_ghost_intents,
            stats: self.stats.clone(),
            errors: vec![],
            warnings: self.generate_warnings(),
        })
    }

    /// Extract intended behavior specifications (ψ_*) from AISP document
    fn extract_intended_behaviors(&self, document: &AispDocument) -> AispResult<Vec<BehaviorSpec>> {
        let mut behaviors = Vec::new();
        
        for block in &document.blocks {
            match block {
                AispBlock::Meta(meta_block) => {
                    // Extract intended behaviors from meta declarations
                    for (key, entry) in &meta_block.entries {
                        if key.starts_with("intended_behavior_") {
                            if let MetaValue::String(value_str) = &entry.value {
                                if let Some(behavior) = self.parse_behavior_spec(value_str)? {
                                    behaviors.push(behavior);
                                }
                            }
                        }
                    }
                }
                AispBlock::Rules(rules_block) => {
                    // Extract behaviors from logical specifications
                    for rule in &rules_block.rules {
                        if let Some(behavior) = self.extract_behavior_from_rule(rule)? {
                            behaviors.push(behavior);
                        }
                    }
                }
                _ => continue,
            }
        }
        
        Ok(behaviors)
    }

    /// Extract current behavior specifications (ψ_have) from AISP document
    fn extract_current_behaviors(&self, document: &AispDocument) -> AispResult<Vec<BehaviorSpec>> {
        let mut behaviors = Vec::new();
        
        // For now, we'll extract behaviors from the basic document structure
        // In a full implementation, this would analyze evidence blocks and function implementations
        
        for block in &document.blocks {
            match block {
                AispBlock::Evidence(_evidence_block) => {
                    // Placeholder: would extract current behaviors from evidence metrics
                    let behavior = BehaviorSpec {
                        id: "evidence_behavior".to_string(),
                        preconditions: vec!["evidence_available".to_string()],
                        postconditions: vec!["system_validated".to_string()],
                        invariants: vec!["delta_positive".to_string()],
                        temporal_constraints: vec![],
                    };
                    behaviors.push(behavior);
                }
                AispBlock::Functions(_functions_block) => {
                    // Placeholder: would extract behaviors from function implementations  
                    let behavior = BehaviorSpec {
                        id: "function_behavior".to_string(),
                        preconditions: vec!["input_valid".to_string()],
                        postconditions: vec!["output_computed".to_string()],
                        invariants: vec!["type_safety".to_string()],
                        temporal_constraints: vec![],
                    };
                    behaviors.push(behavior);
                }
                _ => continue,
            }
        }
        
        Ok(behaviors)
    }

    /// Analyze behavioral gaps: ψ_g ≜ ψ_* ⊖ ψ_have
    fn analyze_behavioral_gaps(
        &mut self,
        intended_behaviors: &[BehaviorSpec],
        current_behaviors: &[BehaviorSpec],
    ) -> AispResult<Vec<GhostIntent>> {
        let mut ghost_intents = Vec::new();
        
        // Create mapping of current behaviors for efficient lookup
        let current_behavior_map: HashMap<String, &BehaviorSpec> = current_behaviors
            .iter()
            .map(|b| (b.id.clone(), b))
            .collect();
        
        // For each intended behavior, check if it's implemented
        for intended in intended_behaviors {
            if let Some(current) = current_behavior_map.get(&intended.id) {
                // Behavior exists, check for completeness
                if let Some(gap) = self.analyze_behavior_completeness(intended, current)? {
                    ghost_intents.push(gap);
                }
            } else {
                // Missing behavior - create ghost intent
                let ghost_intent = GhostIntent {
                    id: format!("ghost_{}", intended.id),
                    description: format!("Missing implementation of behavior '{}'", intended.id),
                    intended_behavior: intended.clone(),
                    current_behavior: None,
                    confidence: 0.9, // High confidence for completely missing behaviors
                    gap_proof: self.generate_gap_proof(intended, None)?,
                };
                ghost_intents.push(ghost_intent);
            }
        }
        
        Ok(ghost_intents)
    }

    /// Analyze completeness of a behavior implementation
    fn analyze_behavior_completeness(
        &mut self,
        intended: &BehaviorSpec,
        current: &BehaviorSpec,
    ) -> AispResult<Option<GhostIntent>> {
        // Check precondition coverage
        let missing_preconditions = self.find_missing_conditions(&intended.preconditions, &current.preconditions);
        
        // Check postcondition coverage
        let missing_postconditions = self.find_missing_conditions(&intended.postconditions, &current.postconditions);
        
        // Check invariant preservation
        let missing_invariants = self.find_missing_conditions(&intended.invariants, &current.invariants);
        
        if missing_preconditions.is_empty() && missing_postconditions.is_empty() && missing_invariants.is_empty() {
            return Ok(None); // Behavior is complete
        }
        
        // Calculate confidence based on missing elements
        let total_intended = intended.preconditions.len() + intended.postconditions.len() + intended.invariants.len();
        let total_missing = missing_preconditions.len() + missing_postconditions.len() + missing_invariants.len();
        let confidence = if total_intended > 0 {
            total_missing as f64 / total_intended as f64
        } else {
            0.5
        };
        
        let description = format!(
            "Incomplete implementation of behavior '{}': missing {} preconditions, {} postconditions, {} invariants",
            intended.id, missing_preconditions.len(), missing_postconditions.len(), missing_invariants.len()
        );
        
        Ok(Some(GhostIntent {
            id: format!("incomplete_{}", intended.id),
            description,
            intended_behavior: intended.clone(),
            current_behavior: Some(current.clone()),
            confidence,
            gap_proof: self.generate_gap_proof(intended, Some(current))?,
        }))
    }

    /// Find missing conditions between intended and current specifications
    fn find_missing_conditions(&self, intended: &[String], current: &[String]) -> Vec<String> {
        let current_set: HashSet<&String> = current.iter().collect();
        intended
            .iter()
            .filter(|condition| !current_set.contains(condition))
            .cloned()
            .collect()
    }

    /// Generate formal proof of behavioral gap
    fn generate_gap_proof(
        &mut self,
        intended: &BehaviorSpec,
        current: Option<&BehaviorSpec>,
    ) -> AispResult<Option<BehaviorGapProof>> {
        if !self.config.enable_formal_verification || self.z3_verifier.is_none() {
            return Ok(None);
        }
        
        self.stats.smt_queries += 1;
        
        let gap_formula = if let Some(current) = current {
            self.generate_completeness_formula(intended, current)
        } else {
            self.generate_existence_formula(intended)
        };
        
        // For now, return a placeholder proof
        // In a full implementation, this would use Z3 to verify the gap
        Ok(Some(BehaviorGapProof {
            id: format!("gap_proof_{}", intended.id),
            gap_formula,
            verification_result: Some(PropertyResult::Unknown),
            certificate: Some("Behavioral gap detected via specification analysis".to_string()),
        }))
    }

    /// Generate SMT formula for behavior existence
    fn generate_existence_formula(&self, intended: &BehaviorSpec) -> String {
        format!(
            "(assert (not (exists ((state State)) \n  (behavior_{} state))))",
            intended.id
        )
    }

    /// Generate SMT formula for behavior completeness
    fn generate_completeness_formula(&self, intended: &BehaviorSpec, current: &BehaviorSpec) -> String {
        format!(
            "(assert (not (forall ((state State)) \n  (=> (behavior_{}_current state) (behavior_{}_intended state)))))",
            current.id, intended.id
        )
    }

    /// Validate detected ghost intent detections
    fn validate_ghost_intent_detections(&self, ghost_intents: Vec<GhostIntent>) -> AispResult<Vec<GhostIntent>> {
        Ok(ghost_intents
            .into_iter()
            .filter(|gi| gi.confidence >= self.config.min_confidence_threshold)
            .collect())
    }

    /// Check if validation is successful
    fn is_validation_successful(&self, ghost_intents: &[GhostIntent]) -> bool {
        // Validation is successful if no high-confidence ghost intents are detected
        ghost_intents.iter().all(|gi| gi.confidence < 0.8)
    }

    /// Update confidence-based statistics
    fn update_confidence_stats(&mut self, ghost_intents: &[GhostIntent]) {
        self.stats.high_confidence_count = ghost_intents.iter().filter(|gi| gi.confidence >= 0.8).count();
        self.stats.medium_confidence_count = ghost_intents.iter().filter(|gi| gi.confidence >= 0.6 && gi.confidence < 0.8).count();
        self.stats.low_confidence_count = ghost_intents.iter().filter(|gi| gi.confidence < 0.6).count();
    }

    /// Generate analysis warnings
    fn generate_warnings(&self) -> Vec<String> {
        let mut warnings = Vec::new();
        
        if self.stats.analysis_time > self.config.max_analysis_time {
            warnings.push("Ghost intent analysis exceeded maximum time limit".to_string());
        }
        
        if self.stats.high_confidence_count > 0 {
            warnings.push(format!(
                "{} high-confidence ghost intents detected - significant behavioral gaps may exist",
                self.stats.high_confidence_count
            ));
        }
        
        warnings
    }

    /// Parse behavior specification from meta value
    fn parse_behavior_spec(&self, _value: &str) -> AispResult<Option<BehaviorSpec>> {
        // Placeholder implementation - would parse AISP behavior syntax
        Ok(None)
    }

    /// Extract behavior from logical rule
    fn extract_behavior_from_rule(&self, _rule: &LogicalRule) -> AispResult<Option<BehaviorSpec>> {
        // Placeholder implementation - would analyze logic rules for behaviors
        Ok(None)
    }

    /// Get validation statistics
    pub fn get_stats(&self) -> &GhostIntentStats {
        &self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{DocumentHeader, DocumentMetadata, Span, Position};

    fn create_test_document() -> AispDocument {
        AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "ghost_intent_test".to_string(),
                date: "2026-01-26".to_string(),
                metadata: None,
            },
            metadata: DocumentMetadata {
                domain: Some("test".to_string()),
                protocol: None,
            },
            blocks: vec![],
            span: Span {
                start: Position { line: 1, column: 1, offset: 0 },
                end: Position { line: 1, column: 1, offset: 0 },
            },
        }
    }

    #[test]
    fn test_ghost_intent_validator_creation() {
        let config = GhostIntentConfig::default();
        let validator = GhostIntentValidator::new(config);
        
        assert_eq!(validator.stats.total_ghost_intents, 0);
        assert_eq!(validator.stats.smt_queries, 0);
    }

    #[test]
    fn test_behavior_spec_creation() {
        let behavior = BehaviorSpec {
            id: "test_behavior".to_string(),
            preconditions: vec!["precond1".to_string(), "precond2".to_string()],
            postconditions: vec!["postcond1".to_string()],
            invariants: vec!["inv1".to_string()],
            temporal_constraints: vec!["temporal1".to_string()],
        };

        assert_eq!(behavior.id, "test_behavior");
        assert_eq!(behavior.preconditions.len(), 2);
        assert_eq!(behavior.postconditions.len(), 1);
    }

    #[test]
    fn test_missing_conditions_detection() {
        let config = GhostIntentConfig::default();
        let validator = GhostIntentValidator::new(config);
        
        let intended = vec!["cond1".to_string(), "cond2".to_string(), "cond3".to_string()];
        let current = vec!["cond1".to_string(), "cond3".to_string()];
        
        let missing = validator.find_missing_conditions(&intended, &current);
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0], "cond2");
    }

    #[test]
    fn test_ghost_intent_validation_basic() {
        let config = GhostIntentConfig {
            enable_formal_verification: false,
            ..Default::default()
        };
        let mut validator = GhostIntentValidator::new(config);
        let document = create_test_document();

        let result = validator.validate_ghost_intents(&document);
        assert!(result.is_ok());
        
        let validation_result = result.unwrap();
        assert!(validation_result.valid); // No ghost intents in empty document
        assert_eq!(validation_result.ghost_intents.len(), 0);
    }

    #[test]
    fn test_confidence_thresholding() {
        let ghost_intents = vec![
            GhostIntent {
                id: "high_conf".to_string(),
                description: "High confidence detection".to_string(),
                intended_behavior: BehaviorSpec {
                    id: "test".to_string(),
                    preconditions: vec![],
                    postconditions: vec![],
                    invariants: vec![],
                    temporal_constraints: vec![],
                },
                current_behavior: None,
                confidence: 0.9,
                gap_proof: None,
            },
            GhostIntent {
                id: "low_conf".to_string(),
                description: "Low confidence detection".to_string(),
                intended_behavior: BehaviorSpec {
                    id: "test2".to_string(),
                    preconditions: vec![],
                    postconditions: vec![],
                    invariants: vec![],
                    temporal_constraints: vec![],
                },
                current_behavior: None,
                confidence: 0.4,
                gap_proof: None,
            },
        ];

        let config = GhostIntentConfig {
            min_confidence_threshold: 0.6,
            ..Default::default()
        };
        let validator = GhostIntentValidator::new(config);
        
        let filtered = validator.validate_ghost_intent_detections(ghost_intents).unwrap();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].confidence, 0.9);
    }

    #[test]
    fn test_statistics_update() {
        let config = GhostIntentConfig::default();
        let mut validator = GhostIntentValidator::new(config);
        
        let ghost_intents = vec![
            GhostIntent {
                id: "high1".to_string(),
                description: "High confidence".to_string(),
                intended_behavior: BehaviorSpec {
                    id: "test1".to_string(),
                    preconditions: vec![],
                    postconditions: vec![],
                    invariants: vec![],
                    temporal_constraints: vec![],
                },
                current_behavior: None,
                confidence: 0.9,
                gap_proof: None,
            },
            GhostIntent {
                id: "med1".to_string(),
                description: "Medium confidence".to_string(),
                intended_behavior: BehaviorSpec {
                    id: "test2".to_string(),
                    preconditions: vec![],
                    postconditions: vec![],
                    invariants: vec![],
                    temporal_constraints: vec![],
                },
                current_behavior: None,
                confidence: 0.7,
                gap_proof: None,
            },
        ];

        validator.update_confidence_stats(&ghost_intents);
        
        assert_eq!(validator.stats.high_confidence_count, 1);
        assert_eq!(validator.stats.medium_confidence_count, 1);
        assert_eq!(validator.stats.low_confidence_count, 0);
    }

    #[test]
    fn test_smt_formula_generation() {
        let config = GhostIntentConfig::default();
        let validator = GhostIntentValidator::new(config);
        
        let intended = BehaviorSpec {
            id: "test_behavior".to_string(),
            preconditions: vec!["pre1".to_string()],
            postconditions: vec!["post1".to_string()],
            invariants: vec!["inv1".to_string()],
            temporal_constraints: vec![],
        };

        let existence_formula = validator.generate_existence_formula(&intended);
        assert!(existence_formula.contains("behavior_test_behavior"));
        assert!(existence_formula.contains("not (exists"));

        let current = BehaviorSpec {
            id: "test_behavior_impl".to_string(),
            preconditions: vec!["pre1".to_string()],
            postconditions: vec![],
            invariants: vec!["inv1".to_string()],
            temporal_constraints: vec![],
        };

        let completeness_formula = validator.generate_completeness_formula(&intended, &current);
        assert!(completeness_formula.contains("behavior_test_behavior_impl_current"));
        assert!(completeness_formula.contains("behavior_test_behavior_intended"));
    }
}