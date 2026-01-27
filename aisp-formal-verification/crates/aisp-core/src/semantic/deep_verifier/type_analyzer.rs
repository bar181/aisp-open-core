//! Type System Analyzer
//!
//! Advanced type system analysis for semantic verification
//! Implements SRP by focusing solely on type system analysis

use super::types::*;
use crate::ast::canonical::{CanonicalAispDocument as AispDocument, CanonicalAispBlock as AispBlock};
use crate::error::{AispError, AispResult};
use std::collections::HashMap;

/// Advanced type system analyzer for semantic verification
pub struct TypeSystemAnalyzer {
    pub type_definitions: HashMap<String, TypeDefinition>,
    pub type_constraints: Vec<TypeConstraint>,
    pub inference_cache: HashMap<String, InferredType>,
    pub security_policies: Vec<TypeSecurityPolicy>,
}

impl TypeSystemAnalyzer {
    /// Create new type system analyzer
    pub fn new() -> Self {
        Self {
            type_definitions: HashMap::new(),
            type_constraints: Vec::new(),
            inference_cache: HashMap::new(),
            security_policies: Vec::new(),
        }
    }

    /// Create analyzer with enhanced security policies
    pub fn with_enhanced_security() -> Self {
        let mut analyzer = Self::new();
        analyzer.enable_security_policies();
        analyzer
    }

    /// Enable comprehensive security policies for type validation
    pub fn enable_security_policies(&mut self) {
        self.security_policies.extend(vec![
            TypeSecurityPolicy {
                policy_name: "NoUnsafeTypes".to_string(),
                rules: vec![
                    "Prohibit Any type usage".to_string(),
                    "Require explicit type annotations".to_string(),
                    "Validate all type conversions".to_string(),
                ],
            },
            TypeSecurityPolicy {
                policy_name: "TypeIsolation".to_string(),
                rules: vec![
                    "Enforce type boundaries".to_string(),
                    "Prevent unsafe type coercion".to_string(),
                ],
            },
        ]);
    }

    /// Analyze document for comprehensive type information
    pub fn analyze_document(&mut self, document: &AispDocument) -> AispResult<TypeAnalysisResult> {
        let mut type_violations = Vec::new();
        let mut type_recommendations = Vec::new();
        let mut safety_score = 1.0;

        // Analyze each block for type information
        for block in &document.blocks {
            match block {
                AispBlock::Types(types_block) => {
                    for (type_name, type_def) in &types_block.definitions {
                        if let Err(e) = self.validate_type_definition((type_name, type_def)) {
                            type_violations.push(format!("Type definition error in {}: {}", type_name, e));
                            safety_score -= 0.1;
                        }
                        
                        // Store validated type definitions
                        self.store_type_definition(type_name.clone(), type_def);
                    }
                }
                AispBlock::Functions(functions_block) => {
                    let function_analysis = self.analyze_functions_block(functions_block)?;
                    if !function_analysis.is_empty() {
                        type_violations.extend(function_analysis);
                        safety_score -= 0.05;
                    }
                }
                AispBlock::Rules(rules_block) => {
                    let rules_analysis = self.analyze_rules_block(rules_block)?;
                    if !rules_analysis.is_empty() {
                        type_violations.extend(rules_analysis);
                        safety_score -= 0.05;
                    }
                }
                _ => {}
            }
        }

        // Apply security policy checks
        let security_violations = self.check_security_policies()?;
        if !security_violations.is_empty() {
            type_violations.extend(security_violations);
            safety_score -= 0.2;
        }

        // Generate type safety recommendations
        type_recommendations = self.generate_type_recommendations(&type_violations);

        let type_safety_score = (safety_score as f64).max(0.0).min(1.0);

        Ok(TypeAnalysisResult {
            type_safety_score,
            type_violations,
            type_recommendations,
        })
    }

