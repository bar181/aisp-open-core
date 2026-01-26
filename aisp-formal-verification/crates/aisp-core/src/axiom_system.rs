//! Axiom System for Theorem Proving
//!
//! This module defines axioms and inference rules for the automated
//! theorem prover, including logical, temporal, and AISP-specific axioms.

use crate::property_types::*;
use std::collections::HashMap;

/// Axiom in the proof system
#[derive(Debug, Clone)]
pub struct Axiom {
    /// Axiom name
    pub name: String,
    /// Axiom formula
    pub formula: FormulaStructure,
    /// Axiom type classification
    pub axiom_type: AxiomType,
    /// Priority for application
    pub priority: u8,
}

/// Types of axioms
#[derive(Debug, Clone, PartialEq)]
pub enum AxiomType {
    /// Logical axioms (propositional, predicate logic)
    Logical,
    /// Arithmetic axioms
    Arithmetic,
    /// Type theory axioms
    TypeTheory,
    /// Temporal logic axioms
    Temporal,
    /// AISP-specific axioms
    AispSpecific,
    /// Domain-specific axioms
    Domain,
}

/// Inference rule in natural deduction
#[derive(Debug, Clone)]
pub struct InferenceRule {
    /// Rule name
    pub name: String,
    /// Rule type
    pub rule_type: RuleType,
    /// Premises required
    pub premises: Vec<FormulaPattern>,
    /// Conclusion derived
    pub conclusion: FormulaPattern,
    /// Side conditions
    pub conditions: Vec<SideCondition>,
    /// Priority for application
    pub priority: u8,
}

/// Types of inference rules
#[derive(Debug, Clone, PartialEq)]
pub enum RuleType {
    /// Introduction rules
    Introduction,
    /// Elimination rules  
    Elimination,
    /// Structural rules
    Structural,
    /// Derived rules
    Derived,
}

/// Formula pattern for rule matching
#[derive(Debug, Clone)]
pub struct FormulaPattern {
    /// Pattern structure
    pub pattern: PatternStructure,
    /// Variable bindings
    pub variables: HashMap<String, String>,
}

/// Pattern structure for matching
#[derive(Debug, Clone)]
pub enum PatternStructure {
    /// Variable pattern (matches any formula)
    Variable(String),
    /// Literal pattern (exact match)
    Literal(FormulaStructure),
    /// Conjunction pattern
    Conjunction(Vec<PatternStructure>),
    /// Disjunction pattern
    Disjunction(Vec<PatternStructure>),
    /// Implication pattern
    Implication(Box<PatternStructure>, Box<PatternStructure>),
    /// Quantification pattern
    Quantified(QuantifierKind, String, Box<PatternStructure>),
}

/// Quantifier kinds for patterns
#[derive(Debug, Clone, PartialEq)]
pub enum QuantifierKind {
    Universal,
    Existential,
}

/// Side conditions for rule application
#[derive(Debug, Clone)]
pub enum SideCondition {
    /// Variable must be fresh
    FreshVariable(String),
    /// Formula must be closed
    ClosedFormula(String),
    /// Type condition
    TypeCheck(String, String),
    /// Custom condition
    Custom(String),
}

/// Axiom system builder and manager
pub struct AxiomSystemBuilder {
    axioms: Vec<Axiom>,
    inference_rules: Vec<InferenceRule>,
}

impl AxiomSystemBuilder {
    /// Create new axiom system builder
    pub fn new() -> Self {
        Self {
            axioms: Vec::new(),
            inference_rules: Vec::new(),
        }
    }

    /// Add propositional logic axioms
    pub fn add_propositional_axioms(&mut self) {
        // Modus Ponens: ((P → Q) ∧ P) → Q
        self.add_axiom(Axiom {
            name: "modus_ponens".to_string(),
            formula: self.create_modus_ponens_formula(),
            axiom_type: AxiomType::Logical,
            priority: 10,
        });

        // Law of Excluded Middle: P ∨ ¬P
        self.add_axiom(Axiom {
            name: "excluded_middle".to_string(),
            formula: self.create_excluded_middle_formula(),
            axiom_type: AxiomType::Logical,
            priority: 8,
        });

        // Double Negation: ¬¬P → P
        self.add_axiom(Axiom {
            name: "double_negation".to_string(),
            formula: self.create_double_negation_formula(),
            axiom_type: AxiomType::Logical,
            priority: 7,
        });

        // Add propositional inference rules
        self.add_propositional_rules();
    }

