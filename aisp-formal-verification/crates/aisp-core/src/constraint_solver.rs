//! Constraint satisfaction and solving for AISP documents
//!
//! This module handles constraint analysis, satisfaction checking,
//! and conflict resolution for relational logic constraints.

use crate::ast::*;
use crate::error::*;
use crate::conflict_types::ConflictSeverity;
use std::collections::{HashMap, HashSet, VecDeque};

/// Constraint solver for relational logic
pub struct ConstraintSolver {
    /// Constraint definitions
    constraints: Vec<Constraint>,
    /// Variable domains
    domains: HashMap<String, Vec<String>>,
    /// Current variable assignments
    assignments: HashMap<String, String>,
}

/// A constraint in the relational system
#[derive(Debug, Clone)]
pub struct Constraint {
    /// Unique constraint identifier
    pub id: String,
    /// Variables involved in the constraint
    pub variables: Vec<String>,
    /// Constraint type
    pub constraint_type: ConstraintType,
    /// Priority level
    pub priority: ConstraintPriority,
    /// Source location
    pub span: Span,
}

/// Types of constraints
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintType {
    /// Set membership constraint (x ∈ S)
    Membership { variable: String, set: String },
    /// Equality constraint (x = y)
    Equality { left: String, right: String },
    /// Inequality constraint (x ≠ y)
    Inequality { left: String, right: String },
    /// Subset constraint (A ⊆ B)
    Subset { subset: String, superset: String },
    /// Function constraint (f(x) = y)
    Function { function: String, input: String, output: String },
    /// Custom logical constraint
    Logical { expression: String },
}

/// Constraint priority levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstraintPriority {
    /// Low priority constraint
    Low = 1,
    /// Medium priority constraint
    Medium = 2,
    /// High priority constraint
    High = 3,
    /// Critical constraint (must be satisfied)
    Critical = 4,
}

/// Result of constraint analysis
#[derive(Debug, Clone)]
pub struct ConstraintAnalysisResult {
    /// All constraints found
    pub constraints: Vec<Constraint>,
    /// Successfully satisfied constraints
    pub satisfied: Vec<String>,
    /// Unsatisfied constraints
    pub unsatisfied: Vec<String>,
    /// Detected conflicts between constraints
    pub conflicts: Vec<ConstraintConflict>,
    /// Overall satisfaction score (0.0-1.0)
    pub satisfaction_score: f64,
}

/// Conflict between constraints
#[derive(Debug, Clone)]
pub struct ConstraintConflict {
    /// Conflicting constraint IDs
    pub constraint_ids: Vec<String>,
    /// Severity of the conflict
    pub severity: ConflictSeverity,
    /// Description of the conflict
    pub description: String,
    /// Suggested resolution
    pub resolution: Option<String>,
}


