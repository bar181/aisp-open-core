//! Temporal operator analysis for AISP documents
//!
//! This module handles detection, validation, and analysis of temporal operators
//! including Linear Temporal Logic (LTL) and Computation Tree Logic (CTL) operators.

use crate::ast::canonical::*;
use crate::error::*;
use std::collections::HashMap;

/// Temporal operator analyzer
pub struct TemporalOperatorAnalyzer {
    /// Mapping of operator symbols to temporal operators
    operator_symbols: HashMap<char, TemporalOperator>,
    /// Detected operators in the document
    detected_operators: Vec<OperatorInstance>,
}

/// Temporal operators in AISP
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TemporalOperator {
    /// Always/Globally (□, G)
    Always,
    /// Eventually/Finally (◊, F)
    Eventually,
    /// Next time (X)
    Next,
    /// Until (U)
    Until,
    /// Release (R)
    Release,
    /// Weak until (W)
    WeakUntil,
    /// Strong release (M)
    StrongRelease,
}

/// Instance of a temporal operator found in the document
#[derive(Debug, Clone)]
pub struct OperatorInstance {
    /// The temporal operator
    pub operator: TemporalOperator,
    /// Location in the document
    pub location: Span,
    /// Context where the operator appears
    pub context: OperatorContext,
    /// Operands (for binary operators)
    pub operands: Vec<String>,
    /// Nesting level
    pub nesting_level: usize,
}

/// Context where temporal operator appears
#[derive(Debug, Clone)]
pub enum OperatorContext {
    /// In a logical rule
    Rule(String),
    /// In a function definition
    Function(String),
    /// In a meta constraint
    MetaConstraint(String),
    /// In an evidence block
    Evidence(String),
}

/// Path quantifier types for CTL
#[derive(Debug, Clone, PartialEq)]
pub enum PathQuantifierType {
    /// For all paths (A)
    AllPaths,
    /// Exists a path (E)
    ExistsPath,
}

/// Path quantifier with temporal operator
#[derive(Debug, Clone)]
pub struct PathQuantifier {
    /// Type of path quantifier
    pub quantifier_type: PathQuantifierType,
    /// Associated temporal operator
    pub temporal_op: TemporalOperator,
    /// Formula being quantified
    pub formula: String,
    /// Location in document
    pub location: Span,
}

/// Formula complexity metrics
#[derive(Debug, Clone)]
pub struct OperatorComplexity {
    /// Number of temporal operators
    pub operator_count: usize,
    /// Maximum nesting depth
    pub max_nesting: usize,
    /// Average nesting depth
    pub avg_nesting: f64,
    /// Operator frequency distribution
    pub operator_frequency: HashMap<TemporalOperator, usize>,
    /// Complexity score (0.0-1.0)
    pub complexity_score: f64,
}

/// Operator validation result
#[derive(Debug, Clone)]
pub struct OperatorValidationResult {
    /// All detected operator instances
    pub operators: Vec<OperatorInstance>,
    /// Path quantifiers found (for CTL)
    pub path_quantifiers: Vec<PathQuantifier>,
    /// Complexity analysis
    pub complexity: OperatorComplexity,
    /// Validation errors
    pub errors: Vec<AispError>,
    /// Warnings about operator usage
    pub warnings: Vec<AispWarning>,
    /// Overall validity
    pub valid: bool,
}

impl TemporalOperatorAnalyzer {
    /// Create a new temporal operator analyzer
    pub fn new() -> Self {
        let mut operator_symbols = HashMap::new();
        operator_symbols.insert('□', TemporalOperator::Always);
        operator_symbols.insert('◊', TemporalOperator::Eventually);
        operator_symbols.insert('X', TemporalOperator::Next);
        operator_symbols.insert('U', TemporalOperator::Until);
        operator_symbols.insert('R', TemporalOperator::Release);
        operator_symbols.insert('W', TemporalOperator::WeakUntil);
        operator_symbols.insert('M', TemporalOperator::StrongRelease);

        Self {
            operator_symbols,
            detected_operators: Vec::new(),
        }
    }