    /// Add predicate logic axioms
    pub fn add_predicate_axioms(&mut self) {
        // Universal Instantiation: ∀x.P(x) → P(t)
        self.add_axiom(Axiom {
            name: "universal_instantiation".to_string(),
            formula: self.create_universal_instantiation_formula(),
            axiom_type: AxiomType::Logical,
            priority: 9,
        });

        // Existential Generalization: P(t) → ∃x.P(x)
        self.add_axiom(Axiom {
            name: "existential_generalization".to_string(),
            formula: self.create_existential_generalization_formula(),
            axiom_type: AxiomType::Logical,
            priority: 8,
        });

        self.add_predicate_rules();
    }

    /// Add temporal logic axioms
    pub fn add_temporal_axioms(&mut self) {
        // Always distributes over implication: □(P → Q) → (□P → □Q)
        self.add_axiom(Axiom {
            name: "always_distribution".to_string(),
            formula: self.create_always_distribution_formula(),
            axiom_type: AxiomType::Temporal,
            priority: 6,
        });

        // Eventually and always duality: ◊P ↔ ¬□¬P
        self.add_axiom(Axiom {
            name: "eventually_always_duality".to_string(),
            formula: self.create_duality_formula(),
            axiom_type: AxiomType::Temporal,
            priority: 5,
        });

        self.add_temporal_rules();
    }

    /// Add AISP-specific axioms
    pub fn add_aisp_axioms(&mut self) {
        // Type safety: ∀x. hasType(x, T) → typeSafe(x)
        self.add_axiom(Axiom {
            name: "type_safety".to_string(),
            formula: self.create_type_safety_formula(),
            axiom_type: AxiomType::AispSpecific,
            priority: 9,
        });

        // Structural validity: ∀x. wellFormed(x) → structurallyValid(x)
        self.add_axiom(Axiom {
            name: "structural_validity".to_string(),
            formula: self.create_structural_validity_formula(),
            axiom_type: AxiomType::AispSpecific,
            priority: 8,
        });
    }

    /// Add axiom to system
    pub fn add_axiom(&mut self, axiom: Axiom) {
        self.axioms.push(axiom);
    }

    /// Add inference rule to system
    pub fn add_rule(&mut self, rule: InferenceRule) {
        self.inference_rules.push(rule);
    }

    /// Build complete axiom system
    pub fn build(self) -> (Vec<Axiom>, Vec<InferenceRule>) {
        (self.axioms, self.inference_rules)
    }

    // Helper methods for creating specific formulas
    fn create_modus_ponens_formula(&self) -> FormulaStructure {
        // ((P → Q) ∧ P) → Q
        let p = FormulaStructure::Atomic(AtomicFormula {
            predicate: "P".to_string(),
            terms: vec![],
            type_signature: None,
        });
        let q = FormulaStructure::Atomic(AtomicFormula {
            predicate: "Q".to_string(),
            terms: vec![],
            type_signature: None,
        });
        let p_implies_q = FormulaStructure::Implication(Box::new(p.clone()), Box::new(q.clone()));
        let premise = FormulaStructure::Conjunction(vec![p_implies_q, p]);
        FormulaStructure::Implication(Box::new(premise), Box::new(q))
    }

    fn create_excluded_middle_formula(&self) -> FormulaStructure {
        // P ∨ ¬P
        let p = FormulaStructure::Atomic(AtomicFormula {
            predicate: "P".to_string(),
            terms: vec![],
            type_signature: None,
        });
        let not_p = FormulaStructure::Negation(Box::new(p.clone()));
        FormulaStructure::Disjunction(vec![p, not_p])
    }

    fn create_double_negation_formula(&self) -> FormulaStructure {
        // ¬¬P → P
        let p = FormulaStructure::Atomic(AtomicFormula {
            predicate: "P".to_string(),
            terms: vec![],
            type_signature: None,
        });
        let not_p = FormulaStructure::Negation(Box::new(p.clone()));
        let not_not_p = FormulaStructure::Negation(Box::new(not_p));
        FormulaStructure::Implication(Box::new(not_not_p), Box::new(p))
    }

    fn create_universal_instantiation_formula(&self) -> FormulaStructure {
        // ∀x.P(x) → P(t) - simplified representation
        let quantifier = Quantifier {
            variable: "x".to_string(),
            variable_type: Some("T".to_string()),
            domain: None,
        };
        let px = FormulaStructure::Atomic(AtomicFormula {
            predicate: "P".to_string(),
            terms: vec![Term::Variable("x".to_string(), Some("T".to_string()))],
            type_signature: None,
        });
        let pt = FormulaStructure::Atomic(AtomicFormula {
            predicate: "P".to_string(),
            terms: vec![Term::Variable("t".to_string(), Some("T".to_string()))],
            type_signature: None,
        });
        let universal = FormulaStructure::Universal(quantifier, Box::new(px));
        FormulaStructure::Implication(Box::new(universal), Box::new(pt))
    }