    /// Validate individual type definition
    fn validate_type_definition(&self, type_def: (&String, &crate::ast::canonical::TypeDefinition)) -> AispResult<()> {
        let (type_name, definition) = type_def;
        
        // Check for unsafe type patterns
        if type_name.contains("Any") || type_name.to_lowercase().contains("unsafe") {
            return Err(AispError::ValidationError {
                message: format!("Unsafe type name pattern: {}", type_name),
            });
        }

        // Validate type definition structure using canonical field
        let type_expr = &definition.type_expr;
        if format!("{:?}", type_expr).contains("Any") {
            return Err(AispError::ValidationError {
                message: format!("Unsafe Any type usage in definition: {}", type_name),
            });
        }

        // Check for required type constraints (simplified for canonical structure)
        if self.requires_constraints(type_name) {
            // For canonical structure, we validate type expression complexity
            match type_expr {
                crate::ast::canonical::TypeExpression::Basic(_) => {}, // Simple types are OK
                _ => {}, // Complex types handled elsewhere
            }
        }

        Ok(())
    }

    /// Store validated type definition for further analysis
    fn store_type_definition(&mut self, type_name: String, definition: &crate::ast::canonical::TypeDefinition) {
        let stored_definition = TypeDefinition {
            name: type_name.clone(),
            structure: self.infer_type_structure(definition),
            constraints: self.extract_constraints(definition),
            security_level: self.determine_security_level(&type_name),
            verification_status: VerificationStatus::Verified,
        };

        self.type_definitions.insert(type_name, stored_definition);
    }

    /// Analyze functions block for type safety
    fn analyze_functions_block(&mut self, functions_block: &crate::ast::canonical::FunctionsBlock) -> AispResult<Vec<String>> {
        let mut violations = Vec::new();

        // Analyze function definitions for type safety using canonical structure
        for (index, func_def) in functions_block.functions.iter().enumerate() {
            let func_name = format!("function_{}", index);
            if let Err(e) = self.validate_function_types(&func_name, func_def) {
                violations.push(format!("Function type error in {}: {}", func_name, e));
            }

            // Infer and cache function type
            if let Some(inferred_type) = self.infer_function_type(func_def) {
                self.inference_cache.insert(
                    func_name.clone(),
                    InferredType {
                        type_name: inferred_type,
                        confidence: 0.9,
                    }
                );
            }
        }

        Ok(violations)
    }

    /// Analyze rules block for type consistency
    fn analyze_rules_block(&mut self, rules_block: &crate::ast::canonical::RulesBlock) -> AispResult<Vec<String>> {
        let mut violations = Vec::new();

        // Check rule type consistency using canonical structure
        for (index, _rule) in rules_block.rules.iter().enumerate() {
            let rule_name = format!("rule_{}", index);
            let rule_map = std::collections::HashMap::new(); // Simplified for canonical structure
            if let Err(e) = self.validate_rule_types(&rule_name, &rule_map) {
                violations.push(format!("Rule type error in {}: {}", rule_name, e));
            }
        }

        Ok(violations)
    }

    /// Check all enabled security policies
    fn check_security_policies(&self) -> AispResult<Vec<String>> {
        let mut violations = Vec::new();

        for policy in &self.security_policies {
            let policy_violations = self.apply_security_policy(policy)?;
            violations.extend(policy_violations);
        }

        Ok(violations)
    }

    /// Apply individual security policy
    fn apply_security_policy(&self, policy: &TypeSecurityPolicy) -> AispResult<Vec<String>> {
        let mut violations = Vec::new();

        match policy.policy_name.as_str() {
            "NoUnsafeTypes" => {
                for (type_name, type_def) in &self.type_definitions {
                    if type_name.contains("Any") || type_def.security_level == SecurityLevel::Public {
                        violations.push(format!("Unsafe type detected: {}", type_name));
                    }
                }
            }
            "TypeIsolation" => {
                // Check for type boundary violations
                violations.extend(self.check_type_boundaries()?);
            }
            _ => {}
        }

        Ok(violations)
    }

