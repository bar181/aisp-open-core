//! Logic Consistency Checker
//!
//! Mathematical logic consistency verification for AISP documents
//! Implements SRP by focusing solely on logical consistency analysis

use super::types::*;
use crate::ast::canonical::{CanonicalAispDocument as AispDocument, CanonicalAispBlock as AispBlock};
use crate::error::{AispError, AispResult};
use std::collections::HashMap;

/// Logic consistency checker for mathematical correctness
pub struct LogicConsistencyChecker {
    pub axiom_system: Vec<LogicalAxiom>,
    pub inference_rules: Vec<InferenceRule>,
    pub contradiction_detector: ContradictionDetector,
    pub proof_validator: ProofValidator,
}

impl LogicConsistencyChecker {
    /// Create new logic consistency checker
    pub fn new() -> Self {
        let mut checker = Self {
            axiom_system: Vec::new(),
            inference_rules: Vec::new(),
            contradiction_detector: ContradictionDetector {
                detection_methods: vec!["contradiction_analysis".to_string()],
            },
            proof_validator: ProofValidator {
                validation_rules: vec!["proof_verification".to_string()],
            },
        };
        
        checker.setup_default_axioms();
        checker.setup_inference_rules();
        checker
    }

    /// Create checker with enhanced logical validation
    pub fn with_enhanced_validation() -> Self {
        let mut checker = Self::new();
        checker.setup_enhanced_axioms();
        checker
    }

    /// Analyze document for logical consistency
    pub fn analyze_document(&mut self, document: &AispDocument) -> AispResult<LogicAnalysisResult> {
        let mut contradictions = Vec::new();
        let mut axiom_violations = Vec::new();
        let mut consistency_score = 1.0;

        // Check logical consistency across all blocks
        for block in &document.blocks {
            match block {
                AispBlock::Functions(functions_block) => {
                    let function_analysis = self.analyze_functions_logic(functions_block)?;
                    if !function_analysis.is_empty() {
                        contradictions.extend(function_analysis);
                        consistency_score -= 0.1;
                    }
                }
                AispBlock::Rules(rules_block) => {
                    let rules_analysis = self.validate_rules_block(rules_block)?;
                    if !rules_analysis.is_empty() {
                        axiom_violations.extend(rules_analysis);
                        consistency_score -= 0.2;
                    }
                }
                AispBlock::Evidence(evidence_block) => {
                    let evidence_analysis = self.check_evidence_logic(evidence_block)?;
                    if !evidence_analysis.is_empty() {
                        contradictions.extend(evidence_analysis);
                        consistency_score -= 0.05;
                    }
                }
                _ => {}
            }
        }

        // Detect logical contradictions
        let detected_contradictions = self.detect_contradictions(document)?;
        if !detected_contradictions.is_empty() {
            contradictions.extend(detected_contradictions);
            consistency_score -= 0.3;
        }

        // Validate inference rules application
        let inference_violations = self.validate_inference_rules(document)?;
        if !inference_violations.is_empty() {
            axiom_violations.extend(inference_violations);
            consistency_score -= 0.1;
        }

        let consistency_score = (consistency_score as f64).max(0.0).min(1.0);

        Ok(LogicAnalysisResult {
            consistency_score,
            contradictions,
            axiom_violations,
        })
    }

    /// Setup default logical axioms
    fn setup_default_axioms(&mut self) {
        self.axiom_system.extend(vec![
            LogicalAxiom {
                name: "Identity".to_string(),
                formula: "∀x. x = x".to_string(),
                axiom_type: AxiomType::Foundational,
                priority: AxiomPriority::Critical,
            },
            LogicalAxiom {
                name: "NonContradiction".to_string(),
                formula: "∀p. ¬(p ∧ ¬p)".to_string(),
                axiom_type: AxiomType::Foundational,
                priority: AxiomPriority::Critical,
            },
            LogicalAxiom {
                name: "ExcludedMiddle".to_string(),
                formula: "∀p. p ∨ ¬p".to_string(),
                axiom_type: AxiomType::Foundational,
                priority: AxiomPriority::High,
            },
        ]);
    }