    /// Analyze temporal operators in the document
    pub fn analyze_operators(&mut self, document: &AispDocument) -> OperatorValidationResult {
        self.detected_operators.clear();

        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut path_quantifiers = Vec::new();

        // Analyze each block for temporal operators
        for block in &document.blocks {
            match block {
                AispBlock::Rules(rules_block) => {
                    self.analyze_rules_operators(rules_block, &mut errors, &mut warnings);
                }
                AispBlock::Functions(functions_block) => {
                    self.analyze_function_operators(functions_block, &mut errors, &mut warnings);
                }
                AispBlock::Meta(meta_block) => {
                    self.analyze_meta_operators(meta_block, &mut errors, &mut warnings);
                }
                AispBlock::Evidence(evidence_block) => {
                    self.analyze_evidence_operators(evidence_block, &mut warnings);
                }
                _ => {} // Types block doesn't contain temporal operators
            }
        }

        // Detect path quantifiers for CTL analysis
        path_quantifiers = self.detect_path_quantifiers();

        // Calculate complexity metrics
        let complexity = self.calculate_complexity();

        // Validate operator usage
        self.validate_operator_usage(&mut errors, &mut warnings);

        let valid = errors.is_empty() && complexity.complexity_score <= 0.8;

        OperatorValidationResult {
            operators: self.detected_operators.clone(),
            path_quantifiers,
            complexity,
            errors,
            warnings,
            valid,
        }
    }

    /// Analyze temporal operators in rules
    fn analyze_rules_operators(
        &mut self,
        rules_block: &RulesBlock,
        errors: &mut Vec<AispError>,
        warnings: &mut Vec<AispWarning>,
    ) {
        for rule in &rules_block.rules {
            let rule_id = format!("rule_{:?}", rule.span);
            let rule_text = self.extract_rule_text(&rule.expression);
            
            let operators = self.extract_operators_from_text(
                &rule_text,
                OperatorContext::Rule(rule_id.clone()),
                &rule.span.clone().unwrap_or_default(),
            );

            // Check for malformed operator usage
            for operator in &operators {
                if self.is_operator_malformed(operator) {
                    errors.push(AispError::ValidationError {
                        message: format!("Invalid temporal operator {:?} in rule {}", operator.operator, rule_id),
                    });
                }
            }

            self.detected_operators.extend(operators);
        }
    }

    /// Analyze temporal operators in functions
    fn analyze_function_operators(
        &mut self,
        functions_block: &FunctionsBlock,
        errors: &mut Vec<AispError>,
        warnings: &mut Vec<AispWarning>,
    ) {
        for function in &functions_block.functions {
            let function_name = &function.name;
            let function_text = self.extract_function_text(&function.lambda);
            
            let operators = self.extract_operators_from_text(
                &function_text,
                OperatorContext::Function(function_name.clone()),
                &function.span.clone().unwrap_or_default(),
            );

            // Warn about complex temporal expressions in functions
            if operators.len() > 3 {
                warnings.push(AispWarning::warning(format!(
                    "Function '{}' contains {} temporal operators - consider simplifying",
                    function_name, operators.len()
                )));
            }

            self.detected_operators.extend(operators);
        }
    }

    /// Analyze temporal operators in meta constraints
    fn analyze_meta_operators(
        &mut self,
        meta_block: &MetaBlock,
        _errors: &mut Vec<AispError>,
        _warnings: &mut Vec<AispWarning>,
    ) {
        for (key, entry) in &meta_block.entries {
            if let MetaValue::Constraint(logical_expr) = &entry.value {
                let constraint_text = format!("{:?}", logical_expr);
                
                let operators = self.extract_operators_from_text(
                    &constraint_text,
                    OperatorContext::MetaConstraint(key.clone()),
                    &entry.span.clone().unwrap_or_default(),
                );

                self.detected_operators.extend(operators);
            }
        }
    }