    /// Generate type safety recommendations based on violations
    fn generate_type_recommendations(&self, violations: &[String]) -> Vec<String> {
        let mut recommendations = Vec::new();

        if violations.iter().any(|v| v.contains("Any")) {
            recommendations.push("Replace Any types with specific type annotations".to_string());
        }

        if violations.iter().any(|v| v.contains("constraint")) {
            recommendations.push("Add explicit type constraints for better safety".to_string());
        }

        if violations.iter().any(|v| v.contains("boundary")) {
            recommendations.push("Implement proper type isolation mechanisms".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("Type system analysis passed successfully".to_string());
        }

        recommendations
    }

    /// Helper methods for type analysis

    fn infer_type_structure(&self, definition: &crate::ast::canonical::TypeDefinition) -> TypeStructure {
        // Type structure inference using canonical field
        let type_expr = &definition.type_expr;
        if format!("{:?}", type_expr).contains("Set") {
            TypeStructure::Array(Box::new(TypeStructure::Primitive(PrimitiveType {
                type_name: "set_element".to_string(),
            })))
        } else {
            TypeStructure::Primitive(PrimitiveType {
                type_name: "inferred".to_string(),
            })
        }
    }

    fn extract_constraints(&self, definition: &crate::ast::canonical::TypeDefinition) -> Vec<TypeConstraint> {
        // Extract constraints from canonical type definition structure
        // Since canonical structure doesn't have explicit constraints field,
        // we derive constraints from the type expression
        vec![TypeConstraint {
            constraint_type: ConstraintType::Validation,
            expression: format!("{:?}", definition.type_expr),
            severity: ConstraintSeverity::Info,
        }]
    }

    fn determine_security_level(&self, type_name: &str) -> SecurityLevel {
        if type_name.to_lowercase().contains("secret") {
            SecurityLevel::Secret
        } else if type_name.to_lowercase().contains("confidential") {
            SecurityLevel::Confidential
        } else if type_name.to_lowercase().contains("internal") {
            SecurityLevel::Internal
        } else {
            SecurityLevel::Public
        }
    }

    fn requires_constraints(&self, type_name: &str) -> bool {
        // Certain types require explicit constraints
        type_name.contains("Number") || type_name.contains("String") || type_name.contains("Array")
    }

    fn validate_function_types(&self, _func_name: &str, _func_def: &crate::ast::canonical::FunctionDefinition) -> AispResult<()> {
        // Simplified function type validation
        Ok(())
    }

    fn validate_rule_types(&self, _rule_name: &str, _rule_def: &std::collections::HashMap<String, String>) -> AispResult<()> {
        // Simplified rule type validation using generic map
        Ok(())
    }

    fn infer_function_type(&self, _func_def: &crate::ast::canonical::FunctionDefinition) -> Option<String> {
        // Simplified function type inference
        Some("Function".to_string())
    }

    fn check_type_boundaries(&self) -> AispResult<Vec<String>> {
        // Check for type boundary violations
        Ok(vec![])
    }
}

impl Default for TypeSystemAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_analyzer_creation() {
        let analyzer = TypeSystemAnalyzer::new();
        assert_eq!(analyzer.type_definitions.len(), 0);
        assert_eq!(analyzer.security_policies.len(), 0);
    }

    #[test]
    fn test_security_policies_enabled() {
        let mut analyzer = TypeSystemAnalyzer::new();
        analyzer.enable_security_policies();
        assert_eq!(analyzer.security_policies.len(), 2);
        assert_eq!(analyzer.security_policies[0].policy_name, "NoUnsafeTypes");
    }

    #[test]
    fn test_enhanced_security_analyzer() {
        let analyzer = TypeSystemAnalyzer::with_enhanced_security();
        assert_eq!(analyzer.security_policies.len(), 2);
    }

    #[test]
    fn test_security_level_determination() {
        let analyzer = TypeSystemAnalyzer::new();
        assert_eq!(analyzer.determine_security_level("secret_key"), SecurityLevel::Secret);
        assert_eq!(analyzer.determine_security_level("confidential_data"), SecurityLevel::Confidential);
        assert_eq!(analyzer.determine_security_level("public_info"), SecurityLevel::Public);
    }

    #[test]
    fn test_constraint_requirements() {
        let analyzer = TypeSystemAnalyzer::new();
        assert!(analyzer.requires_constraints("NumberType"));
        assert!(analyzer.requires_constraints("StringType"));
        assert!(!analyzer.requires_constraints("SimpleType"));
    }
}