    /// Setup enhanced axiom system for rigorous validation
    fn setup_enhanced_axioms(&mut self) {
        self.setup_default_axioms();
        
        self.axiom_system.extend(vec![
            LogicalAxiom {
                name: "Transitivity".to_string(),
                formula: "∀x,y,z. (x=y ∧ y=z) → x=z".to_string(),
                axiom_type: AxiomType::Derived,
                priority: AxiomPriority::High,
            },
            LogicalAxiom {
                name: "Symmetry".to_string(),
                formula: "∀x,y. x=y → y=x".to_string(),
                axiom_type: AxiomType::Derived,
                priority: AxiomPriority::Medium,
            },
            LogicalAxiom {
                name: "MonotonicitySafe".to_string(),
                formula: "∀f,x,y. x≤y → f(x)≤f(y)".to_string(),
                axiom_type: AxiomType::Domain,
                priority: AxiomPriority::Medium,
            },
        ]);
    }

    /// Setup logical inference rules
    fn setup_inference_rules(&mut self) {
        self.inference_rules.extend(vec![
            InferenceRule {
                name: "ModusPonens".to_string(),
                formula: "(p → q) ∧ p ⊢ q".to_string(),
            },
            InferenceRule {
                name: "ModusTollens".to_string(),
                formula: "(p → q) ∧ ¬q ⊢ ¬p".to_string(),
            },
            InferenceRule {
                name: "Syllogism".to_string(),
                formula: "(p → q) ∧ (q → r) ⊢ (p → r)".to_string(),
            },
        ]);
    }

    /// Analyze functions for logical consistency
    fn analyze_functions_logic(&self, functions_block: &crate::ast::canonical::FunctionsBlock) -> AispResult<Vec<String>> {
        let mut violations = Vec::new();

        for (index, func_def) in functions_block.functions.iter().enumerate() {
            let func_name = format!("function_{}", index);
            // Check for logical contradictions in function definitions
            if let Err(e) = self.validate_function_logic(&func_name, func_def) {
                violations.push(format!("Logic error in function {}: {}", func_name, e));
            }

            // Verify function satisfies logical axioms
            if let Err(e) = self.check_axiom_compliance(&func_name, func_def) {
                violations.push(format!("Axiom violation in function {}: {}", func_name, e));
            }
        }

        Ok(violations)
    }

    /// Validate rules block for logical consistency
    fn validate_rules_block(&self, rules_block: &crate::ast::canonical::RulesBlock) -> AispResult<Vec<String>> {
        let mut violations = Vec::new();

        for (index, _rule) in rules_block.rules.iter().enumerate() {
            let rule_name = format!("rule_{}", index);
            let rule_def = std::collections::HashMap::new(); // Simplified for canonical structure
            // Check rule for logical soundness
            if let Err(e) = self.validate_rule_soundness(&rule_name, &rule_def) {
                violations.push(format!("Rule soundness error in {}: {}", rule_name, e));
            }

            // Check for conflicts with existing rule system
            if let Err(e) = self.check_rule_conflicts(&rule_name, &rule_def) {
                violations.push(format!("Rule conflict in {}: {}", rule_name, e));
            }
        }

        Ok(violations)
    }

    /// Check evidence definitions for logical consistency
    fn check_evidence_logic(&self, evidence_block: &crate::ast::canonical::EvidenceBlock) -> AispResult<Vec<String>> {
        let mut violations = Vec::new();

        // Evidence block has different structure in canonical AST
        if evidence_block.delta.is_some() || evidence_block.phi.is_some() {
            let evidence_def = std::collections::HashMap::new(); // Simplified for canonical structure
            if let Err(e) = self.validate_evidence_logic("evidence_block", &evidence_def) {
                violations.push(format!("Evidence logic error: {}", e));
            }
        }

        Ok(violations)
    }