    fn create_existential_generalization_formula(&self) -> FormulaStructure {
        // P(t) → ∃x.P(x) - simplified representation
        let quantifier = Quantifier {
            variable: "x".to_string(),
            variable_type: Some("T".to_string()),
            domain: None,
        };
        let px = FormulaStructure::Atomic(AtomicFormula {
            predicate: "P".to_string(),
            terms: vec![Term::Variable("x".to_string(), Some("T".to_string()))],
            type_signature: None,
        });
        let pt = FormulaStructure::Atomic(AtomicFormula {
            predicate: "P".to_string(),
            terms: vec![Term::Variable("t".to_string(), Some("T".to_string()))],
            type_signature: None,
        });
        let existential = FormulaStructure::Existential(quantifier, Box::new(px));
        FormulaStructure::Implication(Box::new(pt), Box::new(existential))
    }

    fn create_always_distribution_formula(&self) -> FormulaStructure {
        // □(P → Q) → (□P → □Q)
        let p = FormulaStructure::Atomic(AtomicFormula {
            predicate: "P".to_string(),
            terms: vec![],
            type_signature: None,
        });
        let q = FormulaStructure::Atomic(AtomicFormula {
            predicate: "Q".to_string(),
            terms: vec![],
            type_signature: None,
        });
        let p_implies_q = FormulaStructure::Implication(Box::new(p.clone()), Box::new(q.clone()));
        let always_p_implies_q = FormulaStructure::TemporalAlways(Box::new(p_implies_q));
        let always_p = FormulaStructure::TemporalAlways(Box::new(p));
        let always_q = FormulaStructure::TemporalAlways(Box::new(q));
        let always_p_implies_always_q = FormulaStructure::Implication(Box::new(always_p), Box::new(always_q));
        FormulaStructure::Implication(Box::new(always_p_implies_q), Box::new(always_p_implies_always_q))
    }

    fn create_duality_formula(&self) -> FormulaStructure {
        // ◊P ↔ ¬□¬P
        let p = FormulaStructure::Atomic(AtomicFormula {
            predicate: "P".to_string(),
            terms: vec![],
            type_signature: None,
        });
        let not_p = FormulaStructure::Negation(Box::new(p.clone()));
        let always_not_p = FormulaStructure::TemporalAlways(Box::new(not_p));
        let not_always_not_p = FormulaStructure::Negation(Box::new(always_not_p));
        let eventually_p = FormulaStructure::TemporalEventually(Box::new(p));
        FormulaStructure::Biconditional(Box::new(eventually_p), Box::new(not_always_not_p))
    }

    fn create_type_safety_formula(&self) -> FormulaStructure {
        // ∀x. hasType(x, T) → typeSafe(x)
        let quantifier = Quantifier {
            variable: "x".to_string(),
            variable_type: Some("Any".to_string()),
            domain: None,
        };
        let has_type = FormulaStructure::FunctionApplication(
            "hasType".to_string(),
            vec![
                Term::Variable("x".to_string(), Some("Any".to_string())),
                Term::Variable("T".to_string(), Some("Type".to_string())),
            ],
        );
        let type_safe = FormulaStructure::FunctionApplication(
            "typeSafe".to_string(),
            vec![Term::Variable("x".to_string(), Some("Any".to_string()))],
        );
        let implication = FormulaStructure::Implication(Box::new(has_type), Box::new(type_safe));
        FormulaStructure::Universal(quantifier, Box::new(implication))
    }

    fn create_structural_validity_formula(&self) -> FormulaStructure {
        // ∀x. wellFormed(x) → structurallyValid(x)
        let quantifier = Quantifier {
            variable: "x".to_string(),
            variable_type: Some("Any".to_string()),
            domain: None,
        };
        let well_formed = FormulaStructure::FunctionApplication(
            "wellFormed".to_string(),
            vec![Term::Variable("x".to_string(), Some("Any".to_string()))],
        );
        let structurally_valid = FormulaStructure::FunctionApplication(
            "structurallyValid".to_string(),
            vec![Term::Variable("x".to_string(), Some("Any".to_string()))],
        );
        let implication = FormulaStructure::Implication(Box::new(well_formed), Box::new(structurally_valid));
        FormulaStructure::Universal(quantifier, Box::new(implication))
    }

    fn add_propositional_rules(&mut self) {
        // Add conjunction introduction rule
        self.add_rule(InferenceRule {
            name: "and_intro".to_string(),
            rule_type: RuleType::Introduction,
            premises: vec![
                self.create_variable_pattern("P"),
                self.create_variable_pattern("Q"),
            ],
            conclusion: self.create_conjunction_pattern("P", "Q"),
            conditions: vec![],
            priority: 8,
        });
    }