    /// Analyze temporal operators in evidence blocks
    fn analyze_evidence_operators(
        &mut self,
        evidence_block: &EvidenceBlock,
        _warnings: &mut Vec<AispWarning>,
    ) {
        // Evidence block has specific fields, not general entries
        let value_text = format!("{:?}", evidence_block);
        
        let operators = self.extract_operators_from_text(
            &value_text,
            OperatorContext::Evidence("evidence".to_string()),
            &evidence_block.span.clone().unwrap_or_default(),
        );

        self.detected_operators.extend(operators);
    }

    /// Extract operators from text
    fn extract_operators_from_text(
        &self,
        text: &str,
        context: OperatorContext,
        span: &Span,
    ) -> Vec<OperatorInstance> {
        let mut operators = Vec::new();
        let mut nesting_level: usize = 0;
        
        let chars: Vec<char> = text.chars().collect();
        for (i, &ch) in chars.iter().enumerate() {
            // Track nesting level with parentheses
            match ch {
                '(' | '[' | '{' => nesting_level += 1,
                ')' | ']' | '}' => nesting_level = nesting_level.saturating_sub(1),
                _ => {}
            }

            if let Some(operator) = self.operator_symbols.get(&ch) {
                let mut operands = Vec::new();
                
                // Extract operands for binary operators
                if matches!(operator, TemporalOperator::Until | TemporalOperator::Release | 
                           TemporalOperator::WeakUntil | TemporalOperator::StrongRelease) {
                    operands = self.extract_operands(&chars, i);
                }

                operators.push(OperatorInstance {
                    operator: operator.clone(),
                    location: span.clone(),
                    context: context.clone(),
                    operands,
                    nesting_level,
                });
            }
        }

        operators
    }

    /// Extract operands for binary temporal operators
    fn extract_operands(&self, chars: &[char], operator_pos: usize) -> Vec<String> {
        let mut operands = Vec::new();
        
        // Simple operand extraction (left and right of operator)
        // In practice, this would need more sophisticated parsing
        
        // Left operand (preceding tokens)
        let left_start = operator_pos.saturating_sub(10);
        if left_start < operator_pos {
            let left_chars: String = chars[left_start..operator_pos].iter().collect();
            operands.push(left_chars.trim().to_string());
        }
        
        // Right operand (following tokens)
        let right_end = std::cmp::min(operator_pos + 10, chars.len());
        if operator_pos + 1 < right_end {
            let right_chars: String = chars[operator_pos + 1..right_end].iter().collect();
            operands.push(right_chars.trim().to_string());
        }
        
        operands
    }

    /// Extract text from rule expression
    fn extract_rule_text(&self, expression: &LogicalExpression) -> String {
        match expression {
            LogicalExpression::Variable(var) => var.clone(),
            LogicalExpression::Constant(val) => format!("{:?}", val),
            LogicalExpression::Binary { .. } => "binary_expr".to_string(),
            LogicalExpression::Unary { .. } => "unary_expr".to_string(),
            LogicalExpression::Application { .. } => "application_expr".to_string(),
            LogicalExpression::Membership { .. } => "membership_expr".to_string(),
            LogicalExpression::Temporal { .. } => "temporal_expr".to_string(),
            LogicalExpression::Raw(text) => text.clone(),
        }
    }

    /// Extract text from function lambda
    fn extract_function_text(&self, lambda: &LambdaExpression) -> String {
        self.extract_rule_text(&lambda.body)
    }

    /// Check if operator usage is malformed
    fn is_operator_malformed(&self, _operator: &OperatorInstance) -> bool {
        // TODO: Implement operator validation logic
        // This would check for syntactically incorrect usage
        false
    }