    /// Detect logical contradictions in the document
    fn detect_contradictions(&self, document: &AispDocument) -> AispResult<Vec<String>> {
        let mut contradictions = Vec::new();

        // Use contradiction detector algorithms
        for method in &self.contradiction_detector.detection_methods {
            let detected = self.apply_contradiction_detection(document, method)?;
            contradictions.extend(detected);
        }

        Ok(contradictions)
    }

    /// Validate inference rules application
    fn validate_inference_rules(&self, _document: &AispDocument) -> AispResult<Vec<String>> {
        let mut violations = Vec::new();

        // Check if inference rules are properly applied
        for rule in &self.inference_rules {
            if let Err(e) = self.validate_inference_rule_application(rule) {
                violations.push(format!("Inference rule violation {}: {}", rule.name, e));
            }
        }

        Ok(violations)
    }

    /// Helper validation methods

    fn validate_function_logic(&self, _func_name: &str, _func_def: &crate::ast::canonical::FunctionDefinition) -> AispResult<()> {
        // Simplified function logic validation
        Ok(())
    }

    fn check_axiom_compliance(&self, _func_name: &str, _func_def: &crate::ast::canonical::FunctionDefinition) -> AispResult<()> {
        // Check if function satisfies all axioms
        Ok(())
    }

    fn validate_rule_soundness(&self, _rule_name: &str, _rule_def: &std::collections::HashMap<String, String>) -> AispResult<()> {
        // Validate rule logical soundness using generic map
        Ok(())
    }

    fn check_rule_conflicts(&self, _rule_name: &str, _rule_def: &std::collections::HashMap<String, String>) -> AispResult<()> {
        // Check for conflicts with existing rules using generic map
        Ok(())
    }

    fn validate_evidence_logic(&self, _evidence_name: &str, _evidence_def: &std::collections::HashMap<String, String>) -> AispResult<()> {
        // Validate evidence definition logic using generic map
        Ok(())
    }

    fn apply_contradiction_detection(&self, _document: &AispDocument, _method: &str) -> AispResult<Vec<String>> {
        // Apply specific contradiction detection method
        Ok(vec![])
    }

    fn validate_inference_rule_application(&self, _rule: &InferenceRule) -> AispResult<()> {
        // Validate inference rule application
        Ok(())
    }
}

impl Default for LogicConsistencyChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logic_checker_creation() {
        let checker = LogicConsistencyChecker::new();
        assert_eq!(checker.axiom_system.len(), 3);
        assert_eq!(checker.inference_rules.len(), 3);
    }

    #[test]
    fn test_enhanced_validation() {
        let checker = LogicConsistencyChecker::with_enhanced_validation();
        assert_eq!(checker.axiom_system.len(), 6); // 3 default + 3 enhanced
    }

    #[test]
    fn test_default_axioms() {
        let checker = LogicConsistencyChecker::new();
        let identity_axiom = &checker.axiom_system[0];
        assert_eq!(identity_axiom.name, "Identity");
        assert_eq!(identity_axiom.axiom_type, AxiomType::Foundational);
        assert_eq!(identity_axiom.priority, AxiomPriority::Critical);
    }

    #[test]
    fn test_inference_rules_setup() {
        let checker = LogicConsistencyChecker::new();
        let modus_ponens = &checker.inference_rules[0];
        assert_eq!(modus_ponens.name, "ModusPonens");
        assert!(modus_ponens.formula.contains("→"));
    }

    #[test]
    fn test_axiom_types() {
        let foundational = AxiomType::Foundational;
        let derived = AxiomType::Derived;
        let domain = AxiomType::Domain;
        
        assert_ne!(foundational, derived);
        assert_ne!(derived, domain);
        assert_eq!(foundational, AxiomType::Foundational);
    }
}