impl ConstraintSolver {
    /// Create a new constraint solver
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            domains: HashMap::new(),
            assignments: HashMap::new(),
        }
    }

    /// Extract constraints from an AISP document
    pub fn extract_constraints(&mut self, document: &AispDocument) -> ConstraintAnalysisResult {
        self.constraints.clear();
        self.domains.clear();
        
        // Extract constraints from different blocks
        self.extract_type_constraints(document);
        self.extract_rule_constraints(document);
        self.extract_function_constraints(document);
        self.extract_meta_constraints(document);
        
        // Analyze constraint satisfaction
        self.analyze_satisfaction()
    }

    /// Extract type-related constraints
    fn extract_type_constraints(&mut self, document: &AispDocument) {
        for block in &document.blocks {
            if let AispBlock::Types(types_block) = block {
                for (name, definition) in &types_block.definitions {
                    // Add domain constraints for enumeration types
                    if let TypeExpression::Enumeration(values) = &definition.type_expr {
                        self.domains.insert(name.clone(), values.clone());
                        
                        // Create membership constraints for each value
                        for value in values {
                            let constraint = Constraint {
                                id: format!("type_membership_{}_{}", name, value),
                                variables: vec![value.clone(), name.clone()],
                                constraint_type: ConstraintType::Membership {
                                    variable: value.clone(),
                                    set: name.clone(),
                                },
                                priority: ConstraintPriority::High,
                                span: definition.span.clone(),
                            };
                            self.constraints.push(constraint);
                        }
                    }
                }
            }
        }
    }

    /// Extract rule-based constraints
    fn extract_rule_constraints(&mut self, document: &AispDocument) {
        for block in &document.blocks {
            if let AispBlock::Rules(rules_block) = block {
                for (idx, rule) in rules_block.rules.iter().enumerate() {
                    // Parse quantified constraints
                    if let Some(quantifier) = &rule.quantifier {
                        let constraint = Constraint {
                            id: format!("rule_constraint_{}", idx),
                            variables: vec![quantifier.variable.clone()],
                            constraint_type: ConstraintType::Logical {
                                expression: self.extract_logical_expression(&rule.expression),
                            },
                            priority: ConstraintPriority::Medium,
                            span: rule.span.clone(),
                        };
                        self.constraints.push(constraint);
                    }
                }
            }
        }
    }

    /// Extract function-related constraints
    fn extract_function_constraints(&mut self, document: &AispDocument) {
        for block in &document.blocks {
            if let AispBlock::Functions(functions_block) = block {
                for (name, function) in &functions_block.functions {
                    // Create function application constraints
                    for param in &function.lambda.parameters {
                        let constraint = Constraint {
                            id: format!("function_{}_{}", name, param),
                            variables: vec![name.clone(), param.clone()],
                            constraint_type: ConstraintType::Function {
                                function: name.clone(),
                                input: param.clone(),
                                output: "result".to_string(),
                            },
                            priority: ConstraintPriority::Medium,
                            span: function.span.clone(),
                        };
                        self.constraints.push(constraint);
                    }
                }
            }
        }
    }

    /// Extract meta-level constraints
    fn extract_meta_constraints(&mut self, document: &AispDocument) {
        for block in &document.blocks {
            if let AispBlock::Meta(meta_block) = block {
                for (key, entry) in &meta_block.entries {
                    if let MetaValue::Constraint(logical_expr) = &entry.value {
                        let constraint = Constraint {
                            id: format!("meta_constraint_{}", key),
                            variables: vec![key.clone()],
                            constraint_type: ConstraintType::Logical {
                                expression: self.extract_logical_expression(logical_expr),
                            },
                            priority: ConstraintPriority::Critical,
                            span: entry.span.clone(),
                        };
                        self.constraints.push(constraint);
                    }
                }
            }
        }
    }

    /// Extract logical expression as string
    fn extract_logical_expression(&self, expr: &LogicalExpression) -> String {
        match expr {
            LogicalExpression::Variable(var) => var.clone(),
            LogicalExpression::Constant(val) => format!("{:?}", val),
            LogicalExpression::Binary { .. } => "binary_expr".to_string(),
            LogicalExpression::Unary { .. } => "unary_expr".to_string(),
            LogicalExpression::Application { .. } => "application_expr".to_string(),
            LogicalExpression::Membership { .. } => "membership_expr".to_string(),
            LogicalExpression::Temporal { .. } => "temporal_expr".to_string(),
            // Note: Lambda is not in LogicalExpression enum, it's in LambdaExpression
        }
    }

    /// Analyze constraint satisfaction
    fn analyze_satisfaction(&self) -> ConstraintAnalysisResult {
        let mut satisfied = Vec::new();
        let mut unsatisfied = Vec::new();
        let mut conflicts = Vec::new();

        // Simple satisfaction check (in practice, this would use CSP solving)
        for constraint in &self.constraints {
            if self.is_constraint_satisfied(constraint) {
                satisfied.push(constraint.id.clone());
            } else {
                unsatisfied.push(constraint.id.clone());
            }
        }

        // Detect conflicts between constraints
        conflicts.extend(self.detect_conflicts());

        let satisfaction_score = if self.constraints.is_empty() {
            1.0
        } else {
            satisfied.len() as f64 / self.constraints.len() as f64
        };

        ConstraintAnalysisResult {
            constraints: self.constraints.clone(),
            satisfied,
            unsatisfied,
            conflicts,
            satisfaction_score,
        }
    }

    /// Check if a constraint is satisfied
    fn is_constraint_satisfied(&self, constraint: &Constraint) -> bool {
        match &constraint.constraint_type {
            ConstraintType::Membership { variable, set } => {
                // Check if variable is in the domain of the set
                if let Some(domain) = self.domains.get(set) {
                    domain.contains(variable)
                } else {
                    false
                }
            }
            ConstraintType::Equality { left, right } => {
                // Check if both variables have the same assignment
                self.assignments.get(left) == self.assignments.get(right)
            }
            ConstraintType::Inequality { left, right } => {
                // Check if variables have different assignments
                self.assignments.get(left) != self.assignments.get(right)
            }
            _ => true, // Simplified - assume other constraints are satisfied
        }
    }

    /// Detect conflicts between constraints
    fn detect_conflicts(&self) -> Vec<ConstraintConflict> {
        let mut conflicts = Vec::new();

        // Check for conflicting membership constraints
        for i in 0..self.constraints.len() {
            for j in (i + 1)..self.constraints.len() {
                if let Some(conflict) = self.check_constraint_conflict(&self.constraints[i], &self.constraints[j]) {
                    conflicts.push(conflict);
                }
            }
        }

        conflicts
    }

    /// Check if two constraints conflict
    fn check_constraint_conflict(&self, c1: &Constraint, c2: &Constraint) -> Option<ConstraintConflict> {
        match (&c1.constraint_type, &c2.constraint_type) {
            (
                ConstraintType::Equality { left: l1, right: r1 },
                ConstraintType::Inequality { left: l2, right: r2 }
            ) => {
                if (l1 == l2 && r1 == r2) || (l1 == r2 && r1 == l2) {
                    Some(ConstraintConflict {
                        constraint_ids: vec![c1.id.clone(), c2.id.clone()],
                        severity: ConflictSeverity::Error,
                        description: format!("Equality constraint {} conflicts with inequality constraint {}", c1.id, c2.id),
                        resolution: Some("Remove one of the conflicting constraints".to_string()),
                    })
                } else {
                    None
                }
            }
            _ => None, // TODO: Check other types of conflicts
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_document() -> AispDocument {
        let mut type_definitions = HashMap::new();
        type_definitions.insert("State".to_string(), TypeDefinition {
            name: "State".to_string(),
            type_expr: TypeExpression::Enumeration(vec!["A".to_string(), "B".to_string()]),
            span: Span::new(1, 1, 1, 10),
        });

        AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "test".to_string(),
                date: "2026-01-25".to_string(),
                metadata: None,
            },
            metadata: DocumentMetadata {
                domain: None,
                protocol: None,
            },
            blocks: vec![AispBlock::Types(TypesBlock {
                definitions: type_definitions,
                span: Span::new(1, 1, 3, 1),
            })],
            span: Span::new(1, 1, 10, 1),
        }
    }

    #[test]
    fn test_constraint_extraction() {
        let mut solver = ConstraintSolver::new();
        let document = create_test_document();
        
        let result = solver.extract_constraints(&document);
        
        assert!(!result.constraints.is_empty());
        assert!(result.satisfaction_score >= 0.0);
        assert!(result.satisfaction_score <= 1.0);
    }

    #[test]
    fn test_membership_constraint() {
        let solver = ConstraintSolver::new();
        let constraint = Constraint {
            id: "test".to_string(),
            variables: vec!["A".to_string(), "State".to_string()],
            constraint_type: ConstraintType::Membership {
                variable: "A".to_string(),
                set: "State".to_string(),
            },
            priority: ConstraintPriority::High,
            span: Span::new(1, 1, 1, 10),
        };

        // Without domain setup, should be unsatisfied
        assert!(!solver.is_constraint_satisfied(&constraint));
    }

    #[test]
    fn test_conflict_detection() {
        let solver = ConstraintSolver::new();
        let c1 = Constraint {
            id: "eq".to_string(),
            variables: vec!["x".to_string(), "y".to_string()],
            constraint_type: ConstraintType::Equality {
                left: "x".to_string(),
                right: "y".to_string(),
            },
            priority: ConstraintPriority::High,
            span: Span::new(1, 1, 1, 10),
        };
        let c2 = Constraint {
            id: "neq".to_string(),
            variables: vec!["x".to_string(), "y".to_string()],
            constraint_type: ConstraintType::Inequality {
                left: "x".to_string(),
                right: "y".to_string(),
            },
            priority: ConstraintPriority::High,
            span: Span::new(2, 1, 2, 10),
        };

        let conflict = solver.check_constraint_conflict(&c1, &c2);
        assert!(conflict.is_some());
        
        if let Some(conflict) = conflict {
            assert_eq!(conflict.severity, ConflictSeverity::Error);
            assert_eq!(conflict.constraint_ids.len(), 2);
        }
    }

    #[test]
    fn test_constraint_priorities() {
        assert!(ConstraintPriority::Critical > ConstraintPriority::High);
        assert!(ConstraintPriority::High > ConstraintPriority::Medium);
        assert!(ConstraintPriority::Medium > ConstraintPriority::Low);
    }
}