    /// Detect path quantifiers for CTL
    fn detect_path_quantifiers(&self) -> Vec<PathQuantifier> {
        let mut quantifiers = Vec::new();
        
        // Look for patterns like "A□", "E◊", etc.
        for operator in &self.detected_operators {
            // Check if preceded by path quantifier
            // This is a simplified detection - real implementation would parse more carefully
            
            quantifiers.push(PathQuantifier {
                quantifier_type: PathQuantifierType::AllPaths, // Simplified
                temporal_op: operator.operator.clone(),
                formula: format!("A{}", operator.operator),
                location: operator.location.clone(),
            });
        }
        
        quantifiers
    }

    /// Calculate operator complexity metrics
    fn calculate_complexity(&self) -> OperatorComplexity {
        if self.detected_operators.is_empty() {
            return OperatorComplexity {
                operator_count: 0,
                max_nesting: 0,
                avg_nesting: 0.0,
                operator_frequency: HashMap::new(),
                complexity_score: 0.0,
            };
        }

        let operator_count = self.detected_operators.len();
        
        let max_nesting = self.detected_operators
            .iter()
            .map(|op| op.nesting_level)
            .max()
            .unwrap_or(0);
            
        let total_nesting: usize = self.detected_operators
            .iter()
            .map(|op| op.nesting_level)
            .sum();
        let avg_nesting = total_nesting as f64 / operator_count as f64;

        let mut operator_frequency = HashMap::new();
        for operator in &self.detected_operators {
            *operator_frequency.entry(operator.operator.clone()).or_insert(0) += 1;
        }

        // Calculate complexity score based on various factors
        let nesting_factor = (max_nesting as f64 / 10.0).min(1.0);
        let count_factor = (operator_count as f64 / 20.0).min(1.0);
        let diversity_factor = operator_frequency.len() as f64 / 7.0; // 7 total operator types
        
        let complexity_score = (nesting_factor + count_factor + diversity_factor) / 3.0;

        OperatorComplexity {
            operator_count,
            max_nesting,
            avg_nesting,
            operator_frequency,
            complexity_score,
        }
    }

    /// Validate operator usage patterns
    fn validate_operator_usage(&self, errors: &mut Vec<AispError>, warnings: &mut Vec<AispWarning>) {
        // Check for common anti-patterns
        
        // 1. Excessive nesting
        let high_nesting = self.detected_operators
            .iter()
            .filter(|op| op.nesting_level > 5)
            .count();
        
        if high_nesting > 0 {
            warnings.push(AispWarning::warning(format!(
                "{} operators with high nesting level (>5) - consider simplifying",
                high_nesting
            )));
        }

        // 2. Missing operands for binary operators
        for operator in &self.detected_operators {
            if matches!(operator.operator, 
                       TemporalOperator::Until | TemporalOperator::Release | 
                       TemporalOperator::WeakUntil | TemporalOperator::StrongRelease) 
                && operator.operands.len() < 2 {
                
                errors.push(AispError::ValidationError {
                    message: format!("Binary operator {} requires two operands", operator.operator),
                });
            }
        }

        // 3. Conflicting temporal requirements
        self.check_temporal_conflicts(warnings);
    }

    /// Check for conflicting temporal requirements
    fn check_temporal_conflicts(&self, warnings: &mut Vec<AispWarning>) {
        // Look for potentially conflicting operators
        let always_count = self.detected_operators
            .iter()
            .filter(|op| op.operator == TemporalOperator::Always)
            .count();
            
        let eventually_count = self.detected_operators
            .iter()
            .filter(|op| op.operator == TemporalOperator::Eventually)
            .count();

        if always_count > 0 && eventually_count > always_count * 2 {
            warnings.push(AispWarning::warning(
                "Many 'eventually' operators with few 'always' operators - check for potential conflicts"
                    .to_string(),
            ));
        }
    }

    /// Get operator symbol as string
    pub fn operator_symbol(&self, operator: &TemporalOperator) -> char {
        for (&symbol, op) in &self.operator_symbols {
            if op == operator {
                return symbol;
            }
        }
        '?' // Unknown operator
    }
}