    fn add_predicate_rules(&mut self) {
        // Add universal generalization rule
        self.add_rule(InferenceRule {
            name: "universal_gen".to_string(),
            rule_type: RuleType::Introduction,
            premises: vec![self.create_variable_pattern("P")],
            conclusion: self.create_universal_pattern("x", "P"),
            conditions: vec![SideCondition::FreshVariable("x".to_string())],
            priority: 7,
        });
    }

    fn add_temporal_rules(&mut self) {
        // Add temporal always introduction
        self.add_rule(InferenceRule {
            name: "always_intro".to_string(),
            rule_type: RuleType::Introduction,
            premises: vec![self.create_variable_pattern("P")],
            conclusion: self.create_always_pattern("P"),
            conditions: vec![],
            priority: 6,
        });
    }

    // Helper methods for creating patterns
    fn create_variable_pattern(&self, var: &str) -> FormulaPattern {
        FormulaPattern {
            pattern: PatternStructure::Variable(var.to_string()),
            variables: HashMap::new(),
        }
    }

    fn create_conjunction_pattern(&self, left: &str, right: &str) -> FormulaPattern {
        FormulaPattern {
            pattern: PatternStructure::Conjunction(vec![
                PatternStructure::Variable(left.to_string()),
                PatternStructure::Variable(right.to_string()),
            ]),
            variables: HashMap::new(),
        }
    }

    fn create_universal_pattern(&self, var: &str, formula: &str) -> FormulaPattern {
        FormulaPattern {
            pattern: PatternStructure::Quantified(
                QuantifierKind::Universal,
                var.to_string(),
                Box::new(PatternStructure::Variable(formula.to_string())),
            ),
            variables: HashMap::new(),
        }
    }

    fn create_always_pattern(&self, formula: &str) -> FormulaPattern {
        FormulaPattern {
            pattern: PatternStructure::Variable(format!("□{}", formula)),
            variables: HashMap::new(),
        }
    }
}

impl Default for AxiomSystemBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Axiom {
    /// Create new axiom
    pub fn new(name: String, formula: FormulaStructure, axiom_type: AxiomType, priority: u8) -> Self {
        Self {
            name,
            formula,
            axiom_type,
            priority,
        }
    }
}

impl InferenceRule {
    /// Create new inference rule
    pub fn new(
        name: String,
        rule_type: RuleType,
        premises: Vec<FormulaPattern>,
        conclusion: FormulaPattern,
        conditions: Vec<SideCondition>,
        priority: u8,
    ) -> Self {
        Self {
            name,
            rule_type,
            premises,
            conclusion,
            conditions,
            priority,
        }
    }

    /// Check if rule can be applied
    pub fn can_apply(&self, formulas: &[FormulaStructure]) -> bool {
        formulas.len() >= self.premises.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axiom_creation() {
        let formula = FormulaStructure::Atomic(AtomicFormula {
            predicate: "P".to_string(),
            terms: vec![],
            type_signature: None,
        });
        let axiom = Axiom::new("test".to_string(), formula, AxiomType::Logical, 5);
        assert_eq!(axiom.name, "test");
        assert_eq!(axiom.axiom_type, AxiomType::Logical);
        assert_eq!(axiom.priority, 5);
    }

    #[test]
    fn test_axiom_system_builder() {
        let mut builder = AxiomSystemBuilder::new();
        builder.add_propositional_axioms();
        let (axioms, rules) = builder.build();
        assert!(!axioms.is_empty());
        assert!(!rules.is_empty());
    }

    #[test]
    fn test_inference_rule_creation() {
        let rule = InferenceRule::new(
            "test_rule".to_string(),
            RuleType::Introduction,
            vec![],
            FormulaPattern {
                pattern: PatternStructure::Variable("P".to_string()),
                variables: HashMap::new(),
            },
            vec![],
            5,
        );
        assert_eq!(rule.name, "test_rule");
        assert_eq!(rule.rule_type, RuleType::Introduction);
    }

    #[test]
    fn test_rule_application_check() {
        let rule = InferenceRule::new(
            "test".to_string(),
            RuleType::Introduction,
            vec![
                FormulaPattern {
                    pattern: PatternStructure::Variable("P".to_string()),
                    variables: HashMap::new(),
                },
            ],
            FormulaPattern {
                pattern: PatternStructure::Variable("Q".to_string()),
                variables: HashMap::new(),
            },
            vec![],
            5,
        );

        let formulas = vec![
            FormulaStructure::Atomic(AtomicFormula {
                predicate: "P".to_string(),
                terms: vec![],
                type_signature: None,
            }),
            FormulaStructure::Atomic(AtomicFormula {
                predicate: "Q".to_string(),
                terms: vec![],
                type_signature: None,
            }),
        ];

        assert!(rule.can_apply(&formulas));
        assert!(!rule.can_apply(&[]));
    }
}