impl std::fmt::Display for TemporalOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Always => write!(f, "□"),
            Self::Eventually => write!(f, "◊"),
            Self::Next => write!(f, "X"),
            Self::Until => write!(f, "U"),
            Self::Release => write!(f, "R"),
            Self::WeakUntil => write!(f, "W"),
            Self::StrongRelease => write!(f, "M"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_creation() {
        let analyzer = TemporalOperatorAnalyzer::new();
        assert_eq!(analyzer.operator_symbols.len(), 7);
        assert!(analyzer.operator_symbols.contains_key(&'□'));
        assert!(analyzer.operator_symbols.contains_key(&'◊'));
    }

    #[test]
    fn test_operator_display() {
        assert_eq!(format!("{}", TemporalOperator::Always), "□");
        assert_eq!(format!("{}", TemporalOperator::Eventually), "◊");
        assert_eq!(format!("{}", TemporalOperator::Next), "X");
        assert_eq!(format!("{}", TemporalOperator::Until), "U");
    }

    #[test]
    fn test_operator_symbol_lookup() {
        let analyzer = TemporalOperatorAnalyzer::new();
        assert_eq!(analyzer.operator_symbol(&TemporalOperator::Always), '□');
        assert_eq!(analyzer.operator_symbol(&TemporalOperator::Eventually), '◊');
        assert_eq!(analyzer.operator_symbol(&TemporalOperator::Next), 'X');
    }

    #[test]
    fn test_extract_operators_from_text() {
        let analyzer = TemporalOperatorAnalyzer::new();
        let text = "□(p → ◊q)";
        let context = OperatorContext::Rule("test".to_string());
        let span = Span::new(1, 1, 1, 10);
        
        let operators = analyzer.extract_operators_from_text(text, context, &span);
        assert_eq!(operators.len(), 2);
        assert_eq!(operators[0].operator, TemporalOperator::Always);
        assert_eq!(operators[1].operator, TemporalOperator::Eventually);
    }

    #[test]
    fn test_complexity_calculation() {
        let mut analyzer = TemporalOperatorAnalyzer::new();
        
        // Add some test operators
        analyzer.detected_operators.push(OperatorInstance {
            operator: TemporalOperator::Always,
            location: Span::new(1, 1, 1, 5),
            context: OperatorContext::Rule("test1".to_string()),
            operands: vec![],
            nesting_level: 1,
        });
        
        analyzer.detected_operators.push(OperatorInstance {
            operator: TemporalOperator::Eventually,
            location: Span::new(1, 6, 1, 10),
            context: OperatorContext::Rule("test1".to_string()),
            operands: vec![],
            nesting_level: 2,
        });

        let complexity = analyzer.calculate_complexity();
        assert_eq!(complexity.operator_count, 2);
        assert_eq!(complexity.max_nesting, 2);
        assert_eq!(complexity.avg_nesting, 1.5);
        assert_eq!(complexity.operator_frequency.len(), 2);
    }

    #[test]
    fn test_empty_document_analysis() {
        let mut analyzer = TemporalOperatorAnalyzer::new();
        let document = AispDocument {
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
            blocks: vec![],
            span: Span::new(1, 1, 1, 1),
        };

        let result = analyzer.analyze_operators(&document);
        assert!(result.operators.is_empty());
        assert_eq!(result.complexity.operator_count, 0);
        assert!(result.valid);
    }

    #[test]
    fn test_operator_validation() {
        let mut analyzer = TemporalOperatorAnalyzer::new();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
        // Add an operator with missing operands
        analyzer.detected_operators.push(OperatorInstance {
            operator: TemporalOperator::Until,
            location: Span::new(1, 1, 1, 5),
            context: OperatorContext::Rule("test".to_string()),
            operands: vec!["p".to_string()], // Missing second operand
            nesting_level: 1,
        });

        analyzer.validate_operator_usage(&mut errors, &mut warnings);
        assert!(!errors.is_empty()); // Should detect missing operand
    }
}